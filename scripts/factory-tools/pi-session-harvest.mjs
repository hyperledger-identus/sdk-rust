#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { randomUUID } from "node:crypto";
import {
  chmodSync,
  existsSync,
  lstatSync,
  readdirSync,
  readFileSync,
  realpathSync,
  renameSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { TextDecoder } from "node:util";
import { fileURLToPath } from "node:url";
import { parseJsonWithoutDuplicates } from "./strict-json.mjs";
import { validateInvocationFile } from "./supervisor.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const policy = parseJsonWithoutDuplicates(readFileSync(path.join(root, ".factory-policy.json"), "utf8"));
const reasons = new Set([
  "not-measured", "source-missing", "source-malformed", "source-unsafe",
  "unsupported-version", "incomplete-usage", "incomplete-events", "not-applicable",
]);
const aggregateKeys = [
  "schemaVersion", "sessions", "turns", "toolCalls", "inputTokens", "outputTokens",
  "cacheReadTokens", "cacheWriteTokens",
];
const counterKeys = aggregateKeys.filter((key) => key !== "schemaVersion");

function fail(message) {
  throw new Error(message);
}

function nonnegativeInteger(value) {
  return Number.isSafeInteger(value) && value >= 0;
}

function measurement(value, unavailableReason = null) {
  return { value, unavailableReason };
}

function unavailable(reason) {
  return measurement(null, reason);
}

function exactKeys(record, expected) {
  if (!record || typeof record !== "object" || Array.isArray(record)) return false;
  const actual = Object.keys(record).sort();
  const sortedExpected = [...expected].sort();
  return actual.length === sortedExpected.length && actual.every((key, index) => key === sortedExpected[index]);
}

export function validateUsageAggregate(record) {
  const errors = [];
  if (!exactKeys(record, aggregateKeys)) return { ok: false, errors: ["usage aggregate has an unknown or missing field"] };
  if (record.schemaVersion !== 1) errors.push("usage aggregate schema is invalid");
  for (const key of counterKeys) {
    const value = record[key];
    if (!exactKeys(value, ["value", "unavailableReason"])) {
      errors.push(`usage aggregate ${key} is malformed`);
      continue;
    }
    if (value.value === null) {
      if (!reasons.has(value.unavailableReason)) errors.push(`usage aggregate ${key} lacks a closed unavailable reason`);
    } else if (!nonnegativeInteger(value.value) || value.unavailableReason !== null) {
      errors.push(`usage aggregate ${key} is not an exact counter`);
    }
  }
  return { ok: errors.length === 0, errors };
}

function assertWithin(candidate, boundary) {
  const relative = path.relative(boundary, candidate);
  if (relative === "" || (!relative.startsWith(`..${path.sep}`) && relative !== ".." && !path.isAbsolute(relative))) return;
  fail("usage source is outside the private run");
}

function assertNoSymlinkComponents(candidate, boundary) {
  const resolvedBoundary = realpathSync(boundary);
  assertWithin(path.resolve(candidate), resolvedBoundary);
  const relative = path.relative(resolvedBoundary, path.resolve(candidate));
  let current = resolvedBoundary;
  for (const component of relative.split(path.sep).filter(Boolean)) {
    current = path.join(current, component);
    if (!existsSync(current)) fail("usage source is missing");
    if (lstatSync(current).isSymbolicLink()) fail("usage source is unsafe");
  }
}

function readPrivateSource(file, runDirectory, maximumBytes) {
  if (!path.isAbsolute(file) || path.resolve(file) !== file) fail("usage source path must be canonical and absolute");
  const boundary = realpathSync(runDirectory);
  assertWithin(file, boundary);
  assertNoSymlinkComponents(file, boundary);
  const info = lstatSync(file);
  if (!info.isFile() || info.isSymbolicLink() || (info.mode & 0o077) !== 0) fail("usage source is unsafe");
  if (info.size > maximumBytes) fail("usage source exceeds its byte bound");
  return readFileSync(file);
}

function decodeLines(bytes) {
  let text;
  try {
    text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    fail("usage source is malformed");
  }
  const lines = text.split("\n");
  if (lines.at(-1) === "") lines.pop();
  if (lines.length === 0 || lines.length > 100_000 || lines.some((line) => line.length === 0)) fail("usage source is malformed");
  return lines;
}

function safeAdd(total, value) {
  if (!nonnegativeInteger(value) || total > Number.MAX_SAFE_INTEGER - value) fail("usage counter is malformed");
  return total + value;
}

function sessionUnavailable(reason) {
  return {
    sessions: unavailable(reason),
    inputTokens: unavailable(reason),
    outputTokens: unavailable(reason),
    cacheReadTokens: unavailable(reason),
    cacheWriteTokens: unavailable(reason),
  };
}

function parseSession(bytes) {
  let lines;
  try {
    lines = decodeLines(bytes);
    const header = parseJsonWithoutDuplicates(lines[0]);
    if (!header || header.type !== "session") return sessionUnavailable("source-malformed");
    if (header.version !== 3) return sessionUnavailable("unsupported-version");
    const totals = { input: 0, output: 0, cacheRead: 0, cacheWrite: 0 };
    let usageComplete = true;
    for (const line of lines.slice(1)) {
      const entry = parseJsonWithoutDuplicates(line);
      if (entry?.type !== "message" || entry?.message?.role !== "assistant") continue;
      const usage = entry.message.usage;
      if (!usage || ["input", "output", "cacheRead", "cacheWrite"].some((key) => !nonnegativeInteger(usage[key]))) {
        usageComplete = false;
        continue;
      }
      for (const key of Object.keys(totals)) totals[key] = safeAdd(totals[key], usage[key]);
    }
    return {
      sessions: measurement(1),
      inputTokens: usageComplete ? measurement(totals.input) : unavailable("incomplete-usage"),
      outputTokens: usageComplete ? measurement(totals.output) : unavailable("incomplete-usage"),
      cacheReadTokens: usageComplete ? measurement(totals.cacheRead) : unavailable("incomplete-usage"),
      cacheWriteTokens: usageComplete ? measurement(totals.cacheWrite) : unavailable("incomplete-usage"),
    };
  } catch {
    return sessionUnavailable("source-malformed");
  }
}

function eventUnavailable(reason) {
  return { turns: unavailable(reason), toolCalls: unavailable(reason) };
}

function parseEvents(bytes) {
  let lines;
  try {
    lines = decodeLines(bytes);
    const header = parseJsonWithoutDuplicates(lines[0]);
    if (!header || header.type !== "session") return eventUnavailable("source-malformed");
    if (header.version !== 3) return eventUnavailable("unsupported-version");
    let turnStarts = 0;
    let turnActive = false;
    let turnsComplete = true;
    let toolStarts = 0;
    let toolsComplete = true;
    const activeTools = new Set();
    const seenTools = new Set();
    for (const line of lines.slice(1)) {
      const event = parseJsonWithoutDuplicates(line);
      if (event?.type === "turn_start") {
        if (turnActive) turnsComplete = false;
        else {
          turnActive = true;
          turnStarts = safeAdd(turnStarts, 1);
        }
      }
      if (event?.type === "turn_end") {
        if (!turnActive) turnsComplete = false;
        else turnActive = false;
      }
      if (event?.type === "tool_execution_start") {
        const identity = event.toolCallId;
        if (typeof identity !== "string" || identity.length < 1 || identity.length > 256 || seenTools.has(identity)) {
          toolsComplete = false;
        } else {
          seenTools.add(identity);
          activeTools.add(identity);
          toolStarts = safeAdd(toolStarts, 1);
        }
      }
      if (event?.type === "tool_execution_end") {
        const identity = event.toolCallId;
        if (typeof identity !== "string" || !activeTools.delete(identity)) toolsComplete = false;
      }
    }
    if (turnActive) turnsComplete = false;
    if (activeTools.size > 0) toolsComplete = false;
    activeTools.clear();
    seenTools.clear();
    return {
      turns: turnsComplete ? measurement(turnStarts) : unavailable("incomplete-events"),
      toolCalls: toolsComplete ? measurement(toolStarts) : unavailable("incomplete-events"),
    };
  } catch {
    return eventUnavailable("source-malformed");
  }
}

export function harvestPiUsage({
  sessionFile = null,
  eventFile = null,
  runDirectory,
  maximumSessionBytes = policy.supervisor.maximumSessionBytes,
  maximumEventBytes = policy.supervisor.maximumEventBytes,
}) {
  let sessionResult = sessionUnavailable("source-missing");
  let eventResult = eventUnavailable("source-missing");
  if (sessionFile !== null) {
    const bytes = readPrivateSource(sessionFile, runDirectory, maximumSessionBytes);
    sessionResult = parseSession(bytes);
  }
  if (eventFile !== null) {
    const bytes = readPrivateSource(eventFile, runDirectory, maximumEventBytes);
    eventResult = parseEvents(bytes);
  }
  const aggregate = { schemaVersion: 1, ...sessionResult, ...eventResult };
  const validation = validateUsageAggregate(aggregate);
  if (!validation.ok) fail(validation.errors.join("; "));
  return aggregate;
}

function discoverSessionFile(runDirectory) {
  const sessionRoot = path.join(runDirectory, "session");
  if (!existsSync(sessionRoot)) return null;
  assertNoSymlinkComponents(sessionRoot, runDirectory);
  const files = [];
  let entries = 0;
  function visit(directory, depth) {
    if (depth > 4) fail("session directory exceeds its depth bound");
    for (const name of readdirSync(directory)) {
      entries += 1;
      if (entries > 256) fail("session directory exceeds its entry bound");
      const candidate = path.join(directory, name);
      const info = lstatSync(candidate);
      if (info.isSymbolicLink()) fail("session directory contains a symbolic link");
      if (info.isDirectory()) visit(candidate, depth + 1);
      else if (info.isFile() && name.endsWith(".jsonl")) files.push(candidate);
    }
  }
  visit(sessionRoot, 0);
  if (files.length > 1) fail("session directory contains multiple session files");
  return files[0] ?? null;
}

function writeAggregate(runDirectory, aggregate) {
  const target = path.join(runDirectory, "usage.json");
  if (existsSync(target)) {
    const existing = parseJsonWithoutDuplicates(readPrivateSource(target, runDirectory, 32768).toString("utf8"));
    if (JSON.stringify(existing) !== JSON.stringify(aggregate)) fail("existing usage aggregate conflicts with harvested counters");
    return target;
  }
  const temporary = `${target}.${process.pid}.${randomUUID()}.tmp`;
  writeFileSync(temporary, `${JSON.stringify(aggregate, null, 2)}\n`, { flag: "wx", mode: 0o600 });
  renameSync(temporary, target);
  chmodSync(target, 0o600);
  return target;
}

export async function harvestRun(envelopeFile, { repositoryRoot = root } = {}) {
  await validateInvocationFile(envelopeFile, { repositoryRoot });
  const runDirectory = path.dirname(envelopeFile);
  const eventCandidate = path.join(runDirectory, "events.jsonl");
  const aggregate = harvestPiUsage({
    sessionFile: discoverSessionFile(runDirectory),
    eventFile: existsSync(eventCandidate) ? eventCandidate : null,
    runDirectory,
  });
  writeAggregate(runDirectory, aggregate);
  return aggregate;
}

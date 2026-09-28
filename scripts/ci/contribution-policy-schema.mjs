// SPDX-License-Identifier: Apache-2.0

import { parseJsonWithoutDuplicates } from "../factory-tools/strict-json.mjs";

const topLevelKeys = ["schemaVersion", "types", "scopes", "branch", "commit", "bots"];
const branchKeys = ["formats", "protected"];
const commitKeys = [
  "maxSubjectLength",
  "requireScope",
  "requireDco",
  "requireSignature",
  "signatureEnvelopes",
  "requireBreakingChangeFooter",
  "maximumRange",
];
const botKeys = ["branchExempt", "dcoAuthorNames"];
const enforcementFlags = [
  "requireScope",
  "requireDco",
  "requireSignature",
  "requireBreakingChangeFooter",
];

function exactKeys(value, expected) {
  if (!value || typeof value !== "object" || Array.isArray(value)) return false;
  const actual = Object.keys(value).sort();
  const sorted = [...expected].sort();
  return actual.length === sorted.length && actual.every((key, index) => key === sorted[index]);
}

function validateStringSet(value, label, { allowEmpty = false, maximum = 128 } = {}) {
  const errors = [];
  if (!Array.isArray(value) || (!allowEmpty && value.length === 0) || value.length > maximum) {
    return [`${label} must be a ${allowEmpty ? "bounded" : "non-empty bounded"} array`];
  }
  const seen = new Set();
  for (const entry of value) {
    if (typeof entry !== "string" || entry.length === 0 || entry.length > 256) {
      errors.push(`${label} contains an invalid string`);
    } else if (seen.has(entry)) {
      errors.push(`${label} contains a duplicate value`);
    }
    seen.add(entry);
  }
  return errors;
}

export function parseContributionPolicy(text) {
  if (Buffer.byteLength(text, "utf8") > 64 * 1024) throw new Error("contribution policy exceeds 65536 bytes");
  return parseJsonWithoutDuplicates(text, { maximumDepth: 12, maximumNodes: 4_096 });
}

export function validateContributionPolicyShape(document) {
  const errors = [];
  if (!exactKeys(document, topLevelKeys)) return ["policy has an unknown or missing top-level field"];
  if (document.schemaVersion !== 1) errors.push("schemaVersion must be 1");
  errors.push(...validateStringSet(document.types, "types"));
  errors.push(...validateStringSet(document.scopes, "scopes"));

  if (!exactKeys(document.branch, branchKeys)) {
    errors.push("branch has an unknown or missing field");
  } else {
    errors.push(...validateStringSet(document.branch.formats, "branch.formats"));
    errors.push(...validateStringSet(document.branch.protected, "branch.protected"));
  }

  if (!exactKeys(document.commit, commitKeys)) {
    errors.push("commit has an unknown or missing field");
  } else {
    for (const field of ["maxSubjectLength", "maximumRange"]) {
      const value = document.commit[field];
      if (!Number.isSafeInteger(value) || value < 1 || value > 10_000) {
        errors.push(`commit.${field} must be an integer from 1 through 10000`);
      }
    }
    for (const field of enforcementFlags) {
      if (typeof document.commit[field] !== "boolean") errors.push(`commit.${field} must be boolean`);
    }
    errors.push(...validateStringSet(
      document.commit.signatureEnvelopes,
      "commit.signatureEnvelopes",
      { maximum: 16 },
    ));
  }

  if (!document.bots || typeof document.bots !== "object" || Array.isArray(document.bots)) {
    errors.push("bots must be an object");
  } else if (Object.keys(document.bots).length > 32) {
    errors.push("bots exceeds 32 entries");
  } else {
    for (const [actor, bot] of Object.entries(document.bots)) {
      if (actor.length === 0 || actor.length > 128) errors.push("bots contains an invalid actor name");
      if (!exactKeys(bot, botKeys)) {
        errors.push(`bots.${actor} has an unknown or missing field`);
        continue;
      }
      if (typeof bot.branchExempt !== "boolean") errors.push(`bots.${actor}.branchExempt must be boolean`);
      errors.push(...validateStringSet(bot.dcoAuthorNames, `bots.${actor}.dcoAuthorNames`, {
        allowEmpty: true,
        maximum: 32,
      }));
    }
  }
  return errors;
}

function rejectExpansion(baseValues, proposedValues, label, errors) {
  const baseSet = new Set(baseValues);
  if (proposedValues.some((entry) => !baseSet.has(entry))) errors.push(`${label} expands an allowance`);
}

function requireEquivalentSet(baseValues, proposedValues, label, errors) {
  if (baseValues.length !== proposedValues.length) {
    errors.push(`${label} must remain set-equivalent`);
    return;
  }
  const proposedSet = new Set(proposedValues);
  if (baseValues.some((entry) => !proposedSet.has(entry))) errors.push(`${label} must remain set-equivalent`);
}

export function validateContributionPolicyMonotonicity(base, proposed) {
  const baseErrors = validateContributionPolicyShape(base);
  const proposedErrors = validateContributionPolicyShape(proposed);
  const errors = [
    ...baseErrors.map((entry) => `base policy: ${entry}`),
    ...proposedErrors.map((entry) => `proposed policy: ${entry}`),
  ];
  if (errors.length > 0) return { ok: false, errors };

  rejectExpansion(base.types, proposed.types, "types", errors);
  rejectExpansion(base.scopes, proposed.scopes, "scopes", errors);
  requireEquivalentSet(base.branch.formats, proposed.branch.formats, "branch.formats", errors);
  requireEquivalentSet(base.branch.protected, proposed.branch.protected, "branch.protected", errors);

  for (const field of ["maxSubjectLength", "maximumRange"]) {
    if (proposed.commit[field] > base.commit[field]) errors.push(`commit.${field} increases a ceiling`);
  }
  for (const field of enforcementFlags) {
    if (base.commit[field] && !proposed.commit[field]) errors.push(`commit.${field} weakens enforcement`);
  }
  rejectExpansion(
    base.commit.signatureEnvelopes,
    proposed.commit.signatureEnvelopes,
    "commit.signatureEnvelopes",
    errors,
  );

  const baseBots = new Set(Object.keys(base.bots));
  for (const [actor, proposedBot] of Object.entries(proposed.bots)) {
    if (!baseBots.has(actor)) {
      errors.push(`bots.${actor} adds an exemption identity`);
      continue;
    }
    const baseBot = base.bots[actor];
    if (!baseBot.branchExempt && proposedBot.branchExempt) {
      errors.push(`bots.${actor}.branchExempt adds an exemption`);
    }
    rejectExpansion(
      baseBot.dcoAuthorNames,
      proposedBot.dcoAuthorNames,
      `bots.${actor}.dcoAuthorNames`,
      errors,
    );
  }

  return { ok: errors.length === 0, errors };
}

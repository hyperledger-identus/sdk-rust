#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const requiredFiles = [
  "AGENTS.md",
  "AGENT.md",
  "docs/maintenance/agentic-sdlc.md",
  ".github/labels.yml",
  ".github/ISSUE_TEMPLATE/config.yml",
  ".github/ISSUE_TEMPLATE/ssi-capability.yml",
  ".github/ISSUE_TEMPLATE/defect.yml",
  ".github/ISSUE_TEMPLATE/maintenance.yml",
  ".github/DISCUSSION_TEMPLATE/architecture.md",
  ".github/DISCUSSION_TEMPLATE/research.md",
  ".github/pull_request_template.md",
];

const requiredTerms = new Map([
  [
    "AGENTS.md",
    [
      "Agentic SDLC Contract",
      "Agent Roles",
      "GitHub Status Flow",
      "GitHub Discussions",
      "Spec Kit task",
      "GPG-signed",
      "DCO-signed",
      "node tools/check-agent-sdlc.mjs --check",
    ],
  ],
  [
    "docs/maintenance/agentic-sdlc.md",
    [
      "Issue Statuses",
      "GitHub Project Fields",
      "Discussion Workflow",
      "Pull Request Workflow",
      "Harness",
      "status:triage",
      "status:ready-to-merge",
      "agent:security",
      "Fixture Impact",
    ],
  ],
  [
    ".github/pull_request_template.md",
    [
      "Linked Work",
      "Validation",
      "Security Impact",
      "Docs Impact",
      "Conformance Impact",
      "GPG-signed",
      "DCO-signed",
    ],
  ],
]);

const requiredLabels = [
  "status:triage",
  "status:needs-spec",
  "status:ready",
  "status:in-progress",
  "status:blocked",
  "status:review",
  "status:security-review",
  "status:docs-review",
  "status:conformance",
  "status:ready-to-merge",
  "status:released",
  "type:capability",
  "type:defect",
  "type:conformance",
  "type:architecture",
  "type:docs",
  "type:maintenance",
  "type:security",
  "agent:manager",
  "agent:planner",
  "agent:engineer",
  "agent:conformance",
  "agent:reviewer",
  "agent:security",
  "agent:docs",
  "agent:release",
  "agent:maintenance",
  "risk:security-sensitive",
];

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function checkRequiredFiles() {
  for (const relativePath of requiredFiles) {
    assert(
      fs.existsSync(path.join(repoRoot, relativePath)),
      `missing required SDLC file: ${relativePath}`,
    );
  }
}

function checkRequiredTerms() {
  for (const [relativePath, terms] of requiredTerms) {
    const text = read(relativePath);
    for (const term of terms) {
      assert(text.includes(term), `${relativePath} missing required term: ${term}`);
    }
  }
}

function checkLabels() {
  const labels = read(".github/labels.yml");
  for (const label of requiredLabels) {
    assert(labels.includes(`name: ${label}`), `.github/labels.yml missing ${label}`);
  }
}

function checkIssueTemplates() {
  for (const template of [
    ".github/ISSUE_TEMPLATE/ssi-capability.yml",
    ".github/ISSUE_TEMPLATE/defect.yml",
    ".github/ISSUE_TEMPLATE/maintenance.yml",
  ]) {
    const text = read(template);
    for (const term of [
      "Agent",
      "Capability",
      "Acceptance Criteria",
      "Conformance Impact",
      "Docs Impact",
      "Discussion Link",
    ]) {
      assert(text.includes(term), `${template} missing ${term}`);
    }
  }
}

function checkWorkflowGate() {
  const workflow = read(".github/workflows/conformance.yml");
  assert(
    workflow.includes("node tools/check-agent-sdlc.mjs --check"),
    "conformance workflow must run the agentic SDLC harness",
  );
}

function checkSpecTask() {
  const tasks = read("specs/001-sdk-rust-platform-core/tasks.md");
  for (const term of [
    "T097",
    "[x] T097",
    "Agentic SDLC",
    "issue status",
    "GitHub Discussion",
    "node tools/check-agent-sdlc.mjs --check",
  ]) {
    assert(tasks.includes(term), `Spec Kit tasks missing ${term}`);
  }
}

function main() {
  checkRequiredFiles();
  checkRequiredTerms();
  checkLabels();
  checkIssueTemplates();
  checkWorkflowGate();
  checkSpecTask();

  if (process.argv.includes("--check")) {
    console.log("agentic SDLC harness check passed");
  }
}

main();

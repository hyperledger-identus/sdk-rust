# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-11-06
Source retrieval date: 2026-11-06
Research blockers: none

## Problem and existing implementation

`scripts/ci/contribution-policy.mjs` currently removes HTML comments from the
complete body before recognizing fenced code. An unterminated comment opener
inside a code fence therefore hides later live metadata. The same parser does
not exclude four-space or tab-indented CommonMark code, so a code sample can
satisfy a standalone linkage expression.

The privileged `pull_request_target` workflow evaluates exact protected-base
code and fetches exact head commits safely, but it validates only the title,
body, branch, commit messages, and exact-head sidecar. The guarded merge and
metrics paths separately inspect GitHub's `closingIssuesReferences`. A direct
protected UI merge can therefore reach a different verdict from guarded
delivery even though all paths claim the same linkage contract.

## Normative sources

- Issue #590 and the exact-head review ledger on PR #589.
- ADR 0198 and the canonical `factory-operations` requirement.
- GitHub's GraphQL `PullRequest.closingIssuesReferences` connection, whose
  nodes identify the exact issue repository and number.
- Existing protected-base workflow, delivery, metrics, strict JSON, and
  factory-operation tests at `develop@46025b829850073ff65aa93451f2ae7cb49b33c1`.
- CommonMark fenced and indented code-block rules as the authority model for
  repository pull-request Markdown.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Continue stripping comments before code recognition | `not-adopt` | Code bytes must never affect live comment state. | Never under Markdown-based authorization. |
| Add a full Markdown dependency | `not-adopt` | The policy needs only bounded block authority and should not add a mutable parser dependency or execute head code. | Required metadata becomes structurally richer than line-based fields. |
| Protected-base finite-state authority parser | `adopt` | It preserves the narrow syntax, rejects partial constructs, ignores fenced and indented code, and is mutation-testable. | CommonMark authority rules change materially. |
| Trust textual closing keywords only | `not-adopt` | GitHub remains the actual issue-closing authority and can disagree with local parsing. | GitHub removes automatic issue linkage. |
| Query one fixed connection page | `not-adopt` | A matching issue can fall outside the first page and absence would be ambiguous. | GitHub provides an exact single-node lookup for this relation. |
| Bounded cursor pagination captured as untrusted JSON pages | `adopt` | Protected-base code can prove identity, progress, completeness, and resource bounds without executing head code. | Provider pagination semantics change. |

## Compatibility and dependency evidence

The implementation uses existing Node standard-library code, `gh`, Git object
plumbing, and strict bounded validation. It adds no npm, Cargo, Nix, runtime,
or target dependency. Existing valid closing and deferred bodies remain valid.
Only metadata that previously relied on code/comment decoys or incomplete
hosted evidence becomes invalid.

## Security, privacy and maintenance evidence

The workflow runs protected-base code under `pull_request_target`, fetches no
executable head-owned policy, and writes bounded GitHub JSON only to the runner
temporary directory. Validation limits page count and total nodes, requires
monotonic unique cursors, exact PR/head identity, closed node shape, and exact
repository/issue identity. Diagnostics name invariants without printing body,
API pages, tokens, sidecar contents, or comment text.

## Rejected or deferred candidates

Repository-setting changes, protection bypass, workflow reruns, release
changes, and consumer edits are rejected. Issue #308 retains broader historical
metrics pagination and merged-PR fallback work; this slice implements the
required hosted-policy evidence path and reuses its bounded parser where
practical without claiming all #308 acceptance.

## Open questions and blockers

No blocker remains. GitHub may change linkage between a successful policy run
and merge. The protected check cannot make that remote state atomic; guarded
delivery rechecks immediately before merge and post-merge receipt validation
detects drift. Documentation must retain that limitation rather than claim an
impossible atomic guarantee.

## Evidence commands

Focused Node mutation tests, pull-request shell policy tests, factory checks,
bootstrap health, exact-diff local review, one external discovery review, and
protected exact-head `fast` CI.

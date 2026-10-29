# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

The hosted pull-request policy has two layers. `scripts/check-pr-policy.sh`
extracts a repository issue and validates review/constraint/compatibility
fields. `scripts/ci/contribution-policy.mjs` binds the conventional title and
branch issue and currently requires a closing keyword for that branch issue.
The file-backed `scripts/factory delivery pr-preflight` reuses both layers.

The policy has no representation for work whose final acceptance is
intentionally post-merge. Issue #276 first exposed the mismatch and issue #291
recorded it. The current M5 flow reproduced it exactly: PR #585 merged a
readiness packet and mechanically closed #388, then #388 was reopened because
complete slow evidence and independent exact-receipt approval remained. The
close/reopen operation was truthful only after corrective commentary and added
avoidable lifecycle and automation noise.

The protected-base workflow already fetches the exact untrusted pull-request
head object without checking it out or executing it. Base-owned Node code can
therefore read one bounded, path-constrained data blob from that exact object
without trusting or running head code.

## Normative sources

Issue #291 defines the outcome. ADRs 0003, 0004, 0108, and 0127, the canonical
`ai-software-factory`, `spec-driven-delivery`, and `factory-operations`
specifications, `docs/factory/operations.md`, the contribution policy, and
GitHub closing-keyword semantics govern the implementation. PR #585 and issue
#388 are current local evidence; no external dependency or version decision is
required.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Keep mandatory close/reopen | `not-adopt` | It misstates lifecycle state and creates avoidable automation churn. | Never; accurate state is the objective. |
| Accept any `References #N` body text | `not-adopt` | It permits ambiguous or unsupported deferral with no durable acceptance contract. | A stronger issue-native deferred-acceptance primitive exists. |
| Infer deferral from issue labels or milestones | `not-adopt` | Mutable remote metadata is not bound to the reviewed head and is easy to drift. | GitHub exposes immutable reviewed linkage metadata. |
| Bind deferred mode to an archived OpenSpec JSON sidecar | `adopt` | It is exact-head, reviewable, offline-testable, and keeps remaining gates explicit. | The repository replaces OpenSpec as its delivery contract. |
| Parse or execute head-owned checker code | `not-adopt` | `pull_request_target` must keep executable authority in the protected base. | Never under the privileged workflow. |
| Base-owned bounded `git` object read | `adopt` | It treats head content as untrusted data while binding the verdict to the exact head. | GitHub stops exposing the exact head object to protected-base policy. |

## Compatibility and dependency evidence

The change uses existing Node standard-library code, the repository strict JSON
parser, and `git`. It adds no package, lockfile, Rust crate, network, target, or
runtime dependency. Ordinary issue-closing bodies remain valid after adding the
explicit linkage field. Deferred mode is additive and limited to repository
factory metadata.

## Security, privacy and maintenance evidence

The sidecar path is constrained to one dated archived OpenSpec directory and a
fixed filename. The exact Git head and blob identities are validated, input is
size-bounded, duplicate JSON keys fail, and diagnostics never print the PR body
or sidecar contents. The sidecar contains only an issue number, disposition,
rationale, and named acceptance gates. No token, prompt, transcript, secret,
consumer payload, or model/provider identity is retained.

## Rejected or deferred candidates

Live issue labels, free-form reference text, workflow reruns, approval bypass,
branch-protection changes, and release automation are rejected. Metrics
publisher linkage and worktree cleanup semantics remain separate issues.

## Open questions and blockers

No blocker remains. The change must prove both local file-backed preflight and
hosted exact-head validation shapes through deterministic fixtures before it
can be considered ready.

## Evidence sources

- sdk-rust issue #291 and issue #388 with PR #585.
- `scripts/check-pr-policy.sh`, `scripts/ci/contribution-policy.mjs`,
  `scripts/factory-tools/delivery.mjs`, the pull-request policy workflow, and
  existing policy tests at `develop@c429ac494a6eaeb2e22c0b38a5b1d70c0eb6ff62`.
- Canonical OpenSpec and factory operations documentation at the same revision.

## Evidence commands

Before implementation: `scripts/factory backlog-live`, managed worktree audit,
exact issue/PR inspection, and source inspection of both policy layers and
their tests. Mutation tests, full factory checks, bootstrap health, and fresh
independent review remain delivery tasks.

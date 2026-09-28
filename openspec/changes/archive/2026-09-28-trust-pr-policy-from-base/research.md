# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The current implementation uses `pull_request` plus a default checkout, so the
pull-request merge tree supplies `scripts/check-pr-policy.sh`,
`scripts/ci/contribution-policy.mjs`, `.github/contribution-policy.json`, and
the workflow's record-shaping logic. This lets the subject of a policy check
modify the checker that judges it. Consumers are contributors and maintainers;
the public SDK API and runtime dependency graph are unaffected.

## Normative sources

- Issue #341 defines the base-owned checker acceptance boundary and pins the
  repository decision authority.
- GitHub documents that `pull_request_target` runs in the base branch context
  and warns against building or running untrusted pull-request code:
  https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#pull_request_target
- GitHub's security hardening guide requires treating pull-request content as
  untrusted and recommends least-privilege tokens and immutable action pins:
  https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions
- GitHub's checkout action documents exact `ref`, `fetch-depth`, and
  `persist-credentials` behavior:
  https://github.com/actions/checkout/tree/9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0

Pinned source revision is the issue branch merge base
`612f6d1ca8ad49fe2f5fb669578c0b972465d124`; third-party actions retain exact
commit revisions. Source retrieval occurred on the date above.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Execute the pull-request checkout's checker | `not-adopt` | The contribution controls its own judge. | Never for a required security/governance gate. |
| Download checker files from a mutable branch at runtime | `not-adopt` | Loses exact revision identity and adds network interpretation. | A signed policy distribution system exists. |
| Use `pull_request_target`, exact base checkout, and never execute head code | `adopt` | GitHub loads the workflow from trusted base context and the job can retain read-only least authority. | GitHub changes the event security model. |
| Duplicate the checker entirely as inline YAML | `not-adopt` | Creates drift and poor testability without removing workflow trust requirements. | A tiny invariant cannot be expressed safely elsewhere. |
| Evaluate both base and head policies | `conditional-adopt` | Useful monotonicity defense, but only after the base trust root is established in #341. | #339 begins after this slice. |

## Compatibility and dependency evidence

No Cargo version, feature, MSRV, target, direct and resolved dependency cone,
native code, unsafe code, public type, wire format, or facade boundary
changes. The workflow uses the existing pinned checkout and harden-runner
versions plus runner-provided Git, Bash, Node, jq, and GitHub CLI. Public and
wire compatibility are unchanged. Protocol or draft currency is not applicable
to repository governance.

## Security, privacy and maintenance evidence

The security improvement removes execution authority from attacker-controlled
head contents. Explicit `contents`, `issues`, and `pull-requests` read
permissions, disabled persisted credentials, immutable action pins, exact SHA
assertions, and no secrets bound the more privileged event. Pull-request text
remains data rather than shell source. Fetching an exact Git object adds no
checkout, build, dependency installation, hook execution, or native code.

License and provenance are unchanged; all executed repository code is
Identus-owned at a pinned base revision. Supply-chain evidence remains action
pinning and the existing policy/factory test suite. Maintenance remains in the
single workflow and portable checker. Rollback is a normal revert. The release
and security posture is improved without collecting private identity, wallet,
credential, or telemetry data.

## Rejected or deferred candidates

Policy monotonicity, a separately signed checker artifact, GitHub App-based
evaluation, repository ruleset changes, write permissions, and secrets are
rejected for this slice. #339 is the reconsideration trigger for dual
base/head evaluation; #344 owns administrator hardening.

## Open questions and blockers

None. The first natural hosted run of the new event necessarily occurs on the
next pull request after the bootstrap merge. That immediate canary removes the
temporarily retained legacy event before unrelated integration and records the
hosted evidence; this rollout constraint is not a planning blocker.

## Evidence commands

Planning commands: inspect the exact base workflow and policy files; run
`scripts/factory research-ready trust-pr-policy-from-base`,
`scripts/factory constraints-ready trust-pr-policy-from-base`, and issue-bound
preflight. Implementation commands: Node policy tests, shell factory-contract
tests, actionlint, yamllint, full factory checks, signed/DCO review, and hosted
PR CI. Unrun at planning time are the implementation tests, hosted CI, and the
first post-merge natural `pull_request_target` run.

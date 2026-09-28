# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The current implementation executes the exact protected-base checker, but it
does not compare the proposed policy file. A relaxation cannot affect its own
PR anymore, yet becomes the baseline after merge. The current policy has a
small version-1 JSON surface with title/branch allowlists, enforcement flags,
numeric ceilings, signature envelopes, and two bot exemptions.

## Normative sources

- Issue #339 defines monotonic base-versus-head evaluation and deterministic
  offline evidence.
- ADR 0166 and natural run 36487023998 establish the exact base trust root.
- Git's `cat-file` and `<tree-ish>:<path>` object syntax provide non-executing
  exact-object reads: https://git-scm.com/docs/git-cat-file and
  https://git-scm.com/docs/gitrevisions
- GitHub requires treating pull-request contents as untrusted under
  `pull_request_target`:
  https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions

Pinned repository revision is
`7ec1cafd40d37d797c50b5205e1196700d31ee94`; source retrieval occurred on the
date above.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Judge only with base policy and ignore the proposed policy | `not-adopt` | A relaxation becomes authoritative after merge. | Never after #341. |
| Execute both base and proposed checkers | `not-adopt` | Restores head-controlled execution and divergent semantics. | Never in the privileged job. |
| Parse proposed JSON with base code and require exact equality | `not-adopt` | Safe but prevents useful strengthening. | Emergency freeze only. |
| Apply a closed field-aware partial order in base code | `adopt` | Permits strengthening while rejecting every known relaxation deterministically. | Policy schema version changes. |
| Accept an approval phrase in the PR body | `not-adopt` | Untrusted text cannot authorize its own relaxation. | A separate base-owned cryptographic or GitHub-identity approval contract lands first. |

## Compatibility and dependency evidence

No Cargo version, feature, MSRV, target, direct and resolved dependency cone,
native code, unsafe code, public type, facade, or wire format changes. The
implementation reuses repository-owned bounded strict JSON and runner-provided
Git. Consumer evidence is the unchanged contributor-visible policy behavior
for equality and the explicit diagnostics for strengthening/relaxation. Public
and wire compatibility are unchanged; protocol/draft currency is not
applicable.

## Security, privacy and maintenance evidence

The proposed blob is size-checked, duplicate-rejecting, depth/node bounded, and
read through Git without checkout or execution. Unknown schema content fails
closed. Static field diagnostics contain no PR content. Exact base and head SHA
binding remains upstream of comparison. Supply-chain, license, and provenance
are unchanged because no dependency is added. Rollback is a normal revert.

No wallet secret, identity claim, credential, user data, telemetry, or secret
enters the comparison. Maintenance is bounded to the versioned policy schema;
new fields must define their order before use.
The maintenance, release and security posture is unchanged except for the
stronger governance gate; no package or release artifact is produced.

## Rejected or deferred candidates

Approval waivers, remote policy services, GitHub Apps, signature changes, and a
version-2 schema are deferred. Their reconsideration trigger is a dedicated
issue and ADR that can be activated from the protected base without
self-authorization.

## Open questions and blockers

None. Descriptive branch arrays remain equality-only rather than receiving
invented enforcement semantics.

## Evidence commands

Planning: factory research/constraint readiness and issue-bound preflight.
Implementation: Node mutation tests, strict JSON vectors, workflow source
tests, actionlint, yamllint, factory checks, Nix evaluation, signed/DCO review,
and natural exact-head hosted CI. All implementation/hosted checks are unrun at
planning time.

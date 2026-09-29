# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

Natural scheduled run
`https://github.com/hyperledger-identus/sdk-rust/actions/runs/36371225877`
failed its Linux and macOS DID candidate jobs while the other substantive slow
jobs completed. Both matrix jobs execute primary qualification with output
`artifacts/did-matrix/primary`, then execute MSRV qualification in the same
checkout. Candidate preparation requires `git status --porcelain` to be empty,
so the primary receipt becomes untracked input that the MSRV lane rejects.

The current implementation of the matrix contract remains otherwise correct:
ADR 0155 requires clean exact-SHA
staged sources, primary Rust 1.98.1 and MSRV 1.89.0, exact native hosts, bounded
attempt-scoped artifacts, and no PR trigger. ADR 0127 keeps this evidence on the
weekly/manual slow line.

## Normative sources

- Issue #480 and natural run
  `https://github.com/hyperledger-identus/sdk-rust/actions/runs/36371225877`
  provide the consumer and hosted-runner failure evidence.
- ADR 0155 and the archived `qualify-did-candidate-matrix` contract define the
  candidate identity, compiler, host, target, and receipt invariants.
- ADR 0127 and `weekly-slow-evidence` define the fast/slow delivery boundary.
- GitHub Actions exposes a per-job runner temporary directory through
  `RUNNER_TEMP` and the `${{ runner.temp }}` expression, outside the checkout
  and directly usable by the artifact action:
  https://docs.github.com/actions/learn-github-actions/contexts#runner-context

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Weaken or remove candidate clean-source validation | `not-adopt` | Hides real source contamination and weakens exact-revision evidence. | Never for release evidence. |
| Delete/reset checkout artifacts between compiler lanes | `not-adopt` | Makes mutation transient and obscures the invariant the workflow must preserve. | Never while lanes share a checkout. |
| Split primary and MSRV into separate jobs | `not-adopt` | Avoids the symptom but adds runner cost and aggregation complexity without need. | Lanes require independent permissions or environments. |
| Write both lanes below `RUNNER_TEMP` and assert source cleanliness | `adopt` | Preserves candidate policy, shared checkout efficiency, and the existing upload contract. | Runner temporary storage loses job-scoped availability. |
| Add an ignored checkout-local evidence path | `not-adopt` | Couples release evidence to repository ignore policy and permits hidden source-tree writes. | Never for this boundary. |

## Compatibility and dependency evidence

No Cargo dependency, direct or resolved dependency cone, feature, exact tool
version, code API, wire format, public promise, MSRV, or target changes. The
existing facade boundary and candidate receipt schema remain stable.

## Security, privacy and maintenance evidence

No credential or permission changes occur. The evidence files remain bounded
and job-local, and contain no private data. Moving them outside the checkout
narrows coupling and keeps the existing clean-source security invariant
observable. The repository license and provenance remain Apache-2.0 and no
external code is imported. Unsafe and native-code posture, supply-chain
evidence, maintenance, release, and security posture are unchanged. Public and
wire compatibility are unchanged. Protocol or draft currency is not applicable
to this workflow-only repair. Rollback reverts only workflow path/checker
changes and returns to the known failing schedule.

## Rejected or deferred candidates

Weakening cleanliness, cleanup/reset, ignored checkout paths, and separate jobs
are rejected in the decision table. Workflow dispatch, rerun, candidate
publication, compiler changes, and any target/support expansion are deferred to
their existing owners.

## Evidence commands

Exact planned commands are `python3 scripts/check-release-candidates.py`,
`python3 scripts/tests/release-candidates.py`,
`actionlint .github/workflows/nix-checks.yml`, `nix flake check`, and
`./scripts/factory check`, plus strict OpenSpec validation. Local review and
exact-head required CI prove repository integration. The intentionally unrun
check is a hosted natural/authorized slow execution; only that can prove both
native runners now complete, and it remains deferred to #388.

## Open questions and blockers

None. The failure and the least-coupled repair are bounded by existing policy.

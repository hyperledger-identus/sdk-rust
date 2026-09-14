# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-14
Source retrieval date: 2026-09-14
Research blockers: none

## Problem and existing implementation

The current implementation generates Rust checks from
`nix/checks/gates.toml`. Required fast CI selects the existing default-surface
`rust-clippy` check explicitly. The weekly/manual slow workflow runs
`nix flake check`, so adding another generated check expands only slow evidence.

The exact local command `cargo clippy --workspace --all-targets --all-features
-- -D warnings` was run on Rust 1.98.1 at the exact base. It failed on three
`collapsible_if` findings in `identus-conformance` test-only guards and one
`manual_noop_waker` finding in an `identus-credentials` integration test. The
workspace contains two production `too_many_arguments` allowances on public
OID4VCI limits constructors and one test-helper allowance.

## Normative sources

The controlling local sources are issue #268, ADR 0081's temporary fast/slow
CI split, the canonical `nix-tooling` specification, and the Rust 1.98.1 Clippy
diagnostics reproduced above. No external protocol or draft standard applies.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Generated Nix check in the existing weekly/manual workflow | `develop@dbef9923e65d1a8332c4ba38f42532c8a12811f7` | `adopt` | Reuses the declarative gate engine and both supported hosts without expanding required fast CI. | The temporary CI policy is replaced. |
| Add all targets/features to required fast `rust-clippy` | Rust 1.98.1 | `not-adopt` | Contradicts the accepted lean fast lane and repeats slow evidence on every PR. | Measured fast-lane capacity and policy explicitly change. |
| Run only an untracked shell command | Rust 1.98.1 | `not-adopt` | Does not produce hosted, drift-checked evidence. | Never for a canonical gate. |
| Redesign both public OID4VCI constructors | current source | `not-applicable` | Issue #268 excludes public API redesign and #7/#168 semantics. | A focused API/decomposition issue authorizes migration. |

## Compatibility and dependency evidence

Public and wire compatibility remain unchanged. No dependency, feature,
manifest, MSRV, target, native code, unsafe code, or supply-chain input changes.
The new check uses the primary Rust 1.98.1 Crane provider and existing primary
dependency artifacts. The direct and resolved dependency cone is unchanged.
There is no facade or downstream consumer migration.

## Security, privacy and maintenance evidence

The change executes static analysis over already authored targets/features and
does not process credentials, keys, protocol input, or network data. It adds no
unsafe or native code. Narrow `#[expect]` annotations are preferable to
open-ended `#[allow]` because an expectation that stops matching becomes a
warning under the workspace warnings-denied policy. The lint registry records
owner, rationale, and removal condition so temporary readability debt remains
auditable.

Maintenance is limited to keeping one declarative gate and its validator in
sync. Release eligibility remains false and slow failures remain visible
pre-release debt under ADR 0081. Protocol/draft currency is not applicable.

## Rejected or deferred candidates

Expanding fast CI and redesigning public constructors are rejected for this
slice. A standalone third-party lint runner is unnecessary because pinned
Clippy already identifies the complete target/feature surface.

## Open questions and blockers

No blocker remains. The exact gate composition, fast-lane exclusion, exception
disposition, compatibility boundary, and rollback are decided by issue #268
and existing policy.

## Evidence commands

- `scripts/factory doctor` passed at the exact base.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  reproduced the four issue findings and is currently failing before the
  implementation.
- `rg -n '#\\[(allow|expect)\\(clippy::' --glob '*.rs'` identified the three
  existing narrow Clippy allowances.
- Factory readiness, validator mutation tests, Rust tests, the new Nix check,
  and `nix flake check` are unrun implementation evidence at planning time.

## Reconsideration triggers

- ADR 0081 expires or a release-candidate CI policy replaces it.
- Measured fast-lane capacity justifies broader required PR evidence.
- A focused API/decomposition issue supplies a migration for the OID4VCI
  constructors without changing resource semantics.

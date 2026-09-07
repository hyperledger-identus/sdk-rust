# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

At `origin/develop@b5cfc8f30ab9137e3c71b94a73f162aff3d1b48c`,
pull requests run the complete Nix flake on both Ubuntu and macOS, seven Rust
1.85 feature builds, a nightly all-feature etalon build, and applicable
nightly sanitizer workflows. Pushes to `develop` repeat the same matrix.
`factory-contract` is a separate three-minute job and no branch protection
currently makes the long Nix jobs required, so cost is high without a reliable
single merge signal.

## Normative sources

- Sponsor direction: https://github.com/hyperledger-identus/sdk-rust/discussions/172
- Implementation contract: https://github.com/hyperledger-identus/sdk-rust/issues/173
- Prior compiler decisions: ADRs 0002, 0062 and 0064
- Factory merge authority: ADRs 0003 and 0004
- Rust 1.98.1 remains the corrected exact compiler already pinned by the
  repository; Rust 1.98.0 is not selected.

The current implementation revision and all source provenance are recorded
above. The repository and toolchain inputs retain their existing Apache-2.0 or
upstream licenses; this workflow-only change copies no third-party code.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Keep the current PR matrix | `not-adopt` | Release-style evidence blocks active development and duplicates push work. | A release candidate or supported-consumer promise requires the full matrix again. |
| Direct Cargo-only fast lane | `not-adopt` | Faster, but bypasses the pinned Nix toolchain and manifest-derived checks. | Reconsider only if Nix cannot provide acceptable measured latency. |
| Linux Nix fast lane plus weekly/manual full flake | `adopt` | Preserves reproducibility and one merge signal while moving exhaustive evidence out of the critical path. | Review by 2026-12-08 or earlier on release-candidate work. |
| Rust 1.85 plus 1.98 plus nightly on every PR | `not-adopt` for this phase | Three compiler classes provide little value before publication and materially increase feedback time. | Restore an evidence-driven matrix for a release candidate and named consumers. |
| Rust 1.98.1 for floor, development and etalon | `adopt` temporarily | Maximizes current crate access and makes the repository contract honest: no promise below the only compiler continuously used. | Review by 2026-12-08, compiler defect/security update, or release-candidate planning. |
| Pinned nightly sanitizer shell | `conditional-adopt` as tooling only | libFuzzer sanitizer instrumentation still requires nightly; it is not SDK compatibility evidence. | Remove if stable sanitizer tooling becomes sufficient or fuzzing is replaced. |

## Baseline timing evidence

The latest 20 successful `pull_request` runs of `nix-checks.yml`, measured as
GitHub `updatedAt - createdAt` on 2026-09-08, have p50 1,033 seconds (17m13s),
p95 1,467 seconds (24m27s), minimum 593 seconds and maximum 1,559 seconds.
Run IDs, newest first: 34164867720, 34161568745, 34147194565, 34137603135,
34118941767, 34116807069, 34114418687, 34107589814, 34104344825,
34094648588, 34089554980, 34085847040, 34083016220, 34081431045,
34078126363, 34075095905, 34071849006, 34068280866, 34065867490 and
34065496275. Run wall time intentionally includes queue/setup overhead because
developer merge latency is the outcome. A first fast PR is smoke evidence only;
post-change p50/p95 requires the same 20 successful-PR sample.

## Compatibility and dependency evidence

The change adds no Cargo dependency and changes no resolved runtime cone. Rust
1.98.1 already passes the complete workspace, feature and portable-target
matrix. Raising Cargo `rust-version` from 1.85.0 to 1.98.1 is an intentional
temporary pre-release compatibility change; every package is unpublished at
0.0.x and no lower compiler promise remains. Linux and macOS remain
host-tested, and WASM/Android/iOS remain compile-checked; only the cadence and
merge-blocking meaning change.

The direct dependency cone and resolved dependency cone are identical to the
base. Public API and wire compatibility are unchanged. Identus-owned facade
boundaries are unchanged because no dependency type or implementation enters a
crate. Effective MSRV becomes the exact Rust 1.98.1 workspace floor.

## Security, privacy and maintenance evidence

Fast retains source structure, formatting, build, strict Clippy and normal
tests. Slow retains all existing Nix derivations, target/feature checks, docs,
license and advisory evidence. Sanitizers remain isolated in the exact pinned
nightly shell and no longer claim stable or per-PR coverage. No secrets,
personal data, unsafe SDK code, release credentials or downstream state are
introduced. Weekly failure is visible engineering debt and blocks a release
candidate, but does not retroactively invalidate pre-release merges.
Unsafe and native code evidence is unchanged: the workflow/provider rewrite
adds neither, while existing dependency review remains in the slow matrix.

## Rejected or deferred candidates

- Do not make slow green a per-PR requirement; that recreates the current
  critical path.
- Do not silently delete target, feature, supply-chain or sanitizer evidence;
  each receives an explicit weekly/manual owner.
- Do not auto-create external issues from scheduled failure in this slice;
  GitHub failure visibility is sufficient until repeated failures prove a need.
- Do not configure branch protection in this implementation; repository
  settings remain the documented maintainer activation step.
- Do not use one smoke run as p50/p95 or invent a duration budget.

Rollback is one focused PR revert restoring the previous Cargo, Nix, policy
and workflow matrix. Protocol and draft currency are not applicable to a CI
policy change; no SSI profile changes.

## Open questions and blockers

None. The durable direction fixes the product tradeoff. Protected-settings
activation and the 20-run post-change timing sample are follow-up operations,
not implementation blockers.

## Evidence commands

Commands run before production edits: factory doctor, OpenSpec strict validation,
research-ready and constraints-ready. After implementation: focused validator
tests, actionlint, shell/file hygiene, selected fast Nix derivations, complete
local `nix flake check`, factory receipt/archive, and hosted fast smoke evidence.
Unrun checks before implementation are the proposed fast/slow workflows because
they do not exist yet; this is not represented as passing evidence.

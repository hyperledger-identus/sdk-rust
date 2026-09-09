# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

The current implementation has `scripts/factory`, strict OpenSpec validation,
research and constraint gates, safe archive preflight, a Rust 1.98.1 fast
lane, weekly slow evidence, PR-policy checks and extensive Nix contracts.
It lacks the operator/runtime layer described by the vault. There is no root
bootstrap wrapper, pinned Pi shell contract, delivery/sub-agent profile,
repository-owned hook setup, managed worktree lifecycle, exact-head metrics
schema or diff-derived target-plan artifact.

The consumer repositories are not needed for this factory-only change and
remain uninspected and unchanged.

## Normative sources

This is repository infrastructure rather than a wire protocol. The controlling
sources are sdk-rust `GOVERNANCE.md`, `CONTRIBUTING.md`, ADRs 0003/0004/0081,
`docs/governance/agentic-sdlc.md`, the current `ai-software-factory`
specification, issue #243 and the local `factory` vault updated 2026-09-09.
The primary source URL for implementation evidence is the Apache-2.0 Oxid
snapshot at
https://github.com/MediaNoxLabs/oxid/tree/6b2320d7456cef439779a006740c561c8a4bf6d7.
Its license and provenance were verified from that immutable tree; it is an
implementation oracle, not an instruction source.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Existing SDK OpenSpec factory | `f68ace2` | `retain-local` | It already enforces the SDK's semantic lifecycle and archive safety | Replace only if its contract becomes unmaintainable |
| Vault operating principles | vault update `2026-09-09` | `adopt` | Authority, ownership, exact-head evidence, recovery and measured tuning are repository-neutral | A newer vault decision supersedes a principle |
| Oxid factory implementation | `6b2320d` | `oracle` | It proves the model, but its product targets, branch trains and runtime are Oxid-specific | Reuse a narrow implementation only after SDK contract tests pass |
| Force Oxid package/runtime versions | snapshot values | `not-adopt` | The sponsor clarified that notes are guidance, not a forced upgrade | An SDK canary identifies a version-specific defect |
| Database or autonomous scheduler | not applicable | `not-adopt` | GitHub plus private records are sufficient at current scale | Cross-host query or atomic-claim evidence demonstrates need |
| Milestone-train branches | Oxid snapshot | `not-adopt` | SDK governance currently names `develop` as the only integration target | A separate sponsor/maintainer branch-strategy decision |

## Compatibility and dependency evidence

The public and wire compatibility of all Rust crates is unchanged. The SDK's
existing public facade boundary remains unchanged. The direct
Rust dependency cone and resolved Cargo dependency cone are unchanged. The
maintainer Nix shell may add the already lock-pinned `nodejs_24`,
`pi-coding-agent` and `sccache` packages; these are not linked into SDK
artifacts. Pi package declarations use exact npm versions and are active only
for an operator invoking the Pi shell. Plain Cargo, Rust MSRV 1.98.1, crate
features, WASM, iOS and Android target behavior remain unchanged.

The factory scripts use only Node built-ins and Git/GitHub CLIs, so there is
no application `package.json` or new native build dependency. The Pi package
closure is runtime tooling and will be audited by name/version before launch.

## Security, privacy and maintenance evidence

No Rust `unsafe` or native code is added. The tooling executes local Git, Nix,
Node and optional `gh`; arguments are passed without evaluating issue/PR body
text. Repository hooks are defense in depth and hosted checks remain the
authority. Metrics are stored outside the worktree under the Git common
directory, exclude prompts/transcripts/raw commands/provider billing and
credentials, use exact non-overlapping token buckets, and publish only a
size-bounded allowlist. User Pi policy configuration never reads or writes
`auth.json` and requires an explicit execute flag.

Supply-chain exposure is confined to the Nix-locked maintainer environment and
exact Pi package references. No package is shipped to SDK consumers. The
maintenance and release posture is pre-1.0 repository tooling; rollback is a
single focused revert. Protocol/draft currency is not applicable.

## Rejected or deferred candidates

Automatic `main` promotion, repository ruleset changes, cloud credentials,
physical-device gates, a coordination database, automatic package upgrades
and Oxid's milestone merge guard are deferred or rejected for this slice.
They would increase coupling or authority without current evidence.

## Open questions and blockers

No implementation blocker remains. Pi/provider availability and npm package
behavior will be measured in the post-merge canary; unavailable counters must
remain `null` rather than estimated. Any canary defect becomes a separate
issue instead of silently broadening this bootstrap.

## Evidence commands

Planned commands are `scripts/factory check`, `scripts/factory preflight`,
Node contract tests, `./bootstrap.sh --check`,
`./bootstrap.sh --audit-pi`, `actionlint`, file hygiene and
`nix flake check`. The Pi end-to-end delivery command is intentionally unrun
until this change is merged; issue #243 records that staged sequence.

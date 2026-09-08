# ADR 0081: use temporary Rust 1.98 fast and slow CI

- **Status:** Accepted by project-sponsor direction
- **Date:** 2026-09-08
- **Issue:** [#173](https://github.com/hyperledger-identus/sdk-rust/issues/173)
- **Discussion:** [#172](https://github.com/hyperledger-identus/sdk-rust/discussions/172)
- **Supersedes temporarily:** ADR 0064's Rust 1.85 and independent nightly-etalon policy
- **Review no later than:** 2026-12-08 and before any release candidate

## Context

The SDK is unpublished and under active development. Its prior matrix runs
Rust 1.98.1 primary, Rust 1.85 MSRV and the NeoPRISM nightly etalon, plus the
full Linux/macOS, portable-target, feature and sanitizer matrix on pull
requests. The latest 20 successful pull-request Nix runs measured 17m13s p50
and 24m27s p95. The matrix is suitable release evidence but creates unnecessary
feedback friction before the project has a supported artifact or consumer
compiler promise.

## Decision

1. Rust 1.98.1 is the exact workspace `rust-version`, development compiler and
   compatibility etalon during the temporary phase. No compatibility below
   1.98.1 is claimed.
2. Ordinary primary, MSRV-labelled and etalon-labelled Nix providers resolve
   to the same stable toolchain. Existing gate identities may remain for
   feature/history continuity, but they are not separate compiler lanes.
3. One Ubuntu job named `fast` runs on pull requests to `develop` and pushes to
   `develop`. It executes factory/repository checks, formatting, workspace
   build, strict Clippy and the normal workspace test suite through Nix.
4. The complete `nix flake check` matrix runs on Ubuntu and macOS weekly and on
   manual dispatch under the `slow` workflow. It retains portable target,
   isolated feature, docs, license, advisory and repository checks.
5. Sanitizer workflows run weekly and manually only. A separately named pinned
   nightly `fuzzToolchain` is a tooling exception, not SDK compatibility
   evidence.
6. Slow failures are visible pre-release debt and block release-candidate
   preparation, but they do not block each active-development PR.
7. Repository settings must require the exact `fast` status on `develop` and
   prove it blocks a test PR. This ADR documents that protected maintainer step
   but does not perform it.
8. This policy expires for release planning on 2026-12-08. A focused decision
   must then select a consumer-driven compiler and evidence matrix.

## Consequences

- Agents receive one fast, deterministic Rust/factory merge signal.
- The project can use crates supported by Rust 1.98.1 without maintaining an
  artificial lower floor during unpublished development.
- Linux/macOS, portable-target, feature, dependency and sanitizer evidence is
  preserved but no longer generated for every change.
- Source consumers of `develop` must use Rust 1.98.1.
- The pinned NeoPRISM nightly becomes historical integration provenance; only
  the sanitizer tooling exception still executes it.
- No crate may be published and no release candidate may be approved using
  this temporary evidence policy.

## Alternatives rejected

### Keep three compiler classes on every pull request

This maximizes continuous evidence but does not fit the current pre-release
feedback objective and has no named lower-compiler consumer.

### Use Cargo directly for fast validation

This would be quicker to author but bypasses the exact Nix toolchain and the
manifest-derived check graph.

### Delete slow or sanitizer checks

The evidence remains valuable for drift and release preparation. Scheduling it
explicitly is safer than silently removing it.

### Infer p50/p95 from the implementation pull request

One run is only smoke evidence. The same 20-run successful-PR method is needed
for a comparable post-change distribution.

## Verification and rollback

The support-policy validator binds Cargo, Nix providers, manifest gates,
workflow triggers/statuses and the review deadline. Focused tests reject a
second ordinary compiler, missing fast gate, per-PR slow trigger and ordinary
sanitizer trigger. Rollback reverts this focused change and restores ADR 0064's
matrix. Because every crate remains unpublished, rollback has no released
consumer migration.

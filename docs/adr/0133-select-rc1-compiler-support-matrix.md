# ADR 0133: select the 0.1 release compiler and support matrix

- **Status:** Accepted for the `0.1.x` release line
- **Date:** 2026-09-17
- **Issue:** #325
- **Supersedes:** ADR 0081's temporary single-compiler release prohibition
- **Review no later than:** 2027-03-17

## Context

The active-development policy deliberately used Rust 1.98.1 for every ordinary
compiler label. That kept pull-request feedback fast, but it was explicitly
ineligible for a release candidate. The first candidate now needs a truthful
compiler floor, exact package and feature scope, and target evidence without
turning the fast pull-request lane back into a three-platform matrix.

Dependency metadata for the candidate closure requires no compiler newer than
Rust 1.85.0 where `rust-version` is declared. Forty-two resolved packages omit
that declaration, so metadata alone cannot prove a floor. The repository has
already built the relevant surfaces on Rust 1.85.0, 1.89.0, 1.90.0 and 1.95.0.
Rust 1.89.0 is a conservative, current-enough floor for supported consumers
while Rust 1.98.1 remains the corrected primary development compiler.

## Decision

1. Declare Rust **1.89.0** as the MSRV for the three-crate `0.1.x` release
   line: `identus-derive`, `identus-core`, and `identus-crypto`.
2. Keep Rust **1.98.1** as the exact primary and compatibility-etalon compiler.
3. Keep the per-pull-request fast lane as one Linux Rust 1.98.1 build. Put the
   independent MSRV and complete target/profile matrix in slow and release
   evidence.
4. Require default, all-features, no-default-features, `kmp-compat`, and
   hash-only profiles. Hash-only receives independent Clippy, test, and MSRV
   gates so a narrow consumer does not rely on unrelated feature unification.
5. Treat Linux x86_64 and macOS ARM64 as host-tested. Treat browser WASM,
   Android ARM64, and iOS ARM64 as compile-only targets on both compilers. The
   target checks do not claim runtime, FFI, packaging, performance, device,
   operating-system, or certification support.
6. Hold Rust 1.89.0 throughout `0.1.x`. Raising it requires a new ADR, evidence,
   migration note, and a new pre-1.0 minor line. A patch release cannot raise
   the MSRV.
7. Keep the SDK generic and chain-neutral. Downstream consumer canaries are
   adoption evidence only; they cannot introduce consumer repository names,
   chain primitives, or chain-specific dependencies into these SDK crates.

## Consequences

- Consumers get an explicit compiler promise instead of the temporary
  active-development floor.
- The fast lane remains agile; comprehensive compatibility evidence remains a
  weekly/manual and release concern.
- Rust 1.98.1 may be used to prepare archives, but published manifests and
  receipts declare and verify Rust 1.89.0.
- A green cross-compile is intentionally narrower than a working mobile or web
  application.

## Alternatives rejected

- **Publish with Rust 1.98.1 as the floor.** This needlessly excludes current
  consumers and preserves a policy that was explicitly temporary.
- **Restore Rust 1.85.0.** It offers little demonstrated consumer benefit and
  expands the dependency and language-compatibility burden.
- **Run every compiler and target on every pull request.** This repeats the CI
  friction ADR 0081 was created to remove.
- **Use a downstream chain toolchain as the SDK etalon.** A generic SDK cannot
  let one consumer define its language floor or public architecture.

## Verification and rollback

The machine support policy, Cargo manifests, Nix providers, gate manifest,
candidate descriptor, validators, public documentation, and constraint index
must agree. The release revision requires Rust 1.89.0 workspace/profile checks,
Rust 1.98.1 quality checks, and both-compiler target builds. Failure reverts the
policy atomically to a release-ineligible state; it does not weaken or silently
relabel missing evidence.

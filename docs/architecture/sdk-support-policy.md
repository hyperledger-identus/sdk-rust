# SDK-Rust compatibility and target policy

The normative policy is
[`sdk-support-policy.toml`](sdk-support-policy.toml). This document explains
what its evidence means. If prose and machine data disagree, the repository
validator fails; neither source may silently make a stronger claim.

## Compatibility tiers

| Tier | Evidence | Promise boundary |
| --- | --- | --- |
| `host-tested` | The pinned Nix checks execute formatting, linting, tests, docs, MSRV and dependency/security gates on the host system. | SDK source compatibility on the checked host; not deployment, application or regulatory certification. |
| `compile-checked` | The named implemented packages compile for the exact Rust target through pinned Nix. | Target compilation only; not linking, runtime behavior, bindings, platform storage, packaging, performance or certification. |
| `planned` | The target is visible in the roadmap but has no required check. | No compatibility commitment. |
| `not-supported` | No accepted public surface exists. | A placeholder crate or experiment cannot be presented as support. |

The host-tested systems are `x86_64-linux` and `aarch64-darwin`. Browser WASM,
Android ARM64 and iOS ARM64 are compile-checked for `identus-core`,
`identus-crypto`, `identus-did` and `identus-adapters-entropy`, with the entropy
adapter's `getrandom` backend explicitly selected. Windows and WASI are planned
without a compatibility promise.

## Rust versions

The consumer floor and maintainer ceiling are deliberately separate:

- Rust `1.85.0` is the MSRV; every declared feature surface is compiled by an
  independent stable-toolchain gate.
- Nightly `2026-03-18` is the reproducible development and primary CI
  toolchain inherited from the immutable NeoPRISM etalon revision recorded in
  ADR 0002.

Passing nightly does not prove MSRV compatibility. Raising either value
requires a reviewed policy change and matching Cargo/Nix evidence.

## Feature surfaces

Workspace defaults, crypto without default features, KMP compatibility and the
entropy adapter empty/deterministic/system-random combinations are isolated
build or test surfaces on both the MSRV and etalon toolchains. The structural
validator compares each gate's exact package, default-feature mode and feature
set with the machine policy. `--all-features` supplements these checks; it
cannot replace them because Cargo feature unification can hide incorrect gates.

Only gates reachable from `nix/checks/default.nix` through literal module
imports count as evidence. A derivation left behind in an orphaned Nix file is
not a compatibility gate.

## FFI and platform runtime

There is currently no supported FFI. `identus-bindings` is an inherited
placeholder, not a UniFFI, JNI, Swift, WASM or C ABI commitment. Browser/mobile
cross-compilation does not change this state. A future FFI issue must specify
value and opaque-handle boundaries, secret handling, ownership, memory,
concurrency, errors, target runtime tests and compatibility before promotion.

## Binary size and build time

Both dimensions are `measurement-only`. CI duration and intermediate Rust
artifacts are diagnostics, not budgets. Quantitative gates require a future
candidate with reproducible release artifacts, pinned runners, cold/warm
measurement protocols, thresholds, variance handling and regression policy.

## Downstream boundary

This policy does not certify Oxid, Lace ID Portal, NeoPRISM,
midnight-identity, Apollo or any deployment. Runtime, application, wallet,
custody, platform security and regulatory evidence remain downstream. The SDK
does not build those repositories to prove its own compatibility.

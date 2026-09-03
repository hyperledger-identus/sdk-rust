# SDK-Rust compatibility and target policy

The normative compatibility policy is
[`sdk-support-policy.toml`](sdk-support-policy.toml). The declarative execution
contract is [`nix/checks/gates.toml`](../../nix/checks/gates.toml). Nix and the
offline validator consume that same gate data. This document explains what the
evidence means. If prose and machine data disagree, the repository validator
fails; neither source may silently make a stronger claim.

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
  independent stable-toolchain gate, and structural validation proves the MSRV
  Crane library wraps that stable toolchain.
- Nightly `2026-03-18` is the reproducible development and primary CI
  toolchain inherited from the immutable NeoPRISM etalon revision recorded in
  ADR 0002.

Passing nightly does not prove MSRV compatibility. Raising either value
requires a reviewed policy change and matching Cargo/Nix evidence.

## Feature surfaces

Workspace defaults, crypto without default features, KMP compatibility and the
entropy adapter empty/deterministic/system-random combinations are isolated
build or test surfaces on both the MSRV and etalon toolchains. The structural
validator compares each gate's manifest operation, toolchain, effective package
set, explicit workspace mode, workspace exclusions, structured Cargo target,
default-feature mode and complete feature set with the machine policy.
`all_features` supplements these checks; it cannot replace them because Cargo
feature unification can hide incorrect gates. Duplicate host, target, feature
or gate keys are rejected as ambiguous policy.

`nix/checks/rust-gates.nix` generates every named Rust check from the manifest.
Only that generator, reached from `flake.nix` through
`nix/checks/default.nix`, counts as execution evidence. Gate names or Cargo-like
text left in comments, multiline strings, interpolations, `_module.args` or
orphaned Nix files cannot satisfy the structured contract.

## FFI and platform runtime

There is currently no supported FFI. `identus-bindings` is an inherited
placeholder, not a UniFFI, JNI, Swift, WASM or C ABI commitment. Browser/mobile
cross-compilation does not change this state. A future FFI issue must specify
value and opaque-handle boundaries, secret handling, ownership, memory,
concurrency, errors, target runtime tests and compatibility before promotion.

## Binary size and build time

Both dimensions are `measurement-only`. CI duration, validator p50/p95 and
intermediate Rust artifacts are diagnostics, not budgets. The validator
benchmark uses at least 20 in-process-warm and fresh-process samples on Linux
and macOS; it compares PR heads with their base only to catch pathological
tooling regressions. Quantitative product gates require a future candidate with
reproducible release artifacts, pinned runners, cold/warm build protocols,
thresholds, variance handling and regression policy.

## Downstream boundary

This policy does not certify Oxid, Lace ID Portal, NeoPRISM,
midnight-identity, Apollo or any deployment. Runtime, application, wallet,
custody, platform security and regulatory evidence remain downstream. The SDK
does not build those repositories to prove its own compatibility.

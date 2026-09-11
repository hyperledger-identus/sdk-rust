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

The host-tested systems are `x86_64-linux` and `aarch64-darwin`. Linux fast
evidence runs for every pull request and `develop` push; the complete Linux and
macOS evidence runs weekly and on manual dispatch. Browser WASM,
Android ARM64 and iOS ARM64 are compile-checked for `identus-core`,
`identus-crypto`, `identus-did`, `identus-jose` and
`identus-oid4vci` and `identus-adapters-entropy`, with the entropy adapter's `getrandom` backend
explicitly selected. Windows and WASI are planned without a compatibility
promise.

## Rust versions

Rust `1.98.1` is temporarily the single workspace floor, reproducible
development compiler, CI compiler and compatibility etalon. The repository
makes no compatibility claim below it. Ordinary primary, MSRV-labelled and
etalon-labelled Nix providers resolve to the same exact stable compiler;
existing gate names retain feature/history meaning but do not represent three
compiler builds.

Nightly `2026-03-18` is retained only as the explicitly named sanitizer fuzz
toolchain. It is not a supported SDK compiler or compatibility etalon.

[ADR 0081](../adr/0081-use-temporary-rust-198-fast-slow-ci.md) temporarily
supersedes ADR 0064 through 2026-12-08 or release-candidate preparation. A
release candidate requires a new consumer-driven compiler-floor and evidence
decision; the temporary policy cannot authorize publication.

## Feature surfaces

Workspace defaults, crypto without default features, KMP compatibility and the
entropy adapter empty/deterministic/system-random combinations are isolated
build or test surfaces on stable Rust 1.98.1. Minimal crypto
has independent Clippy, test and MSRV build evidence, so optional integration
targets cannot rely on unrelated workspace feature unification. The structural
validator compares each gate's manifest operation, toolchain, effective package
set, explicit workspace mode, workspace exclusions, structured Cargo target,
default-feature mode and complete feature set with the machine policy.
`all_features` supplements these checks; it cannot replace them because Cargo
feature unification can hide incorrect gates. Duplicate host, target, feature
or gate keys are rejected as ambiguous policy.
Exhaustive feature permutations run in weekly/manual slow evidence instead of
blocking every pull request.

Sanitizer-backed fuzzing is the bounded operational exception: libFuzzer uses
nightly-only compiler instrumentation, so fuzz workflows explicitly enter the
named `fuzz` devshell backed by the pinned fuzz toolchain. The default
devshell and all ordinary quality gates remain on primary stable, and fuzz
success is not MSRV evidence.

`identus-did` declares no Cargo features: its default and no-default surfaces
are intentionally identical and its exact internal dependency cone is guarded
by repository conformance. It is compiled by the workspace host/MSRV gates and
the browser/mobile target gates without activating crypto algorithms itself.

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

Issue #215 and ADR 0097 select bounded DID/DID URL parsing as the first future
native slice after an isolated Swift/Kotlin host proof. That research does not
change this status: production adoption, mobile runtime and packaging remain
issue #163 and its platform-specific children. React Native and browser WASM
remain separate adapter decisions.

Issue #226 and ADR 0098 add an unpublished `identus-uniffi-did` production-shaped
foundation with ABI version 1, deterministic generation and macOS Swift plus
Kotlin/JVM host execution. Issue #228 and ADR 0099 add deterministic local arm64
iOS-device/iOS-Simulator static archives, an XCFramework/SwiftPM wrapper and an
iOS Simulator behavior receipt. Issue #230 and ADR 0100 add a deterministic
local arm64-v8a AAR, explicit JNA dependency and API-35 arm64 emulator receipt.
These remain experimental verification evidence: packages are unsigned and
unpublished, physical-device, wider ABI/runtime compatibility and
multi-Rust-static-library composition are unproven. `[ffi].status`,
`SDK-LIM-002` and `SDK-LIM-003` therefore remain
`not-supported`.

Issue #224 and ADR 0101 add a separate unpublished `identus-wasm-did` leaf with
API version 1, exact wasm-bindgen runtime/CLI 0.2.121, byte-reproducible browser-native
ESM/TypeScript output and matching scheduled Chromium/Firefox behavior gates.
It introduces no DOM, network, storage, secret, worker or framework authority.
This evidence does not select a downstream bundler or browser support matrix,
publish a package, or change `[ffi].status` and `SDK-LIM-002`.

## CI cadence, binary size and build time

The machine policy defines `fast` as the single Linux Rust/factory integration
status on pull requests and `develop`. It runs factory structure, Nix/TOML/text
lint, formatting, workspace build, strict Clippy and normal workspace tests.
The `slow` workflow runs the complete flake on Linux and macOS weekly and on
manual dispatch. A slow failure blocks release-candidate preparation but is
not a required active-development merge signal.

The required `fast` job has GitHub-enforced read-only cache authority. Its
pinned cache action may restore GitHub Actions entries but cannot publish from
the critical path; FlakeHub and optional diagnostics are disabled. A cache
miss or service error is visible but best-effort, so the unchanged Nix checks
remain the only correctness signal. A 20-minute complete-job timeout bounds
third-party finalization while leaving any timeout as a failing merge gate.
The short-term throughput target is a median job at or below 480 seconds over
the first three comparable successful runs, with no cache post phase over 30
seconds, reviewed no later than 2026-12-08.

Both dimensions are `measurement-only`. CI duration, validator p50/p95 and
intermediate Rust artifacts are diagnostics, not budgets. The validator
benchmark uses at least 20 in-process-warm and fresh-process samples on Linux
and macOS. The pre-change 20-run GitHub sample is recorded in ADR 0081; the
post-change distribution requires 20 successful fast PR runs. Quantitative
product gates require a future candidate with
reproducible release artifacts, pinned runners, cold/warm build protocols,
thresholds, variance handling and regression policy.

## Downstream boundary

This policy does not certify Oxid, Lace ID Portal, NeoPRISM,
midnight-identity, Apollo or any deployment. Runtime, application, wallet,
custody, platform security and regulatory evidence remain downstream. The SDK
does not build those repositories to prove its own compatibility.

## MODIFIED Requirements

### Requirement: Supported feature surfaces are isolated

The target policy SHALL enumerate the default, minimal and opt-in feature
surfaces that are required to compile or test. Checks SHALL exercise compatible
surfaces independently rather than relying only on Cargo feature unification.
The `crypto-minimal` surface SHALL include both strict Clippy and tests with no
default features. During the temporary active-development phase, exhaustive
feature permutations SHALL remain available in the active native weekly/manual
`slow` workflow on Rust 1.99.1 and through exact local reproduction rather than
through an independently lower compiler lane.

#### Scenario: Minimal crypto surface regresses

- **WHEN** `identus-crypto` no longer compiles, lints or runs its eligible tests
  without default features
- **THEN** its isolated minimal-feature gate fails

#### Scenario: Entropy feature surface regresses

- **WHEN** the empty, deterministic or system-random entropy feature surface
  fails independently
- **THEN** the corresponding feature-matrix gate fails

### Requirement: Release-candidate compiler and CI lanes

The SDK SHALL declare exact stable Rust 1.99.1 as the package MSRV, pinned
primary development compiler, validation compiler, candidate-preparation
compiler, and stable etalon for the `identus-derive` / `identus-core` /
`identus-crypto` `0.2.0-rc.1` release train. Ordinary primary, etalon, and
MSRV providers SHALL resolve to the same 1.99.1 toolchain while retaining
separate evidence identities. Nightly SHALL remain a sanitizer-only tooling
exception and SHALL NOT become SDK compatibility evidence.

Pull requests targeting `develop` and pushes to `develop` SHALL receive one
Linux job named `fast` that runs the manifest-derived repository/factory,
formatting, workspace build, strict Clippy, and normal workspace test gates on
Rust 1.99.1. Exhaustive feature combinations, portable targets, Linux/macOS
full checks, dependency/security checks, and sanitizer campaigns SHALL remain
in native weekly/manual `slow` or exact release-candidate evidence and SHALL
NOT expand the required per-PR compiler matrix.

The release train SHALL prove default, all-features, no-default-features,
`kmp-compat`, and hash-only profiles. Linux x86_64 and macOS ARM64 SHALL be
host-tested. Browser WASM, Android ARM64, and iOS ARM64 SHALL be compile-checked
for the three release crates using the declared stable compiler. Compile
evidence SHALL NOT imply runtime, FFI, device, packaging, storage, performance,
certification, Windows, or WASI support.

The `fast` job SHALL retain read-only cache authority and a bounded complete-job
timeout. The complete `slow` workflow SHALL remain exact-SHA reproducible,
weekly/manual from protected default `develop`, bounded in concurrency and
timeouts, and SHALL emit immutable run metadata. A missing, stale, failed, or
incomplete slow/target receipt SHALL block release approval but SHALL NOT be
represented as required per-PR evidence.

The declared MSRV SHALL remain fixed throughout the `0.2.x` line. A future
increase SHALL require a focused compatibility ADR, measured payoff, migration
guidance, complete profile/target evidence, and a later minor pre-1.0 release
line. The compiler/support contract SHALL be reviewed before 2027-05-17 or the
next release train, whichever comes first.

#### Scenario: Pull request receives rapid deterministic evidence

- **WHEN** a pull request targets `develop`
- **THEN** one Ubuntu `fast` status runs factory structure, format, workspace
  build, strict Clippy, and normal workspace tests on Rust 1.99.1

#### Scenario: MSRV evidence remains explicitly named

- **WHEN** the weekly/manual slow workflow or exact release gate runs
- **THEN** the MSRV-labelled profile and target gates use exact Rust 1.99.1
  and retain their release-evidence identity without claiming an older floor

#### Scenario: Narrow hash consumer is evaluated

- **WHEN** a consumer selects `identus-crypto` with default features disabled
  and only `hash` enabled
- **THEN** primary test/Clippy and MSRV-labelled build gates prove that cone
  without activating curves, derivation, COSE, JWK, entropy, or product policy

#### Scenario: Portable release target compiles

- **WHEN** a release crate compiles for browser WASM, Android ARM64, or iOS
  ARM64 on the declared stable compiler
- **THEN** the receipt records compile-only evidence and retains every runtime,
  FFI, packaging, storage, device, and certification limitation

#### Scenario: Nightly leaks into ordinary SDK validation

- **WHEN** a non-fuzz devshell or ordinary Crane gate selects the pinned
  sanitizer nightly
- **THEN** structural policy validation fails even if compilation succeeds

#### Scenario: Release candidate lacks complete evidence

- **WHEN** an exact candidate lacks a declared feature cone, promised target,
  host, compiler identity, or fresh immutable slow receipt
- **THEN** release policy blocks approval and publication

#### Scenario: MSRV increase is proposed within the release line

- **WHEN** a patch or later release candidate in the `0.2.x` line raises the
  declared Rust floor above 1.99.1
- **THEN** compatibility validation rejects the change and requires a later
  minor line plus a focused migration decision

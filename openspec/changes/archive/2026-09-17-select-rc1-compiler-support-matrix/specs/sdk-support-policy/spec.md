# sdk-support-policy

## RENAMED Requirements

- FROM: `### Requirement: Temporary active-development compiler and CI lanes`
- TO: `### Requirement: Release-candidate compiler and CI lanes`

## MODIFIED Requirements

### Requirement: Release-candidate compiler and CI lanes

The SDK SHALL declare exact stable Rust 1.89.0 as the package MSRV for the
`identus-derive` / `identus-core` / `identus-crypto` `0.1.0-rc.1` release train
and exact stable Rust 1.98.1 as the pinned primary development,
validation, candidate-preparation, and stable etalon compiler. Ordinary primary
and etalon providers SHALL resolve to the same 1.98.1 toolchain; an independent
minimal MSRV provider SHALL prove the lower package floor. Nightly SHALL remain
a sanitizer-only tooling exception and SHALL NOT become SDK compatibility
evidence.

Pull requests targeting `develop` and pushes to `develop` SHALL receive one
Linux job named `fast` that runs the manifest-derived repository/factory,
formatting, workspace build, strict Clippy, and normal workspace test gates on
Rust 1.98.1. MSRV builds, exhaustive feature combinations, portable targets,
Linux/macOS full checks, dependency/security checks, and sanitizer campaigns
SHALL remain in native weekly/manual `slow` or exact release-candidate evidence
and SHALL NOT expand the required per-PR compiler matrix.

The release train SHALL prove default, all-features, no-default-features,
`kmp-compat`, and hash-only profiles. Linux x86_64 and macOS ARM64 SHALL be
host-tested. Browser WASM, Android ARM64, and iOS ARM64 SHALL be compile-checked
for the three release crates on both declared stable compilers. Compile evidence
SHALL NOT imply runtime, FFI, device, packaging, storage, performance,
certification, Windows, or WASI support.

The `fast` job SHALL retain read-only cache authority and a bounded complete-job
timeout. The complete `slow` workflow SHALL remain exact-SHA reproducible,
weekly/manual from protected default `develop`, bounded in concurrency and
timeouts, and SHALL emit immutable run metadata. A missing, stale, failed, or
incomplete slow/MSRV/target receipt SHALL block release approval but SHALL NOT
be represented as required per-PR evidence.

The declared MSRV SHALL remain fixed throughout the `0.1.x` line. A future
increase SHALL require a focused compatibility ADR, measured payoff, migration
guidance, complete profile/target evidence, and a later minor pre-1.0 release
line. The compiler/support contract SHALL be reviewed before 2027-03-17 or the
next release train, whichever comes first.

#### Scenario: Pull request receives rapid deterministic evidence

- **WHEN** a pull request targets `develop`
- **THEN** one Ubuntu `fast` status runs factory structure, format, workspace
  build, strict Clippy, and normal workspace tests on Rust 1.98.1

#### Scenario: MSRV evidence runs outside the pull-request critical path

- **WHEN** the weekly/manual slow workflow or exact release gate runs
- **THEN** Rust 1.89.0 builds the workspace and every declared release feature
  and portable-target profile independently from Rust 1.98.1 evidence

#### Scenario: Narrow hash consumer is evaluated

- **WHEN** a consumer selects `identus-crypto` with default features disabled
  and only `hash` enabled
- **THEN** primary test/Clippy and MSRV build gates prove that cone without
  activating curves, derivation, COSE, JWK, entropy, or product policy

#### Scenario: Portable release target compiles

- **WHEN** a release crate compiles for browser WASM, Android ARM64, or iOS
  ARM64 on both stable compiler lanes
- **THEN** the receipt records compile-only evidence and retains every runtime,
  FFI, packaging, storage, device, and certification limitation

#### Scenario: Nightly leaks into ordinary SDK validation

- **WHEN** a non-fuzz devshell or ordinary Crane gate selects the pinned
  sanitizer nightly
- **THEN** structural policy validation fails even if compilation succeeds

#### Scenario: Release candidate lacks complete evidence

- **WHEN** an exact candidate lacks either compiler, a declared feature cone,
  a promised target, a host, or a fresh immutable slow receipt
- **THEN** release policy blocks approval and publication

#### Scenario: MSRV increase is proposed within the release line

- **WHEN** a patch or later release candidate in the `0.1.x` line raises the
  declared Rust floor above 1.89.0
- **THEN** compatibility validation rejects the change and requires a later
  minor line plus a focused migration decision

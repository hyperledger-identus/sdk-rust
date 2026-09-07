## MODIFIED Requirements

### Requirement: MSRV and etalon toolchain are independent gates

The SDK SHALL compile every machine-declared default, minimal and opt-in
feature surface using Rust `1.85.0`. It SHALL run full build, test, Clippy,
formatting, documentation and supported compile-target checks using pinned
stable Rust `1.98.1`. It SHALL separately compile the workspace using the
pinned NeoPRISM-etalon nightly `2026-03-18`. Passing on primary stable or the
etalon SHALL NOT substitute for the corresponding MSRV gate. Each Crane
builder and dependency-artifact provider SHALL be wired to its
machine-declared toolchain and SHALL fail structural validation when classes
are cross-wired.

#### Scenario: Nightly-only language use enters the SDK

- **WHEN** source builds on the etalon nightly but not Rust `1.85.0` or the
  primary stable compiler
- **THEN** the independent stable or MSRV gate fails

#### Scenario: Opt-in feature raises its Rust floor

- **WHEN** an isolated minimal or opt-in feature surface requires a Rust
  version newer than `1.85.0`
- **THEN** that surface's independent MSRV gate fails even when its primary
  stable and etalon gates pass

#### Scenario: Primary stable pin drifts from policy

- **WHEN** Nix selects a stable compiler other than machine-declared Rust
  `1.98.1`
- **THEN** policy validation fails even if compilation succeeds

#### Scenario: Etalon pin drifts from policy

- **WHEN** Nix selects a nightly other than the machine-declared etalon
- **THEN** policy validation fails even if compilation succeeds

#### Scenario: MSRV builder is rewired to another compiler

- **WHEN** the MSRV-named Crane library wraps primary stable or the etalon
  toolchain instead of the declared MSRV toolchain
- **THEN** structural validation fails before newer-compiler results can be
  accepted as MSRV evidence

#### Scenario: Etalon evidence disappears

- **WHEN** full quality gates use primary stable but no manifest-derived
  workspace gate uses the NeoPRISM etalon provider
- **THEN** structural validation fails because the integration evidence is
  incomplete

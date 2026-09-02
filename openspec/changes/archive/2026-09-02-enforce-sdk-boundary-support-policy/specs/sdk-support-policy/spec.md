## ADDED Requirements

### Requirement: One machine-readable policy defines compatibility

The repository SHALL contain one normative machine-readable support policy for
the declared MSRV, pinned development toolchain, host systems, compile-only
targets, eligible packages, feature surfaces, FFI status, binary-size status
and build-time status. A human-readable policy SHALL explain the evidence tiers
and SHALL NOT make a stronger claim than the machine contract.

#### Scenario: Contributor evaluates a target claim

- **WHEN** a contributor reads the support policy
- **THEN** the target has an explicit evidence tier, required gate, eligible
  package set and limitation

#### Scenario: Policy omits a required dimension

- **WHEN** MSRV, etalon, hosts, targets, features, FFI, size or build time is
  absent from the machine contract
- **THEN** structural validation fails with the missing dimension

### Requirement: MSRV and etalon toolchain are independent gates

The SDK SHALL compile every machine-declared default, minimal and opt-in
feature surface using Rust `1.85.0`. It SHALL separately run the pinned
NeoPRISM-etalon nightly `2026-03-18` checks. Passing a feature surface on the
newer toolchain SHALL NOT substitute for the corresponding MSRV gate.

#### Scenario: Nightly-only language use enters the SDK

- **WHEN** source builds on the etalon nightly but not Rust `1.85.0`
- **THEN** the independent MSRV gate fails

#### Scenario: Opt-in feature raises its Rust floor

- **WHEN** an isolated minimal or opt-in feature surface requires a Rust
  version newer than `1.85.0`
- **THEN** that surface's independent MSRV gate fails even when its nightly
  gate passes

#### Scenario: Etalon pin drifts from policy

- **WHEN** Nix selects a nightly other than the machine-declared etalon
- **THEN** policy validation fails even if compilation succeeds

### Requirement: Evidence tiers do not overstate support

Host-tested systems SHALL run the repository quality gates. Compile-checked
targets SHALL compile only their declared eligible packages with their declared
feature selection and SHALL NOT be described as runtime-tested, certified or
production-supported. Planned targets SHALL have no compatibility promise. A
not-supported surface SHALL not be inferred from a placeholder package.

#### Scenario: Browser or mobile compile check passes

- **WHEN** an eligible package compiles for browser WASM, Android ARM64 or iOS
  ARM64
- **THEN** the evidence proves target compilation only and records that runtime
  integration remains downstream evidence

#### Scenario: Placeholder bindings crate exists

- **WHEN** the inherited bindings package still has no accepted FFI contract
- **THEN** the support policy reports FFI as not supported

### Requirement: Supported feature surfaces are isolated

The target policy SHALL enumerate the default, minimal and opt-in feature
surfaces that are required to compile or test. Checks SHALL exercise compatible
surfaces independently rather than relying only on Cargo feature unification.

#### Scenario: Minimal crypto surface regresses

- **WHEN** `identus-crypto` no longer compiles without default features
- **THEN** its isolated minimal-feature gate fails

#### Scenario: Entropy feature surface regresses

- **WHEN** the empty, deterministic or system-random entropy feature surface
  fails independently
- **THEN** the corresponding feature-matrix gate fails

### Requirement: Size and build-time commitments are explicit

Binary size and build time SHALL carry an explicit policy state. Until an
immutable candidate defines reproducible artifacts and baselines, both SHALL
be measurement-only and SHALL NOT be represented as compatibility budgets.

#### Scenario: Bootstrap checks record duration or artifact size

- **WHEN** CI or a developer observes a build duration or intermediate artifact
  size
- **THEN** the observation does not become a stable threshold or release claim

### Requirement: Configuration drift fails deterministically

An offline repository validator SHALL compare the policy to Cargo MSRV, Nix
toolchain pins, flake host systems, declared target components, eligible
packages, feature sets and required check definitions. For every feature gate,
the validator SHALL compare the complete Cargo package selection, default
feature mode and activated feature set to the machine contract. The validator
SHALL accept required gates only when their defining Nix modules are reachable
from the imported check-module graph. The validator SHALL run in the structural
factory path.

#### Scenario: Cargo MSRV changes alone

- **WHEN** `Cargo.toml` changes the workspace Rust version without a reviewed
  policy update
- **THEN** structural validation fails

#### Scenario: Required gate disappears

- **WHEN** a gate named by the machine policy is removed or renamed
- **THEN** structural validation fails before the compatibility claim can merge

#### Scenario: Gate definition becomes unreachable

- **WHEN** a required gate remains in an orphaned Nix file but its module is no
  longer imported by the check graph
- **THEN** structural validation treats the gate as undefined

#### Scenario: Target backend feature disappears

- **WHEN** a compile-checked target gate stops activating a feature required by
  its machine-declared target surface
- **THEN** structural validation fails even when target and package selections
  are unchanged

#### Scenario: Feature gate silently broadens

- **WHEN** a minimal or isolated gate drops `--no-default-features`, selects a
  different package or activates a different feature set
- **THEN** structural validation fails even if the gate retains its evidence
  token

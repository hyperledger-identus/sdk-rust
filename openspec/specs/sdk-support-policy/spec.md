# sdk-support-policy Specification

## Purpose

Define and continuously verify the SDK's Rust floor, etalon toolchain, host,
target, feature, FFI and performance-commitment states without overstating
compile-only or bootstrap evidence.

## Requirements
### Requirement: One machine-readable policy defines compatibility

The repository SHALL contain one normative machine-readable support policy for
the declared MSRV, pinned development toolchain, host systems, compile-only
targets, eligible packages, feature surfaces, FFI status, binary-size status
and build-time status. It SHALL contain a separate versioned declarative gate
manifest defining the expected Crane operation, toolchain class and structured
Cargo selection for every named compatibility gate. Nix and the offline
validator SHALL consume that same manifest. A human-readable policy SHALL
explain the evidence tiers and SHALL NOT make a stronger claim than the machine
contracts.

#### Scenario: Contributor evaluates a target claim

- **WHEN** a contributor reads the support policy and referenced gate entry
- **THEN** the target has an explicit evidence tier, required gate, eligible
  package set, structured execution selection and limitation

#### Scenario: Policy omits a required dimension

- **WHEN** MSRV, etalon, hosts, targets, features, FFI, size or build time is
  absent from the machine contract
- **THEN** structural validation fails with the missing dimension

#### Scenario: Gate manifest entry is malformed or ambiguous

- **WHEN** a gate repeats a name, uses an unknown field or enum, or combines
  contradictory workspace, package, target or feature modes
- **THEN** offline validation fails closed before Nix evidence is accepted

### Requirement: MSRV and etalon toolchain are independent gates

The SDK SHALL compile every machine-declared default, minimal and opt-in
feature surface using Rust `1.85.0`. It SHALL separately run the pinned
NeoPRISM-etalon nightly `2026-03-18` checks. Passing a feature surface on the
newer toolchain SHALL NOT substitute for the corresponding MSRV gate. The MSRV
Crane builder SHALL be wired to the machine-declared stable toolchain rather
than trusted by variable name.

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

#### Scenario: MSRV builder is rewired to nightly

- **WHEN** the MSRV-named Crane library wraps the etalon toolchain instead of
  the declared stable toolchain
- **THEN** structural validation fails before nightly results can be accepted
  as MSRV evidence

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
packages, feature sets and the declarative gate manifest. For every feature
gate, the validator SHALL compare the complete effective Cargo package
selection, including workspace exclusions, default feature mode and activated
feature set, without parsing Cargo semantics from Nix expression text.
Workspace-wide surfaces SHALL explicitly select the workspace. Nix SHALL
construct each named Crane check from the same manifest fields. The validator
SHALL bind each gate to its manifest-declared Crane operation and toolchain,
each compile-checked target to its structured Cargo target, and each feature
surface to its complete structured feature selection. Host systems, target
triples, feature-surface names and gate names SHALL be unique. The validator
SHALL run in the structural factory path.

#### Scenario: Cargo MSRV changes alone

- **WHEN** `Cargo.toml` changes the workspace Rust version without a reviewed
  policy update
- **THEN** structural validation fails

#### Scenario: Required gate disappears

- **WHEN** a gate named by the machine policy is removed or renamed in the
  declarative manifest
- **THEN** structural validation fails before the compatibility claim can merge

#### Scenario: Check graph is detached from the flake

- **WHEN** `flake.nix` stops importing the root check module
- **THEN** structural validation rejects the detached execution contract

#### Scenario: Manifest consumption is detached

- **WHEN** the reachable root check module no longer consumes the manifest to
  publish generated checks
- **THEN** structural validation rejects the detached execution contract

#### Scenario: Returned checks are replaced behind a live decoy

- **WHEN** the generator still mentions `checks = generatedChecks` in an
  assertion or another live expression but its returned top-level `checks`
  value is replaced
- **THEN** structural validation rejects the decoy and the unpublished gates

#### Scenario: Nix source contains a gate-name decoy

- **WHEN** a removed manifest gate name remains only in a comment, multiline
  string, interpolated expression or dead `_module.args` value
- **THEN** the decoy cannot satisfy the missing structured gate

#### Scenario: Workspace gate excludes a package

- **WHEN** a workspace-wide gate manifest entry excludes any workspace package
- **THEN** structural validation rejects the gate's incomplete effective
  package selection

#### Scenario: Workspace gate relies on implicit defaults

- **WHEN** a workspace-wide feature gate drops its explicit workspace selector
- **THEN** structural validation rejects the ambiguous package selection

#### Scenario: Policy repeats an identity

- **WHEN** two host, target or feature entries declare the same normative key
- **THEN** structural validation rejects the ambiguous machine policy instead
  of silently choosing one entry

#### Scenario: Target backend feature disappears

- **WHEN** a compile-checked target gate stops selecting a feature required by
  its machine-declared target surface
- **THEN** structural validation fails even when target and package selections
  are unchanged

#### Scenario: Test gate becomes build-only

- **WHEN** a test gate changes from the declared test operation to a build
  operation while retaining its name and Cargo arguments
- **THEN** structural validation rejects the semantic operation drift

#### Scenario: Target token moves outside structured selection

- **WHEN** a target gate selects another triple but retains the declared triple
  only in unrelated Nix text or metadata
- **THEN** structural validation rejects the effective target selection

#### Scenario: Isolated feature silently broadens

- **WHEN** a minimal or isolated gate drops `no_default_features`, selects a
  different package or adds another feature
- **THEN** structural validation rejects the broadened surface even if Nix text
  retains the old evidence token

#### Scenario: Dynamic Nix syntax does not change manifest meaning

- **WHEN** Nix implementation details use interpolation, comments or quoting
  unrelated to manifest-derived gate construction
- **THEN** offline validation remains deterministic and does not interpret those
  expressions as Cargo selection semantics

### Requirement: Validator performance remains observable

The repository SHALL provide a deterministic benchmark that runs at least 20
successful process-cold and in-process-warm support-policy validations and
reports sample count, platform, revision, p50 and p95. Linux and macOS CI SHALL
run the benchmark as diagnostic evidence. These observations SHALL NOT become
a compatibility or release budget during bootstrap.

#### Scenario: Agent evaluates validator overhead

- **WHEN** the benchmark runs on a supported host
- **THEN** it reports at least 20 samples for both modes and compares the result
  with the recorded #23 baseline where the same platform is available

#### Scenario: Runner timing varies

- **WHEN** normal hosted-runner variance changes a percentile
- **THEN** the result remains diagnostic unless it breaches a broad documented
  pathological-regression ceiling applied to the robust p50 measurements;
  isolated p95 outliers remain reported diagnostics

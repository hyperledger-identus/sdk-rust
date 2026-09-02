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
and build-time status, including the expected Crane operation for every named
compatibility gate. A human-readable policy SHALL explain the evidence tiers
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
packages, feature sets and required check definitions. For every feature gate,
the validator SHALL compare the complete effective Cargo package selection,
including workspace exclusions, default feature mode and activated feature set
to the machine contract. Workspace-wide surfaces SHALL explicitly select the
workspace. The validator SHALL accept required gates only when their defining
Nix modules are reachable from the check-module import rooted in `flake.nix`.
Commented imports and gate definitions SHALL NOT count as reachable evidence.
The validator SHALL bind each gate to its machine-declared Crane operation,
each compile-checked target to the value of Cargo's `--target` option, and each
feature surface to the complete comma- or space-separated feature option.
Host systems, target triples and feature-surface names SHALL be unique within
the machine policy. The validator SHALL run in the structural factory path.

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

#### Scenario: Module import is commented out

- **WHEN** a required module path remains as a Nix comment in an import block
- **THEN** structural validation treats its gates as unreachable

#### Scenario: Check graph is detached from the flake

- **WHEN** `flake.nix` stops importing the root check module while the check
  files remain present
- **THEN** structural validation treats every policy gate as unreachable

#### Scenario: Workspace gate excludes a package

- **WHEN** a workspace-wide feature gate adds an exclusion that removes any
  workspace package
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

- **WHEN** a compile-checked target gate stops activating a feature required by
  its machine-declared target surface
- **THEN** structural validation fails even when target and package selections
  are unchanged

#### Scenario: Test gate becomes build-only

- **WHEN** a test gate changes from the declared test operation to a build
  operation while retaining its name and Cargo arguments
- **THEN** structural validation rejects the semantic operation drift

#### Scenario: Target token moves outside Cargo arguments

- **WHEN** a target gate compiles another triple but retains the declared
  triple in unrelated Nix metadata
- **THEN** structural validation rejects the actual Cargo target selection

#### Scenario: Isolated feature adds a space-separated feature

- **WHEN** a gate appends another feature using Cargo's space-separated feature
  syntax
- **THEN** structural validation observes the complete feature list and rejects
  the broadened surface

#### Scenario: Feature gate silently broadens

- **WHEN** a minimal or isolated gate drops `--no-default-features`, selects a
  different package or activates a different feature set
- **THEN** structural validation fails even if the gate retains its evidence
  token

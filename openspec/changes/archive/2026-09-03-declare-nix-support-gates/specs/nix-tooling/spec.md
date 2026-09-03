## ADDED Requirements

### Requirement: Rust checks are generated from declarative gate data

The Nix checks module SHALL read the versioned Rust gate manifest with Nix's
TOML evaluator and generate exactly one Crane check for every declared entry.
The generator SHALL select the declared etalon or MSRV Crane library, source and
artifact class, operation and deterministically rendered Cargo arguments.
Hand-written Nix modules SHALL NOT duplicate named Rust compatibility gates.

#### Scenario: Gate manifest changes a structured selector

- **WHEN** a reviewed entry changes its package, feature, target or operation
- **THEN** the generated Crane derivation uses the new value and the policy
  validator independently accepts or rejects the corresponding claim

#### Scenario: Manifest declares an unsupported execution combination

- **WHEN** an entry names an unknown operation or an invalid toolchain,
  artifact or source combination
- **THEN** evaluation or offline validation fails rather than emitting a
  best-effort check

#### Scenario: Old hand-written definition remains

- **WHEN** a contributor adds a second Nix definition for a manifest-owned Rust
  gate
- **THEN** repository structural validation rejects the duplicated execution
  representation

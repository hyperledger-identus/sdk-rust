## MODIFIED Requirements

### Requirement: Crate stubs at minimal code depth

The workspace SHALL classify `identus-agent`, `identus-bindings`,
`identus-credentials`, `identus-messaging`, `identus-openid4vc`,
`identus-presentations`, `identus-trust` and `identus-wallet` as quarantined
runtime placeholders at minimal code depth. Each placeholder's `src/lib.rs`
SHALL contain only a module doc-comment, an
`identus_core::Component` import and a public `COMPONENT` constant with stable
current `name` and `summary` metadata. The name and layer membership preserve
seed evidence only; neither is a release, namespace or future capability
commitment.

`identus-core`, `identus-derive`, `identus-crypto`, `identus-did` and
`identus-adapters-entropy` contain implemented experimental foundations and
SHALL NOT be described as stubs. `identus-conformance` contains
verification-only guards and SHALL NOT be described as a runtime stub.

#### Scenario: Each placeholder self-describes via COMPONENT

- **WHEN** `COMPONENT.name` is inspected for each quarantined placeholder
- **THEN** it equals the package name recorded in the bootstrap inventory

#### Scenario: Placeholder source stays at marker depth

- **WHEN** `cargo build --workspace` and inventory validation run
- **THEN** every placeholder compiles with only its documentation, import and
  `COMPONENT` marker while implemented and verification crates retain their
  real code

### Requirement: No cross-crate dev-dependencies in the conformance crate yet

`identus-conformance` SHALL depend on `identus-core` in production for its
`COMPONENT` marker and on `syn` plus `toml` as development-only source and
manifest parsing tools. It SHALL NOT declare `serde_json` or an `identus-*`
domain crate as a development dependency. The current architecture guards
inspect source and manifests without compiling consumer crates into the
conformance package.

#### Scenario: Conformance dev-dependencies are tooling only

- **WHEN** `crates/conformance/Cargo.toml` is inspected
- **THEN** its development dependencies are exactly `syn` and `toml`, with no
  domain crate or JSON fixture parser

### Requirement: identus-adapters-entropy crate

The workspace SHALL contain the implemented experimental
`identus-adapters-entropy` package in the `outer-boundary` layer and root
workspace dependency map. It SHALL depend inward on `identus-core` and
`identus-crypto`, expose no concrete adapter by default, and provide the
independent opt-in `getrandom` and `deterministic` feature surfaces. The system
adapter SHALL use optional `getrandom`; deterministic entropy SHALL remain a
test/conformance aid and SHALL NOT be represented as production randomness.

#### Scenario: Entropy adapter is a workspace and layer member

- **WHEN** the workspace map and `LAYER_RULES` are inspected
- **THEN** `identus-adapters-entropy` appears in both and is classified as an
  implemented experimental outer-boundary package

#### Scenario: Entropy features remain isolated

- **WHEN** the package is built with no defaults, `getrandom`,
  `deterministic`, or all features
- **THEN** each declared surface is exercised by its support-policy gate and
  only `getrandom` activates an external entropy backend

## REMOVED Requirements

### Requirement: Full intended inward dependency edges

**Reason:** Declaring unused future edges in metadata-only placeholders creates
speculative coupling and presents an unaccepted architecture as current fact.
Placeholder packages now depend only on `identus-core`, which supplies their
component marker. Real dependency cones are introduced by focused component
contracts when implementation lands.

**Migration:** Remove every placeholder dependency other than `identus-core`.
The existing inward-direction rule remains the upper bound for future accepted
edges; removing speculative edges cannot create an outward dependency.

### Requirement: No docs/architecture files introduced

**Reason:** This was a historical per-change restriction, not an enduring
crate-ring invariant. The accepted SDK blueprint and machine-readable
architecture policies now live under `docs/architecture/`.

**Migration:** Architecture evidence remains governed by issue-linked OpenSpec
changes and deterministic drift checks.

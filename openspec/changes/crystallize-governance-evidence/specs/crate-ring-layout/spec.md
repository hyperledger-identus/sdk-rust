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

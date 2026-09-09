# language-bindings-research Specification

## Purpose
TBD - created by archiving change select-first-uniffi-did-slice. Update Purpose after archive.
## Requirements
### Requirement: First binding slice is selected through isolated runtime evidence

Before the SDK activates a supported foreign-language interface, it SHALL
select a bounded capability through an isolated, unpublished proof. The proof
SHALL keep generic domain crates free of binding-framework dependencies, SHALL
expose only SDK-owned value and error types, SHALL record exact generator and
runtime versions and dependency cones, and SHALL execute consumer-shaped
success and failure paths in every language claimed by the decision.

#### Scenario: native value slice succeeds

- **WHEN** Swift and Kotlin call the generated DID and DID URL parsing APIs
- **THEN** valid values SHALL round-trip with their components unchanged and
  invalid or oversized values SHALL produce stable errors without reflecting
  caller input

#### Scenario: domain implementation remains binding-free

- **WHEN** the research wrapper consumes `identus-did`
- **THEN** UniFFI annotations, types and dependencies SHALL remain outside the
  domain crate and no Rust or third-party domain type SHALL cross the ABI

#### Scenario: generation is reproducible and reviewable

- **WHEN** bindings are generated twice from the same pinned source and tool
- **THEN** normalized public API snapshots SHALL be identical and an explicit
  drift check SHALL fail if the reviewed call shape changes

#### Scenario: platform claims remain separate

- **WHEN** native Swift/Kotlin evidence passes
- **THEN** React Native, Node and browser React/WASM SHALL remain unsupported
  until their own generator, runtime, packaging and consumer evidence passes

#### Scenario: research does not activate FFI support

- **WHEN** the research decision is merged
- **THEN** the support policy SHALL continue to report FFI as not supported
  and production adoption SHALL require a separate issue and contract

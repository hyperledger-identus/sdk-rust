## MODIFIED Requirements

### Requirement: In-source layer rulebook

`identus-conformance` SHALL encode all workspace crates in exactly one of the
seven accepted layers and SHALL enforce inward runtime dependencies from that
typed rulebook. The credential-semantics layer SHALL contain
`identus-credentials`, `identus-presentations` and `identus-jose`.
`identus-jose` SHALL retain only its accepted workspace runtime dependency on
the inward foundation crate `identus-core`.

#### Scenario: JOSE is reusable credential semantics

- **WHEN** the rulebook and `identus-jose` manifest are inspected
- **THEN** `identus-jose` appears exactly once in credential-semantics and its
  only `identus-*` runtime dependency is `identus-core`

### Requirement: Crate stubs at minimal code depth

The workspace SHALL distinguish implemented experimental crates from
quarantined marker-only placeholders. `identus-jose` SHALL be classified as
implemented credential semantics and SHALL NOT be described as a stub.

#### Scenario: JOSE implementation is represented honestly

- **WHEN** the crate-ring contract and bootstrap inventory are inspected
- **THEN** both classify `identus-jose` as implemented rather than placeholder

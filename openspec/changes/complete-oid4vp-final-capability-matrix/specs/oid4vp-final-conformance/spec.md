# OID4VP Final conformance evidence

## ADDED Requirements

### Requirement: Final wallet-core coverage is machine traceable

The repository SHALL maintain one closed matrix mapping applicable OpenID4VP
1.0 Final capability groups to bounded SDK status, implementation, canonical
contract, executable tests, limitations, provenance, and focused gap owner.

#### Scenario: implemented behavior is claimed

- **WHEN** a row has status `implemented`
- **THEN** it names existing repository-local implementation, canonical spec,
  and executable test paths
- **AND** the offline validator accepts each path as a regular file

#### Scenario: required behavior is incomplete

- **WHEN** a required row is partial or unsupported
- **THEN** it states the exact limitation and focused open owner
- **AND** IDR-024 remains in progress

### Requirement: reference execution preserves Rust ownership

The repository SHALL execute clean-room Final vectors through public
`identus-oid4vp` APIs and SHALL demonstrate consumer adapter feasibility
without exporting consumer DTOs or errors from the Rust core.

#### Scenario: a TypeScript- or Swift-shaped fixture is used

- **WHEN** the test adapter receives independently authored foreign-shaped data
- **THEN** it translates at the test boundary into Rust-native inputs
- **AND** the public crate API remains unchanged

### Requirement: the report is not a certification claim

The report SHALL distinguish source traceability and repository tests from
official certification, live interoperability, transport security, verifier
trust, presentation authenticity, consent, privacy, and product readiness.

#### Scenario: the matrix contains no required gaps

- **WHEN** every member of the pinned required capability set has status
  `implemented`
- **THEN** the report may recommend IDR-024 delivery
- **AND** it still does not claim certification or downstream adoption

# OID4VCI generic interoperability vectors

## ADDED Requirements

### Requirement: generic wallet vectors are clean-room and drift protected

The repository SHALL retain a versioned Apache-2.0 wallet-core fixture packet
whose bytes are independently authored from OpenID4VCI 1.0 Final and contain
no copied Oxid or Portal fixture content.

#### Scenario: a vector enters the suite

- **WHEN** a fixture is retained as executable SDK evidence
- **THEN** its manifest entry records a stable ID, repository authorship,
  license, normative section, relative path, SHA-256, transformation, expected
  result and public SDK entry point
- **AND** the test rejects provenance or byte drift before executing it

#### Scenario: consumer evidence is inspected

- **WHEN** an Oxid or Portal artifact informs the compatibility assessment
- **THEN** the manifest records its repository, immutable revision, paths,
  license disposition and `reference-only` status
- **AND** no consumer byte is represented as an SDK-authored fixture

### Requirement: retained vectors execute through public wallet APIs

The suite SHALL exercise one coherent positive Pre-Authorized Code wallet flow
and relevant negative legacy inputs exclusively through public
`identus-oid4vci` and approved public JOSE APIs.

#### Scenario: the positive packet executes

- **WHEN** the versioned positive fixtures are loaded
- **THEN** offer, metadata, Transaction Code, Token Request/Response, Credential
  Request and immediate Credential Response transitions succeed with exact
  bounded lineage and expected wire evidence

#### Scenario: a legacy wire class executes

- **WHEN** extra offer query data, a null Transaction Code or a singular
  Credential Response is loaded
- **THEN** the matching public SDK boundary rejects it with the recorded stable
  error class

### Requirement: consumer compatibility evidence remains bounded

The conformance report SHALL map immutable Oxid and Portal design/source
evidence to generic accepted, rejected or downstream-only SDK behavior without
claiming live product interoperability or current human approval.

#### Scenario: the generic suite is complete

- **WHEN** every manifest entry passes provenance, drift and public-API tests
- **THEN** the cross-consumer matrix row may become `implemented`
- **AND** issuer-only behavior, Midnight format semantics, transport, trust,
  certification and downstream adoption remain explicit non-claims

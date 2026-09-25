# oid4vci-final-conformance Specification

## Purpose
TBD - created by archiving change close-oid4vci-m4-conformance. Update Purpose after archive.
## Requirements
### Requirement: Final wallet-core coverage is machine traceable

The repository SHALL maintain one closed matrix mapping OpenID4VCI 1.0 Final
sections 4 through 12 to the bounded SDK status, public surface, canonical
contract, executable tests, explicit limitations and focused gap owner.

#### Scenario: implemented behavior is claimed

- **WHEN** a matrix row has status `implemented`
- **THEN** it names existing repository-local implementation, canonical spec
  and executable test paths
- **AND** the offline validator accepts each path as a regular file

#### Scenario: incomplete behavior remains visible

- **WHEN** a row is partial, unsupported or missing
- **THEN** it states the exact limitation
- **AND** missing required wallet-core behavior names one focused open issue

### Requirement: consumer fixtures retain truthful provenance

Consumer-derived evidence SHALL record immutable source revisions, license
disposition and genericity without treating reference-only files as imported or
executed SDK conformance evidence.

#### Scenario: a source cannot be copied safely

- **WHEN** an exact donor revision lacks an explicit license or the fixture is
  product/chain-specific
- **THEN** the report classifies it as reference-only
- **AND** no donor fixture bytes enter the SDK repository

### Requirement: the matrix is not a certification claim

The conformance report SHALL distinguish traceability and repository tests from
official OpenID certification, transport provenance, trust, credential-format
verification and downstream interoperability acceptance.

#### Scenario: M4 is evaluated

- **WHEN** the reviewed matrix is complete
- **THEN** the report recommends close or continue using explicit residual
  limitations and focused owners
- **AND** it does not convert optional or unsupported Final behavior into a
  release, certification or consumer-support promise

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


# OID4VCI Final conformance evidence

## ADDED Requirements

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

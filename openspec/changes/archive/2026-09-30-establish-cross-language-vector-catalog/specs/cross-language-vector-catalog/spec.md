# cross-language-vector-catalog

## ADDED Requirements

### Requirement: Vector packets have immutable portable identity

The SDK SHALL store language-neutral vector packets under a versioned fixture
path. Every packet SHALL have a stable ID, schema version, repository-relative
path, SHA-256 digest, authorship, license, redistribution decision, public-data
declaration, and target applicability. Normal validation SHALL be offline.

#### Scenario: a fixture byte changes

- **WHEN** packet bytes no longer match the catalog digest
- **THEN** validation fails before any behavioral test treats the payload as
  approved evidence

### Requirement: Every vector records provenance and authority

Every active vector SHALL identify its packet case, capability, authority
class, exact repository revision/path or standard version/section, operation,
expected success or stable error, profile scope, boundary/resource class,
known limitations, owner, and exact Rust selector. Repeated donor cases SHALL
NOT acquire normative authority through repetition.

#### Scenario: three language SDKs contain the same DID case

- **WHEN** the case is retained for migration compatibility
- **THEN** it is classified as consumer-regression evidence with pinned source
  selectors while normative or accepted Identus sources retain precedence

### Requirement: Catalog validation fails closed

The validator SHALL reject unknown fields, duplicate or malformed IDs,
unpinned repository sources, invalid hashes, unsafe paths, missing licensing or
redistribution decisions, non-public payloads, dangling cases or references,
selectorless active vectors, incomplete packet coverage, and invalid
supersession chains.

#### Scenario: a new metadata field appears without a schema migration

- **WHEN** an unrecognized field is added to any source, packet, vector, or
  fixture case
- **THEN** the offline validator rejects the catalog or packet

### Requirement: The first packet proves bounded generic DID behavior

The first packet SHALL cover generic DID and DID URL positive, negative,
exact-boundary, over-boundary, stable-error, and redaction cases. A Rust test
SHALL load that same packet and exercise `identus-did`. Method resolution,
documents, networks, credentials, and protocols SHALL remain outside it.

#### Scenario: a DID is one byte above the accepted limit

- **WHEN** the portable packet materializes the over-limit input
- **THEN** the Rust proof returns `did.invalid_did` without exposing caller
  input

### Requirement: Stable records evolve explicitly

Stable vector IDs SHALL NOT be silently repurposed. Replacement or
supersession SHALL be explicit, acyclic, and retain the historical record. A
semantic packet or catalog change SHALL increment the corresponding version.

#### Scenario: an expected result needs semantic correction

- **WHEN** maintainers accept a corrected case
- **THEN** a new version or replacement record is added and the former stable
  ID remains auditable

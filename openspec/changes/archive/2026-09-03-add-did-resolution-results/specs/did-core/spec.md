## ADDED Requirements

### Requirement: Validated DID resolution result states

The DID Core capability SHALL provide an immutable `DidResolutionResult` with
`didResolutionMetadata`, an optional `didDocument` and `didDocumentMetadata`.
It SHALL accept exactly three states: success has a document and no error;
ordinary failure has an error, no document and empty document metadata; and
deactivation has no document/error with `deactivated` metadata equal to true.
Native constructors and JSON deserialization SHALL enforce identical states.

#### Scenario: current W3C states remain distinct

- **WHEN** success, not-found and deactivated results are constructed or parsed
- **THEN** each SHALL retain its normative field shape without representing a
  failure as deactivation or exposing a contradictory document/error pair

#### Scenario: requested and returned identifiers agree

- **WHEN** a result is validated against the requested DID
- **THEN** a present document id SHALL equal that DID and canonical/equivalent
  identifiers SHALL use the same DID method without claiming semantic equality

### Requirement: Current URI-valued resolution errors

Resolution and dereferencing metadata SHALL carry a bounded RFC 9457-style
error object with a required absolute `type` URI and optional bounded title,
detail, instance URI and extension members. The capability SHALL classify all
nine error URLs defined by the 28 August 2026 DID Resolution Candidate
Recommendation Draft while preserving other valid URLs. Strict serde SHALL NOT
accept a legacy keyword string in place of the object.

#### Scenario: standard and extension errors round-trip

- **WHEN** any standard W3C error URL or another valid extension URL is parsed
- **THEN** its exact URI and bounded public problem details SHALL survive a
  semantic JSON round trip and its standard classification SHALL be available

#### Scenario: estate keywords migrate explicitly

- **WHEN** an adapter deliberately passes a recognized legacy error keyword
- **THEN** a migration helper SHALL map it to the current W3C URL while unknown
  keywords and bare legacy wire values SHALL fail deterministically

### Requirement: Typed common DID document metadata

`DidDocumentMetadata` SHALL type the common `created`, `updated`, `deactivated`,
`nextUpdate`, `versionId`, `nextVersionId`, `equivalentId` and `canonicalId`
properties and preserve bounded proof/method-specific extension JSON. Datetimes
SHALL be valid whole-second UTC values; version ids SHALL be non-empty printable
ASCII; equivalent-id sets SHALL be non-empty and unique; extensions SHALL NOT
shadow common properties.

#### Scenario: shared producer metadata is portable

- **WHEN** PRISM- or Midnight-shaped document metadata is supplied
- **THEN** Lace and Oxid consumers SHALL be able to read common fields and
  retain method extensions without importing chain or method dependencies

#### Scenario: malformed common metadata fails closed

- **WHEN** metadata contains an invalid datetime/version, duplicate equivalent
  DID, cross-method canonical/equivalent DID or reserved extension collision
- **THEN** validation SHALL reject it without reflecting attacker data publicly

### Requirement: Bounded open DID URL dereferencing results

The capability SHALL provide an immutable serialized DID URL dereferencing
result with `didUrlDereferencingMetadata`, optional `content` and
`contentMetadata`. Success SHALL have bounded JSON content and no error;
failure SHALL have an error, no content and empty content metadata. Open content
SHALL preserve semantic JSON and offer typed construction/access for DID
documents, verification methods, services and URIs without using a closed enum.

#### Scenario: known and future JSON resources remain representable

- **WHEN** dereferenced content is a DID document, verification method, service,
  URI or method-defined JSON resource
- **THEN** it SHALL round-trip semantically and known projections SHALL validate
  through their existing domain types without freezing the at-risk standard

#### Scenario: transport policy remains outside the result

- **WHEN** arbitrary native bytes or HTTP behavior are required
- **THEN** a binding adapter SHALL own their encoding and transport policy while
  this result remains JSON, chain, runtime and protocol neutral

### Requirement: Uniform bounded resolution validation

Raw resolution and dereferencing JSON SHALL be limited to 512 KiB before
deserialization. Open collections SHALL contain at most 128 entries; maps at
most 64 entries with names no longer than 256 bytes; open trees at most 4,096
nodes and 32 levels; and arbitrary strings at most 64 KiB. Media types,
datetimes, version ids and problem-detail strings SHALL have smaller documented
limits. Native and serde validation SHALL share the same resource policy and
map failures to a stable redaction-safe DID error.

#### Scenario: native construction cannot bypass wire limits

- **WHEN** an excessive metadata/content tree is supplied through a constructor
  or JSON deserialization
- **THEN** both paths SHALL reject it under the same public resource policy

#### Scenario: bounded result parsing remains observable

- **WHEN** representative result envelopes are parsed repeatedly in release mode
- **THEN** throughput SHALL be recorded as evidence without a machine-specific
  CI threshold

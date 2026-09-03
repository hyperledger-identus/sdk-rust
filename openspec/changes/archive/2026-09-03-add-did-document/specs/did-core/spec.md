## ADDED Requirements

### Requirement: Bounded absolute URI value

The DID Core capability SHALL provide an immutable owned `Uri` value for DID
document fields whose normative type is an RFC 3986 URI rather than a DID URL.
It SHALL require an absolute ASCII scheme, accept only generic URI characters
and complete percent escapes, reject fragments with a second `#`, and reject
values larger than 4,096 bytes before allocation. Parsing SHALL be linear and
preserve exact spelling without dereferencing or scheme-specific normalization.

#### Scenario: generic absolute URI schemes remain open

- **WHEN** HTTPS, DID, URN and custom absolute URI values satisfy RFC 3986
  generic syntax
- **THEN** they SHALL be accepted without assigning transport or trust policy

#### Scenario: malformed or relative values fail safely

- **WHEN** a URI is relative, contains whitespace/control/non-ASCII bytes, has
  a malformed escape, invalid scheme or exceeds its byte limit
- **THEN** native and serde construction SHALL reject it without reflecting the
  caller value through the public error bridge

### Requirement: Extensible DID document structure

The capability SHALL provide an immutable `DidDocument` with a required typed
`Did` subject and optional context, controllers, aliases, verification methods,
five core verification relationships, services and unknown extension members.
It SHALL preserve absent/present state, scalar-or-array cardinality where DID
Core permits both, array order and unknown JSON values across semantic JSON
round trips. Extensions SHALL NOT shadow reserved core properties.

#### Scenario: W3C and downstream-shaped documents round-trip

- **WHEN** a standards-shaped document includes mixed cardinalities, PRISM or
  Midnight method names, and Lace/Oxid extension members
- **THEN** validated typed accessors SHALL expose its core values and
  serialization SHALL retain the same semantic JSON value without importing
  downstream method or chain dependencies

#### Scenario: required and reserved structure is protected

- **WHEN** a document omits its id, supplies an empty present collection, uses
  a malformed identifier or attempts a reserved extension collision
- **THEN** construction SHALL fail deterministically

### Requirement: Open public verification method boundary

A `VerificationMethod` SHALL contain a typed `Uri` id, a bounded non-empty open
type string, a typed controller `Did` and bounded suite-defined properties.
DID-shaped ids MAY be parsed separately as `DidUrl` by downstream method
adapters. The capability SHALL recognize and expose `publicKeyJwk` maps and
`publicKeyMultibase` strings while preserving other properties without
cryptosuite interpretation. It SHALL reject simultaneous recognized material
forms and all registered private JWK members. The five core relationship
properties SHALL contain one or more embedded verification methods and/or typed
`Uri` references.

#### Scenario: embedded and referenced relationships coexist

- **WHEN** a document assigns embedded and referenced methods across
  authentication, assertion, key agreement, capability invocation and
  capability delegation
- **THEN** the exact relationship variants and order SHALL remain available
  without imposing curve, controller or chain policy

#### Scenario: secret or ambiguous known material is rejected

- **WHEN** a JWK contains private material or a verification method contains
  both `publicKeyJwk` and `publicKeyMultibase`
- **THEN** every construction path SHALL fail before the document is exposed

### Requirement: Extensible DID services

A `Service` SHALL contain a unique typed `Uri` id, a bounded non-empty string
or set of strings as its type, and a service endpoint represented by a typed URI
string, bounded map, or non-empty mixed set of URI strings and maps. Unknown
service properties SHALL be preserved and SHALL NOT shadow core service keys.

#### Scenario: all service endpoint representations are accepted

- **WHEN** a document contains services with string, map and mixed-array
  endpoints and scalar or array service types
- **THEN** their wire cardinality, order and extension values SHALL round-trip
  semantically

#### Scenario: invalid or duplicate services are rejected

- **WHEN** a service omits a required member, uses an invalid endpoint shape or
  URI, has an empty type/endpoint set, or duplicates another service id
- **THEN** document construction SHALL fail with a stable invalid-document error

### Requirement: Uniform bounded document validation

Raw DID document JSON SHALL be limited to 256 KiB before deserialization.
Document collections SHALL contain at most 128 entries; extension maps SHALL
contain at most 64 entries with property names no longer than 256 bytes;
arbitrary extension trees SHALL contain at most 4,096 nodes, be at most 32
levels deep and contain strings no longer than 64 KiB. Native construction and
serde deserialization SHALL share these structural checks. Failures SHALL map
to `did.invalid_uri` or `did.invalid_document` with capability `did`,
`InvalidInput` kind and no caller-controlled public detail.

#### Scenario: native construction cannot bypass wire limits

- **WHEN** the same excessive collection or extension tree is supplied through
  a constructor or JSON deserialization
- **THEN** both paths SHALL reject it under the same public resource policy

#### Scenario: bounded validation remains performant

- **WHEN** a representative standards-shaped document is parsed repeatedly in
  a release diagnostic
- **THEN** throughput SHALL be recorded as observational evidence without an
  environment-specific CI pass threshold

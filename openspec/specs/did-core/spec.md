# did-core Specification

## Purpose

`did-core` is the chain-neutral identifier and document capability for the
Identus Rust SDK. It provides small validated DID, DID URL and URI values plus
a bounded extensible DID document structural model. Resolution results and
ports extend the same capability through separately reviewed changes.

Generic syntax is deliberately independent of DID method registration,
method-specific semantics, resolution and trust. PRISM, Midnight and later
method crates layer those policies on top. The capability remains portable to
native, wasm and mobile targets, bounds untrusted input, avoids broad DID/URL
dependencies where the normative grammar is closed, and maps failures through
the SDK's stable redaction-safe error contract.
## Requirements
### Requirement: Validated absolute DID

The DID Core capability SHALL provide an immutable owned `Did` value that
accepts exactly the generic absolute DID grammar from W3C DID Core 1.0. A DID
SHALL contain lowercase literal `did:`, a non-empty method of lowercase ASCII
letters or digits, and a non-empty method-specific identifier made from ASCII
letters, digits, `.`, `-`, `_`, `:`, or complete percent-encoded octets. The
method-specific identifier SHALL NOT end with `:`. A bare `Did` SHALL reject
path, query and fragment delimiters. Parsing SHALL preserve the exact valid
representation without percent-decoding or normalization.

#### Scenario: chain-neutral method shapes are accepted

- **WHEN** syntactically valid `did:prism`, `did:midnight`, `did:web` and
  `did:key` shaped identifiers are parsed
- **THEN** each SHALL be accepted without the generic layer asserting that
  its method is registered, resolvable or semantically valid

#### Scenario: malformed DID grammar is rejected

- **WHEN** a DID has a missing component, uppercase or punctuation-bearing
  method, trailing method-specific colon, malformed percent escape, raw
  non-ASCII byte, whitespace, path, query or fragment
- **THEN** construction SHALL fail without preserving or displaying the
  rejected identifier in the error

### Requirement: Validated absolute DID URL

The DID Core capability SHALL provide an immutable owned `DidUrl` value that
accepts a valid `Did` followed by RFC 3986 `path-abempty`, optional query and
optional fragment components as composed by W3C DID Core. Paths SHALL be empty
or slash-led. Path bytes SHALL be RFC 3986 `pchar`; query and fragment bytes
SHALL be `pchar`, `/` or `?`; every percent escape SHALL contain two ASCII hex
digits. Empty present query and fragment components SHALL remain
distinguishable from absent components. Parsing SHALL preserve the exact valid
representation without decoding or normalization.

#### Scenario: a DID alone is also a DID URL

- **WHEN** a valid bare DID is parsed as `DidUrl`
- **THEN** it SHALL be accepted with no path, query or fragment

#### Scenario: DID URL components are exposed precisely

- **WHEN** `did:example:123/a/b?version=1#key-1` is parsed
- **THEN** the DID view SHALL be `did:example:123`, path SHALL be `/a/b`,
  query SHALL be `version=1`, and fragment SHALL be `key-1`

#### Scenario: malformed RFC 3986 components are rejected

- **WHEN** a DID URL contains an invalid path/query/fragment byte, incomplete
  percent escape, raw whitespace or a second fragment delimiter
- **THEN** construction SHALL fail with a redacted error

### Requirement: Bounded single-pass parsing and component views

`Did` SHALL reject input larger than 2,048 bytes and `DidUrl` SHALL reject
input larger than 4,096 bytes before scanning or allocation. Validation SHALL
perform one linear ASCII byte pass without regex, URL or external DID parser
dependencies. Successful values SHALL cache byte offsets so method,
method-specific identifier, DID, path, query and fragment reads return borrowed
slices without allocation. `TryFrom<String>` SHALL reuse the supplied string
allocation; converting `Did` into the equivalent `DidUrl` SHALL move it.

#### Scenario: oversized input is rejected before semantic parsing

- **WHEN** a bare DID or DID URL exceeds its public SDK byte limit
- **THEN** it SHALL be rejected as too long without scanning the full grammar
  or reflecting the input in an error

#### Scenario: repeated component access does not allocate

- **WHEN** any component accessor is called repeatedly on a valid value
- **THEN** it SHALL return a borrowed slice from the one owned representation

### Requirement: Equivalent native and wire validation

`Did` and `DidUrl` SHALL implement `FromStr`, `TryFrom<String>`, transparent
string serialization and validating deserialization. Every construction path
SHALL enforce the same grammar and limits. Failures SHALL map to stable,
redaction-safe `IdentusError` codes `did.invalid_did` and
`did.invalid_did_url` under capability `did` with `InvalidInput` kind. Neither
local nor public errors SHALL contain caller-supplied identifier text.

#### Scenario: native and serde construction agree

- **WHEN** the same valid or invalid string is supplied through native parsing
  and JSON deserialization
- **THEN** both paths SHALL make the same accept/reject decision and valid
  serialization SHALL preserve the exact string

#### Scenario: public error is stable and redacted

- **WHEN** a DID or DID URL parse failure is mapped and displayed
- **THEN** its stable code and capability SHALL identify the failed boundary
  while the rejected input SHALL NOT appear

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

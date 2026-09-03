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

### Requirement: Bounded extensible DID resolution options

The DID Core capability SHALL provide immutable `ResolutionOptions` containing
optional `accept`, `expandRelativeUrls`, `versionId` and `versionTime` values
plus method/extension members. Common members SHALL reuse their existing typed
validators. Absence and explicit false SHALL remain distinct, extensions SHALL
NOT shadow common keys, and native construction and serde SHALL enforce the
same bounded metadata policy.

#### Scenario: common and method options remain portable

- **WHEN** a PRISM- or Midnight-shaped resolver receives common options and a
  method extension
- **THEN** it SHALL observe the exact typed values and preserved extension JSON
  without importing chain, method, transport or runtime types

#### Scenario: malformed resolution options fail closed

- **WHEN** an option map contains a malformed media type, datetime or version,
  a reserved extension collision, excessive input or invalid open JSON
- **THEN** every public construction path SHALL reject it with the existing
  redaction-safe DID resolution error boundary

### Requirement: Bounded extensible DID URL dereferencing options

The capability SHALL provide immutable `DereferencingOptions` containing
optional `accept` and `verificationRelationship` values plus extension members.
Verification relationship names SHALL be non-empty bounded printable ASCII and
remain open to extension vocabularies. The option type SHALL be independent of
resolution options because W3C DID URL dereferencing is at risk.

#### Scenario: verification relationship remains open and safe

- **WHEN** a core or extension verification relationship is supplied
- **THEN** its exact valid ASCII spelling SHALL reach the dereferencer while
  empty, control-bearing, padded or oversized values are rejected

#### Scenario: at-risk dereferencing does not freeze the resolver

- **WHEN** the dereferencing contract evolves or an implementation supports
  resolution only
- **THEN** `ResolutionOptions` and `DidResolver` SHALL remain independently
  usable without implementing or depending on dereferencing

### Requirement: Object-safe runtime-neutral DID query ports

The capability SHALL define `DidResolver` and `DidUrlDereferencer` as
`Send + Sync`, object-safe `#[identus::port]` traits. Resolution SHALL accept a
borrowed validated `Did` and `ResolutionOptions` and asynchronously return a
`DidResolutionResult`. Dereferencing SHALL accept a borrowed validated `DidUrl`
and `DereferencingOptions` and asynchronously return a
`DidUrlDereferencingResult`. Named boxed `Send` future aliases SHALL make the
allocation and lifetime contract explicit without selecting an async runtime.

#### Scenario: independent method implementations use one injection seam

- **WHEN** PRISM- and Midnight-shaped mock implementations are stored and
  invoked through trait objects
- **THEN** both SHALL receive exact SDK inputs and return valid SDK result
  envelopes without method, chain, HTTP, executor or product dependencies

#### Scenario: standards failures use one result channel

- **WHEN** a resolver encounters unsupported options/methods, not-found or an
  internal implementation failure
- **THEN** it SHALL return the corresponding W3C error result rather than a
  second generic transport/error result channel

### Requirement: Uniform bounded option validation

Raw resolution and dereferencing option JSON SHALL be limited to 64 KiB before
deserialization. Extension maps SHALL contain at most 64 members with names no
longer than 256 bytes; arbitrary strings SHALL be at most 64 KiB; open values
SHALL be at most 32 levels and 4,096 aggregate nodes. Common scalar values SHALL
retain their smaller purpose-specific limits.

#### Scenario: native construction cannot bypass option limits

- **WHEN** the same excessive option member or extension tree is supplied to a
  builder/constructor or JSON entry point
- **THEN** both paths SHALL reject it under the same resource policy

#### Scenario: option and dynamic dispatch cost remains observable

- **WHEN** representative options are parsed and a trait-object resolver is
  invoked repeatedly in a release diagnostic
- **THEN** throughput SHALL be recorded as evidence without a machine-specific
  CI pass threshold

### Requirement: Immutable bounded DID method registry

The DID Core capability SHALL provide a `DidMethodBinding` containing one
validated method name, one object-safe resolver and an optional independent
dereferencer. A builder SHALL reject duplicate method ownership and more than
64 bindings before producing a cheaply cloneable immutable registry whose
lookups require no lock or allocation.

#### Scenario: duplicate method ownership fails during setup

- **WHEN** two independent bindings claim the same exact validated DID method
- **THEN** registration SHALL fail deterministically before a registry can be
  built, without exposing the supplied method through the public error bridge

#### Scenario: a built registry is deterministic and concurrently readable

- **WHEN** registered methods are inspected or dispatched from cloned registry
  handles on concurrent threads
- **THEN** names SHALL appear in lexical order and exact lookups SHALL require
  no mutation, lock, runtime or chain-specific type

### Requirement: Resolution dispatch through the standard port

`DidMethodRegistry` SHALL implement `DidResolver` by selecting exactly the
registered binding whose method equals `Did::method()`. It SHALL forward the
original validated DID and options and return the adapter's existing W3C
result envelope without another generic error channel.

#### Scenario: independent DID methods share one resolver seam

- **WHEN** PRISM- and Midnight-shaped resolvers are registered and the registry
  is invoked through `Arc<dyn DidResolver>`
- **THEN** each exact DID method SHALL reach only its owner with unchanged
  options and result envelope

#### Scenario: unknown methods are standards-shaped failures

- **WHEN** no binding owns the exact requested DID method
- **THEN** resolution SHALL return a valid W3C `methodNotSupported` failure
  envelope without invoking a prefix, fallback or arbitrary adapter

### Requirement: Independent dereferencing dispatch

`DidMethodRegistry` SHALL implement `DidUrlDereferencer` independently. An
unknown method SHALL return `methodNotSupported`; a known method whose binding
has no dereferencer SHALL return `featureNotSupported`; and a registered
dereferencer SHALL receive the original DID URL and options unchanged.

#### Scenario: resolution-only methods remain usable

- **WHEN** a method binding supplies a resolver without a dereferencer
- **THEN** resolution SHALL operate normally while dereferencing returns a
  standards-shaped `featureNotSupported` result

#### Scenario: registered dereferencing routes exactly

- **WHEN** a binding supplies an independent dereferencer and its exact method
  is requested
- **THEN** the registry SHALL forward the validated DID URL and options and
  return its existing result envelope unchanged

### Requirement: Stable registry setup errors and observability

Duplicate ownership SHALL bridge to stable code `did.invalid_method_registry`
with `Conflict` kind. Capacity exhaustion SHALL use the same code with
`InvalidInput` kind. Registry count, emptiness, exact membership,
dereferencing-support and lexically sorted method names SHALL be observable,
while registered adapter objects SHALL remain encapsulated.

#### Scenario: registry setup errors are redaction safe

- **WHEN** duplicate ownership or capacity exhaustion is mapped to the shared
  SDK error surface
- **THEN** the error SHALL identify the DID registry capability and kind
  without containing a registered method name or adapter detail

#### Scenario: dispatch cost remains observable

- **WHEN** a representative multi-method registry is dispatched repeatedly in
  release mode
- **THEN** throughput SHALL be recorded as evidence without a machine-specific
  CI pass threshold

### Requirement: Current no-cache resolution option

`ResolutionOptions` SHALL represent the optional W3C `noCache` boolean.
Absent and explicit false SHALL remain distinguishable on the data type while
both permit configured caching; true SHALL request a fresh VDR result and
bypass generic cache reads and writes.

#### Scenario: no-cache remains an explicit opt-in control

- **WHEN** a caller supplies absent, false or true `noCache`
- **THEN** the exact value SHALL round-trip, and only true SHALL bypass an
  attached caching resolver without changing the base resolver port

### Requirement: Bounded normalized DID resolution cache identity

The DID capability SHALL derive a bounded equality/hash cache key from the
exact validated DID and every result-affecting resolution option except
`noCache`. Extension JSON object members SHALL be recursively normalized so
member order cannot create distinct entries. Debug output SHALL NOT expose the
full DID, option values or encoded key.

#### Scenario: semantic option order cannot bypass cache identity

- **WHEN** two option maps differ only in JSON object member insertion order
  or in absent versus false `noCache`
- **THEN** they SHALL produce the same cache key, while a distinct version,
  representation, expansion choice or extension value produces another key

#### Scenario: oversized identity remains safe

- **WHEN** an otherwise valid native option map would create a key larger than
  64 KiB
- **THEN** caching SHALL be bypassed or fail closed according to explicit
  policy without logging or truncating the key

### Requirement: Injectable bounded resolution cache

The DID capability SHALL define an object-safe asynchronous
`DidResolutionCache` port with a declared capacity from 1 through 4,096,
lookup/store, per-key invalidation and all-options-for-DID invalidation.
Entries SHALL contain a typed result and process-epoch monotonic insertion and
exclusive-expiry ticks and SHALL NOT serialize.

#### Scenario: stale or poisoned entries are never served

- **WHEN** an entry is expired, observes clock regression, or fails result
  validation against the requested DID
- **THEN** it SHALL be invalidated and the resolver SHALL refresh or fail
  closed without returning that cached content

#### Scenario: registration can invalidate every request variant

- **WHEN** a DID update or deactivation succeeds
- **THEN** an outer lifecycle coordinator SHALL be able to invalidate every
  representation, version and extension-option entry for that exact DID

### Requirement: Opt-in caching resolver policy

An immutable caching resolver SHALL decorate any `DidResolver` using injected
cache and monotonic-clock ports. Positive/deactivated results MAY use a bounded
positive TTL up to 24 hours. Only `notFound` failures MAY use a separately
enabled negative TTL up to five minutes. Other failure results SHALL NOT be
cached. W3C document/version metadata, including `nextUpdate`, SHALL NOT be
interpreted as a generic TTL.

#### Scenario: cache policy does not invent standards semantics

- **WHEN** a result carries created, updated, next-update, version or canonical
  metadata
- **THEN** the decorator SHALL preserve that envelope unchanged and use only
  its injected TTL policy for cache expiry

#### Scenario: cache infrastructure failure is explicit

- **WHEN** the clock, key construction or cache backend fails
- **THEN** configured bypass mode SHALL continue safely without that cache,
  while fail-closed mode SHALL return a valid W3C `internalError` envelope

### Requirement: Redaction-safe cache observability and concurrency

The caching resolver SHALL expose typed hit, bypass, stored-miss,
not-stored-miss and refresh/backend-bypass outcomes without request content.
It SHALL be safe to clone and call concurrently. The portable contract SHALL
explicitly permit duplicate concurrent fills and require stores to tolerate
them; it SHALL NOT block executor threads or imply cancellation-unsafe
single-flight behavior.

#### Scenario: concurrent misses preserve correctness

- **WHEN** concurrent callers miss the same key before either fill completes
- **THEN** each MAY invoke the upstream resolver, but no stale/invalid result
  SHALL be served and subsequent valid hits SHALL observe a bounded entry

#### Scenario: cache overhead remains observable

- **WHEN** a representative cache-hit path is invoked repeatedly in release
  mode
- **THEN** throughput SHALL be recorded without a machine-specific CI pass
  threshold

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

Raw DID document JSON SHALL be limited to 256 KiB before duplicate scanning or
deserialization. The streaming scanner SHALL reject more than 64 nested
containers, 16,384 visited values, 128 members in one object, or 128 KiB of
simultaneously retained decoded names. Document collections SHALL contain at
most 128 entries; extension maps SHALL contain at most 64 entries with property
names no longer than 256 bytes; arbitrary extension trees SHALL contain at most
4,096 nodes, be at most 32 levels deep and contain strings no longer than 64
KiB. Representable native construction and serde deserialization SHALL share
the semantic structural checks. Failures SHALL map to `did.invalid_uri` or
`did.invalid_document` with capability `did`, `InvalidInput` kind and no
caller-controlled public detail.

#### Scenario: raw preflight is independently bounded

- **WHEN** bounded-size raw JSON exceeds scanner depth, node, per-object member,
  or live decoded-name limits
- **THEN** it SHALL fail before typed deserialization with no caller data in
  local or public diagnostics

#### Scenario: native construction cannot bypass semantic limits

- **WHEN** the same representable excessive collection or extension tree is
  supplied through a constructor or semantic JSON deserialization
- **THEN** both paths SHALL reject it under the same public resource policy

#### Scenario: bounded validation remains performant

- **WHEN** a representative standards-shaped document is scanned and parsed
  repeatedly in a release diagnostic
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
SHALL implement the bounded XML Schema 1.1 `dateTime` lexical intersection that
is ASCII, adjusted to UTC with a terminal `Z`, and has whole seconds only.
Version ids SHALL be non-empty printable ASCII; equivalent-id sets SHALL be
non-empty and unique; extensions SHALL NOT shadow common properties.

#### Scenario: complete adjusted datetime profile is accepted

- **WHEN** metadata uses a calendar-valid four-digit, extended, negative, or
  astronomical-zero XML Schema year and a normal or `24:00:00Z` whole-second
  UTC time within the public byte limit
- **THEN** construction and serde SHALL preserve the exact datetime spelling

#### Scenario: non-adjusted or malformed datetime fails closed

- **WHEN** a datetime has an invalid calendar day/year spelling, unnecessary
  leading year zero, numeric offset, absent timezone, fraction, non-ASCII byte,
  invalid end-of-day time, or exceeds its byte limit
- **THEN** construction and serde SHALL reject it with the stable redaction-safe
  resolution boundary

### Requirement: Bounded open DID URL dereferencing results

The capability SHALL provide an immutable serialized DID URL dereferencing
result with `didUrlDereferencingMetadata`, optional `content` and
`contentMetadata`. Success SHALL have bounded JSON content and no error;
failure SHALL have an error, no content and empty content metadata. Open content
SHALL preserve semantic JSON and offer typed construction/access for DID
documents, verification methods, services and URIs without using a closed enum.
Arbitrary native content SHALL remain owned by an explicit binding value with
an exact media type and bounded opaque bytes, with no guessed text/base64
conversion into the core JSON envelope.

#### Scenario: known and future JSON resources remain representable

- **WHEN** dereferenced content is a DID document, verification method, service,
  URI or method-defined JSON resource
- **THEN** it SHALL round-trip semantically and known projections SHALL validate
  through their existing domain types without freezing the at-risk standard

#### Scenario: arbitrary native bytes remain binding-owned

- **WHEN** an HTTP or local binding returns a non-JSON native representation
- **THEN** that binding SHALL pair an exact media type with bounded opaque bytes
  and SHALL NOT make the core result guess UTF-8, base64, transport, or retrieval
  policy

### Requirement: Uniform bounded resolution validation

Raw resolution and dereferencing JSON SHALL be limited to 512 KiB before
deserialization. The raw entry points SHALL reject duplicate decoded object
names recursively before typed deserialization and SHALL preflight at most 64
nested containers, 16,384 JSON values, 128 members per object, and 128 KiB of
decoded names held by simultaneously open objects. Open collections SHALL
contain at most 128 entries; extension maps at most 64 entries with names no
longer than 256 bytes; open trees at most 4,096 nodes and 32 levels; and
arbitrary strings at most 64 KiB. Media types, datetimes, version ids and
problem-detail strings SHALL have smaller documented limits. Every failure
SHALL map to a stable redaction-safe DID resolution error.

#### Scenario: duplicate names fail before semantic collapse

- **WHEN** any raw resolution or serialized dereferencing object repeats a
  decoded property name at the envelope, metadata, error, document, content,
  content-metadata, or extension level
- **THEN** the raw entry point SHALL reject it without selecting a value or
  disclosing the name, value, position, or input bytes

#### Scenario: raw scanner resource limits fail deterministically

- **WHEN** bounded-size raw JSON exceeds scanner depth, node, per-object member,
  or live decoded-name ceilings
- **THEN** it SHALL fail before typed deserialization with a static resolution
  reason and stable public `did.invalid_resolution` boundary

#### Scenario: native construction cannot bypass semantic limits

- **WHEN** the same representable excessive metadata/content tree is supplied
  through a constructor or semantic JSON deserialization
- **THEN** both paths SHALL reject it under the same semantic resource policy

#### Scenario: bounded result parsing remains observable

- **WHEN** representative result envelopes are parsed repeatedly in release mode
- **THEN** fixed work units, bytes processed, and normalized scanner overhead
  SHALL be recorded without a machine-specific CI timing threshold

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

### Requirement: Bounded unambiguous DID URL parameter preparation

The DID capability SHALL parse query parameters without HTML form semantics,
percent-decode names and values exactly once, preserve literal plus signs and
reject empty names, malformed encoding, invalid text, bounds violations or
duplicate decoded names. Resolution parameters SHALL be projected to bounded
`ResolutionOptions`; resource selectors SHALL also be projected without
authorizing resolver-side retrieval and SHALL be consumed only by dereferencing.

#### Scenario: duplicate or aliased parameters fail closed

- **WHEN** a DID URL repeats a decoded parameter name or uses malformed percent
  encoding
- **THEN** generic dereferencing SHALL return `invalidDidUrl` without invoking
  resource selection or exposing the input in error output

#### Scenario: resolution inputs reach the resolver

- **WHEN** a URL supplies version, hash-link or extension parameters
- **THEN** the generic adapter SHALL invoke the injected resolver once with the
  base DID and their bounded unambiguous resolution-option representation

### Requirement: Generic DID document and exact-resource dereferencing

An opt-in generic adapter over any `DidResolver` SHALL return a resolved DID
document for a bare DID URL and SHALL match a fragment only to an exact
verification-method or service identifier in that document. It SHALL preserve
upstream errors and document metadata and SHALL return `notFound` for absent or
unsupported custom path/query resources.

#### Scenario: chain methods share the portable algorithm

- **WHEN** representative PRISM and Midnight resolvers return conformant DID
  documents
- **THEN** the same generic adapter SHALL return their bare documents and exact
  fragment resources without depending on either chain or method

#### Scenario: prefix matches are never resources

- **WHEN** a fragment is only a prefix, suffix or encoded lookalike of a
  document resource identifier
- **THEN** dereferencing SHALL return `notFound`

### Requirement: Verification-relationship authorization

When `verificationRelationship` is requested, the fragment SHALL identify a
verification method associated by exact reference or embedded object with one
of the five DID Core relationship arrays. Missing methods and invalid or
unassociated relationships SHALL use the current CID error identifiers.

#### Scenario: relationship membership is explicit

- **WHEN** a verification method exists but is not associated with the requested
  relationship
- **THEN** dereferencing SHALL fail with
  `INVALID_RELATIONSHIP_FOR_VERIFICATION_METHOD` rather than return the method

### Requirement: Service selection and bounded representation

The generic adapter SHALL select services by exact `service`, exact
`serviceType`, or their conjunction. Absent accept and supported DID document
media types SHALL return a document filtered to selected services;
`text/uri-list` SHALL return only string endpoint URIs with content-type
dereferencing metadata. No match SHALL return `notFound`, and unsupported media
SHALL return `representationNotSupported`.

#### Scenario: endpoint maps are not guessed

- **WHEN** selected services contain URI strings and structured endpoint maps
- **THEN** URI-list output SHALL contain only the string URIs and SHALL NOT
  interpret map members as transport locations

### Requirement: Scope-preserving relative service references

`relativeRef` SHALL require service selection and SHALL resolve only against
string service endpoint URIs. Absolute/network-path inputs, backslashes,
control characters, unstable nested encoding, dot traversal at any decoded
layer, authority changes or results outside the endpoint base-directory scope
SHALL fail as `invalidOptions`.

#### Scenario: encoded traversal cannot escape service scope

- **WHEN** a relative reference contains direct, encoded, double-encoded or
  platform-style parent traversal
- **THEN** the adapter SHALL reject it before returning an endpoint URI

#### Scenario: safe relative resolution stays deterministic

- **WHEN** a selected string endpoint and safe path/query/fragment relative
  reference remain within its route scope
- **THEN** the adapter SHALL return the RFC 3986-resolved bounded URI without
  retrieving it

### Requirement: Retrieval isolation and portable operation

Generic dereferencing SHALL be deterministic, redaction-safe, runtime-neutral,
`Send + Sync` and free of ambient network, filesystem, clock, cache, chain and
global registry access. Recursive retrieval, redirects, cycle/depth controls
and SSRF enforcement SHALL remain outer transport concerns.

#### Scenario: endpoint output has no hidden I/O

- **WHEN** a service endpoint URI is selected
- **THEN** the adapter SHALL return the URI value without DNS, HTTP, redirect or
  filesystem activity

#### Scenario: performance remains observable

- **WHEN** representative bare-document dereferencing is repeatedly exercised
  in release mode
- **THEN** throughput SHALL be recorded without a machine-specific CI threshold

### Requirement: Isolated safe DID Registration profile

The DID capability SHALL provide a chain-neutral Rust lifecycle profile based
on DIF DID Registration draft commit
`ec1bf38f7860b361eb6692c02f742b9dfc48291b`. It SHALL NOT claim direct DIF
JSON/HTTP compatibility, expose raw secret exchange, or import method, ledger,
wallet, persistence, transport, cache, or runtime policy.

#### Scenario: draft volatility remains replaceable

- **WHEN** a method implements the portable registration port
- **THEN** a future DIF wire adapter SHALL be independently replaceable without
  changing that method's resolver or dereferencer implementation

### Requirement: Bounded immutable registration requests

Create, update, deactivate, continue, and cancel SHALL be distinct immutable
request variants. Every request SHALL carry a bounded idempotency key and exact
validated method; continuation and cancellation SHALL carry a method-scoped
opaque job. Update SHALL preserve a non-empty ordered list of document
operations.

#### Scenario: invalid mutation input never dispatches

- **WHEN** a request has an empty/excessive operation list, contradictory
  method/DID/job identity, malformed identifier, or excessive public data
- **THEN** construction SHALL fail with a stable redacted registration error
  before a registrar is invoked

#### Scenario: update order remains method-owned

- **WHEN** standard and method-specific update operations are combined
- **THEN** their exact order SHALL reach the selected adapter without generic
  diffing, reordering, resolution, or an atomicity claim

### Requirement: Opaque secret and custody modes

Internal, external, and client-managed secret modes SHALL contain no private
key, seed, password, decrypted payload, or arbitrary secret bag. Internal mode
SHALL require storage or return of an opaque handle; external mode SHALL use an
opaque custody handle; client-managed mode SHALL exchange only bounded public
action data.

#### Scenario: destructive internal secret policy fails closed

- **WHEN** internal mode would neither store generated material nor return an
  opaque handle
- **THEN** request construction SHALL reject it rather than permit silent loss
  of DID control

#### Scenario: diagnostics never reveal custody values

- **WHEN** requests, results, jobs, actions, handles, or validation failures are
  formatted for diagnostics or bridged to the SDK error surface
- **THEN** opaque identifiers, payloads, documents, public-data contents, and
  all private material SHALL be absent

### Requirement: Coherent terminal and non-terminal states

A finished or failed registration result SHALL have no job. An action or wait
result SHALL have one method-scoped job. Action state SHALL correlate the
job's expected action id exactly; wait state SHALL carry no expected action and
MAY provide an advisory wait no greater than 24 hours.

#### Scenario: contradictory draft examples are rejected

- **WHEN** an action/wait result omits a job or a terminal result retains one
- **THEN** the safer SDK profile SHALL reject it as an invalid state even if a
  draft example depicts that combination

#### Scenario: completed identity matches its request

- **WHEN** a result is validated for create, update, deactivate, continue, or
  cancel
- **THEN** its DID and job method SHALL match the request and create completion
  SHALL contain a DID

### Requirement: Explicit continuation and action correlation

An action continuation SHALL contain exactly one response whose id equals the
job's expected action id. A wait continuation SHALL contain no response. The
generic layer SHALL NOT sign, decrypt, redirect, poll, sleep, or spawn retries.

#### Scenario: stale or injected action response fails closed

- **WHEN** a continuation response is absent, unexpected, or uses another
  action id
- **THEN** request construction SHALL reject it before adapter dispatch

### Requirement: Deterministic idempotency and honest cancellation

Registrar implementations SHALL bind an idempotency key to one canonical
request. Identical replay SHALL return the same logical job/outcome; a different
request with that key SHALL fail as conflict. Dropping a Rust future SHALL NOT
mean the operation was cancelled. Cancellation SHALL be an explicit best-effort
request and SHALL NOT claim rollback of irreversible work.

#### Scenario: retry cannot duplicate a mutation silently

- **WHEN** an uncertain create/update/deactivate attempt is retried with its
  original idempotency key
- **THEN** the adapter SHALL resume or return the original logical outcome
  rather than submit a second mutation

#### Scenario: cancellation reports observed truth

- **WHEN** cancellation arrives after an adapter has submitted irreversible
  work
- **THEN** the adapter SHALL return its actual terminal or continuing state and
  SHALL NOT fabricate a cancelled rollback

### Requirement: Bounded public registration data

Public registration data SHALL be bounded and secret-free. Open option, update,
action, response, registration-metadata, and document-metadata maps SHALL be public-only and bounded to 64 properties per
map, 128 collection items, 32 levels, 4,096 nodes, and 64 KiB strings. Raw
public JSON entry points SHALL be at most 512 KiB and SHALL reject
private-material-shaped members recursively.

#### Scenario: native and JSON public data share one boundary

- **WHEN** excessive, malformed, reserved, control-bearing, or secret-shaped
  public data is supplied natively or through JSON
- **THEN** both entry points SHALL reject it with equivalent redacted reason

### Requirement: Object-safe registrar and exact method dispatch

The DID capability SHALL define a `Send + Sync` object-safe asynchronous
`DidRegistrar` over the validated request/result contract. A method binding MAY
add a registrar independently of its resolver and dereferencer. The immutable
registry SHALL dispatch by exact request/job method with no fallback, prefix,
allocation, mutation, or chain knowledge.

#### Scenario: independent methods share the lifecycle seam

- **WHEN** PRISM- and Midnight-shaped registrars are bound and invoked through
  `Arc<dyn DidRegistrar>`
- **THEN** each exact method SHALL receive only its original request and return
  its existing valid result without either method type entering the core

#### Scenario: missing capability is precise

- **WHEN** a method is unknown or a known method has no registrar
- **THEN** the registry SHALL return a valid terminal `methodNotSupported` or
  `featureNotSupported` result respectively

### Requirement: Runtime and cache isolation

Generic registration SHALL perform no ambient network, filesystem, clock,
randomness, persistence, signing, cache, global-registry, chain, or executor
operation. Successful update/deactivate results SHALL expose their exact DID so
an outer coordinator MAY invalidate all resolution cache variants explicitly.

#### Scenario: performance remains observable

- **WHEN** representative exact registry dispatch is repeatedly exercised in
  release mode
- **THEN** throughput SHALL be recorded without a machine-specific CI threshold

### Requirement: Duplicate-free raw DID document JSON

The bounded raw DID document entry points SHALL stream every JSON object name
through a duplicate detector before typed deserialization. The detector SHALL
reject duplicate decoded names within the same object at every nesting level,
including known fields, extensions, contexts, verification material, services,
and endpoint maps. Escaped and literal spellings that decode to the same name
SHALL collide. Names repeated only in different objects SHALL remain valid.

Failures SHALL expose a stable non-sensitive document reason and the existing
public `did.invalid_document` code without the rejected name, value, offset, or
document bytes. Native maps and materialized semantic JSON values SHALL NOT be
claimed to preserve a lexical condition they cannot represent.

#### Scenario: ambiguity fails before semantic collapse

- **WHEN** raw document JSON repeats a top-level, verification-method, JWK,
  service, context, endpoint, or arbitrary nested extension name
- **THEN** the document SHALL fail before typed construction regardless of
  which duplicate value a last-value-wins parser would otherwise retain

#### Scenario: decoded names define equality

- **WHEN** one object contains literal and escaped spellings that decode to the
  same JSON member name
- **THEN** the scanner SHALL reject the object without exposing that name

#### Scenario: object-local names do not create false collisions

- **WHEN** sibling or nested objects legitimately reuse the same member name
  but no individual object repeats it
- **THEN** duplicate scanning SHALL succeed and ordinary document validation
  SHALL decide the result

### Requirement: Independently evidenced URI recognition

The dependency-free production `Uri` parser SHALL remain governed by the RFC
3986 `URI` production and the SDK's documented ASCII and 4,096-byte resource
profile. Deterministic conformance tests SHALL compare accept/reject decisions
with the exact development-only `uriparse` 0.6.4 oracle used by NeoPRISM over
scheme, authority, path, query, fragment, percent-encoding, IPv4, IPv6, and
IPvFuture classes. Every mismatch SHALL be a pinned regression with an explicit
normative or SDK-policy classification; oracle normalization SHALL NOT change
the preserved SDK spelling.

#### Scenario: standards-shaped component combinations agree

- **WHEN** bounded RFC 3986 URI component combinations are generated
  deterministically
- **THEN** the SDK and oracle SHALL agree except for explicitly recorded SDK
  resource/profile differences

#### Scenario: development evidence does not widen runtime dependencies

- **WHEN** production, minimal-feature, mobile, or WASM dependency cones are
  built
- **THEN** `uriparse` SHALL NOT be a normal or target dependency of
  `identus-did`

### Requirement: Reproducible DID document hardening evidence

The DID Core test suite SHALL deterministically generate bounded URI/document
structures and raw duplicate mutations with a fixed reproducible algorithm.
It SHALL cover native and unique-name semantic-wire equivalence, every scanner
limit, malformed/trailing input, and retained minimized regressions. A release
diagnostic SHALL report hardened scan-and-parse throughput without a
machine-specific pass threshold. Sanitizer-backed DID/DID URL lexical fuzzing
SHALL use the separately bounded campaign contract, and resolution-envelope
scanning SHALL retain its own deterministic evidence.

#### Scenario: generated evidence is stable in ordinary CI

- **WHEN** the focused conformance suite runs on the same revision
- **THEN** it SHALL exercise the same bounded cases without randomness,
  network, filesystem, clock, nightly compiler, or consumer repository

#### Scenario: performance remains observable but portable

- **WHEN** the representative hardened document parser runs in release mode
- **THEN** throughput and the scanner's explicit resource shape SHALL be
  recorded without a hardware-specific acceptance threshold

### Requirement: Pinned resolution boundary conformance

The DID Core capability SHALL use the W3C DID Resolution v1 Candidate
Recommendation Snapshot dated 6 August 2026 as its compatibility baseline and
SHALL track later drafts by immutable revision. Deterministic generated tests
SHALL cover every public resolution scalar parser, all RFC 9457 error kinds,
resolution/dereferencing states, raw-entry and constructor/serde equivalence,
and every documented JSON ceiling. Portable assertions from the official
implementation-report suite SHALL be mapped without importing HTTP or method
behavior.

#### Scenario: moving draft changes are classified before adoption

- **WHEN** a later editor or Candidate Recommendation Draft revision is audited
- **THEN** normative result/binding changes SHALL require an explicit
  compatibility decision while non-normative changes SHALL NOT silently alter
  the SDK wire contract

#### Scenario: official portable result assertions are executable

- **WHEN** successful, invalid-DID, unsupported-method, or deactivated result
  vectors are evaluated
- **THEN** required members, returned/document DID equality, failure metadata,
  and standard error URLs SHALL match the pinned W3C assertions without an HTTP
  server or DID-method dependency

### Requirement: Explicit legacy resolution error migration

Strict result JSON SHALL accept only the current RFC 9457-shaped W3C error
object. A recognized legacy keyword SHALL migrate only through the explicit
adapter helper, and unknown keywords SHALL fail.

#### Scenario: legacy producers cannot create dual wire semantics

- **WHEN** a NeoPRISM, Midnight, Lace, or Oxid adapter receives a legacy keyword
- **THEN** it SHALL deliberately map a recognized keyword to the exact standard
  URL object before SDK construction rather than relying on permissive raw JSON

### Requirement: Sanitizer-backed DID lexical fuzzing

The DID Core capability SHALL provide separate sanitizer-backed fuzz targets
for the public `Did` and `DidUrl` lexical boundaries. Targets SHALL accept
arbitrary bytes, exercise every UTF-8 value within a bounded generator profile,
and treat rejection as valid. Every accepted value SHALL preserve exact
spelling across borrowed access, Display, owned construction, `FromStr`, and
serde round trips. Accepted DID URL component views SHALL select valid in-bounds
UTF-8 subslices and reconstruct the complete owned value exactly.

Fuzz-only dependencies SHALL remain in an independent workspace outside every
published crate dependency cone. Method-specific validation, resolution, chain
state, trust, custody and product policy SHALL NOT enter these targets.

#### Scenario: hostile input cannot violate a public invariant

- **WHEN** arbitrary bounded bytes contain valid, invalid, non-ASCII, malformed,
  delimiter-heavy, percent-encoded, or over-limit identifier text
- **THEN** parsing SHALL either reject it without panic or return a value whose
  complete public construction, serialization and component invariants hold

#### Scenario: fuzz tooling does not become an SDK dependency

- **WHEN** production, minimal-feature, mobile, WASM, or downstream dependency
  cones are built
- **THEN** cargo-fuzz, libFuzzer and sanitizer support SHALL NOT be required by
  or exposed from an SDK crate

### Requirement: Reproducible bounded fuzz campaigns

The repository SHALL pin its sanitizer-capable compiler, fuzz runner and runtime
binding and SHALL expose one documented command interface for committed-corpus
replay, deterministic fixed-run smoke, and time-boxed soak modes. Pull-request
and integration smoke SHALL use a fixed seed, run count, input ceiling, timeout,
memory ceiling and one worker. Scheduled/manual soak SHALL be bounded separately
so ordinary delivery latency does not depend on a long random campaign.

Original seed corpora and dictionaries SHALL cover generic W3C and consumer-
shaped lexical boundaries without asserting method semantics. Failing artifacts
SHALL be retained for triage; each accepted defect SHALL be minimized and
promoted to committed corpus and deterministic regression evidence. Execution
rate and wall time MAY be recorded but SHALL NOT have a machine-specific pass
threshold.

#### Scenario: ordinary CI is repeatable and fast

- **WHEN** the same revision runs the pull-request fuzz gate
- **THEN** each target SHALL receive the same seed, run count and resource
  limits through the pinned Nix environment and SHALL terminate deterministically

#### Scenario: longer search remains bounded and diagnosable

- **WHEN** a scheduled or manually dispatched soak finds a sanitizer or
  invariant failure
- **THEN** the target SHALL stop within its documented resource/time envelope
  and preserve the failing artifact for minimization without printing its bytes
  as trusted data

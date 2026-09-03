## ADDED Requirements

### Requirement: Bounded unambiguous DID URL parameter preparation

The DID capability SHALL parse query parameters without HTML form semantics,
percent-decode names and values exactly once, preserve literal plus signs and
reject empty names, malformed encoding, invalid text, bounds violations or
duplicate decoded names. Resolution parameters SHALL be projected to bounded
`ResolutionOptions`; resource selectors SHALL remain local to dereferencing.

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

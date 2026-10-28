# did:web method specification

## ADDED Requirements

### Requirement: The method profile is strict and bounded

The SDK SHALL expose an independently selectable `identus-did-web` crate that
accepts only the approved bounded ASCII FQDN, optional canonical encoded port,
and colon-path profile. It SHALL reject IP literals, user info, query,
fragment, ambiguous percent escapes, noncanonical ports, empty or dot path
segments, and inputs exceeding public limits with content-free errors.

#### Scenario: a path DID is valid

- **WHEN** `did:web:example.com:users:alice` is parsed
- **THEN** the validated value derives `https://example.com/users/alice/did.json`

#### Scenario: URL authority is ambiguous

- **WHEN** a DID contains an IP host, user info, raw percent escape, default
  port, query, fragment, or invalid DNS label
- **THEN** parsing fails before any fetch port is invoked

### Requirement: Pure URL derivation follows the pinned method

The SDK SHALL map a bare domain to
`https://<domain>/.well-known/did.json`, map path components to slash-delimited
segments ending in `/did.json`, and decode only the canonical authority `%3A`
port delimiter. Derivation SHALL perform no I/O or runtime URL normalization.

#### Scenario: encoded port and path are mapped

- **WHEN** `did:web:example.com%3A3000:user:alice` is prepared
- **THEN** the request URL is exactly
  `https://example.com:3000/user/alice/did.json`

### Requirement: Retrieval is caller-injected and least-authority

The SDK SHALL expose an executor-neutral asynchronous fetch port accepting one
immutable bounded GET request. A resolution SHALL invoke it at most once. The
crate SHALL NOT provide a concrete HTTP, DNS, TLS, redirect, proxy, cache,
retry, timer, executor, or SSRF implementation.
The request SHALL advertise only representations that can satisfy the selected
resolution output and SHALL expose the caller's cache-bypass intent to the
adapter.

#### Scenario: a runtime resolves a web DID

- **WHEN** a caller supplies a fetch adapter and resolves a valid `did:web`
- **THEN** the core sends the exact prepared request through that adapter and
  retains all network and QoS policy in the caller

### Requirement: Responses are bounded and exactly correlated

The SDK SHALL validate status and bounded supported JSON media type before
parsing a non-empty bounded body with `identus-did`. Success SHALL require the
resolved DID document `id` to equal the requested DID exactly. Errors SHALL not
contain the DID, URL, header, body, or transport diagnostics.
Only status 404 SHALL map to `notFound`; other non-success statuses SHALL remain
non-cacheable internal failures. A requested JSON-LD representation SHALL
require both `application/did+ld+json` and a document context rather than
relabeling ordinary DID JSON.

#### Scenario: a fetched document belongs to another DID

- **WHEN** a valid response document has an `id` different from the requested DID
- **THEN** resolution fails with the fixed document-mismatch category

#### Scenario: response work exceeds a public ceiling

- **WHEN** content type or body exceeds the configured limit
- **THEN** binding fails before avoidable parsing or document allocation

#### Scenario: an upstream outage is not a missing DID

- **WHEN** the adapter returns status 500 or 503
- **THEN** resolution reports an internal failure rather than `notFound`

#### Scenario: the requested representation does not match

- **WHEN** JSON-LD is requested but the response is DID JSON or lacks a context
- **THEN** resolution fails without labeling the document as JSON-LD

### Requirement: The production port has deterministic end-to-end evidence

The repository SHALL include an in-memory adapter E2E that uses the public
fetch trait and proves request mapping, invocation, bounded response parsing,
exact ID correlation, and redacted failure behavior without external network
access.

#### Scenario: portable E2E resolves successfully

- **WHEN** the in-memory adapter returns a valid media-typed document for the
  exact request
- **THEN** the public resolver returns that validated document and records one
  expected fetch invocation

# oid4vp-authorization-request-routing Specification

## Purpose
TBD - created by archiving change validate-oid4vp-final-routing. Update Purpose after archive.
## Requirements
### Requirement: Authorization semantics are one consuming composed transition

The SDK SHALL consume one `VerifiedRequestObject`, deserialize its already
bounded duplicate-safe payload once, and return one state that preserves JAR
evidence and owns one valid DCQL query.

#### Scenario: one supported request is valid

- **WHEN** a verified request has exact supported authorization fields,
  direct-post routing, and one valid DCQL query
- **THEN** the result owns the composed evidence without retaining or exposing
  the raw payload

#### Scenario: a caller still needs query-only evidence

- **WHEN** the existing DCQL-only transition is used
- **THEN** its public behavior and narrower evidence claim remain unchanged

### Requirement: The first profile supports only VP token direct post

The SDK SHALL require exact `response_type=vp_token`, one supported exact
response mode, one bounded valid nonce, one mode-matching bounded destination,
and one object-valued DCQL query with no scope-based selection.

For `direct_post` and `direct_post.jwt`, it SHALL require one absolute HTTPS
`response_uri` and no `redirect_uri`. For `fragment`, `query`, and `form_post`,
it SHALL require one absolute `redirect_uri` authorized by the exact verifier
trust decision and no `response_uri`.

It SHALL accept `direct_post.jwt` only with bounded verifier metadata that
selects exactly one supported public recipient and content-encryption profile.

#### Scenario: direct-post routing is coherent

- **WHEN** every required value has the supported exact spelling and the
  response destination meets the HTTPS syntax policy
- **THEN** the result reports safe response enums and exposes retained strings
  only through explicitly sensitive accessors

#### Scenario: redirect routing is coherent

- **WHEN** `fragment`, `query`, or `form_post` carries one exact bounded
  verifier-authorized redirect destination
- **THEN** the result retains only that destination for the matching inert response constructor

#### Scenario: protected direct-post metadata is coherent

- **WHEN** `direct_post.jwt` metadata selects exactly one public P-256
  `ECDH-ES` recipient and the A128GCM content-encryption profile
- **THEN** the validated request privately retains the bounded encryption plan

#### Scenario: routing is absent, conflicting, or unsupported

- **WHEN** a required value is missing or malformed, an unsupported value is
  supplied, the destination does not match the mode, or both destinations are present
- **THEN** validation terminates with the corresponding static category

#### Scenario: the nonce is invalid or outside policy

- **WHEN** `nonce` is empty, oversized, non-string, or contains a byte outside
  the Final ASCII unreserved grammar
- **THEN** validation terminates before destination or DCQL processing

#### Scenario: the response URI is unsafe or outside policy

- **WHEN** `response_uri` is oversized, not absolute HTTPS, lacks a host, or
  contains user information or a fragment
- **THEN** validation terminates without performing network work

#### Scenario: the redirect URI is unsafe or outside policy

- **WHEN** `redirect_uri` is oversized, not absolute, contains user information,
  or violates mode-specific component/collision policy
- **THEN** validation terminates without performing browser work

#### Scenario: protected routing metadata is absent or ambiguous

- **WHEN** `direct_post.jwt` metadata is absent, malformed, unsupported or
  selects zero or multiple recipients
- **THEN** validation terminates with a static category before response construction

### Requirement: Validation order and diagnostics are deterministic and redacted

The SDK SHALL validate response type, response mode, nonce, destination, and
DCQL in that order and SHALL keep raw payload, client, nonce, URI, query,
credential, and presentation values out of diagnostics.

#### Scenario: several independent fields are invalid

- **WHEN** one request violates more than one semantic rule
- **THEN** the first failure follows the documented order independent of map
  insertion order

#### Scenario: hostile values carry canaries

- **WHEN** verifier-controlled fields fail validation
- **THEN** `Debug`, `Display`, errors, metadata, and metrics contain only safe
  enum, count, length, and static category evidence

### Requirement: Routing validity does not imply transport or trust

The SDK SHALL document that the composed state does not authorize the verifier
key, trust the response endpoint, execute HTTP, verify credentials, record
consent, construct a presentation, or prove response safety.

#### Scenario: the HTTPS destination resolves to an unsafe network address

- **WHEN** a syntactically valid response URI would violate caller network
  policy
- **THEN** the SDK state confers no permission to execute that request

### Requirement: Authorization state retains bounded response lineage

The SDK SHALL retain optional bounded `state` and only the private DCQL
response-shape evidence needed by a consuming response transition while it
constructs the existing composed authorization state.

#### Scenario: bounded state is present

- **WHEN** a supported request contains optional bounded state
- **THEN** the composed result retains it for exact return by the consuming
  response transition without exposing it in diagnostics

#### Scenario: optional state is outside policy

- **WHEN** `state` is empty, not a string, contains non-visible ASCII, or
  exceeds the configured bound
- **THEN** request validation fails with a stable redacted category before DCQL
  response construction can occur

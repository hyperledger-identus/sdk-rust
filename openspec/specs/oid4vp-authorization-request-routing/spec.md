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

The SDK SHALL require exact `response_type=vp_token`, exact
`response_mode=direct_post`, one bounded valid nonce, one bounded absolute
HTTPS `response_uri`, no `redirect_uri`, and one object-valued DCQL query with
no scope-based selection.

#### Scenario: direct-post routing is coherent

- **WHEN** every required value has the supported exact spelling and the
  destination meets the HTTPS syntax policy
- **THEN** the result reports safe response enums and exposes retained strings
  only through explicitly sensitive accessors

#### Scenario: routing is absent, conflicting, or unsupported

- **WHEN** a required value is missing/malformed, an unsupported value is
  supplied, or both response and redirect destinations are present
- **THEN** validation terminates with the corresponding static category

#### Scenario: the nonce is invalid or outside policy

- **WHEN** `nonce` is empty, oversized, non-string, or contains a byte outside
  the Final ASCII unreserved grammar
- **THEN** validation terminates before destination or DCQL processing

#### Scenario: the response URI is unsafe or outside policy

- **WHEN** `response_uri` is oversized, not absolute HTTPS, lacks a host, or
  contains user information or a fragment
- **THEN** validation terminates without performing network work

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

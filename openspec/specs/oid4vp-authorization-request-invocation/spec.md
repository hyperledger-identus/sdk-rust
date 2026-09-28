# oid4vp-authorization-request-invocation Specification

## Purpose
Define exact bounded parsing for an OpenID4VP Authorization Request that
references its Request Object by HTTPS URI, without granting transport or
verifier authority.
## Requirements
### Requirement: Reference invocation is exact and bounded

The SDK SHALL parse one bounded `openid4vp:?…` Authorization Request reference
at the Final static `openid4vp:` authorization endpoint containing exactly one
decoded non-empty `client_id`, exactly one decoded HTTPS `request_uri`, and at
most one `request_uri_method` whose supported case-sensitive explicit values
are `get` and `post`.

#### Scenario: Final by-reference shape arrives

- **WHEN** a bounded invocation carries `client_id` and `request_uri` without a method
- **THEN** the result retains redacted owned values and reports GET retrieval

#### Scenario: GET is requested explicitly

- **WHEN** `request_uri_method=get` occurs once
- **THEN** the result reports GET retrieval

#### Scenario: POST capability negotiation is requested

- **WHEN** `request_uri_method=post` occurs once
- **THEN** the result reports POST without performing transport

#### Scenario: a configurable boundary is exceeded

- **WHEN** invocation bytes, pairs, names, values, client id or URI exceed the configured maximum
- **THEN** parsing fails before retaining a partial result

### Requirement: Ambiguous and unsupported transports fail closed

The SDK SHALL reject decoded duplicate parameters, malformed form data,
authority/path/fragment endpoint variants, unsafe references, by-value or inline
transports and unsupported transaction data.

#### Scenario: a product-specific route is supplied

- **WHEN** the invocation uses `openid4vp://authorize` instead of the static endpoint
- **THEN** parsing returns the invalid-invocation category

#### Scenario: encoded names collide

- **WHEN** literal and percent-encoded forms decode to the same parameter name
- **THEN** parsing returns the duplicate-parameter category

#### Scenario: an unsupported transport is supplied

- **WHEN** `request` or inline protocol parameters replace `request_uri`
- **THEN** parsing returns the unsupported-transport category

#### Scenario: an unknown extension is supplied

- **WHEN** a bounded unique unknown outer parameter is present
- **THEN** it is ignored and cannot shadow a recognized field

### Requirement: Parsing grants no verifier authority

The SDK SHALL expose the Request URI only as untrusted syntax and SHALL NOT
fetch, authenticate, verify, execute DCQL, choose credentials, record consent,
or construct an Authorization Response.

#### Scenario: a syntactically valid reference parses

- **WHEN** the reference is valid HTTPS syntax
- **THEN** no trust, provenance, reachability or signature claim is produced

### Requirement: Diagnostics are stable and redacted

Public errors, `Debug`, `Display` and component metadata SHALL not contain any
client identifier, URI, query value, Request Object, or verifier canary.

#### Scenario: hostile values fail

- **WHEN** verifier-controlled canaries are included in malformed input
- **THEN** every public diagnostic contains only stable category metadata

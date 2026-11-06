# oid4vp-direct-post-response Specification

## ADDED Requirements

### Requirement: protocol-error construction consumes validated request lineage

The SDK SHALL construct a plain direct-post Authorization Error Response only
by consuming one `ValidatedAuthorizationRequest` and an explicit SDK-owned
protocol error code. It SHALL derive the destination and optional state from
that request and SHALL NOT automatically translate SDK or transport errors.

#### Scenario: caller selects a Final protocol error

- **WHEN** a caller explicitly selects a supported error after applying its
  privacy and consent policy
- **THEN** the SDK returns one inert error envelope bound to the validated
  response URI and exact request state

#### Scenario: internal processing fails

- **WHEN** request parsing, validation, transport or SDK processing returns an
  internal error
- **THEN** no implicit conversion produces a protocol error response

### Requirement: the protocol vocabulary and optional fields are bounded

The SDK SHALL expose the RFC 6749 authorization errors and OpenID4VP 1.0 Final
errors named in sections 8.5 and Appendix E as a closed enum. Optional
`error_description` SHALL be non-empty RFC NQSCHAR ASCII under an explicit
limit. Optional `error_uri` SHALL be a bounded syntactically valid URI-reference.
All variable values SHALL have redacted diagnostics and SHALL NOT imply display,
resolution, provenance, privacy or consent safety.

#### Scenario: bounded developer information is supplied

- **WHEN** description and URI-reference satisfy their exact syntax and limits
- **THEN** the SDK retains them as untrusted redacted protocol values

#### Scenario: optional information is malformed or oversized

- **WHEN** a value is empty, violates RFC grammar, fails URI-reference parsing,
  or exceeds policy
- **THEN** construction fails with a static redacted category

### Requirement: error form output is deterministic, exclusive and bounded

The SDK SHALL emit bounded UTF-8 `application/x-www-form-urlencoded` material
ordered as `error`, optional `error_description`, optional `error_uri`, and
exact request `state` when present. Error output SHALL never contain `vp_token`.
The final encoded body SHALL be checked with overflow-safe arithmetic before
allocation and returned in a redaction-safe inert envelope.

#### Scenario: Final example is constructed

- **WHEN** `invalid_request`, description `unsupported client_id_prefix`, and
  request state are supplied
- **THEN** the form matches the Final field order and encoding

#### Scenario: encoded output exceeds policy

- **WHEN** optional values or percent encoding exceed the configured form bound
- **THEN** construction fails without returning a partial envelope

### Requirement: error construction owns no effects or privacy decision

The SDK SHALL NOT perform HTTP, resolve an error URI, authorize disclosure,
claim End-User interaction, expose holder data, or infer successful delivery.

#### Scenario: a caller delivers or displays error material

- **WHEN** a caller reveals the inert target/body or optional developer fields
- **THEN** its adapter remains responsible for transport, UI, localization,
  user interaction, consent and privacy policy

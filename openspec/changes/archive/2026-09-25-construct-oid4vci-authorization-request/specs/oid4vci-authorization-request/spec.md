## ADDED Requirements

### Requirement: The request uses one exact Authorization Details credential intent

The SDK SHALL consume one `CredentialOfferWithAuthorizationRequestInput` and
construct exactly one `openid_credential` Authorization Detail naming the
already selected offered `credential_configuration_id`. It SHALL emit one
`locations` value equal to the validated Credential Issuer identifier exactly
when Credential Issuer Metadata explicitly contains `authorization_servers`.
It SHALL NOT emit `scope` or `resource`.

#### Scenario: default issuer-as-server omits locations

- **WHEN** issuer metadata omits `authorization_servers`
- **THEN** the deterministic Authorization Details object contains only
  `type` and `credential_configuration_id`

#### Scenario: explicit authorization servers require locations

- **WHEN** issuer metadata explicitly contains one or more Authorization
  Servers
- **THEN** the object additionally contains exactly one `locations` array
  value equal to the Credential Issuer identifier

### Requirement: Existing Authorization Endpoint query parameters are retained safely

The SDK SHALL retain the exact existing Authorization Endpoint query bytes
only after strictly validating a positive bounded number of fields. Every
decoded name SHALL be non-empty and unique; every decoded name and value SHALL
meet independent byte ceilings and strict form syntax. The SDK SHALL reject a
name colliding with a managed request parameter, `scope`, `resource`,
`request`, or `request_uri`.

#### Scenario: a safe extension is retained exactly

- **WHEN** the endpoint contains one unique bounded extension parameter with
  valid form encoding
- **THEN** its original bytes remain first and managed parameters follow one
  ampersand

#### Scenario: decoded collision fails

- **WHEN** a literal or percent-encoded endpoint name decodes to a reserved
  name, or two names decode identically
- **THEN** construction fails with a static collision error before producing a
  request URI

#### Scenario: malformed or oversized query fails

- **WHEN** a query field is malformed, contains invalid UTF-8/NUL, exceeds a
  component bound, or exceeds the parameter-count bound
- **THEN** construction fails with the corresponding static bounded error

### Requirement: Authorization Request serialization is deterministic and bounded

The SDK SHALL append one application/x-www-form-urlencoded query containing,
in order, `response_type=code`, `client_id`, `redirect_uri`, `state`,
`code_challenge`, `code_challenge_method=S256`, `authorization_details`, and
optional exact offered `issuer_state`. Form output SHALL use space-to-plus and
uppercase percent-encoding. Authorization Details JSON and the final exact
request URI SHALL have independent positive byte ceilings checked with safe
arithmetic.

#### Scenario: exact request bytes are stable

- **WHEN** the same validated predecessor and limits are supplied twice
- **THEN** both requests contain identical minified JSON, parameter order,
  form encoding and URI bytes

#### Scenario: offered issuer state is included exactly

- **WHEN** the Authorization Code offer contains `issuer_state`
- **THEN** one final encoded `issuer_state` parameter carries that exact value

#### Scenario: output boundary is fail-closed

- **WHEN** Authorization Details or the final request URI exceeds its exact
  ceiling
- **THEN** construction returns a static size error without partial output or
  caller/issuer data

### Requirement: The request remains a transport-neutral correlation predecessor

The request SHALL retain the complete issue #352 predecessor, own the exact URI
in zeroizing storage, expose it only through an explicitly sensitive accessor,
and keep Debug data-free except for byte count and issuer-state presence. It
SHALL NOT execute PAR/HTTP/browser behavior, accept arbitrary caller
extensions, parse or correlate an Authorization Response, validate returned
state/issuer/redirect, exchange a code, or claim authorization completion.

#### Scenario: construction does not overclaim authorization

- **WHEN** a valid request URI is constructed
- **THEN** its API and documentation preserve the verifier/state lineage while
  naming response correlation and execution as separate responsibilities

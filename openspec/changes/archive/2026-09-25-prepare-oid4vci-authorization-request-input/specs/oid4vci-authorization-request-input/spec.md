# oid4vci-authorization-request-input Specification

## ADDED Requirements

### Requirement: Request inputs advance only a capable server-bound offer

The SDK SHALL consume one
`CredentialOfferWithAuthorizationCodeServer`, require an in-range index into
its ordered offered Credential Configuration list, and return an owned
Authorization Request input predecessor. The predecessor SHALL retain the
server-bound state and expose the selected existing Credential Configuration
by shared reference without copying a caller-provided identifier.

#### Scenario: one offered configuration is selected

- **WHEN** the caller supplies an in-range offered configuration index and all
  request inputs are valid
- **THEN** the resulting state exposes that exact existing configuration and
  preserves the offer, issuer metadata, selected Authorization Server and
  optional `issuer_state`

#### Scenario: configuration index is absent

- **WHEN** the index is outside the offered list
- **THEN** construction fails with a static configuration-missing error before
  retaining caller input

### Requirement: OAuth caller inputs are bounded and syntactically validated

Input limits SHALL be positive. The client identifier and CSRF state SHALL be
non-empty RFC 6749 `VSCHAR` sequences within their byte limits. The redirect
URI SHALL be within its byte limit, absolute, and contain neither a fragment
nor userinfo. Validation SHALL allow structurally valid HTTPS, loopback HTTP
and private-use custom-scheme callbacks without asserting registration policy.

#### Scenario: exact input limits succeed

- **WHEN** client identifier, redirect URI and state meet their exact positive
  byte ceilings and syntax
- **THEN** the SDK retains them under redacted owned types

#### Scenario: bounds precede syntax

- **WHEN** an input is one byte over its limit and also malformed
- **THEN** the corresponding size error wins and contains none of the input

#### Scenario: unsafe redirect structure is rejected

- **WHEN** a redirect is relative or contains a fragment or userinfo
- **THEN** construction fails with the static invalid-redirect error

### Requirement: PKCE S256 is derived from a validated verifier

The SDK SHALL accept only a caller-generated code verifier of 43 through 128
ASCII unreserved characters. It SHALL compute S256 as canonical unpadded
base64url of SHA-256 over the verifier ASCII bytes with existing audited SDK
primitives. It SHALL NOT accept a challenge argument, generate entropy, expose
an external crypto type, or support `plain`.

#### Scenario: RFC 7636 Appendix B matches

- **WHEN** the Appendix B verifier is supplied
- **THEN** the derived challenge is
  `E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM` and the method is `S256`

#### Scenario: verifier grammar fails safely

- **WHEN** a verifier is shorter than 43, longer than 128, non-ASCII, or
  contains a character outside ALPHA, DIGIT, hyphen, period, underscore and
  tilde
- **THEN** construction returns a static bounded error without panic or input
  disclosure

### Requirement: The state is a preparation boundary, not authorization

The predecessor SHALL expose borrowed validated values needed by a later
serializer and SHALL use redacted Debug output. It SHALL NOT serialize a URL,
choose scope or `authorization_details`, execute PAR/HTTP/browser/callback
behavior, compare an Authorization Response, exchange a code, or claim CSRF,
mix-up, redirect-registration, server-trust or authorization completion.

#### Scenario: input preparation does not overclaim

- **WHEN** a valid predecessor is created
- **THEN** documentation states the remaining serializer, response-correlation
  and execution boundaries explicitly

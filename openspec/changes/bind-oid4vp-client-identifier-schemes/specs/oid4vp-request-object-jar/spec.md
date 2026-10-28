## ADDED Requirements

### Requirement: Client identifier prefixes are classified before trust policy

The SDK SHALL classify the first-colon client identifier syntax, preserve the
full identifier, support the Final signed schemes through typed evidence, and
fail closed for unsigned, DC-API-only, or unknown schemes in this profile.

#### Scenario: a signed Final scheme is recognized

- **WHEN** a request uses pre-registration, Federation, DID, verifier attestation, X.509 SAN DNS, or X.509 hash syntax
- **THEN** the SDK supplies typed bounded scheme evidence to verifier authorization

#### Scenario: a redirect URI or origin scheme is used

- **WHEN** the signed non-DC-API pipeline receives `redirect_uri` or `origin`
- **THEN** it rejects the request before a trust adapter is invoked

#### Scenario: an unknown prefix is used

- **WHEN** a colon is present and its first component is not a supported Final prefix
- **THEN** the default profile rejects rather than silently treating it as pre-registration

### Requirement: Verifier authorization binds the exact verified key

The SDK SHALL consume request security evidence through a caller-owned trust
port with the exact full client identifier, scheme evidence, protected header,
and public key that verified the correlated JWS.

#### Scenario: the adapter accepts the exact binding

- **WHEN** scheme syntax and metadata are valid and the port accepts the exact verifier/key/request evidence
- **THEN** the SDK returns verifier-authorization evidence eligible for typed routing

#### Scenario: trust is rejected or unavailable

- **WHEN** the port rejects or cannot decide
- **THEN** validation fails closed with a distinct static redacted category

### Requirement: Verifier metadata and verifier info are bounded evidence

The SDK SHALL validate applicable inline verifier metadata, Federation trust
chains, and verifier-info structures under finite limits and enforce all
verifier-info credential references against the request's DCQL query.

#### Scenario: bounded metadata is structurally valid

- **WHEN** metadata follows the source rules and verifier-info references known DCQL credential ids
- **THEN** borrowed bounded evidence, including the exact textual signed
  `response_uri` when present, is available to the trust port

#### Scenario: metadata is malformed, oversized, ambiguous, or mis-sourced

- **WHEN** collection, string, data-shape, uniqueness, credential-reference, or scheme source rules fail
- **THEN** validation stops before trust-port invocation

### Requirement: Typed routing requires verifier authorization evidence

The SDK SHALL expose authorization-routing successors only from consumed
verifier-authorization evidence.

#### Scenario: only request security evidence exists

- **WHEN** verifier/key authorization has not succeeded
- **THEN** the public typed API does not permit routing to authorization processing

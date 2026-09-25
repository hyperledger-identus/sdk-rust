# oid4vci-authorization-code-server Specification

## Purpose
TBD - created by archiving change bind-oid4vci-authorization-code-server. Update Purpose after archive.
## Requirements
### Requirement: Authorization Code server selection is an ordered owned transition

The SDK SHALL consume one `CredentialOfferWithMetadata` and one
`AuthorizationServerMetadataCore` to create a
`CredentialOfferWithAuthorizationCodeServer` only after all requirements in
this capability pass. It SHALL first require an offered Authorization Code
grant, then effective server advertisement, exact hint agreement, effective
grant support, and Authorization Endpoint presence in that order.

The result SHALL own both validated predecessors and expose them by shared
reference. It SHALL NOT reparse, normalize or copy their remote values.

#### Scenario: a capable selected server advances the state

- **WHEN** an Authorization Code offer selects an advertised server whose
  metadata effectively supports `authorization_code` and contains an
  Authorization Endpoint
- **THEN** the transition returns an owned authorization-code server state

#### Scenario: absent grant wins diagnostic precedence

- **WHEN** the offer has no Authorization Code grant regardless of supplied
  server metadata
- **THEN** the transition returns the static grant-missing error without
  evaluating server capabilities

### Requirement: Selection preserves exact cross-document agreement

The selected server issuer SHALL equal one effective Authorization Server in
the matched Credential Issuer Metadata. When the Authorization Code grant has
an `authorization_server` hint, the selected issuer SHALL also equal that hint
exactly. Existing identifier validation and issuer-metadata multiplicity rules
SHALL remain unchanged.

#### Scenario: an unadvertised server is rejected

- **WHEN** the selected issuer is not an effective Authorization Server
- **THEN** the existing unadvertised-server diagnostic is returned

#### Scenario: an exact grant hint mismatch is rejected

- **WHEN** the selected issuer is advertised but differs from the grant hint
- **THEN** the static Authorization Code hint-mismatch diagnostic is returned

### Requirement: Effective capability follows RFC 8414 defaults

The selected server SHALL effectively support exact `authorization_code`.
When `grant_types_supported` is omitted, the existing RFC 8414 defaults
`authorization_code` and `implicit` SHALL apply and satisfy this requirement.
When the list is present, it SHALL contain exact `authorization_code`.

The selected server SHALL expose a validated Authorization Endpoint. Token
Endpoint presence SHALL NOT be required by this transition.

#### Scenario: omitted grant metadata uses the standard default

- **WHEN** an otherwise valid selected server omits `grant_types_supported`
- **THEN** the transition succeeds because `authorization_code` is effective

#### Scenario: explicit unsupported grant fails before endpoint presence

- **WHEN** an advertised selected server explicitly omits
  `authorization_code` and also has no Authorization Endpoint
- **THEN** the grant-not-supported diagnostic is returned

#### Scenario: supported grant without endpoint is rejected

- **WHEN** an advertised selected server supports `authorization_code` but has
  no Authorization Endpoint
- **THEN** the Authorization Endpoint required diagnostic is returned

### Requirement: The state is least-authority and redacted

The state SHALL preserve optional `issuer_state` through the existing grant
owner. Debug, Display and error output SHALL NOT expose the issuer state,
server identifier, endpoints or retained JSON. No new dependency, feature,
lockfile, unsafe/native, network, target, consumer, chain or product behavior
SHALL be introduced.

The state SHALL NOT claim metadata provenance, trust, endpoint reachability,
client eligibility, request readiness, PKCE/CSRF protection, Final section
12.3 mix-up protection, authorization or successful issuance.

#### Scenario: optional issuer state remains accessible but redacted

- **WHEN** the accepted grant contains `issuer_state`
- **THEN** callers can borrow it through the retained predecessor while Debug
  contains none of its remote text


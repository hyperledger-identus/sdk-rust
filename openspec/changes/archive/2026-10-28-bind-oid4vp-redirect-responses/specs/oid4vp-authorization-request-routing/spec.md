## MODIFIED Requirements

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

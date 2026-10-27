# oid4vp-authorization-request-routing delta

## MODIFIED Requirements

### Requirement: The first profile supports only VP token direct post

The SDK SHALL require exact `response_type=vp_token`, exact
`response_mode=direct_post`, one bounded valid nonce, one bounded absolute
HTTPS `response_uri`, no `redirect_uri`, and one object-valued DCQL query with
no scope-based selection.

It SHALL additionally accept exact `response_mode=direct_post.jwt` only with
bounded verifier metadata that selects exactly one supported public recipient
and content-encryption profile.

#### Scenario: direct-post routing is coherent

- **WHEN** every required value has the supported exact spelling and the
  destination meets the HTTPS syntax policy
- **THEN** the result reports safe response enums and exposes retained strings
  only through explicitly sensitive accessors

#### Scenario: protected direct-post metadata is coherent

- **WHEN** `direct_post.jwt` metadata selects exactly one public P-256
  `ECDH-ES` recipient and the A128GCM content-encryption profile
- **THEN** the validated request privately retains the bounded encryption plan

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

#### Scenario: protected routing metadata is absent or ambiguous

- **WHEN** `direct_post.jwt` metadata is absent, malformed, unsupported or
  selects zero or multiple recipients
- **THEN** validation terminates with a static category before response
  construction

## MODIFIED Requirements

### Requirement: Final JWT Credential Request construction is state-bound and bounded

The SDK SHALL expose positive `JwtCredentialRequestLimits` with independent
maximums for proof count, one compact proof byte length, the complete JSON body
byte length, and the complete Authorization field-value byte length. Every
maximum SHALL be non-zero; invalid limits SHALL fail with one fieldless static
invalid-limits error. Defaults SHALL be finite.

`CredentialOfferWithMetadata::try_create_jwt_credential_request` SHALL borrow
that matched state, one `TokenResponseCore`, a zero-based offered Credential
Configuration index, a non-empty ordered slice of
`identus_oid4vci::Oid4vciProofJwt`, and the limits. It SHALL succeed only when
the index selects an ID from the matched offer, the Token Response has no
unvalidated Authorization Details, its token type is case-insensitively
`Bearer`, its exact access token matches RFC 6750 `b64token`, the proof list and
every compact proof are within bounds, and the resulting authorization/body
values are within bounds.

`CredentialOfferWithMetadata::try_create_authorized_jwt_credential_request`
SHALL instead borrow one `TokenResponseWithAuthorizationDetails`, checked
zero-based authorization-detail and Credential Dataset identifier indices, the
proofs and limits. It SHALL require the selected detail's configuration ID to
equal an offered configuration ID before applying the same token, proof and
allocation checks.

#### Scenario: validated states produce one request

- **WHEN** matched offer/metadata, a Bearer Token Response without Authorization
  Details, one valid offered index and bounded holder-produced JWT proofs are
  supplied
- **THEN** construction returns one owned request for the selected offered
  configuration while preserving proof order

#### Scenario: selection is not a raw caller claim

- **WHEN** the offered index is outside the validated offer list
- **THEN** construction fails before copying token or proof material and cannot
  emit a request for an unoffered configuration

#### Scenario: authorized dataset selection is state-bound

- **WHEN** checked indices select a dataset identifier from a recognized detail
  whose configuration was offered
- **THEN** construction returns one owned request for that authorized dataset
- **AND WHEN** either index is invalid or its configuration was not offered
- **THEN** construction fails before copying token or proof material

#### Scenario: incompatible token route or syntax fails closed

- **WHEN** the presence-only Token Response contains Authorization Details,
  either route advertises a non-Bearer token type, or the access token is
  outside the RFC 6750 Bearer grammar
- **THEN** construction fails with the corresponding static state/type error
  and does not guess Credential identifiers or token presentation syntax

#### Scenario: proof and allocation limits are independent

- **WHEN** limits are zero, the proof list is empty or excessive, one proof is
  oversized, or the final authorization/body value is oversized
- **THEN** construction returns the corresponding fieldless limit error without
  returning a partial request

# Design

## Consuming transition

Add `CorrelatedAuthorizationCodeTokenResponse::try_into_authorized_jwt_credential_request`,
taking one dataset index, borrowed JWT proofs and the existing
`JwtCredentialRequestLimits`. The method consumes the correlated state and
moves its lineage, Token Response core and zeroizing identifier vector through
a crate-private decomposition helper.

Select the identifier by checked index. Derive the Credential Endpoint from
the retained issuer metadata and pass it, the retained Token Response core and
selected identifier to the existing serializer logic.

## Serializer reuse

Generalize the current private method into one module-private function that
accepts a validated Credential Endpoint, Token Response core and private
selector enum. Both existing constructors and the new transition call it.
Preserve validation order and exact output bytes.

## Ownership and failure

The returned `JwtCredentialRequest` remains the same type. It independently
owns a duplicated validated endpoint and zeroizing Authorization/body values.
The consumed correlated state cannot authorize a second request. Any missing
index, token, proof or output failure drops its secret and dataset owners.

## Compatibility and rollback

No existing public method changes. Rollback removes the additive method and
private correlated-state decomposition, then restores the helper as an impl
method; every existing constructor and wire output remains available.

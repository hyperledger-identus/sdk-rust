# Design

## Minimal continuation capability

`JwtCredentialRequest` additionally owns the exact validated Credential Issuer
Identifier and optional Deferred Credential Endpoint copied from its matched
metadata. Its existing zeroizing Authorization field is the only bearer
allocation. A crate-private consuming transition moves those three values and
the proof count while dropping the proof body and obsolete Credential Endpoint.

Full issuer metadata, Token Response, refresh token, scope, Credential Offer,
Authorization lineage and proof JWTs are not retained.

## HTTP 202 transition

1. Select status without inspecting media or body.
2. For non-202 branches, preserve #366 behavior: read proof count and drop the
   complete request before parsing any remote field.
3. For 202, consume the request into the minimal continuation capability,
   thereby erasing its proof body and Credential Endpoint.
4. Validate bounded Content-Type and parse the bounded deferred response.
5. Bind the parsed transaction state to that exact capability. Any error drops
   the retained capability.

## Public surface

- `RequestBoundDeferredCredentialResponse` retains the private capability and
  continues to expose proof count and parsed response accessors.
- Its consuming `try_into_deferred_credential_request` accepts only
  `DeferredCredentialRequestLimits`.
- The result is a distinct `RequestBoundDeferredCredentialRequest` that owns
  the existing `DeferredCredentialRequest`, exact issuer, zeroizing
  Authorization and originating proof count.
- It proxies endpoint, HTTP method, media type and JSON-body access and exposes
  issuer, proof count, Authorization length and an explicit sensitive
  Authorization accessor.

No method accepts replacement metadata, endpoint, token, transaction or proof
count. Neither bound type implements Clone or generic Serde; Debug is redacted.

## Internal reuse and compatibility

Extract a crate-private Deferred Credential Request constructor parameterized
by the already validated endpoint and transaction. Both the legacy detached
metadata method and the new bound method reuse it. The legacy method remains
behavior-compatible and continues to return the structural token-free type.

## Error and privacy model

Reuse `DeferredCredentialEndpointRequired` and the existing bounded serializer
errors. Add no error. Debug/Display never include issuer, endpoint, bearer,
transaction, proof or request body. The new secret lifetime is limited to an
exact 202 parse and its resulting continuation request.

## Rejected alternatives

- Full metadata/token lineage retains unrelated authority and data.
- A token argument or metadata argument permits substitution.
- Adding a bearer field to the legacy structural request silently changes its
  contract and repeated-construction semantics.
- Transport or polling behavior is outside this reversible protocol slice.

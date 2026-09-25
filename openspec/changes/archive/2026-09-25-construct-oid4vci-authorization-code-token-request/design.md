# Design

## Consuming state transition

Add `AuthorizationCodeTokenRequest` and
`AuthorizationCodeTokenRequestLimits`. Expose
`CorrelatedAuthorizationCode::try_into_public_client_token_request`, whose
name makes the unauthenticated public-client policy explicit.

The transition consumes the response, Authorization Request and request-input
states through crate-private decomposition methods. It moves the exact code,
client ID, redirect URI and verifier into one encoded `Zeroizing<String>`,
drops state, challenge, Authorization Request URI and the consumed Credential
Offer, and retains only public issuer/server metadata plus the selected
configuration. No sensitive input
is cloned into a parallel reusable field.

## Ordered validation and serialization

Construction applies checks in this order:

1. Require the selected server Token Endpoint.
2. Require its exact byte length not to exceed the request endpoint cap.
3. Calculate the exact five-field encoded body size with checked arithmetic.
4. Require that size not to exceed the request body cap.
5. Allocate exactly once and append the fixed fields in RFC-base-plus-PKCE
   order.

The result exposes the validated endpoint, POST method, form media type, exact
body length, explicit sensitive body accessor, retained issuer/server/configuration
lineage and unchanged issuer-identification evidence. Debug exposes only body
length and issuer-evidence enum.

## Error and compatibility surface

Append three fieldless errors: invalid request limits, oversized request Token
Endpoint, and oversized request form body. Place them after the existing
authorization-response catalogue in one focused token-request catalogue. All
prior variants, codes, messages, categories and discriminants remain exact.

The feature is additive and unpublished. No existing constructor, parser,
serialization, dependency, feature, target or error row changes.

## Risks and mitigations

- Client-auth ambiguity: the method name and documentation admit only the
  unauthenticated public-client profile and always include `client_id`.
- Code replay: consume the only correlated success and retain no separate code
  field after body construction.
- Endpoint substitution: derive the endpoint only from retained selected
  server metadata.
- Redirect/PKCE drift: move exact predecessor values into the body.
- Secret disclosure: zeroize the body and values during transition; redact
  Debug and static errors.
- Mix-up overclaim: preserve the #356 evidence enum exactly.
- Resource exhaustion: checked exact sizing precedes allocation and endpoint
  and body have independent positive caps.

## Rollback

Remove the additive module/export/limits/errors/tests/ADR and the private
decomposition helpers. Existing public APIs and behavior remain unchanged.

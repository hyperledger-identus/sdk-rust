# ADR 0149: preserve OID4VCI deferred continuation authority

- **Status:** Accepted
- **Date:** 2026-09-25
- **Issue:** [#368](https://github.com/hyperledger-identus/sdk-rust/issues/368)
- **Decision authority:** ADR 0109, ADR 0147, ADR 0148 and issue #368

## Context

The request-bound Credential Endpoint classifier consumes the exact
`JwtCredentialRequest`, but its initial HTTP 202 result previously retained
only the transaction and originating proof count. Constructing the required
Deferred Credential Request therefore needed detached issuer metadata and an
independently supplied bearer token, breaking the authority chain established
by the Authorization Code flow.

OpenID4VCI 1.0 Final requires the wallet to send the response transaction to
the issuer-advertised Deferred Credential Endpoint with an access token valid
for issuance of the previously requested credential. Retaining full metadata
and Token Response state would unnecessarily extend unrelated data and secret
lifetimes.

## Decision

1. Extend `JwtCredentialRequest` privately with the exact validated Credential
   Issuer Identifier and optional advertised Deferred Credential Endpoint.
2. Keep one zeroizing bearer Authorization allocation; move it rather than
   duplicating it when the request receives an exact HTTP 202 response.
3. Select status before remote media/body inspection. For HTTP 202, erase the
   proof body and obsolete Credential Endpoint before parsing while retaining
   only issuer, deferred endpoint, bearer and proof count. For every other
   status, preserve ADR 0148's complete request erasure before parsing.
4. Bind a successfully parsed HTTP 202 transaction to that private minimal
   capability. Drop the capability on every parse or construction error.
5. Add a consuming transition that accepts only the existing Deferred
   Credential Request limits and returns a distinct
   `RequestBoundDeferredCredentialRequest`.
6. Expose the exact issuer, endpoint, proof count, method/media metadata and
   explicit sensitive Authorization/body accessors. Accept no replacement
   metadata, endpoint, token, transaction or proof count.
7. Reuse one private bounded transaction serializer for both the new path and
   the legacy structural, token-free Deferred Credential Request constructor.
8. Keep both bound types non-Clone, non-Serde and redacted. Add no error,
   dependency, feature, HTTP, trust, token validation/storage, scheduling,
   retry, polling, credential processing, chain or product behavior.
9. Defer consuming Deferred Credential Endpoint response binding to focused
   issue [#370](https://github.com/hyperledger-identus/sdk-rust/issues/370).

## Consequences

The strongest path can construct a transport-ready deferred request without
caller-supplied authority and cannot replay the consumed initial request. The
bearer secret deliberately survives a valid initial HTTP 202 response because
the Final continuation requires it; proof material and the obsolete endpoint
do not. The legacy structural API remains available and behavior-compatible.

The type proves structural lineage only. It does not prove HTTP origin, issuer
control, token validity, endpoint reachability, TLS safety, transaction
freshness, retry safety, credential validity or trust.

## Reconsideration and rollback

Reconsider the retained capability when encrypted responses, token rotation or
a separately evidenced transport boundary requires different ownership.
Rollback removes the additive bound type/transition and private retained fields,
then restores ADR 0148's uniform full-request erasure without changing the
legacy structural request API.

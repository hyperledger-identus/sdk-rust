# Semantic contract review

## Scope reviewed

- Issue #135 and exact base
  `82d0e507c4c2c34d909f433b2392740f0e3470d6`.
- OpenID4VCI 1.0 Final section 7.1 and immutable document hash.
- Existing issuer metadata, Nonce Endpoint, static errors, redaction, crate
  boundary and target policy.
- Read-only Oxid and Lace evidence and license posture listed in the issue and
  design.

## Findings

1. **Resolved — endpoint absence fails before transport.** Final says omission
   means the issuer does not require `c_nonce`. A caller asking specifically
   for a Nonce Request receives `NonceEndpointRequired`; the SDK invents no
   fallback and does not weaken metadata parsing where absence remains valid.
2. **Resolved — ownership does not create an unchecked URL path.** The request
   duplicates only the already-bounded, HTTPS-validated endpoint from metadata
   and exposes no arbitrary-string constructor.
3. **Resolved — the zero-length body is an explicit canonical choice.** Final
   defines no request parameters and illustrates `Content-Length: 0`. The SDK
   exposes an empty byte slice and no media type rather than an extensible body
   builder that could drift from the profile.
4. **Resolved — unprotected means no bearer-token requirement.** The request
   exposes false access-token guidance and carries no token or authorization
   header. This does not claim that transport, network or issuer trust is safe.
5. **Resolved — request construction is not execution.** DNS, TLS, redirects,
   private-network policy, response status/media/cache/DPoP, provenance and
   nonce lifecycle remain outside this type.
6. **Resolved — diagnostics remain content-free.** Request and endpoint Debug
   omit the URL; the missing-endpoint error is fieldless and bridges to static
   metadata without JSON, endpoints or parser causes.
7. **Resolved — consumer evidence is not imported.** Oxid is used only to
   confirm independent POST/no-body behavior. No Oxid or unlicensed Lace source
   or fixture is copied, transformed, linked or modified.

## Decision

The proposal, design, capability requirements, program replacement and task
map are semantically complete, objectively testable, reversible and within the
standing mandate. No correctness, security, privacy, compatibility,
provenance, target or product-scope blocker remains before implementation.

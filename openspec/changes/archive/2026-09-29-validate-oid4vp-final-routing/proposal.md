# Why

`identus-oid4vp` can prove a signed Request Object and independently consume it
into a valid DCQL query, but it cannot yet prove that the request has coherent
Final authorization and response-routing semantics. A headless wallet must not
construct or send a presentation from signature or query validity alone.

# What changes

- Consume one `VerifiedRequestObject` into a composed authorization-request
  state that preserves signature evidence and owns one validated DCQL query.
- Support the narrow Final route `response_type=vp_token` with
  `response_mode=direct_post`, one bounded nonce, and one bounded HTTPS
  `response_uri`.
- Reject missing, ambiguous, malformed, conflicting, or unsupported routing in
  a deterministic order with static diagnostics.
- Parse request semantics once after the existing complete bounded scanner and
  share the resulting request map with DCQL extraction.
- Keep the existing DCQL-only transition source-compatible while making the
  composed state the safe entry point for later response construction.

# Capabilities

## New capabilities

- `oid4vp-authorization-request-routing`: bounded Final request semantics and
  runtime-neutral direct-post destination validation.

## Modified capabilities

- None. JAR verification and DCQL-only validation retain their existing
  contracts and narrower evidence claims.

# Non-goals

- No HTTP, DNS, TLS, redirect, decompression, retry, timeout, or SSRF runtime.
- No verifier-key authorization, prefix trust, audience/time/replay policy,
  consent, credential verification, presentation construction, or response
  transmission.
- No `direct_post.jwt`, redirect response modes, SIOPv2, DC API, HAIP, JWE,
  transaction data, scope expansion, or format-specific DCQL policy.
- No release, publication, downstream mutation, chain primitive, or product
  policy.

# Delivery

Issue #447 and ADR 0168 control this additive unpublished slice under IDR-024.
Planning and durable preflight evidence precede production code. A signed/DCO
PR targets `develop` and may merge only after required exact-head CI is green.

# ADR 0168: compose OID4VP Final direct-post routing evidence

- **Status:** Accepted under the IDR-024 roadmap mandate
- **Date:** 2026-09-29
- **Issue:** [#447](https://github.com/hyperledger-identus/sdk-rust/issues/447)
- **Decision authority:** issue #447 under ADR 0004 standing authority
- **Extends:** ADRs 0158 and 0165

## Context

`VerifiedRequestObject` proves signed JAR integrity and client/wallet-nonce
correlation. `ValidatedDcqlQuery` proves bounded query structure and selection.
The current query transition consumes the verified payload, so a caller cannot
also prove response routing without retaining or reparsing sensitive request
bytes. Neither state proves a complete Authorization Request.

OpenID4VP 1.0 Final defines several response profiles with materially different
browser, origin, encryption, subject-identity, and transport obligations. The
SDK needs a small safe prerequisite for future headless response construction,
not a general OAuth runtime.

## Decision

1. Add a consuming `ValidatedAuthorizationRequest` state that preserves the
   accepted JAR header/algorithm/client lineage and owns one validated DCQL
   query plus routing evidence.
2. Support only exact `response_type=vp_token` with exact
   `response_mode=direct_post` in this first transition.
3. Require one bounded nonce using the Final ASCII unreserved grammar.
4. Require one bounded absolute HTTPS `response_uri`; reject `redirect_uri`,
   user information, fragments, missing host, and non-HTTPS schemes.
5. Treat HTTPS validation as syntax/routing evidence only. HTTP execution, DNS,
   private-address policy, TLS, redirects, retries, timeouts, and SSRF defense
   remain with caller adapters.
6. Deserialize request semantics once after the established complete bounded
   duplicate-safe scan, and pass the same request map to routing and DCQL
   validation.
7. Preserve the existing DCQL-only transition and its narrower behavior.
8. Use SDK-owned types, explicit limits, consuming states, sensitive accessors,
   and stable redacted errors. Add no dependency.
9. Defer redirect modes, `direct_post.jwt`, SIOPv2, DC API, HAIP, JWE,
   transaction data, scope expansion, presentation construction, consent,
   credential verification, key authorization, and verifier trust.

## Consequences

Consumers receive one portable state proving a narrow coherent Final request
route and DCQL query without raw-payload retention or a network runtime. Later
response construction can require this state rather than separately trusting
booleans or reparsed request input.

The single-value enums intentionally do not promise unsupported modes. Adding a
mode is a new protocol/trust decision with its own vectors and bounds. The
public change is additive on an unpublished `0.0.0` crate.

## Alternatives rejected

- **Put routing on `ValidatedDcqlQuery`:** gives the query engine OAuth
  responsibility and still loses signature evidence.
- **Make callers retain the payload:** duplicates sensitive state and permits
  routing/query divergence.
- **Change the existing DCQL transition:** breaks a useful narrow evidence
  state and compatibility without need.
- **Adopt a broad OAuth/OID4VC framework:** adds public-model, runtime, and
  policy coupling for a small validation transition.
- **Support every response mode now:** hides unresolved redirect, origin,
  encryption, key-management, and platform decisions.

## Verification and rollback

Clean-room positive, negative, precedence, limit, URI, redaction, and
compatibility vectors are required together with strict focused/workspace
quality gates, error/public API contracts, MSRV/portable builds, and exact-diff
architecture/security review. Rollback deletes the additive state, limits,
errors, ADR, and spec without changing existing JAR/DCQL behavior or requiring
consumer/data migration.

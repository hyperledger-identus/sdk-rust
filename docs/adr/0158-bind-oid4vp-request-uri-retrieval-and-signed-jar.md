# ADR 0158: bind OID4VP Request URI retrieval and signed JAR validation

- **Status:** Accepted under the IDR-024 roadmap mandate
- **Date:** 2026-09-28
- **Issue:** [#396](https://github.com/hyperledger-identus/sdk-rust/issues/396)
- **Decision authority:** issue #396 under ADR 0004 standing authority
- **Extends:** ADR 0157; applies ADR 0156

## Context

ADR 0157 established an unpublished `identus-oid4vp` ingress that retains a
bounded client identifier, HTTPS Request URI, and GET/POST intent. A headless
wallet cannot yet retrieve the referenced Request Object without reimplementing
wire details, and a syntactically valid URI says nothing about the returned
object's authenticity.

OpenID4VP 1.0 Final defines omitted/default GET, explicit `get`, and explicit
POST retrieval. It requires a signed, optionally encrypted JAR response,
protected `typ=oauth-authz-req+jwt`, exact outer/inner client-id equality, and
exact wallet-nonce correlation when POST sent a nonce. RFC 9101 requires
algorithm- and client-associated signature verification. The SDK already owns
bounded compact JWS and cryptographic verification in `identus-jose`, but it
does not own HTTP runtimes or verifier trust policy.

## Decision

1. Correct ingress to accept the Final explicit `request_uri_method=get`.
2. Consume a referenced invocation into bounded runtime-neutral HTTP request
   data. The SDK fixes method, headers, deterministic form encoding, and limits;
   consumers execute HTTP and own DNS, TLS, redirects, decompression, retries,
   timeouts, caching, and nonce entropy.
3. Bind only a 2xx, correctly typed, non-empty, bounded compact response.
4. Support signed compact JWS now. Reject compact JWE explicitly; defer JWE and
   JWS JSON serialization to separately authorized slices.
5. Parse and verify through consuming `UnverifiedRequestObject` and
   `VerifiedRequestObject` states. Require the protected Final `typ` before
   claims are exposed.
6. Reuse `identus-jose` for suite allowlisting, exact algorithm/key binding,
   and signature verification. Do not add another JOSE/JWT dependency.
7. Parse the verified payload under explicit JSON limits and require one exact
   matching `client_id` and, when sent, one exact matching `wallet_nonce`.
8. Treat the caller-provided verification key as cryptographic input only.
   Client-prefix key authorization, DID/X.509/federation/attestation trust,
   audience, time, replay, OAuth semantics, DCQL, consent, and responses remain
   unproven later transitions.
9. Keep retained verifier-controlled data zeroizing and all diagnostics static
   and redacted.

## Consequences

Consumers gain a portable request/response envelope and a reusable
cryptographically verified JAR transition without adopting a network stack or
trust framework. The new internal dependency follows the accepted
`identus-oid4vp -> identus-jose` direction and adds no external package.

The verified type is intentionally narrow: it proves signature validity with
the selected key plus Final type/client/nonce correlation, not authorization or
trust. Callers cannot safely process presentations until later prefix,
audience, freshness, request-semantics, and DCQL transitions succeed.

## Alternatives rejected

- **Perform HTTP in the SDK:** couples platform/runtime security policy.
- **Return raw bytes:** permits consumers to bypass media and resource checks.
- **Expose claims before verification:** creates unverified/verified confusion.
- **Infer key authorization from a caller-supplied key:** overclaims trust.
- **Add a general JWT crate:** duplicates the existing bounded JOSE owner.
- **Implement JWE and every prefix now:** expands the slice beyond one cohesive
  and reviewable state transition.

## Verification and rollback

Final-profile GET/POST, response, JWS, signature, type, client-id, nonce,
duplicate, resource, and redaction tests are required together with workspace,
factory, dependency, MSRV, and portable compile gates. Rollback removes this
additive unpublished transition and restores ingress-only behavior without
registry, stored-data, release, or downstream migration.

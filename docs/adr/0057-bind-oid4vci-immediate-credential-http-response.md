# ADR 0057: bind an immediate Credential HTTP response to its JWT request

- **Status:** Accepted for issue #143
- **Date:** 2026-09-07
- **Decision authority:** standing component authority under ADR 0004
- **Related work:** issues #7, #20, #139, #141, #143; OpenID4VCI 1.0 Final
  section 8.3; RFC 9110

## Context

The SDK can construct the supported configuration-ID/JWT-proof Credential
Request and parse an immediate Credential Response body. A reusable headless
boundary still needs to validate the caller-supplied HTTP status and effective
media type, then enforce the response-count constraint that the request state
can actually prove.

Final requires status 200 for an immediate response and `application/json` for
the unencrypted form. It also requires the response to contain no more
credentials than keys supplied through request proofs, while allowing fewer.
The current proof values are intentionally opaque at this layer, so counting
proofs is only a necessary upper bound and cannot establish distinct keys or
credential-to-key binding.

## Decision

1. Add `JwtCredentialRequest::validate_immediate_response`, accepting only the
   caller-supplied status, effective Content-Type, body and explicit limits.
2. Require exact status 200. Classify 202 as the existing unsupported deferred
   branch before inspecting media or body; reject every other status.
3. Require a bounded RFC-shaped, case-insensitive `application/json` media
   type. Reuse the private parser extracted from the Nonce response boundary,
   retaining the Nonce Cache-Control behavior unchanged.
4. Parse through `ImmediateCredentialResponseCore`, then require
   `credential_count <= request.proof_count`. Return an owned
   `RequestBoundImmediateCredentialResponse` exposing only that proof count
   and the parsed response.
5. Keep all new errors fieldless and diagnostics data-free. Do not require
   Cache-Control: Final section 8.3 makes it normative for neither the general
   response nor the immediate response, despite its appearance in examples.

## Consequences

- Headless adapters can validate the supported immediate HTTP envelope and its
  necessary request cardinality constraint without duplicating protocol rules.
- The state does not prove HTTP execution, origin, proof-key uniqueness,
  credential binding, format validity, trust, notification execution, storage
  safety or replay policy.
- Error responses, deferred polling, encrypted responses, DPoP, Authorization
  Details, format verification, downstream adoption, release and `main` remain
  separate issue-first work.
- The additive unpublished API changes no dependency, feature or target state.

## Provenance

- OpenID4VCI 1.0 Final section 8.3, HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- RFC 9110, HTML SHA-256
  `d431760660ea44e130f6e919dab216df2d0b3a490567a98089267523368fe1e5`.
- Oxid conformance-only evidence:
  `MediaNoxLabs/oxid@5ba38b9bbc9326c294b353daaf2a074eca18c22f`,
  `crates/adapters/openid4vci/src/lib.rs` and contract tests, Apache-2.0.
- Lace ID Portal incompatible legacy evidence:
  `input-output-hk/lace-id-portal@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`,
  `crates/issuer-services/src/credential.rs`; no source or fixture was copied.

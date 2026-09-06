# Semantic contract review

## Scope reviewed

- Issue #131 and exact base
  `bbffe94c8731b2cadc33949761e93bfffc7fbf39`.
- OpenID4VCI 1.0 Final sections 7, 7.1, 7.2, 8.2, and 12.2.4.
- Existing OID4VCI strict scanner, response limits, static errors, redaction,
  crate boundary, and target policy.
- Read-only Oxid and Lace evidence and license posture listed in the issue and
  design.

## Findings

1. **Resolved — Final defines an opaque string, not base64url.** The parser
   requires a non-empty bounded JSON string and preserves Unicode without
   importing the Portal fixture's fixed alphabet or size.
2. **Resolved — unpredictability cannot be inferred from parsing.** The type
   proves body syntax only and does not claim entropy, freshness, Issuer origin,
   correlation, reuse prevention, or acceptance by a Credential Endpoint.
3. **Resolved — the Final response has no expiry member.** Historical
   `c_nonce_expires_in` is treated only as a bounded unknown extension and is
   not exposed as Final behavior.
4. **Resolved — transport requirements stay visible but separate.** The design
   records POST, unprotected access, 2xx, JSON, no-store, and optional DPoP as
   later transport/header responsibilities rather than properties of a parsed
   JSON body.
5. **Resolved — nonce values are correlation-sensitive.** Retained data uses
   zeroizing storage, has an explicitly sensitive accessor, and is absent from
   Debug, Display, Serde, errors, raw response access, and FFI.
6. **Resolved — extension tolerance remains bounded.** Unknown members are
   duplicate/depth/node/syntax checked and discarded; they cannot expand the
   public semantic or diagnostic surface.
7. **Resolved — private unlicensed evidence is not imported.** Lace is used
   only as a read-only independent behavior reference. No Lace source or
   fixture is copied, transformed, vendored, or linked.

## Decision

The proposal, design, capability requirements, program replacement, and task
map are semantically complete, objectively testable, reversible, and within
the standing mandate. No correctness, security, privacy, compatibility,
provenance, target, or product-scope blocker remains before implementation.

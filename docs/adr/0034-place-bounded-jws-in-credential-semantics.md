# ADR 0034: place bounded JWS in credential semantics

- **Status:** Accepted for experimental implementation
- **Date:** 2026-09-05
- **Related work:** issue #95, parent #8, `IDR-004`, OpenSpec change
  `add-bounded-jws-compact`

## Context

Oxid and Lace ID Portal independently implement JWS Compact parsing before
applying product-specific OID4VC proof, DID, time and signature policy. Future
SDK credential formats and OpenID protocol crates also require the same wire
foundation. The current workspace has no JOSE package; placing the behavior in
`identus-crypto` would mix wire serialization with cryptographic primitives,
while placing it in `identus-openid4vc` would force credential formats to
depend on one protocol family.

RFC 7515 requires exact compact-segment signing input. RFC 8725 requires
explicit algorithm selection and key/algorithm binding by the verifying
application. OpenID4VCI Final further profiles header and claim behavior. The
first reusable boundary therefore must preserve wire truth without claiming
proof or verification truth.

## Decision

1. Add `identus-jose` to the credential-semantics ring, below protocol engines
   and beside format-neutral credential/presentation semantics.
2. Implement only bounded JWS Compact encoding/parsing with a closed
   `alg`/`typ`/`kid` protected-header surface.
3. Return `UnverifiedCompactJws` and preserve the exact received signing input
   and compact text; never reserialize received JSON for verification.
4. Stage encoding through `JwsSigningInput`, allowing external signing without
   giving this crate keys or a crypto backend.
5. Require positive configurable limits, canonical unpadded base64url, exact
   delimiter structure, duplicate/unknown-member rejection, and non-empty
   signatures. Preserve RFC-compatible empty payloads.
6. Reject `alg: none`; leave all positive algorithm allowlisting and
   key/suite binding to a later verifier with caller policy.
7. Use static redaction-safe errors and omit compact text, payload, signature
   and `kid` from diagnostics.
8. Depend only on `identus-core`, `base64`, `serde` and `serde_json`. Keep the
   crate experimental, unpublished and featureless.

## Consequences

- Credential-format and OpenID protocol crates can share one exact wire
  representation without a protocol-to-product or format-to-protocol edge.
- Callers cannot confuse successful parsing with successful verification by
  type or method naming.
- Closed header handling rejects extensions until SDK-Rust understands and
  tests their security semantics.
- Original compact ownership adds bounded memory overhead but prevents
  signing-input drift and avoids public lifetime coupling.
- Proof builders, signature suites, DID authorization, claims/time policy and
  OpenID4VCI profiles remain separate reviewable deliveries under #8.

## Rejected alternatives

- **Add compact parsing to `identus-crypto`:** combines serialization and JWT
  policy with key/curve primitives and makes dependency ownership unclear.
- **Add it to `identus-openid4vc`:** prevents reuse by SD-JWT and non-OpenID
  formats without an outward dependency.
- **Use a general JOSE dependency:** broadens the dependency/security surface
  and does not enforce this SDK's bounds, closed headers or unverified state.
- **Normalize received JSON:** changes signed bytes and can invalidate or
  misverify a legitimate compact value.
- **Accept all headers and ignore unknown members:** silently admits critical
  semantics the SDK does not implement.

## Rollback

Before publication, revert issue #95 and remove the new package, rulebook and
inventory entries. No consumer branch, stored data, release or migration is
involved.

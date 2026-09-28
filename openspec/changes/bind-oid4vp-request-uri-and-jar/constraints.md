# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/396
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`: request handling remains chain- and product-neutral.
- `SDK-ARCH-002`: `identus-oid4vp -> identus-jose` points inward along the
  accepted target dependency graph.
- `SDK-COMPAT-001`, `002`, `004`, `005`: Rust 1.89/1.98 and target evidence
  policy remains unchanged.
- `SDK-SEC-001`, `002`, `003`: unsafe prohibition, redaction, and bounded input
  requirements apply to HTTP metadata, compact JWS, and JSON.
- `SDK-REL-001`: `identus-oid4vp` remains unpublished and outside release trains.
- `SDK-LIM-005`: the SDK does not own verifier trust, wallet consent, or product
  policy.
- `SDK-LIM-007`: HTTP execution, DNS, TLS, redirects, decompression, retries,
  timeouts, and entropy remain caller-owned adapter work.

## Introduced or changed constraints

Signed JAR validation must use `identus-jose`; no parallel algorithm parser or
signature implementation is accepted. The verified type proves only the
cryptographic signature with the caller-supplied algorithm-bound key plus
Final `typ`, outer/inner client-id, and optional wallet-nonce correlation.

## Introduced or changed limitations

- Signed compact JWS is supported; JWE and JWS JSON serialization are not.
- Client identifier prefix/key authorization is caller-owned and not proven by
  the generic envelope.
- Audience, temporal, replay, full OAuth request, DCQL, response, consent, and
  credential semantics remain unsupported.
- Portable evidence is compile-only; no browser/mobile runtime claim is made.
- The crate remains source-only `0.0.0`.

## Consumer and product impact

Headless consumers receive a deterministic request/response boundary and a
cryptographically verified JAR transition without committing to an HTTP stack
or trust framework. No Oxid, Lace, Midnight, or other downstream repository is
changed by this slice.

## Activation and rollback

The API activates only after issue-linked reviewed merge to `develop`.
Rollback removes the additive modules, dependency edge, ADR, and specs and
restores ingress-only behavior; no release, stored data, registry action, or
downstream migration exists.

## Evidence

Issue #396 is exact material authority. ADR 0158 records the architecture.
Focused resource/redaction/security tests, dependency guards, portable
compiles, and exact-diff review prove compliance.

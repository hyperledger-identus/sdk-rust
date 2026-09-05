# ADR 0035: bind JWS signature capabilities explicitly

- **Status:** Accepted for experimental implementation
- **Date:** 2026-09-05
- **Related work:** issue #98, parent #8, `IDR-004`, OpenSpec change
  `add-jws-signature-capabilities`
- **Supersedes:** ADR 0034 decision 8 only for the accepted
  `identus-jose -> identus-crypto` dependency

## Context

The bounded codec returns explicit unverified JWS values and exact signing
input but intentionally owns no cryptographic operation. Reusable proof and
credential protocols now need a key-custody-neutral signing seam and a
fail-closed transition to cryptographically verified evidence.

RFC 8725 requires an application-selected algorithm set and exact binding
between header, operation and key. RFC 7518 fixes ES256 to P-256/SHA-256 with a
64-byte raw `R || S` signature. RFC 9864 now registers fully specified JOSE
`Ed25519` and deprecates the older polymorphic `EdDSA` identifier, while
existing wallet ecosystems still contain RFC 8037-era `EdDSA` values.

## Decision

1. Add closed `Ed25519`, `ES256` and explicitly legacy `EdDSA` algorithm
   values. Recommended configuration includes only `Ed25519` and `ES256`.
2. Bind every selected public JWK to exactly one algorithm before verification
   and require exact equality among header, key and registered suite.
3. Make a positive, at-most-16-entry registry the caller's algorithm allowlist;
   reject duplicates and unregistered algorithms without fallback.
4. Add synchronous object-safe signer and verifier capabilities over exact
   public signing-input bytes. Runtime, transport, retry and cancellation
   remain outer-layer responsibilities. Preflight the fixed output against
   signature and compact bounds before invoking an external signer.
5. Expose typed software signers that borrow accepted crypto private keys;
   never add raw-key or custody APIs to JOSE.
6. Reuse strict Ed25519 verification. Add fixed-width P-256 primitive methods
   to `identus-crypto` and keep inherited DER APIs unchanged.
7. Only successful registered verification may construct
   `VerifiedCompactJws`; that type makes no claim about DID authorization,
   claims, time, trust or protocol acceptance.
8. Permit `identus-jose` to depend inward on `identus-crypto` in addition to
   `identus-core`. No protocol, product, chain, network or storage dependency
   is accepted.
9. Keep errors and Debug redaction-safe and use no async/runtime dependency.

## Consequences

- Consumers receive one narrow software and external-provider seam for the two
  current asymmetric wallet algorithms.
- Current `EdDSA` values remain verifiable only under explicit legacy policy,
  while new integrations naturally select `Ed25519`.
- Algorithm confusion and DER/raw ES256 confusion fail before or at one
  clearly typed boundary.
- DID key resolution/authorization and proof-claim policy remain independently
  deliverable under #99 and #5.
- An external verifier capability is trusted code chosen by the registry owner;
  SDK guarantees apply to the built-in implementations and dispatch contract.

## Rejected alternatives

- **Keep only `EdDSA`:** contradicts the current fully specified IETF registry
  and extends a deprecated negotiation ambiguity.
- **Drop `EdDSA` immediately:** breaks known RFC 8037-era wallet
  interoperability without a migration window.
- **Select crypto from header alone:** violates explicit allowlist and
  key/algorithm binding guidance.
- **Put ports in a custody or product crate:** prevents reuse and reverses the
  dependency direction.
- **Choose an async trait now:** imposes executor, cancellation and object
  semantics before real remote-provider consumers converge.
- **Convert P-256 DER inside JOSE:** duplicates primitive parsing and obscures
  the exact RFC 7518 wire contract.

## Rollback

Before publication, revert issue #98. The compact codec and existing DER crypto
APIs remain intact, no consumer was modified, and no stored-data migration is
required.

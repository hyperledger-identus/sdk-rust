# Add the OID4VCI proof JWT issuer verifier

## Why

The SDK can construct and cryptographically verify bounded compact JWS values,
but an issuer still has to assemble the OpenID4VCI proof profile, DID
authorization, certificate-provider, freshness, nonce, client and replay rules
itself. That leaves the highest-risk part of holder binding duplicated in
products and allows a parsed or merely signed JWT to be mistaken for an
issuer-accepted proof.

## What changes

- Add bounded parsing for the OpenID4VCI 1.0 Final
  `openid4vci-proof+jwt` issuer profile.
- Add explicit parsed, cryptographically verified/key-reference-bound and
  issuer-authorized states.
- Verify inline JWK references through the existing signature-suite registry.
- Resolve DID URL `kid` references through the existing DID dereferencer while
  requiring the exact `authentication` verification relationship.
- Add a narrow injected `x5c` leaf-key provider; all certificate and trust
  processing remains provider-owned.
- Add explicit client, audience, nonce, clock, freshness/skew and atomic replay
  policy inputs with redaction-safe errors.
- Add Final-spec, DID-backed, certificate-provider and consumer-shaped
  positive/negative evidence plus a release-only throughput diagnostic.

## Non-goals

This change does not implement non-DID `kid` lookup, X.509 parsing or path/
revocation validation, trust anchors, `key_attestation`, `trust_chain`, HTTP,
storage, an issuance workflow, a default clock/replay store, custody, product
trust policy, Midnight/Cardano behavior, consumer adoption, publication or a
new cryptographic primitive.

## Impact

- **Issue:** #99, second reversible delivery under #8 and #20 / `IDR-004`.
- **Owner:** `identus-jose`, with the blueprint-approved inward dependency on
  `identus-did` and the existing `identus-core` clock port.
- **Compatibility:** additive and unreleased. Existing compact, signature and
  holder-builder APIs and wire values remain unchanged.
- **Dependencies:** one workspace edge, `identus-jose -> identus-did`; no new
  third-party, chain, product, executor, HTTP, storage or trust dependency.
- **Rollback:** revert this focused change before publication; holder proof
  construction and generic JWS verification remain available.

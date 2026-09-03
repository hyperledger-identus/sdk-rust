# ADR 0007: derive RFC 7638 identifiers from validated public JWKs

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** IDR-004 roadmap mandate and sdk-rust issue #32
- **Related work:** issues #9, #5, #8 and OpenSpec `add-jwk-thumbprints`

## Context

The SDK validates public JWK structure but does not provide a stable key
identifier. Consumer-local whole-JSON hashes are sensitive to serialization
and optional metadata. The audited donor repositories do not implement RFC
7638, so this behavior must be derived from the standard rather than copied.

RFC 7638 defines a thumbprint over only the required public JWK members in
lexicographic order. The validated SDK profiles make that input fixed and
bounded. RFC 8037 provides an independent Ed25519 vector; RFC 7518 and RFC
8812 establish the supported EC member sets.

## Decision

1. `identus-crypto` owns a typed public `JwkThumbprint` and derives it through
   `PublicKeyJwk::thumbprint_sha256()`.
2. OKP uses only `crv`, `kty`, `x`; EC uses only `crv`, `kty`, `x`, `y`.
   Optional extensions never participate.
3. Fixed JSON fragments and already-validated values stream directly into
   SHA-256. No generic JSON canonicalizer or canonicalization allocation is
   introduced.
4. Digest bytes are immutable; base64url output is canonical and unpadded.
5. `jwk-thumbprint` composes the existing `jwk` and `hash` features and is
   enabled by default. Minimal `jwk` remains independent of SHA-2.
6. A thumbprint identifies key material only. Algorithm, key-use, DID
   relationship and trust policy remain above the crypto representation layer.

## Consequences

- DID Core and JOSE can share one interoperable key identifier without
  importing product policy.
- Extensions can change or round-trip without destabilizing key identity.
- The supported operation has fixed work and no canonicalization allocation;
  callers allocate only when requesting base64url text.
- Hash agility, RFC 9278 URIs and additional JWK families need new issues.
- Oxid's deployment-manifest whole-JWK digest remains a distinct downstream
  trust artifact until separately migrated.

## Provenance

| Evidence | Revision | Result |
| --- | --- | --- |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | Apache-2.0; no thumbprint implementation |
| NeoPRISM | `8becb225132efb1d9302b2c5f6ed4d87b84e8685` | Apache-2.0; basic JWK shape only |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | Apache-2.0; structural JWK only |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | Evidence-only; repository license unresolved |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | Apache-2.0; product-specific whole-JWK digest only |

No donor source is copied.

## Rollback

Revert the issue #32 pull request. No published version, persistent SDK format
or downstream repository changes in this decision.

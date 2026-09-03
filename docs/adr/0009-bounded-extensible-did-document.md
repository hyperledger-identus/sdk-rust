# ADR 0009: own a bounded extensible DID document model

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** IDR-005 roadmap mandate and sdk-rust issue #37
- **Related work:** issues #5, #3, #35 and OpenSpec `add-did-document`

## Context

The SDK has validated DID and DID URL syntax but no shared DID document. PRISM,
Midnight, Lace and Oxid currently use different records and mix structural DID
Core concerns with method, cryptosuite, chain or wallet policy. A reusable core
must remain open to standards extensions while placing hard resource and secret
handling boundaries around untrusted resolver output.

NeoPRISM provides permissive serde shapes; midnight-identity and Oxid provide
useful duplicate and bound checks but add product-specific policy. Lace shows
the need for lossless extension handling and Apollo supplies crypto compatibility
context. These are compatibility inputs rather than source foundations.

## Decision

1. `identus-did` owns private-field, validated DID document, verification
   method, relationship and service values derived from W3C DID Core 1.0.
2. Existing `Did`/`DidUrl` values remain authoritative. A dependency-free
   `Uri` adds bounded absolute RFC 3986 generic syntax for aliases and services.
3. Scalar-or-array forms and unknown properties are preserved semantically.
   Extension maps cannot shadow reserved members and are recursively bounded.
4. Verification method type and suite properties remain open. The core
   recognizes JWK and multibase public material, forbids their simultaneous use
   and rejects private JWK members, but does not validate curves or proofs.
5. Relationship references may target methods outside the current document.
   Controller equality and relationship authorization are downstream policy.
6. Services preserve string/map/mixed-array endpoints. URI syntax is validated;
   scheme-specific normalization, HTTPS/SSRF and dereferencing policy are not.
7. Raw documents are capped at 256 KiB, collections at 128 items, extension
   maps at 64 entries, property names at 256 bytes, arbitrary strings at 64 KiB,
   extension depth at 32 and extension nodes at 4,096.
8. The current unpublished package name remains. Publication/rename is owned by
   #3; resolution and downstream ports are separate IDR-005/IDR-006 slices.

## Consequences

- Downstream adapters gain a small shared structural boundary without IOTA
  Identity, JSON-LD, URI parser or chain dependencies.
- Resolver-controlled data has deterministic CPU/memory ceilings and public
  DID documents cannot accidentally carry recognized JWK private material.
- Unknown standards extensions survive semantic JSON round trips, but their
  cryptographic or protocol meaning remains untrusted until an adapter checks
  them.
- Exact JSON bytes, object order, JSON-LD equivalence and scheme-specific URI
  normalization are intentionally outside this API.

## Provenance

| Evidence | Revision | Result |
| --- | --- | --- |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | Apache-2.0; crypto compatibility context only |
| NeoPRISM | `8becb225132efb1d9302b2c5f6ed4d87b84e8685` | Apache-2.0; adaptable document shapes, dependencies/policy rejected |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | Apache-2.0; adaptable validation ideas, method policy rejected |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | Evidence-only; repository license unresolved |
| Oxid | `685f9670af4846d52697a4cfeb94779758ae1075` | Apache-2.0; bounded Midnight consumer shapes only |

Normative behavior is derived from W3C DID Core 1.0 and RFC 3986. No donor
source is copied.

## Rollback

Revert the issue #37 pull request. No published package, stored SDK data or
downstream repository is changed by this decision.

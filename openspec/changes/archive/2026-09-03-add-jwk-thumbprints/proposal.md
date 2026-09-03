## Why

Backlog item `IDR-004` requires reusable key identifiers before DID Core and
JOSE can stop inventing consumer-local digests. `identus-crypto` now validates
public JWKs, but it cannot derive the representation-independent identifier
defined by RFC 7638. Hashing the whole serialized object downstream is not
equivalent: member order and optional metadata can change the result.

GitHub issue #32 defines this child of parent #9. It adds only SHA-256 JWK
thumbprints for the four already-supported public profiles. JOSE algorithm
policy, thumbprint URIs, new key families and downstream migrations remain
separate work.

## What Changes

- Add a typed public `JwkThumbprint` and
  `PublicKeyJwk::thumbprint_sha256()`.
- Hash only the required profile members, in RFC 7638 lexicographic order.
- Exclude extensions such as `alg`, `kid` and `use` from key identity.
- Expose digest bytes and canonical unpadded base64url output.
- Add an opt-in `jwk-thumbprint` feature composing `jwk` and `hash`, enabled by
  default while preserving the existing minimal `jwk` feature.
- Pin RFC 8037 and independent EC vectors, extension-invariance checks and a
  bounded zero-canonicalization-allocation implementation contract.
- Record the boundary and donor audit in ADR 0007.

## Capabilities

### Modified Capabilities

- `crypto`: add RFC 7638 SHA-256 thumbprints and feature-cone guarantees to
  the validated public JWK surface.

## Impact

- **Issue:** #32, child of #9 (`IDR-004`).
- **API:** additive, feature-gated `JwkThumbprint` and one `PublicKeyJwk`
  method.
- **Dependencies:** no new crate; the feature composes existing `sha2`,
  base64url and JWK capabilities.
- **Consumers:** DID Core #5 and JOSE #8 gain a stable key identifier. No
  downstream repository is modified in this slice.
- **Rollback:** revert the issue #32 change. No released API, persistence or
  downstream migration is involved.

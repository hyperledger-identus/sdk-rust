## Why

Backlog item `IDR-004` requires policy-neutral key and JWK primitives before
the generic DID model in `IDR-005` can stabilize. `identus-crypto` currently
exposes a stringly `Jwk` with public fields and optional coordinates. Callers
can construct impossible profiles, such as `OKP/P-256`, omit `x`, attach `y`
to an OKP key, or accidentally carry private `d` material in an ad-hoc wire
object. The same invalid states can then leak into DID documents.

GitHub issue #28 defines the bounded first child of parent #9. It replaces the
experimental model with one validated, public-only JWK boundary while keeping
JOSE operations, DID policy, chain curves, custody and downstream adoption out
of scope.

## What Changes

- Replace `Jwk` with `PublicKeyJwk`, private fields, typed `JwkKeyType` and
  `JwkCurve`, fallible constructors and read-only accessors.
- Accept only the four profiles already implemented by the SDK:
  `OKP/Ed25519`, `OKP/X25519`, `EC/P-256` and `EC/secp256k1`.
- Validate canonical unpadded base64url and exact 32-byte coordinate lengths;
  require `y` for EC and reject it for OKP.
- Add validating JSON serialization/deserialization that rejects private `d`
  material and preserves additional public members without letting them
  shadow structural members.
- Change `EncodeJwk` to return `PublicKeyJwk` and preserve the existing curve
  encoders' wire bytes.
- Add a dedicated redaction-safe JWK error surface and RFC/negative vectors.
- Record the public API and trust-boundary decision in ADR 0005.

## Capabilities

### Modified Capabilities

- `crypto`: harden the JWK data boundary, serde feature cone, error catalogue
  and curve encoder contract.

## Impact

- **Issue:** #28, child of #9 (`IDR-004`).
- **API:** the unreleased, non-publishable `Jwk` type is replaced by
  `PublicKeyJwk`; direct field reads migrate to accessors. This is intentionally
  breaking before any supported SDK release.
- **Dependencies:** `serde` and `serde_json`, already pinned at workspace level,
  become optional dependencies enabled by the `jwk` feature.
- **Consumers:** no downstream repository is modified. DID Core #5 can consume
  the stable boundary in a later issue.
- **Rollback:** revert this change. No release, publication, persistent-data
  migration or downstream adoption is part of the slice.

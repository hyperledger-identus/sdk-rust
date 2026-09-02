## Why

Backlog item `IDR-004` requires a policy-neutral COSE and CBOR key boundary
before mdoc, DID, credential and protocol crates can share compact public-key
material. The JWK boundary delivered by issue #28 covers JSON consumers, but
the SDK has no equivalent for COSE. A caller would currently need to expose a
third-party `CoseKey`, duplicate validation, or import a product-specific CBOR
codec.

GitHub issue #30 defines the bounded second child of parent #9. It adds one
validated, public-only COSE Key representation for the four algorithms already
implemented by `identus-crypto`. COSE messages, private keys, algorithm policy,
generic CBOR, publication and downstream adoption stay outside the slice.

## What Changes

- Add `PublicKeyCose`, private fields, typed `CoseKeyType`, `CoseCurve` and
  `CoseEcY`, fallible constructors and read-only accessors.
- Accept `OKP/Ed25519`, `OKP/X25519`, `EC2/P-256` and `EC2/secp256k1`, with
  exact 32-byte public coordinates and the RFC-defined full or sign-bit EC2
  `y` form.
- Add bounded, exact-end CBOR parsing that rejects duplicate labels, invalid
  profiles, missing coordinates, private `d` material and oversized inputs.
- Emit deterministic untagged CBOR and retain uninterpreted public/common
  parameters without exposing third-party CBOR values in the SDK API.
- Add `EncodeCose` for the four existing public-key implementations and
  key-material conversion to and from `PublicKeyJwk`.
- Add a redaction-safe COSE key error surface, RFC-derived fixtures, misuse
  tests, minimal-feature/WASM checks and a release-mode throughput observation.
- Record the public API, dependency and wire decision in ADR 0006.

## Capabilities

### Modified Capabilities

- `crypto`: add the validated public COSE Key boundary, `cose` feature,
  dependency cone, error catalogue and curve encoder contract.

## Impact

- **Issue:** #30, child of #9 (`IDR-004`).
- **API:** additive, unreleased `PublicKeyCose` and `EncodeCose` APIs. The
  existing JWK and curve APIs remain source-compatible.
- **Wire:** RFC 9052/9053 COSE_Key labels for four current curves; the encoder
  emits normalized integer registry identifiers and deterministic CBOR.
- **Dependencies:** Apache-2.0 `coset` is workspace-pinned, optional and hidden
  behind the `cose` feature with default features disabled.
- **Consumers:** no downstream repository is modified. Adoption remains a
  separate issue after an immutable candidate exists.
- **Rollback:** revert this focused change; no release or persistent migration
  is part of the slice.

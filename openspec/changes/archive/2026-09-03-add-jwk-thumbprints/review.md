# Semantic, crypto and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #32 (child of #9 / `IDR-004`)
- **Develop base:** `3237baf4c6abf48019c9c9b5b37407e31bd1e22c`
- **Reviewed implementation head:** `958ab87a6d398df81a5db9312bf0f4373d1612d6`
- **Result:** no unresolved blocker; suitable for exact-head hosted review

## Pre-implementation findings

1. **Ownership:** RFC 7638 key identity is reusable SSI/crypto
   infrastructure and belongs beside the validated `PublicKeyJwk`. DID
   authorization, JOSE algorithm policy, custody and product trust artifacts
   remain outside this slice.
2. **Provenance:** Apollo, NeoPRISM, midnight-identity and Lace expose JWK
   structures but no thumbprint implementation. Oxid's whole-serialized-JWK
   deployment digest is different product policy. The implementation is
   standards-derived and copies no donor source.
3. **Standards:** RFC 7638 selects required members and lexicographic order;
   RFC 8037 supplies the exact OKP vector. RFC 7518 and RFC 8812 establish the
   required EC members for the two supported EC profiles.
4. **Compatibility:** the API is additive in an unpublished crate, adds no
   dependency, remains wasm-safe and keeps minimal `jwk` independent of
   SHA-2.

## Post-implementation review

The complete `develop...958ab87` diff was reviewed afresh after implementation
and after all local gate corrections.

1. **Canonical input:** fixed fragments produce exactly `crv,kty,x` for OKP
   and `crv,kty,x,y` for EC. A test-only sink asserts both byte sequences.
   The production closure streams those same fragments directly into SHA-256.
2. **Escaping safety:** registry strings are closed enums and coordinates are
   validated canonical base64url. These alphabets contain no JSON characters
   requiring escaping, so direct fragment hashing is unambiguous.
3. **Optional-member confusion:** the production path cannot reach the
   extension map. Tests vary `alg`, `kid`, `use` and an extra extension while
   preserving the thumbprint, then change required coordinate and curve
   material and observe different results.
4. **Profile safety:** the operation is available only on a fully constructed
   `PublicKeyJwk`; private fields and existing constructors guarantee that EC
   has `y`, OKP omits it, profiles match and coordinates are canonical.
5. **Type safety:** `JwkThumbprint` cannot be created with arbitrary length,
   exposes immutable digest bytes and formats only as canonical unpadded
   base64url. It is intentionally distinct from a generic payload digest.
6. **Resource behavior:** canonicalization performs zero heap allocation and
   a fixed number of SHA-256 updates over at most two 43-character
   coordinates plus closed registry strings. Text allocation occurs only when
   base64url output or `Display` is requested.
7. **Conformance:** the RFC 8037 digest and text match exactly. P-256 and
   secp256k1 generator-point expectations were independently derived with
   Python's standard JSON/hash/base64 implementations and cross-checked with
   OpenSSL SHA-256.
8. **Feature posture:** `jwk-thumbprint` composes `jwk` and `hash`; `jwk`
   remains usable without SHA-2. Default, minimal, MSRV, wasm, Android, iOS,
   lint, docs, audit and license gates pass.

## Corrections made during review

- Removed a redundant method-level `must_use` after clippy confirmed the
  returned semantic type already carries that contract.
- Applied the repository Taplo formatter after the first full Nix run found
  only TOML alignment drift, then reran the entire flake successfully.
- Added explicit curve-sensitivity coverage in addition to coordinate
  sensitivity.
- Expanded both `MODIFIED` requirements before committing the archive so the
  canonical sync preserves every inherited JWK, COSE-feature, KMP and minimal
  build scenario while adding the thumbprint contract.

Residual risks are explicit: a thumbprint does not prove curve membership,
authorize a DID relationship, select a signature algorithm, bind extensions
or provide hash agility. Those are separate protocol and roadmap decisions,
not blockers for this representation-level capability.

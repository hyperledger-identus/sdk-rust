# ADR 0006: validate the public COSE Key boundary in `identus-crypto`

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** IDR-004 roadmap mandate and sdk-rust issue #30
- **Related work:** issues #9, #28 and OpenSpec `validate-public-cose-key`

## Context

The SDK has a validated public JWK boundary but no equivalent for compact CBOR
protocols. Exposing a general mutable COSE key would admit private material,
unsupported profiles, duplicate-label ambiguity and unbounded parser work.
Writing a new CBOR/COSE parser would create unnecessary security-sensitive
code, while copying Oxid's credential-specific CBOR parser would move a product
format into the generic SDK.

## Decision

1. `identus-crypto` owns an additive `PublicKeyCose` value with private fields,
   typed supported profiles, bounded parsing and deterministic encoding.
2. The initial profiles exactly match the implemented public-key algorithms:
   Ed25519, X25519, P-256 and secp256k1. EC2 supports both full and registered
   sign-bit `y` forms; all public coordinates are exactly 32 bytes.
3. Apache-2.0 `coset` supplies the general RFC 9052/9053 wire codec behind the
   SDK wrapper. It is optional, workspace-declared, default-feature-disabled
   and never appears in a public SDK signature.
4. Inputs are capped at 4096 bytes and nesting depth 16, require one untagged
   exact-end map, and reject duplicates, private label `-4`, incompatible
   profiles, malformed coordinates, more than 32 additional parameters and
   floating-point extension values.
5. Supported `kty` and `crv` identifiers normalize to assigned integers.
   Encoding recursively
   emits definite-length, preferred integer forms with RFC 8949 length-first
   map ordering (encoded-key length, then bytewise lexical order). Unknown
   public/common parameters round trip but are not treated as authorization or
   trust policy.
6. Full-coordinate COSE and JWK values convert without changing key material.
   Format-specific metadata is not translated, and compressed EC2-to-JWK
   conversion fails rather than introducing curve decompression.
7. The `cose` feature is independently buildable and enabled by default. Curve
   encoder implementations require both `cose` and the curve feature.
8. `CoseKeyError` maps to `crypto.invalid_cose_key` and never renders raw CBOR
   or extension values.

## Consequences

- Generic DID, mdoc and credential/protocol crates can share one public COSE
  key boundary without depending directly on a codec or product repository.
- The dependency graph gains `coset` and its small CBOR dependency cone only
  when `cose` is enabled.
- The representation is intentionally not a COSE message implementation or an
  algorithm-policy engine.
- Additional curves, private keys, typed metadata and wider parser bounds need
  separate evidence-backed issues.
- Consumers still perform curve-point/subgroup checks and operation-specific
  algorithm, key-use and trust validation.

## Rollback

Revert the issue #30 pull request. No released API, publication, persisted SDK
format or downstream repository is changed by this decision.

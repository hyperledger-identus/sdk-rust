## Context

`PublicKeyJwk` guarantees one of four supported public profiles and canonical,
fixed-width base64url coordinates. This makes its RFC 7638 hash input fully
bounded and avoids the generic JSON canonicalization problem.

The donor audit found no RFC 7638 implementation to extract. Apollo
`ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` and NeoPRISM baseline
`8becb225132efb1d9302b2c5f6ed4d87b84e8685` stop at basic JWK encoding.
midnight-identity `427f8571950c42967a18726cbcbefecc19ef8d79` and Lace ID Portal
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` have structural JWK behavior but
no thumbprints. Oxid `bfe3b481568dc738f0732c2b27548fab8721fd95`
hashes a whole serialized Jubjub JWK for a product deployment manifest; that
policy is not RFC 7638. Lace remains evidence-only because repository license
evidence is unresolved. No donor source is copied.

Normative behavior comes from RFC 7638. RFC 8037 Appendix A.3 supplies the
Ed25519 vector; RFC 7518 and RFC 8812 define required EC public members for
P-256 and secp256k1.

## Goals / Non-Goals

**Goals:**

- Produce interoperable SHA-256 thumbprints for every supported public JWK.
- Make optional metadata irrelevant to key identity.
- Keep the operation infallible, bounded and free of generic canonicalizers.
- Preserve wasm and minimal-feature builds.

**Non-Goals:**

- JOSE `alg`, `use` or `key_ops` policy; DID authorization; RFC 9278 URIs;
  JCS; RSA; symmetric/private JWKs; new curves; downstream adoption.

## Decisions

### Decision 1: thumbprints belong beside the validated JWK

`PublicKeyJwk::thumbprint_sha256()` returns `JwkThumbprint`. The semantic
newtype prevents a raw hash from being confused with a payload, certificate or
whole-document digest. It exposes immutable 32-byte access and a canonical
unpadded base64url representation.

The operation is infallible. All selected values are closed registry strings
or validated base64url coordinates, so none require JSON escaping. An EC JWK
always has `y`; an OKP JWK never does.

### Decision 2: hash the fixed canonical sequence directly

The implementation feeds fixed JSON fragments and validated values directly
to SHA-256 in this exact order:

- OKP: `{"crv":"<crv>","kty":"OKP","x":"<x>"}`
- EC: `{"crv":"<crv>","kty":"EC","x":"<x>","y":"<y>"}`

This performs no canonicalization heap allocation and cannot include an
extension accidentally. The final base64url conversion allocates only when a
caller asks for text. A generic serializer or JCS dependency is rejected: it
would widen the supported problem, increase code/dependency weight and make
optional-member exclusion less obvious.

### Decision 3: optional members never identify the key

RFC 7638 intentionally selects required key-representation members. The
`extensions` map is neither serialized nor inspected by the thumbprint path.
Consequently `kid`, `alg`, `use`, certificate metadata and private extension
names cannot alter the result. Protocol layers remain responsible for
authorizing those attributes separately.

### Decision 4: use a compositional Cargo feature

`jwk-thumbprint = ["jwk", "hash"]` is enabled by default. Existing callers
that need only validated JWK parsing can continue using `--features jwk`
without pulling SHA-2. The new feature requires no new dependency or backend
and remains wasm-safe.

## Threat Contract

**Assets:** stable public-key identity, cross-implementation interoperability
and separation of key material from mutable metadata.

**Threats addressed:** serialization-order drift, whitespace drift,
optional-member confusion, accidental whole-object hashing, padded base64url
output and unbounded generic canonicalization.

**Residual boundaries:** a thumbprint identifies JWK key material but does not
authorize a key, prove curve membership, select a signature algorithm or bind
optional attributes. SHA-256 agility and RFC 9278 identifiers require future
issues if consumers demonstrate need.

## Test and Verification Strategy

- Match RFC 8037 Appendix A.3 digest and base64url output exactly.
- Pin independent P-256 and secp256k1 expected values computed by a separate
  standards-conforming implementation and review their canonical inputs.
- Prove extension invariance and required-material sensitivity.
- Unit-test the canonical byte sequence through a test-only sink while the
  production path streams directly into SHA-256.
- Verify default, all-feature, minimal `jwk`, minimal `jwk-thumbprint`, wasm,
  docs, clippy, formatting, tests, OpenSpec and factory gates under Nix.
- Run separate crypto/misuse-resistance and security reviews at the exact PR
  head.

## Migration Plan

1. Land this issue-linked OpenSpec contract and ADR as a signed commit.
2. Implement the type, streaming hash and focused vectors.
3. Sync the canonical crypto spec, complete evidence and archive the change.
4. Merge only after exact-head CI and reviews are green. Adoption by DID Core,
   JOSE and downstream repositories remains separately issue-driven.

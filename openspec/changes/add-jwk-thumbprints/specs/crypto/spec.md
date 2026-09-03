## MODIFIED Requirements

### Requirement: JWK

The crate SHALL provide a public-only `PublicKeyJwk` with private fields,
typed `JwkKeyType` and `JwkCurve`, fallible constructors, read-only accessors,
validating JSON serialization/deserialization, and an `EncodeJwk` trait. The
supported profiles SHALL be `OKP/Ed25519`, `OKP/X25519`, `EC/P-256`, and
`EC/secp256k1`. Coordinates SHALL be canonical unpadded base64url and decode
to exactly 32 bytes. The public type SHALL reject private `d`, preserve public
extensions without interpreting them, and prevent extensions from shadowing
structural members.

When `jwk-thumbprint` is enabled, `PublicKeyJwk::thumbprint_sha256()` SHALL
return a typed `JwkThumbprint` computed according to RFC 7638. OKP hash input
SHALL contain only `crv`, `kty`, and `x`; EC input SHALL contain only `crv`,
`kty`, `x`, and `y`. Members SHALL be lexicographically ordered with no
whitespace and encoded as UTF-8. Extensions SHALL NOT affect the result. The
thumbprint SHALL expose immutable 32-byte SHA-256 digest access and canonical
unpadded base64url text.

#### Scenario: RFC 8037 Ed25519 thumbprint matches exactly

- **WHEN** the RFC 8037 Appendix A.2 public JWK is thumbprinted
- **THEN** its digest SHALL equal
  `90facafea9b1556698540f70c0117a22ea37bd5cf3ed3c47093c1707282b4b89`
  and its base64url value SHALL equal
  `kPrK_qmxVWaYVA9wwBF6Iuo3vVzz7TxHCTwXBygrS4k`

#### Scenario: optional metadata cannot change key identity

- **WHEN** two JWKs have identical required key members but different `kid`,
  `alg`, `use`, or other public extensions
- **THEN** their SHA-256 JWK thumbprints SHALL be equal

#### Scenario: required key material changes key identity

- **WHEN** a required coordinate or supported curve differs
- **THEN** the SHA-256 JWK thumbprint SHALL differ

#### Scenario: canonicalization is fixed and bounded

- **WHEN** an OKP or EC JWK is thumbprinted
- **THEN** fixed JSON fragments and validated values SHALL stream directly
  into SHA-256 without a generic JSON canonicalizer or canonicalization heap
  allocation

### Requirement: Feature-gated with all-on default

The crate SHALL expose cargo features `ed25519`, `x25519`, `secp256k1`,
`secp256r1`, `hash`, `hex`, `base64`, `jwk`, `jwk-thumbprint`, `cose`,
`derivation`, and `kmp-compat`, with `default` enabling all except
`kmp-compat`. `jwk-thumbprint` SHALL compose `jwk` and `hash`; `jwk` SHALL
remain independently usable without SHA-2. All supported minimal combinations
SHALL remain wasm-safe.

#### Scenario: minimal JWK thumbprint feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features
  jwk-thumbprint` is run
- **THEN** validated JWK, SHA-256 thumbprint and base64url output SHALL compile
  without a curve backend

#### Scenario: minimal JWK parsing remains independent

- **WHEN** `cargo build -p identus-crypto --no-default-features --features
  jwk` is run
- **THEN** the validated JWK wire surface SHALL compile without SHA-2 or a
  curve backend

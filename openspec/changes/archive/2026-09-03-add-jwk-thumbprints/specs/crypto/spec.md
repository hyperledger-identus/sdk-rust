## MODIFIED Requirements

### Requirement: JWK

The crate SHALL provide a public-only `PublicKeyJwk` with private fields,
typed `JwkKeyType` and `JwkCurve`, fallible constructors, read-only accessors,
validating JSON serialization/deserialization, and an `EncodeJwk` trait
(`encode_jwk() -> PublicKeyJwk`) implemented for every supported public key
type. The supported profiles SHALL be `OKP/Ed25519`, `OKP/X25519`,
`EC/P-256`, and `EC/secp256k1`. Every coordinate SHALL be canonical unpadded
base64url and decode to exactly 32 bytes. OKP profiles SHALL contain `x` and
SHALL NOT contain `y`; EC profiles SHALL contain both `x` and `y`. Native
construction and deserialization SHALL enforce the same invariants. The public
type SHALL reject the private `d` member. Additional public members SHALL
round-trip without being interpreted and SHALL NOT shadow `kty`, `crv`, `x`,
or `y`.

When `jwk-thumbprint` is enabled, `PublicKeyJwk::thumbprint_sha256()` SHALL
return a typed `JwkThumbprint` computed according to RFC 7638. OKP hash input
SHALL contain only `crv`, `kty`, and `x`; EC input SHALL contain only `crv`,
`kty`, `x`, and `y`. Members SHALL be lexicographically ordered with no
whitespace and encoded as UTF-8. Extensions SHALL NOT affect the result. The
thumbprint SHALL expose immutable 32-byte SHA-256 digest access and canonical
unpadded base64url text.

#### Scenario: RFC 8037 Ed25519 public key is accepted exactly

- **WHEN** the RFC 8037 Appendix A.2 public JWK is deserialized
- **THEN** it SHALL produce `OKP/Ed25519`, preserve the exact `x`, omit `y`,
  and serialize to an equivalent public JWK without `d`

#### Scenario: curve encoders preserve their public coordinates

- **WHEN** Ed25519, X25519, P-256 or secp256k1 public keys call `encode_jwk()`
- **THEN** the result SHALL use the correct typed profile and SHALL contain
  the same canonical coordinate bytes as the public key encoding

#### Scenario: EC and OKP shapes are enforced

- **WHEN** an EC JWK omits `y`, an OKP JWK contains `y`, or `kty` and `crv`
  are incompatible
- **THEN** native construction and deserialization SHALL reject the value

#### Scenario: coordinates are canonical and full width

- **WHEN** a coordinate has padding, an invalid alphabet, non-zero trailing
  bits, or decodes to any length other than 32 bytes
- **THEN** native construction and deserialization SHALL reject the value

#### Scenario: private material is rejected

- **WHEN** a public JWK contains a `d` member
- **THEN** deserialization and extension-aware construction SHALL reject it
  without including the private value in an error

#### Scenario: unknown public extensions survive a round trip

- **WHEN** a valid public JWK contains additional public members such as
  `kid` or a collision-resistant extension name
- **THEN** deserialize/serialize SHALL preserve their JSON values while the
  crypto crate SHALL NOT interpret their policy

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
`derivation`, and `kmp-compat`, with `default` enabling all of them except
`kmp-compat`. The `cose` feature SHALL enable only optional,
default-feature-disabled `coset`. The `jwk` feature SHALL enable `base64`,
`serde`, and `serde_json`; curve features SHALL continue to imply `jwk`.
`jwk-thumbprint` SHALL compose `jwk` and `hash`, while `jwk` SHALL remain
independently usable without SHA-2. Curve `EncodeCose` implementations SHALL
be available when the curve and `cose` features are both enabled; JWK
conversion SHALL additionally require `jwk`. A minimal COSE build SHALL not
require serde, JSON, base64 or a curve backend. There SHALL be no
`securerandom` feature because the zero-dependency `SecureRandom` port is
always available, and no `wasm` feature because concrete entropy adapters
belong to `identus-adapters-entropy`. `kmp-compat` SHALL remain opt-in, gate
the KMP interop surface, and introduce no dependency outside the default
dependency set. The full default and supported minimal feature combinations
SHALL remain wasm-safe.

#### Scenario: Default features compile the full surface

- **WHEN** `cargo build -p identus-crypto` is run with default features
- **THEN** all curve, hashing, derivation, JWK, JWK thumbprint, COSE, and
  `SecureRandom`-port modules SHALL compile and be available, and the
  KMP-interop surface SHALL NOT be present

#### Scenario: Default features compile on wasm32

- **WHEN** `cargo build -p identus-crypto --target wasm32-unknown-unknown` is
  run with default features
- **THEN** the build SHALL succeed

#### Scenario: minimal JWK feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features jwk`
  is run
- **THEN** the validated JWK and JSON wire surface SHALL compile without
  SHA-2 or a curve backend

#### Scenario: minimal JWK thumbprint feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features
  jwk-thumbprint` is run
- **THEN** validated JWK, SHA-256 thumbprint and base64url output SHALL compile
  without a curve backend

#### Scenario: a minimal curve feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features ed25519`
  is run
- **THEN** Ed25519 and its validated JWK wire surface SHALL compile

#### Scenario: minimal COSE feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features cose`
  is run
- **THEN** the validated COSE Key and CBOR wire surface SHALL compile without
  JSON, base64 or a curve backend

#### Scenario: curve plus COSE feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features ed25519,cose`
  is run
- **THEN** Ed25519 and its `EncodeCose` implementation SHALL compile

#### Scenario: The kmp-compat feature is opt-in

- **WHEN** `cargo build -p identus-crypto --features kmp-compat` is run
- **THEN** the build SHALL succeed and `create_seed_kmp` SHALL be present on
  `MnemonicHelper`

#### Scenario: kmp-compat introduces no new external dependency

- **WHEN** the `kmp-compat` feature is enabled with its required base features
- **THEN** the build SHALL succeed without pulling a crate not already
  required by the default feature set

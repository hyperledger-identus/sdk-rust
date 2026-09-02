## MODIFIED Requirements

### Requirement: JWK

The crate SHALL provide a public-only `PublicKeyJwk` with private fields, typed `JwkKeyType` and `JwkCurve`, fallible constructors, read-only accessors, validating JSON serialization/deserialization, and an `EncodeJwk` trait (`encode_jwk() -> PublicKeyJwk`) implemented for every supported public key type. The supported profiles SHALL be `OKP/Ed25519`, `OKP/X25519`, `EC/P-256`, and `EC/secp256k1`. Every coordinate SHALL be canonical unpadded base64url and decode to exactly 32 bytes. OKP profiles SHALL contain `x` and SHALL NOT contain `y`; EC profiles SHALL contain both `x` and `y`. Native construction and deserialization SHALL enforce the same invariants. The public type SHALL reject the private `d` member. Additional public members SHALL round-trip without being interpreted and SHALL NOT shadow `kty`, `crv`, `x`, or `y`.

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

### Requirement: Two-surface error bridging

The crate SHALL provide an idiomatic `crypto::Error` enum carrying runtime detail for primitive operations and a dedicated `JwkError` for the validated JWK boundary. Both SHALL map to redaction-safe `identus_core::IdentusError` values under `CapabilityId("crypto")`. The stable catalogue SHALL include `crypto.invalid_jwk` in addition to the existing crypto codes. JWK errors and their serde rendering SHALL identify only the failed invariant and SHALL NOT contain coordinate or extension values.

#### Scenario: invalid JWK maps to a stable redacted code

- **WHEN** a `JwkError` is mapped through `to_identus_error()`
- **THEN** the code SHALL be `crypto.invalid_jwk`, the kind SHALL be
  `InvalidInput`, the capability SHALL be `crypto`, and the rendered value
  SHALL NOT contain caller-supplied coordinate or extension values

### Requirement: Feature-gated with all-on default

The crate SHALL expose cargo features `ed25519`, `x25519`, `secp256k1`, `secp256r1`, `hash`, `hex`, `base64`, `jwk`, `derivation`, and `kmp-compat`, with `default` enabling all except `kmp-compat`. The `jwk` feature SHALL enable `base64`, `serde`, and `serde_json`; curve features SHALL continue to imply `jwk`. `kmp-compat` SHALL remain opt-in. The full default and supported minimal feature combinations SHALL remain wasm-safe.

#### Scenario: minimal JWK feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features jwk`
  is run
- **THEN** the validated JWK and JSON wire surface SHALL compile without a
  curve backend

#### Scenario: a minimal curve feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features ed25519`
  is run
- **THEN** Ed25519 and its validated JWK wire surface SHALL compile

### Requirement: Workspace-level external dependency declaration

The crate's external dependencies (`ed25519-dalek`, `k256`, `p256`, `x25519-dalek`, `sha2`, `hmac`, `pbkdf2`, `base64`, `hex`, `serde`, and `serde_json`) SHALL be declared in root `[workspace.dependencies]` and referenced via `<dep>.workspace = true`; no inline external version pin SHALL appear in `crates/crypto/Cargo.toml`. The list SHALL NOT include `ring`.

#### Scenario: crypto external deps use the workspace form

- **WHEN** `crates/crypto/Cargo.toml` is inspected
- **THEN** every external dependency entry SHALL use `.workspace = true`,
  `serde` and `serde_json` SHALL be optional edges of `jwk`, and `ring` SHALL
  NOT be present

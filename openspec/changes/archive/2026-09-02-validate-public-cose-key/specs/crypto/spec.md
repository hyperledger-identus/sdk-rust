## ADDED Requirements

### Requirement: validated public COSE Key boundary

The crate SHALL provide a public-only `PublicKeyCose` with private fields,
typed `CoseKeyType`, `CoseCurve` and `CoseEcY`, fallible constructors,
read-only accessors, bounded `from_cbor` parsing, deterministic `to_cbor`
encoding, and an `EncodeCose` trait implemented for every supported public key
type when the applicable features are enabled. Supported profiles SHALL be
`OKP/Ed25519`, `OKP/X25519`, `EC2/P-256` and `EC2/secp256k1`. Coordinates SHALL
be exactly 32 bytes. OKP SHALL contain `x` and no `y`; EC2 SHALL contain `x`
and either a 32-byte `y` or its registered boolean sign form. Label `-4`
private material SHALL always be rejected.

The parser SHALL reject inputs over 4096 bytes before decoding, use a nesting
limit of 16, require one untagged map with exact end of input, reject duplicate
labels and allow no more than 32 additional top-level parameters. Common and
unknown public parameters SHALL survive parse/encode without being interpreted
or exposed as third-party wire types. Floating-point extension values SHALL be
rejected. The encoder SHALL normalize supported `kty` and `crv` registry names
to integers and emit definite-length, shortest-form CBOR with recursively
length-first map-key ordering: shorter deterministic key encodings first, then
bytewise lexical order for keys of equal length.

#### Scenario: registered OKP and EC2 fixtures are accepted

- **WHEN** valid assigned-integer COSE keys for Ed25519, X25519, P-256 and
  secp256k1 are parsed
- **THEN** they SHALL produce the matching typed profile and exact public
  coordinate bytes

#### Scenario: registered text names normalize to integer identifiers

- **WHEN** a supported key uses the registered text spelling for `kty` or
  `crv`
- **THEN** parsing SHALL accept the profile and deterministic encoding SHALL
  emit the assigned integer identifiers

#### Scenario: EC2 supports full and compressed y forms

- **WHEN** an EC2 public key contains a 32-byte `y` or a boolean sign value
- **THEN** the typed value SHALL preserve the selected form and emit an
  equivalent deterministic key

#### Scenario: private and incompatible key shapes are rejected

- **WHEN** label `-4` is present, OKP contains `y`, EC2 omits `y`, or `kty`
  and `crv` are incompatible
- **THEN** parsing SHALL fail without rendering caller-controlled bytes

#### Scenario: coordinate types and widths are enforced

- **WHEN** `x` is not a byte string, `y` has the wrong CBOR type, or a public
  coordinate is not exactly 32 bytes
- **THEN** parsing SHALL reject the key

#### Scenario: parser resources and message boundaries are enforced

- **WHEN** input exceeds 4096 bytes, nesting exceeds 16, a tag or trailing
  item is present, or more than 32 additional parameters are supplied
- **THEN** parsing SHALL reject the input before returning a key

#### Scenario: duplicate labels cannot be smuggled

- **WHEN** a top-level or retained nested map repeats a deterministically
  equivalent key
- **THEN** parsing or deterministic encoding SHALL reject it

#### Scenario: public extensions round trip deterministically

- **WHEN** a valid key contains bounded common or unknown public parameters,
  including explicitly present empty common byte strings, without
  floating-point values
- **THEN** repeated `to_cbor` calls SHALL return identical bytes with RFC 8949
  length-first map ordering and parsing those bytes SHALL retain equivalent
  parameters

#### Scenario: curve encoders preserve public key bytes

- **WHEN** Ed25519, X25519, P-256 or secp256k1 public keys call
  `encode_cose()`
- **THEN** the result SHALL use the correct typed profile and exact public
  coordinate bytes

#### Scenario: full-coordinate JWK conversion is lossless for key material

- **WHEN** a supported structural JWK converts to COSE and back, or a
  full-coordinate COSE key converts to JWK and back
- **THEN** key type, curve and public coordinate bytes SHALL be unchanged,
  while format-specific metadata SHALL NOT be inferred

#### Scenario: compressed EC2 does not masquerade as a JWK

- **WHEN** conversion of a sign-bit EC2 key to `PublicKeyJwk` is requested
- **THEN** conversion SHALL fail explicitly without performing curve
  decompression

## MODIFIED Requirements

### Requirement: Two-surface error bridging

The crate SHALL provide an idiomatic `crypto::Error` enum carrying runtime
detail for primitive operations and dedicated `JwkError` and `CoseKeyError`
types for validated representation boundaries. Each SHALL map to redaction-safe
`identus_core::IdentusError` values under `CapabilityId("crypto")`. The stable
`ErrorCode` catalogue SHALL include `crypto.invalid_key_size`,
`crypto.key_parsing`, `crypto.signature_invalid`, `crypto.unsupported_curve`,
`crypto.derivation_failed`, `crypto.mnemonic_invalid`,
`crypto.secure_random_failure`, `crypto.invalid_jwk` and
`crypto.invalid_cose_key`. `IdentusError::Display` SHALL render only
`"{code}: {public_message}"` and SHALL NOT include runtime detail. JWK and COSE
key errors SHALL identify only the failed invariant and SHALL NOT contain raw
coordinate, CBOR or extension values.

#### Scenario: InvalidKeySize maps to a stable code with no runtime detail

- **WHEN** `crypto::Error::InvalidKeySize { expected: 32, actual: 31, key_type }` is mapped via `to_identus_error()` and displayed
- **THEN** the `ErrorCode` SHALL be `crypto.invalid_key_size`, the `CapabilityId` SHALL be `"crypto"`, and the rendered string SHALL NOT contain `31`, `32`, or the key type

#### Scenario: Signature verification failure maps to VerificationFailed

- **WHEN** a signature-verification failure is mapped via `to_identus_error()`
- **THEN** the `ErrorKind` SHALL be `VerificationFailed` and the `ErrorCode` SHALL be `crypto.signature_invalid`

#### Scenario: invalid JWK maps to a stable redacted code

- **WHEN** a `JwkError` is mapped through `to_identus_error()`
- **THEN** the code SHALL be `crypto.invalid_jwk`, the kind SHALL be
  `InvalidInput`, the capability SHALL be `crypto`, and the rendered value
  SHALL NOT contain caller-supplied coordinate or extension values

#### Scenario: invalid COSE key maps to a stable redacted code

- **WHEN** a `CoseKeyError` is mapped through `to_identus_error()`
- **THEN** the code SHALL be `crypto.invalid_cose_key`, the kind SHALL be
  `InvalidInput`, the capability SHALL be `crypto`, and the rendered value
  SHALL NOT contain caller-supplied CBOR or extension values

### Requirement: Feature-gated with all-on default

The crate SHALL expose cargo features `ed25519`, `x25519`, `secp256k1`,
`secp256r1`, `hash`, `hex`, `base64`, `jwk`, `cose`, `derivation`, and
`kmp-compat`, with `default` enabling all of them except `kmp-compat`. The
`cose` feature SHALL enable only optional, default-feature-disabled `coset`.
The `jwk` feature SHALL enable `base64`, `serde`, and `serde_json`; curve
features SHALL continue to imply `jwk`.
Curve `EncodeCose` implementations SHALL be available when the curve and
`cose` features are both enabled; JWK conversion SHALL additionally require
`jwk`. A minimal COSE build SHALL not require serde, JSON, base64 or a curve
backend. There SHALL be no `securerandom` feature because the zero-dependency
`SecureRandom` port is always available, and no `wasm` feature because
concrete entropy adapters belong to `identus-adapters-entropy`. `kmp-compat`
SHALL remain opt-in, gate the KMP interop surface, and introduce no dependency
outside the default dependency set. The full default and supported minimal
feature combinations SHALL remain wasm-safe.

#### Scenario: Default features compile the full surface

- **WHEN** `cargo build -p identus-crypto` is run with default features
- **THEN** all curve, hashing, derivation, JWK, COSE, and `SecureRandom`-port
  modules SHALL compile and be available, and the KMP-interop surface SHALL NOT
  be present

#### Scenario: Default features compile on wasm32

- **WHEN** `cargo build -p identus-crypto --target wasm32-unknown-unknown` is run with default features
- **THEN** the build SHALL succeed

#### Scenario: minimal JWK feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features jwk`
  is run
- **THEN** the validated JWK and JSON wire surface SHALL compile without a
  curve backend

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
- **THEN** the build SHALL succeed and `create_seed_kmp` SHALL be present on `MnemonicHelper`

#### Scenario: kmp-compat introduces no new external dependency

- **WHEN** the `kmp-compat` feature is enabled with its required base features
- **THEN** the build SHALL succeed without pulling a crate not already required by the default feature set

### Requirement: Workspace-level external dependency declaration

All external dependencies SHALL be declared in root `[workspace.dependencies]`,
including `ed25519-dalek`, `k256`, `p256`, `x25519-dalek`, `sha2`, `hmac`,
`pbkdf2`, `base64`, `hex`, `serde`, `serde_json`, and `coset`,
and referenced via `<dep>.workspace = true` per
`workspace-dependency-conventions`; no inline external version pin SHALL
appear in `crates/crypto/Cargo.toml`. `coset` SHALL be optional,
default-feature-disabled and enabled only by `cose`. The list SHALL NOT include
`ring`, whose concrete entropy concern belongs to `identus-adapters-entropy`.

#### Scenario: COSE dependency remains optional and encapsulated

- **WHEN** the workspace and crypto manifests plus public Rust API are
  inspected
- **THEN** `coset` SHALL be workspace-declared, optional and feature-gated,
  and no public SDK signature SHALL expose a `coset` or `ciborium` type

#### Scenario: crypto external deps use the workspace form

- **WHEN** `crates/crypto/Cargo.toml` is inspected
- **THEN** every external dependency entry SHALL use `.workspace = true`,
  `serde` and `serde_json` SHALL be optional edges of `jwk`, `coset` SHALL be
  an optional edge of `cose`, and `ring` SHALL NOT be present

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
rejected. The encoder SHALL normalize known registry names to integers and
emit definite-length, shortest-form CBOR with recursively length-first map-key
ordering: shorter deterministic key encodings first, then bytewise lexical
order for keys of equal length.

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

- **WHEN** a valid key contains bounded common or unknown public parameters
  without floating-point values
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
Curve `EncodeCose` implementations SHALL be available when the curve and
`cose` features are both enabled; JWK conversion SHALL additionally require
`jwk`. A minimal COSE build SHALL not require serde, JSON, base64 or a curve
backend. The full default and supported minimal feature combinations SHALL
remain wasm-safe.

#### Scenario: minimal COSE feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features cose`
  is run
- **THEN** the validated COSE Key and CBOR wire surface SHALL compile without
  JSON, base64 or a curve backend

#### Scenario: curve plus COSE feature compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features ed25519,cose`
  is run
- **THEN** Ed25519 and its `EncodeCose` implementation SHALL compile

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

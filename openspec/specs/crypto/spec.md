## Purpose

`identus-crypto` is the canonical cryptographic capability of the Identus Rust SDK — the reference implementation that the TypeScript, Swift, and KMP bindings are ported *from*. It owns the domain-primitive cryptographic building blocks (key types, signatures, key generation, JWK, COSE Key, hashing, derivation, secure random, ed→x25519 conversion) that the DID, trust, credential, and protocol crates build on. It sits in the `domain-primitives` layer and depends only on `identus-core`. Its enduring rules are:

- **Canonical reference.** The Rust crate is the port source for bindings, not a consumer of another repo's crypto crate. The source is ported in from neoprism's `lib/apollo` and completed to the full KMP `apollo` capability surface.
- **Full lifecycle per curve.** Every curve exposes `PublicKey` + `PrivateKey` + `KeyPair` with `generate` / `sign` / `verify` (and DH for X25519). Verify-only is a resolver's concern, not an SDK's.
- **Two-surface error bridging.** A rich idiomatic `crypto::Error` carries runtime detail for local debugging; `to_identus_error()` maps to the redaction-safe `IdentusError` with a stable `ErrorCode` catalogue and `CapabilityId("crypto")`. Runtime detail (e.g. key sizes, actual bytes) never crosses into `IdentusError`. `crypto` is the first adopter of the `core-error-conventions` bridging pattern.
- **Cross-node compatibility.** secp256k1 verification preserves the JVM-compat fallbacks (normalize-s + bitcoin-transcode) so signatures produced by the JVM PRISM node verify correctly.
- **Feature-gated, all-on default.** Algorithms are cargo features mirroring neoprism's gating; `default` enables the full surface so the canonical build and every binding's default path compile the whole capability.
- **Audited supply chain.** External crates are chosen for audit posture (RustCrypto / dalek) and declared at workspace level per `workspace-dependency-conventions`; `cargo-deny` (per `nix-tooling`) audits the dependency graph and `rust-audit` (crane `cargoAudit`) checks for known advisories. The `ring` entropy adapter's supply chain is audited in its own crate (`identus-adapters-entropy`), not here.
- **Primitive operations, not key management.** The crate owns primitive crypto operations on key *material* (bytes-in, bytes-out), neoprism-`apollo`-style. The only infrastructure port is `SecureRandom` (entropy) — **defined here**; its concrete adapters live in the outer-boundary `identus-adapters-entropy` crate, injected into `generate`/`create_random_mnemonics`. It does **not** define `KeyHandle`, `KeyStore`, `SecretResolver`, non-exportable signing, or hardware/`KMS`-bound signers — those are an `identus-wallet` concern per the secure-storage boundary.
- **Concrete where backends don't vary.** Curve operations, hashing, derivation, and conversion are concrete (no `Signer`/`Digest` ports): `ed25519-dalek`/`k256`/`p256`/`x25519-dalek`/`sha2` build on every target including `wasm32`, so a single-adapter port would document nothing. Hashing uses pure `sha2` (no `ring`), staying wasm-safe. The crate has **no `ring` dependency** (the `ring` entropy adapter is in `identus-adapters-entropy`), so it builds on `wasm32` with default features.
## Requirements
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

### Requirement: Encoding traits

The crate SHALL provide `EncodeVec` (`encode_vec() -> Vec<u8>`), `EncodeArray<const N>` (`encode_array() -> [u8; N]`), and `Verifiable` (`verify(&self, message: &[u8], signature: &[u8]) -> bool`) traits, ported from neoprism.

#### Scenario: Public key encodes to a fixed-size array

- **WHEN** an `EncodeArray<N>` key calls `encode_array()`
- **THEN** it SHALL return a `[u8; N]` of the canonical encoding

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
unpadded base64url text. The serde, thumbprint, and structural JWK/COSE
invariants SHALL remain under the bounded sanitizer campaign.

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

#### Scenario: accepted wire values remain coherent under mutation

- **WHEN** sanitizer-guided mutation produces a JWK accepted by the public
  serde boundary
- **THEN** round-trip equality, coordinate shape, canonical thumbprint, and
  supported structural conversion SHALL hold without panic

### Requirement: Base64URL-no-pad, Hex, and SHA-2 primitives

The crate SHALL provide `Base64UrlStrNoPad`, `HexStr`, `Sha256Digest`, and `Sha512Digest`, backed by pure `sha2` (wasm-safe; `ring` is **not** used for hashing).

#### Scenario: SHA-256 digest of a known input

- **WHEN** a known input is hashed with `Sha256Digest`
- **THEN** the digest SHALL match the known SHA-256 vector

### Requirement: Ed25519 full lifecycle

The crate SHALL provide `Ed25519PublicKey`, `Ed25519PrivateKey`, and
`Ed25519KeyPair` with fallible
`generate(rng: &mut impl SecureRandom) -> Result<Self, Error>`, `sign`,
`verify` (strict), `EncodeArray<32>`, `EncodeVec`, `Verifiable`, and
`EncodeJwk`.

#### Scenario: Ed25519 keypair generates, signs, and verifies

- **WHEN** a successful injected `SecureRandom` is passed to
  `Ed25519KeyPair::generate(rng)` and the resulting key signs a message
- **THEN** generation SHALL return `Ok`; verification SHALL succeed and a
  tampered message SHALL fail verification

#### Scenario: Ed25519 entropy failure is recoverable

- **WHEN** the injected entropy provider fails
- **THEN** generation SHALL return `Error::SecureRandomFailure` without panic
  or key construction

### Requirement: X25519 full lifecycle with DH

The crate SHALL provide `X25519PublicKey`, `X25519PrivateKey`, and
`X25519KeyPair` with fallible
`generate(rng: &mut impl SecureRandom) -> Result<Self, Error>`,
`derive_shared` (Diffie-Hellman shared secret), `EncodeArray<32>`, `EncodeVec`,
and `EncodeJwk`.

#### Scenario: X25519 DH yields a shared secret

- **WHEN** two keypairs are successfully generated and perform
  `derive_shared` with each other's public keys
- **THEN** both sides SHALL derive the same shared secret

#### Scenario: X25519 entropy failure is recoverable

- **WHEN** the injected entropy provider fails
- **THEN** generation SHALL return `Error::SecureRandomFailure` without panic
  or key construction

### Requirement: secp256k1 full lifecycle with JVM-compat verification

The crate SHALL provide `Secp256k1PublicKey` (compressed 33-byte /
uncompressed 65-byte / `CurvePoint`), `Secp256k1PrivateKey`, and
`Secp256k1KeyPair` with fallible
`generate(rng: &mut impl SecureRandom) -> Result<Self, Error>`, `sign`
(DER-encoded), `verify`, `EncodeArray<33>`/`EncodeArray<65>`, `EncodeVec`, and
`EncodeJwk`. Generation SHALL return immediately on provider failure and SHALL
return `Error::SecureRandomFailure` after at most sixteen successfully filled
but invalid scalar candidates. `verify` SHALL try, in order, raw verify,
normalize-s verify, and bitcoin-transcode verify, returning `true` if any
succeeds — preserving cross-node (JVM PRISM) compatibility.

#### Scenario: secp256k1 verifies a JVM-signed signature

- **WHEN** a signature from the JVM-compat 50-vector suite is verified
- **THEN** `verify` SHALL return `true` via one of the three fallback paths

#### Scenario: secp256k1 public key round-trips compressed/uncompressed

- **WHEN** a public key's compressed and uncompressed encodings are converted
  between each other
- **THEN** they SHALL resolve to the same curve point

#### Scenario: secp256k1 random generation fails without panic

- **WHEN** entropy fails or sixteen filled candidates are invalid scalars
- **THEN** generation SHALL return `Error::SecureRandomFailure` without panic
  or exposing candidate bytes

### Requirement: secp256r1 (P-256) full lifecycle

The crate SHALL provide `P256PublicKey`, `P256PrivateKey`, and `P256KeyPair`
with fallible `generate(rng: &mut impl SecureRandom) -> Result<Self, Error>`,
DER-compatible `sign`/`verify`, fixed-width `sign_fixed`/`verify_fixed`, and
`EncodeJwk`, for parity with the KMP
`KMMEllipticCurve::SECP256r1` surface (a gap in neoprism's port). Generation
SHALL return immediately on provider failure and SHALL return
`Error::SecureRandomFailure` after at most sixteen successfully filled but
invalid scalar candidates. The fixed-width methods SHALL use exactly 64 bytes
containing the unsigned big-endian P-256 `r` value followed by `s`; adding
them SHALL NOT change the inherited DER surface.

#### Scenario: P-256 keypair generates, signs, and verifies

- **WHEN** a successful injected `SecureRandom` is passed to
  `P256KeyPair::generate(rng)` and the resulting key signs a message
- **THEN** generation SHALL return `Ok` and verification SHALL succeed against
  known P-256 test vectors

#### Scenario: P-256 random generation fails without panic

- **WHEN** entropy fails or sixteen filled candidates are invalid scalars
- **THEN** generation SHALL return `Error::SecureRandomFailure` without panic
  or exposing candidate bytes

#### Scenario: P-256 supports fixed-width protocol signatures additively

- **WHEN** a typed P-256 private key calls `sign_fixed` and its public key
  calls `verify_fixed` over the same message
- **THEN** the 64-byte `r || s` value verifies, tampering fails, and the
  existing DER `sign`/`verify` path remains unchanged

### Requirement: SecureRandom — the single infrastructure port

The crate SHALL provide a `SecureRandom` trait (the only infrastructure port in
the crate) with
`fill_bytes(output: &mut [u8]) -> Result<(), crypto::Error>`, as the canonical
entropy source for key generation and mnemonic creation. The caller SHALL own
the buffer and allocation policy; implementations SHALL fill the entire slice
on success and SHALL return `Error::SecureRandomFailure` on provider failure.
The crate SHALL define the **port only**; it SHALL NOT ship any concrete
`SecureRandom` implementation in this crate. Concrete backends live in the
outer-boundary `identus-adapters-entropy` crate: a `getrandom`-backed adapter
and the test-only deterministic adapter. Key generation (`*KeyPair::generate`)
and mnemonic creation (`MnemonicHelper::create_random_mnemonics`) SHALL take an
injected `&mut impl SecureRandom`. No other primitive operation is exposed as a
port.

#### Scenario: SecureRandom fills caller-owned storage

- **WHEN** `fill_bytes` is called with a 32-byte slice and the provider succeeds
- **THEN** it SHALL fill the entire slice and return `Ok(())` without allocating
  storage for the caller

#### Scenario: SecureRandom accepts an empty slice

- **WHEN** `fill_bytes` is called with an empty slice
- **THEN** it SHALL return `Ok(())`

#### Scenario: SecureRandom failure is stable and redacted

- **WHEN** the backing entropy provider fails
- **THEN** the call SHALL return `Error::SecureRandomFailure`, whose public
  bridge uses `crypto.secure_random_failure` without backend detail or bytes

#### Scenario: crypto declares no ring dependency

- **WHEN** `crates/crypto/Cargo.toml` is inspected
- **THEN** it SHALL NOT list `ring` as a dependency

#### Scenario: the getrandom adapter lives in identus-adapters-entropy

- **WHEN** `crates/adapters-entropy/` is inspected behind its `getrandom` feature
- **THEN** it SHALL provide `GetrandomSystemRandomAdapter` implementing the port

### Requirement: BIP32 secp256k1 hierarchical derivation

The crate SHALL provide an `HDKey` for secp256k1 with `derive(path)`, `derive_child(index)`, hardened derivation, and `init_from_seed(seed)`, ported from the KMP `derivation.HDKey`.

#### Scenario: BIP32 derivation matches known vectors

- **WHEN** a known seed is derived along a known path
- **THEN** the resulting key SHALL match the published BIP32 test vector

### Requirement: SLIP-0010 ed25519 hierarchical derivation

The crate SHALL provide an `EdHDKey` for ed25519 with `derive(path)`, `derive_child(index)`, and `init_from_seed(seed)`, ported from the KMP `derivation.EdHDKey` (SLIP-0010, hardened-only ed25519 derivation).

#### Scenario: SLIP-0010 derivation matches known vectors

- **WHEN** a known seed is derived along a known SLIP-0010 path
- **THEN** the resulting key SHALL match the published SLIP-0010 ed25519 test vector

### Requirement: BIP39 mnemonic helper

The crate SHALL provide a `MnemonicHelper` with fallible
`create_random_mnemonics(rng: &mut impl SecureRandom) -> Result<Vec<String>, Error>`,
`create_seed(mnemonics, passphrase)` (PBKDF2-SHA512, 2048 iterations, 64-byte
dk, salt = `"mnemonic" + passphrase`), fallible
`create_random_seed(rng) -> Result<Vec<u8>, Error>`, and
`is_valid_mnemonic_code`, with an English wordlist. Random creation SHALL use a
fixed 32-byte entropy buffer and propagate provider failure. The crate SHALL
additionally provide `create_seed_kmp(mnemonics, passphrase)` behind the
`kmp-compat` feature using the KMP salt (`passphrase` without the `"mnemonic"`
prefix) for one-way legacy PRISM wallet import; it SHALL require an explicit
passphrase. Both deterministic seed functions SHALL validate the mnemonic and
return `Error::MnemonicInvalid` on failure.

#### Scenario: BIP39 seed derivation matches known vectors

- **WHEN** a known mnemonic and passphrase are fed to `create_seed`
- **THEN** the 64-byte seed SHALL match the published BIP39 test vector

#### Scenario: Default passphrase is the standard empty string

- **WHEN** `create_random_seed(rng)` succeeds
- **THEN** the derived seed SHALL use salt `"mnemonic"` and return `Ok`

#### Scenario: KMP-compat seed derivation matches KMP apollo

- **WHEN** a known mnemonic and passphrase are fed to `create_seed_kmp` with
  `kmp-compat` enabled
- **THEN** the 64-byte seed SHALL match KMP `apollo` for the same inputs

#### Scenario: KMP-compat seed matches cloud-agent legacy wallets

- **WHEN** `create_seed_kmp(words, "")` is called with `kmp-compat` enabled
- **THEN** the 64-byte seed SHALL match the seed `cloud-agent` produces via
  `MnemonicHelper.createSeed(words, "")` with salt `""`

#### Scenario: create_seed_kmp requires an explicit passphrase

- **WHEN** the `create_seed_kmp` signature is inspected
- **THEN** the `passphrase` parameter SHALL have no default value

#### Scenario: Invalid mnemonic is rejected by both variants

- **WHEN** a mnemonic containing a word not in the wordlist is validated
- **THEN** `is_valid_mnemonic_code` SHALL return false, and both `create_seed`
  and `create_seed_kmp` SHALL error with `crypto.mnemonic_invalid`

#### Scenario: create_seed_kmp is unavailable without the feature

- **WHEN** the crate is compiled without `--features kmp-compat`
- **THEN** `create_seed_kmp` SHALL NOT be present on `MnemonicHelper`

#### Scenario: Random mnemonic creation reports entropy failure

- **WHEN** the injected entropy provider fails
- **THEN** random mnemonic and random seed creation SHALL return
  `Error::SecureRandomFailure` without panic or mnemonic construction

### Requirement: Ed25519 to X25519 conversion

The crate SHALL provide `ConvertEd25519` converting an Ed25519 private key to an X25519 private key (SHA-512 hash of the first 32 bytes + clamping), ported from the KMP `ConvertEd25519`.

#### Scenario: Ed25519 private key converts to a valid X25519 private key

- **WHEN** an Ed25519 private key is converted to X25519 and used in DH
- **THEN** the derived shared secret SHALL match the conversion-derived key's DH result against known conversion vectors

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

### Requirement: Layer conformance — depends only on identus-core and the proc-macro attribute provider

The crate SHALL declare `identus-core` as its only runtime workspace-internal dependency, plus the proc-macro attribute provider `identus-derive` (referenced via `identus-derive.workspace = true` under `[dependencies]`) per `naming-conventions` ("Port-owning crates depend on the proc-macro attribute provider"); it SHALL NOT depend on any other workspace-internal crate. It SHALL NOT depend on `identus-adapters`, `identus-bindings`, `identus-conformance`, or any `identus-adapters-<family>` crate (e.g. `identus-adapters-entropy`). It SHALL NOT depend on `ring` (the `ring` entropy adapter is in `identus-adapters-entropy`). All external crates SHALL be declared at workspace level per `workspace-dependency-conventions`. The `crate-ring-layout` conformance guard SHALL pass.

#### Scenario: crypto declares only identus-core and identus-derive inward

- **WHEN** `crates/crypto/Cargo.toml` `[dependencies]` is inspected for workspace-internal crates
- **THEN** it SHALL contain only `identus-core` and `identus-derive` (the proc-macro attribute provider, per `naming-conventions`), and no other workspace-internal crate

#### Scenario: crypto does not depend on an adapter-family crate

- **WHEN** `crates/crypto/Cargo.toml` is inspected
- **THEN** it SHALL NOT list `identus-adapters-entropy` (or any `identus-adapters-*`); the conformance guard SHALL reject such an edge

#### Scenario: Layer guard passes for crypto

- **WHEN** `cargo test -p identus-conformance` is run
- **THEN** the dep-graph guard SHALL pass for `identus-crypto` (no outward edges)

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

### Requirement: Wasm-clean by default

The crate SHALL build on `wasm32-unknown-unknown` with default features,
because it has no `ring` dependency. Its external cryptographic dependencies
are wasm-safe. The domain primitive crate is wasm-clean; no infrastructure
it depends on introduces a `wasm32`-incompatible backend.

#### Scenario: crypto builds on wasm32 with default features

- **WHEN** `cargo build -p identus-crypto --target wasm32-unknown-unknown` is
run with default features
- **THEN** the build SHALL succeed

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
labels and allow no more than 32 non-structural top-level parameters. Only
`kty`, `crv`, `x`, and `y` are structural for this bound; common parameters
such as `alg`, `key_ops`, `kid`, and Base IV count together with unknown public
parameters. Common and unknown public parameters SHALL survive parse/encode
without being interpreted or exposed as third-party wire types. Floating-point
extension values SHALL be rejected. The encoder SHALL normalize supported
`kty` and `crv` registry names to integers and emit definite-length,
shortest-form CBOR with recursively length-first map-key ordering: shorter
deterministic key encodings first, then bytewise lexical order for keys of
equal length. The resource, encoding, and structural conversion invariants
SHALL remain under the bounded sanitizer campaign.

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
  item is present, or more than 32 common-plus-unknown parameters are supplied
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

#### Scenario: accepted binary values remain coherent under mutation

- **WHEN** sanitizer-guided mutation produces a COSE Key accepted by the
  public bounded parser
- **THEN** deterministic re-encoding, reparse equality, resource ceilings, and
  supported structural conversion SHALL hold without panic

### Requirement: Sanitizer-backed public-key representation fuzzing

The crypto capability SHALL provide separate sanitizer-backed targets for its
public JWK and public COSE Key representation boundaries. Targets SHALL accept
arbitrary bytes and treat rejection as valid. Every accepted JWK SHALL preserve
its validated value across serde round trips, produce deterministic canonical
RFC 7638 SHA-256 thumbprints, and preserve required public key material through
full-coordinate COSE conversion. Every accepted COSE Key SHALL preserve its
validated value across deterministic encoding and reparsing and SHALL preserve
required public material through supported JWK conversion. Compressed EC2
conversion SHALL remain an explicit failure.

Fuzz-only dependencies SHALL remain in an independent workspace outside every
published crate dependency cone. The targets SHALL NOT add algorithm, curve,
key-use, signature, custody, trust, method, chain, or product policy.

#### Scenario: hostile JWK bytes cannot violate public invariants

- **WHEN** arbitrary bounded bytes contain valid, invalid, non-UTF-8, private,
  shadowed, incompatible, malformed, padded, deeply extended, or oversized
  JSON Web Key text
- **THEN** deserialization SHALL reject it without panic or return a public key
  whose serde, coordinate, thumbprint, and structural conversion invariants hold

#### Scenario: hostile COSE bytes cannot violate public invariants

- **WHEN** arbitrary bounded bytes contain valid, invalid, tagged, trailing,
  duplicate, private, floating-point, deeply nested, wrong-width, compressed,
  or over-limit CBOR
- **THEN** parsing SHALL reject it without panic or return a public key whose
  resource, deterministic encoding, and structural conversion invariants hold

#### Scenario: fuzz tooling is not a crypto dependency

- **WHEN** production, minimal-feature, MSRV, mobile, WASM, or downstream
  dependency cones are built
- **THEN** cargo-fuzz, libFuzzer, sanitizer, and corpus transport support SHALL
  NOT be required by or exposed from `identus-crypto`

### Requirement: Reproducible bounded crypto fuzz campaigns

The repository SHALL expose one documented crypto command interface for
committed-corpus replay, deterministic fixed-run smoke, and time-boxed soak
modes through its pinned sanitizer compiler, runner, and runtime binding.
Pull-request and integration smoke SHALL fix seed, run count, input ceiling,
execution timeout, memory ceiling, mutation reload, and worker count. Scheduled
and manual soak SHALL remain separately bounded.

Original corpora and dictionaries SHALL cover standards-shaped public-key and
consumer-shaped representation boundaries without containing production key
material or asserting trust. Exact binary COSE seeds MAY use a documented
text-only transport decoded solely by the harness. Failure artifacts SHALL be
retained for triage; an accepted defect SHALL be minimized and promoted to
committed corpus and deterministic regression evidence. Performance SHALL be
recorded without a hardware-specific pass threshold.

#### Scenario: ordinary crypto fuzz CI is repeatable

- **WHEN** the same revision runs the pull-request crypto fuzz gate
- **THEN** both targets SHALL receive the same seed, run count, and resource
  limits through the pinned Nix environment and terminate deterministically

#### Scenario: longer search remains bounded and diagnosable

- **WHEN** scheduled or manually dispatched crypto soak finds a sanitizer or
  invariant failure
- **THEN** the target SHALL stop inside the documented envelope and preserve
  its untrusted artifact for minimization without custom logging of its bytes

### Requirement: Best-effort lifecycle protection for owned secret buffers

The crypto capability SHALL prevent raw secret material from entering safe
`Debug`, `Display`, error, serialization, or FFI surfaces. `HDKey` and
`EdHDKey` SHALL implement `Zeroize` and `ZeroizeOnDrop`; their formatting SHALL
show only public derivation metadata and SHALL omit private key and chain-code
bytes. SDK-owned temporary entropy, internally consumed mnemonic word strings,
mnemonic entropy, HD HMAC input/output, and PBKDF2 output buffers SHALL use
zeroizing storage and preserve every existing algorithm and byte output.

This is a best-effort owned-buffer contract. It SHALL NOT claim to erase
caller-created or compiler-created copies, allocator state, swap, crash dumps,
or hardware state. Raw secret-returning APIs SHALL remain explicit and their
returned values SHALL be caller-owned.

#### Scenario: HD debug output is redacted

- **WHEN** an `HDKey` or `EdHDKey` containing known private key and chain-code
  bytes is formatted with `Debug`
- **THEN** the output SHALL identify the type and public derivation metadata
  but SHALL NOT contain either secret byte sequence

#### Scenario: HD keys expose an enforceable erasure contract

- **WHEN** compile-time assertions inspect `HDKey` and `EdHDKey`
- **THEN** both SHALL implement `Zeroize` and `ZeroizeOnDrop`

#### Scenario: explicit erasure clears owned state

- **WHEN** `Zeroize::zeroize` is invoked on an HD key
- **THEN** its private key, chain code, depth, and child index SHALL be reset to
  zero without panic

#### Scenario: random construction intermediates are scoped for erasure

- **WHEN** Ed25519, X25519, secp256k1, P-256, or BIP-39 random construction
  succeeds or returns an error
- **THEN** the SDK-owned entropy buffer SHALL be guarded by drop-time
  zeroization and SHALL NOT be included in formatting or errors

#### Scenario: internally consumed random mnemonic words are scoped for erasure

- **WHEN** random seed convenience creation converts SDK-owned entropy into
  mnemonic word strings and consumes them internally
- **THEN** the word vector and its strings SHALL be guarded by drop-time
  zeroization while caller-returned mnemonic words remain caller-owned

#### Scenario: derivation outputs remain compatible

- **WHEN** the existing BIP32, SLIP-0010, BIP39, and KMP compatibility vectors
  are evaluated after lifecycle hardening
- **THEN** every derived byte sequence and public error result SHALL remain
  unchanged

#### Scenario: erasure limits are documented truthfully

- **WHEN** a consumer reads the secret lifecycle documentation
- **THEN** it SHALL distinguish SDK-owned buffers from caller or platform
  copies and SHALL NOT describe best-effort memory erasure as custody or secure
  storage

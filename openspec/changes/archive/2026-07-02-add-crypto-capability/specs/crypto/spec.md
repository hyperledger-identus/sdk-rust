## Purpose

`identus-crypto` is the canonical cryptographic capability of the Identus Rust SDK — the reference implementation that the TypeScript, Swift, and KMP bindings are ported *from*. It owns the domain-primitive cryptographic building blocks (key types, signatures, key generation, JWK, hashing, derivation, secure random, ed→x25519 conversion) that the DID, trust, credential, and protocol crates build on. It sits in the `domain-primitives` layer and depends only on `identus-core`. Its enduring rules are:

- **Canonical reference.** The Rust crate is the port source for bindings, not a consumer of another repo's crypto crate. The source is ported in from neoprism's `lib/apollo` and completed to the full KMP `apollo` capability surface.
- **Full lifecycle per curve.** Every curve exposes `PublicKey` + `PrivateKey` + `KeyPair` with `generate` / `sign` / `verify` (and DH for X25519). Verify-only is a resolver's concern, not an SDK's.
- **Two-surface error bridging.** A rich idiomatic `crypto::Error` carries runtime detail for local debugging; `to_identus_error()` maps to the redaction-safe `IdentusError` with a stable `ErrorCode` catalogue and `CapabilityId("crypto")`. Runtime detail (e.g. key sizes, actual bytes) never crosses into `IdentusError`. `crypto` is the first adopter of the `core-error-conventions` bridging pattern.
- **Cross-node compatibility.** secp256k1 verification preserves the JVM-compat fallbacks (normalize-s + bitcoin-transcode) so signatures produced by the JVM PRISM node verify correctly.
- **Feature-gated, all-on default.** Algorithms are cargo features mirroring neoprism's gating; `default` enables the full surface so the canonical build and every binding's default path compile the whole capability.
- **Audited supply chain.** External crates are chosen for audit posture (RustCrypto / dalek) and declared at workspace level per `workspace-dependency-conventions`; `cargo-deny` (per `nix-tooling`) audits the dependency graph and `rust-audit` (crane `cargoAudit`) checks for known advisories. The `ring` entropy adapter's supply chain is audited in its own crate (`identus-adapters-entropy`), not here.
- **Primitive operations, not key management.** The crate owns primitive crypto operations on key *material* (bytes-in, bytes-out), neoprism-`apollo`-style. The only infrastructure port is `SecureRandom` (entropy) — **defined here**; its concrete adapters live in the outer-boundary `identus-adapters-entropy` crate, injected into `generate`/`create_random_mnemonics`. It does **not** define `KeyHandle`, `KeyStore`, `SecretResolver`, non-exportable signing, or hardware/`KMS`-bound signers — those are an `identus-wallet` concern per the secure-storage boundary.
- **Concrete where backends don't vary.** Curve operations, hashing, derivation, and conversion are concrete (no `Signer`/`Digest` ports): `ed25519-dalek`/`k256`/`p256`/`x25519-dalek`/`sha2` build on every target including `wasm32`, so a single-adapter port would document nothing. Hashing uses pure `sha2` (no `ring`), staying wasm-safe. The crate has **no `ring` dependency** (the `ring` entropy adapter is in `identus-adapters-entropy`), so it builds on `wasm32` with default features.

## ADDED Requirements

### Requirement: Two-surface error bridging

The crate SHALL provide an idiomatic `crypto::Error` enum carrying runtime detail (e.g. `InvalidKeySize { expected: usize, actual: usize, key_type: &'static str }`, `KeyParsing { source }`) and a `to_identus_error()` mapping to `identus_core::IdentusError` with a stable `ErrorCode` and `CapabilityId("crypto")`. The stable `ErrorCode` catalogue SHALL include at least `crypto.invalid_key_size`, `crypto.key_parsing`, `crypto.signature_invalid`, `crypto.unsupported_curve`, `crypto.derivation_failed`, `crypto.mnemonic_invalid`, `crypto.secure_random_failure`. `IdentusError::Display` SHALL render only `"{code}: {public_message}"` and SHALL NOT include runtime detail.

#### Scenario: InvalidKeySize maps to a stable code with no runtime detail

- **WHEN** `crypto::Error::InvalidKeySize { expected: 32, actual: 31, key_type }` is mapped via `to_identus_error()` and displayed
- **THEN** the `ErrorCode` SHALL be `crypto.invalid_key_size`, the `CapabilityId` SHALL be `"crypto"`, and the rendered string SHALL NOT contain `31`, `32`, or the key type

#### Scenario: Signature verification failure maps to VerificationFailed

- **WHEN** a signature-verification failure is mapped via `to_identus_error()`
- **THEN** the `ErrorKind` SHALL be `VerificationFailed` and the `ErrorCode` SHALL be `crypto.signature_invalid`

### Requirement: Encoding traits

The crate SHALL provide `EncodeVec` (`encode_vec() -> Vec<u8>`), `EncodeArray<const N>` (`encode_array() -> [u8; N]`), and `Verifiable` (`verify(&self, message: &[u8], signature: &[u8]) -> bool`) traits, ported from neoprism.

#### Scenario: Public key encodes to a fixed-size array

- **WHEN** an `EncodeArray<N>` key calls `encode_array()`
- **THEN** it SHALL return a `[u8; N]` of the canonical encoding

### Requirement: JWK

The crate SHALL provide a `Jwk` struct (`kty`, `crv`, `x`, `y`) and an `EncodeJwk` trait (`encode_jwk() -> Jwk`) implemented for every public key type.

#### Scenario: Ed25519 public key encodes to JWK

- **WHEN** an Ed25519 public key calls `encode_jwk()`
- **THEN** the `Jwk` SHALL have `kty = "OKP"`, `crv = "Ed25519"`, `x` set, `y = None`

#### Scenario: secp256k1 public key encodes to JWK

- **WHEN** a secp256k1 public key calls `encode_jwk()`
- **THEN** the `Jwk` SHALL have `kty = "EC"`, `crv = "secp256k1"`, `x` and `y` set to the curve-point coordinates

### Requirement: Base64URL-no-pad, Hex, and SHA-2 primitives

The crate SHALL provide `Base64UrlStrNoPad`, `HexStr`, `Sha256Digest`, and `Sha512Digest`, backed by pure `sha2` (wasm-safe; `ring` is **not** used for hashing).

#### Scenario: SHA-256 digest of a known input

- **WHEN** a known input is hashed with `Sha256Digest`
- **THEN** the digest SHALL match the known SHA-256 vector

### Requirement: Ed25519 full lifecycle

The crate SHALL provide `Ed25519PublicKey`, `Ed25519PrivateKey`, and `Ed25519KeyPair` with `generate(rng: &mut impl SecureRandom)`, `sign`, `verify` (strict), `EncodeArray<32>`, `EncodeVec`, `Verifiable`, and `EncodeJwk`.

#### Scenario: Ed25519 keypair generates, signs, and verifies

- **WHEN** an `Ed25519KeyPair::generate(rng)` (with an injected `SecureRandom`) signs a message and the public key verifies it
- **THEN** verification SHALL succeed; a tampered message SHALL fail verification

### Requirement: X25519 full lifecycle with DH

The crate SHALL provide `X25519PublicKey`, `X25519PrivateKey`, and `X25519KeyPair` with `generate(rng: &mut impl SecureRandom)`, `derive_shared` (Diffie-Hellman shared secret), `EncodeArray<32>`, `EncodeVec`, and `EncodeJwk`.

#### Scenario: X25519 DH yields a shared secret

- **WHEN** two keypairs (each generated via `generate(rng)`) perform `derive_shared` with each other's public keys
- **THEN** both sides SHALL derive the same shared secret

### Requirement: secp256k1 full lifecycle with JVM-compat verification

The crate SHALL provide `Secp256k1PublicKey` (compressed 33-byte / uncompressed 65-byte / `CurvePoint`), `Secp256k1PrivateKey`, and `Secp256k1KeyPair` with `generate(rng: &mut impl SecureRandom)`, `sign` (DER-encoded), `verify`, `EncodeArray<33>`/`EncodeArray<65>`, `EncodeVec`, and `EncodeJwk`. `verify` SHALL try, in order, raw verify, normalize-s verify, and bitcoin-transcode verify, returning `true` if any succeeds — preserving cross-node (JVM PRISM) compatibility.

#### Scenario: secp256k1 verifies a JVM-signed signature

- **WHEN** a signature from the JVM-compat 50-vector suite is verified
- **THEN** `verify` SHALL return `true` via one of the three fallback paths

#### Scenario: secp256k1 public key round-trips compressed/uncompressed

- **WHEN** a public key's compressed and uncompressed encodings are converted between each other
- **THEN** they SHALL resolve to the same curve point

### Requirement: secp256r1 (P-256) full lifecycle

The crate SHALL provide `P256PublicKey`, `P256PrivateKey`, and `P256KeyPair` with `generate(rng: &mut impl SecureRandom)`, `sign`, `verify`, and `EncodeJwk`, for parity with the KMP `KMMEllipticCurve::SECP256r1` surface (a gap in neoprism's port).

#### Scenario: P-256 keypair generates, signs, and verifies

- **WHEN** a `P256KeyPair::generate(rng)` (with an injected `SecureRandom`) signs a message and the public key verifies it
- **THEN** verification SHALL succeed against known P-256 test vectors

### Requirement: SecureRandom — the single infrastructure port

The crate SHALL provide a `SecureRandom` trait (the only infrastructure port in the crate) with `generate_seed(num_bytes) -> Vec<u8>`, as the canonical entropy source for key generation and mnemonic creation. The crate SHALL define the **port only**; it SHALL NOT ship a `ring`-backed implementation in this crate — the `ring`-backed adapter lives in the outer-boundary `identus-adapters-entropy` crate (created by the consumed `add-adapter-crate-layer` change) and is filled by this change. `SecureRandom` is a port because a known second backend exists — a `getrandom`-backed wasm adapter (needed by the future TypeScript/wasm binding, since `ring` does not build on `wasm32-unknown-unknown`), also landing in `identus-adapters-entropy` and **deferred to that future binding change**. Key generation (`*KeyPair::generate`) and mnemonic creation (`MnemonicHelper::create_random_mnemonics`) SHALL take an injected `&mut impl SecureRandom` (dependency injection), since the domain crate cannot reach an outer-boundary adapter. No other primitive operation is exposed as a port: curve sign/verify, hashing, derivation, and conversion are concrete (see the "Concrete where backends don't vary" Purpose rule).

#### Scenario: SecureRandom generates the requested length

- **WHEN** `generate_seed(32)` is called on an adapter implementing the port
- **THEN** it SHALL return 32 bytes

#### Scenario: crypto declares no ring dependency

- **WHEN** `crates/crypto/Cargo.toml` is inspected
- **THEN** it SHALL NOT list `ring` as a dependency (the `ring` adapter lives in `identus-adapters-entropy`)

#### Scenario: the ring adapter lives in identus-adapters-entropy

- **WHEN** `crates/adapters-entropy/` (behind its `ring` feature) is inspected
- **THEN** it SHALL provide the `SecureRandom` adapter implementing `identus_crypto::SecureRandom` via `ring`

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

The crate SHALL provide a `MnemonicHelper` with `create_random_mnemonics(rng: &mut impl SecureRandom)`, `create_seed(mnemonics, passphrase)` (PBKDF2-SHA512, 2048 iterations, 64-byte dk), and `is_valid_mnemonic_code`, with an English wordlist (other KMP wordlists deferred).

#### Scenario: BIP39 seed derivation matches known vectors

- **WHEN** a known mnemonic + passphrase is fed to `create_seed`
- **THEN** the 64-byte seed SHALL match the published BIP39 test vector

#### Scenario: Invalid mnemonic is rejected

- **WHEN** a mnemonic containing a word not in the wordlist is validated
- **THEN** `is_valid_mnemonic_code` SHALL return false and `create_seed` SHALL error with `crypto.mnemonic_invalid`

### Requirement: Ed25519 to X25519 conversion

The crate SHALL provide `ConvertEd25519` converting an Ed25519 private key to an X25519 private key (SHA-512 hash of the first 32 bytes + clamping), ported from the KMP `ConvertEd25519`.

#### Scenario: Ed25519 private key converts to a valid X25519 private key

- **WHEN** an Ed25519 private key is converted to X25519 and used in DH
- **THEN** the derived shared secret SHALL match the conversion-derived key's DH result against known conversion vectors

### Requirement: Feature-gated with all-on default

The crate SHALL expose cargo features `ed25519`, `x25519`, `secp256k1`, `secp256r1`, `hash`, `hex`, `base64`, `jwk`, `derivation`, with `default` enabling all of them (no `securerandom` feature — the `SecureRandom` port trait is zero-dependency and always available; no `wasm` feature — that is an `identus-adapters-entropy` feature, not a crypto one). Feature-to-dependency edges SHALL mirror neoprism's gating (e.g. `ed25519 = ["jwk","dep:ed25519-dalek"]`). A `wasm` feature (gating the `getrandom`-backed `SecureRandom` adapter) is **not** introduced in this change; it is deferred to the future TS/wasm binding change and lives in `identus-adapters-entropy`.

#### Scenario: Default features compile the full surface

- **WHEN** `cargo build -p identus-crypto` is run with default features
- **THEN** all curve, hashing, derivation, and `SecureRandom`-port modules SHALL compile and be available

#### Scenario: Default features compile on wasm32

- **WHEN** `cargo build -p identus-crypto --target wasm32-unknown-unknown` is run with default features
- **THEN** the build SHALL succeed (crypto has no `ring` dependency; the wasm limitation is confined to `identus-adapters-entropy`)

#### Scenario: A minimal feature subset compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features ed25519` is run
- **THEN** the build SHALL succeed with only the ed25519 (+jwk) surface

### Requirement: Layer conformance — depends only on identus-core

The crate SHALL declare `identus-core` as its only workspace-internal dependency and SHALL NOT depend on `identus-adapters`, `identus-bindings`, `identus-conformance`, or any `identus-adapters-<family>` crate (e.g. `identus-adapters-entropy`). It SHALL NOT depend on `ring` (the `ring` entropy adapter is in `identus-adapters-entropy`). All external crates SHALL be declared at workspace level per `workspace-dependency-conventions`. The `crate-ring-layout` conformance guard SHALL pass.

#### Scenario: crypto declares only identus-core inward

- **WHEN** `crates/crypto/Cargo.toml` `[dependencies]` is inspected for workspace-internal crates
- **THEN** it SHALL contain `identus-core` only

#### Scenario: crypto does not depend on an adapter-family crate

- **WHEN** `crates/crypto/Cargo.toml` is inspected
- **THEN** it SHALL NOT list `identus-adapters-entropy` (or any `identus-adapters-*`); the conformance guard SHALL reject such an edge

#### Scenario: Layer guard passes for crypto

- **WHEN** `cargo test -p identus-conformance` is run
- **THEN** the dep-graph guard SHALL pass for `identus-crypto` (no outward edges)

### Requirement: Workspace-level external dependency declaration

The crate's external dependencies (`ed25519-dalek`, `k256`, `p256`, `x25519-dalek`, `sha2`, `hmac`, `pbkdf2`, `base64`, `hex`) SHALL be declared in root `[workspace.dependencies]` and referenced via `<dep>.workspace = true` (per `workspace-dependency-conventions`); no inline external version pin SHALL appear in `crates/crypto/Cargo.toml`. The list SHALL NOT include `ring` — `ring` is declared by `identus-adapters-entropy` (the entropy adapter crate), not by `identus-crypto`.

#### Scenario: crypto external deps use the workspace form

- **WHEN** `crates/crypto/Cargo.toml` is inspected
- **THEN** every external dependency entry SHALL use `.workspace = true` and resolve to a root `[workspace.dependencies]` entry, and `ring` SHALL NOT be present

### Requirement: Wasm-clean by default

The crate SHALL build on `wasm32-unknown-unknown` with default features, because it has no `ring` dependency (the `ring` entropy adapter lives in `identus-adapters-entropy`). Only the entropy adapter crate is wasm-incompatible; the domain primitive crate is not.

#### Scenario: crypto builds on wasm32 with default features

- **WHEN** `cargo build -p identus-crypto --target wasm32-unknown-unknown` is run with default features
- **THEN** the build SHALL succeed

#### Scenario: only the adapter crate is wasm-incompatible

- **WHEN** `cargo build -p identus-adapters-entropy --features ring --target wasm32-unknown-unknown` is run
- **THEN** the build SHALL fail (the `ring` adapter is wasm-incompatible), while `identus-crypto` itself remains wasm-clean
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

The crate SHALL provide a `SecureRandom` trait (the only infrastructure port in the crate) with `generate_seed(num_bytes) -> Vec<u8>`, as the canonical entropy source for key generation and mnemonic creation. The crate SHALL define the **port only**; it SHALL NOT ship any concrete `SecureRandom` implementation in this crate. `SecureRandom` is a port because a known second backend exists: the concrete backends live in the outer-boundary `identus-adapters-entropy` crate — a `getrandom`-backed adapter (`GetrandomSystemRandomAdapter`, the cross-platform backend that builds on `wasm32-unknown-unknown` and is the canonical entropy source for the Kotlin/WASM bindings) and, where retained, a `ring`-backed adapter. (The `getrandom` adapter was previously described as "deferred to that future binding change"; it is now realized by the consumed `add-getrandom-entropy-adapter` change.) Key generation (`*KeyPair::generate`) and mnemonic creation (`MnemonicHelper::create_random_mnemonics`) SHALL take an injected `&mut impl SecureRandom` (dependency injection), since the domain crate cannot reach an outer-boundary adapter. No other primitive operation is exposed as a port: curve sign/verify, hashing, derivation, and conversion are concrete (see the "Concrete where backends don't vary" Purpose rule).

#### Scenario: SecureRandom generates the requested length

- **WHEN** `generate_seed(32)` is called on an adapter implementing the port
- **THEN** it SHALL return 32 bytes

#### Scenario: crypto declares no ring dependency

- **WHEN** `crates/crypto/Cargo.toml` is inspected
- **THEN** it SHALL NOT list `ring` as a dependency (concrete entropy adapters live in `identus-adapters-entropy`, never in `identus-crypto`)

#### Scenario: the getrandom adapter lives in identus-adapters-entropy

- **WHEN** `crates/adapters-entropy/` (behind its `getrandom` feature) is inspected
- **THEN** it SHALL provide the `GetrandomSystemRandomAdapter` implementing `identus_crypto::SecureRandom` via `getrandom`

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

The crate SHALL provide a `MnemonicHelper` with `create_random_mnemonics(rng: &mut impl SecureRandom)`, `create_seed(mnemonics, passphrase)` (PBKDF2-SHA512, 2048 iterations, 64-byte dk, salt = `"mnemonic" + passphrase"` — the standard BIP-39 salt), `create_random_seed(rng)` (a convenience that creates a random mnemonic and derives its seed with the default passphrase `""`), and `is_valid_mnemonic_code`, with an English wordlist (other KMP wordlists deferred). The crate SHALL additionally provide `create_seed_kmp(mnemonics, passphrase)` behind the `kmp-compat` feature, which derives a 64-byte seed using the **KMP salt** (`passphrase` with no `"mnemonic"` prefix) for one-way legacy PRISM wallet import; `create_seed_kmp` SHALL require an explicit passphrase (no default). Both `create_seed` and `create_seed_kmp` SHALL validate the mnemonic via `is_valid_mnemonic_code` and SHALL error with `Error::MnemonicInvalid` (`crypto.mnemonic_invalid`) on failure. No new `Error` variant or `ErrorCode` is introduced.

#### Scenario: BIP39 seed derivation matches known vectors

- **WHEN** a known mnemonic + passphrase is fed to `create_seed`
- **THEN** the 64-byte seed SHALL match the published BIP39 test vector (standard salt `"mnemonic" + passphrase`)

#### Scenario: Default passphrase is the standard empty string

- **WHEN** `create_random_seed(rng)` is called
- **THEN** the derived seed SHALL use salt `"mnemonic"` (passphrase `""`), NOT `"mnemonicAtalaPrism"` or any `"AtalaPrism"`-derived salt

#### Scenario: KMP-compat seed derivation matches KMP apollo

- **WHEN** a known mnemonic + passphrase is fed to `create_seed_kmp` (with `kmp-compat` enabled)
- **THEN** the 64-byte seed SHALL match the seed produced by KMP `apollo`'s `MnemonicHelper.createSeed` for the same inputs (salt = `passphrase` with no `"mnemonic"` prefix)

#### Scenario: KMP-compat seed matches cloud-agent legacy wallets

- **WHEN** `create_seed_kmp(words, "")` is called (with `kmp-compat` enabled)
- **THEN** the 64-byte seed SHALL match the seed `cloud-agent` produces via `MnemonicHelper.createSeed(words, "")` (salt = `""`)

#### Scenario: create_seed_kmp requires an explicit passphrase

- **WHEN** the `create_seed_kmp` signature is inspected
- **THEN** the `passphrase` parameter SHALL have no default value (callers must state the source wallet's passphrase explicitly)

#### Scenario: Invalid mnemonic is rejected by both variants

- **WHEN** a mnemonic containing a word not in the wordlist is validated
- **THEN** `is_valid_mnemonic_code` SHALL return false, and both `create_seed` and `create_seed_kmp` SHALL error with `crypto.mnemonic_invalid`

#### Scenario: create_seed_kmp is unavailable without the feature

- **WHEN** the crate is compiled without `--features kmp-compat`
- **THEN** `create_seed_kmp` SHALL NOT be present on `MnemonicHelper` (the KMP-interop surface is opt-in and invisible by default)

### Requirement: Ed25519 to X25519 conversion

The crate SHALL provide `ConvertEd25519` converting an Ed25519 private key to an X25519 private key (SHA-512 hash of the first 32 bytes + clamping), ported from the KMP `ConvertEd25519`.

#### Scenario: Ed25519 private key converts to a valid X25519 private key

- **WHEN** an Ed25519 private key is converted to X25519 and used in DH
- **THEN** the derived shared secret SHALL match the conversion-derived key's DH result against known conversion vectors

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

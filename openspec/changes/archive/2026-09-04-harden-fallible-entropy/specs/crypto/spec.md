## MODIFIED Requirements

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
`sign`, `verify`, and `EncodeJwk`, for parity with the KMP
`KMMEllipticCurve::SECP256r1` surface (a gap in neoprism's port). Generation
SHALL return immediately on provider failure and SHALL return
`Error::SecureRandomFailure` after at most sixteen successfully filled but
invalid scalar candidates.

#### Scenario: P-256 keypair generates, signs, and verifies

- **WHEN** a successful injected `SecureRandom` is passed to
  `P256KeyPair::generate(rng)` and the resulting key signs a message
- **THEN** generation SHALL return `Ok` and verification SHALL succeed against
  known P-256 test vectors

#### Scenario: P-256 random generation fails without panic

- **WHEN** entropy fails or sixteen filled candidates are invalid scalars
- **THEN** generation SHALL return `Error::SecureRandomFailure` without panic
  or exposing candidate bytes

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

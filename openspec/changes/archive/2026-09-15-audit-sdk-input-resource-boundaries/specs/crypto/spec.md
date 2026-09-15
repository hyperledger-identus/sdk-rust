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

The standalone crypto facade SHALL retain at most 32 extension members, 16
levels of extension JSON, 1,024 total extension JSON nodes, and 65,536
aggregate UTF-8 bytes across extension keys and string values. Native and serde
construction SHALL enforce the same budgets before retention and SHALL return
only a redacted JWK error without extension names or values. The enclosing JSON
transport/deserializer remains responsible for bounding allocation before the
typed facade receives the extension map.

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

#### Scenario: unknown bounded public extensions survive a round trip

- **WHEN** a valid JWK contains public extension members within all four budgets
- **THEN** deserialize/serialize SHALL preserve their JSON values while the
  crypto crate SHALL NOT interpret their policy

#### Scenario: each extension budget fails closed

- **WHEN** extension members, depth, nodes or aggregate key/string bytes are
  exactly at their respective ceiling
- **THEN** native and serde construction accept an otherwise valid JWK
- **WHEN** any one budget is exceeded
- **THEN** construction rejects before retention with no extension data in the error

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

Validation SHALL enforce BIP-39 English word counts and checksums, not word
membership alone. Entropy conversion SHALL accept exactly 16, 20, 24, 28 or 32
bytes and preserve its existing empty-vector failure shape for every other
length. Before dependency entry or allocating a joined sentence, validation
SHALL reject more than 24 words and any word longer than the longest adopted
English BIP-39 word. Before NFKD normalization or PBKDF2, both seed functions
SHALL reject a passphrase above 4,096 UTF-8 bytes with the same redacted
`Error::MnemonicInvalid` contract. Mnemonic sentences and standard passphrases
SHALL use NFKD before seed derivation. Any normalized text owned by the
implementation and every fixed seed temporary SHALL be zeroized on drop.
Dependency mnemonic/error/formatter types SHALL remain private.

The KMP path SHALL share strict normalized mnemonic validation and the
passphrase byte ceiling but SHALL keep the accepted legacy passphrase bytes and
unprefixed salt unchanged. Its local PBKDF2 mechanics SHALL remain isolated
behind `kmp-compat`; the standard path SHALL use the adopted BIP-39 engine.

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

#### Scenario: Oversized mnemonic input is rejected before joining

- **WHEN** more than 24 words or one word above the adopted English-word byte
  ceiling is passed to validation or either seed function
- **THEN** validation fails with the stable redacted mnemonic result before
  constructing a joined phrase or invoking the BIP-39 dependency

#### Scenario: Passphrase work has an exact SDK ceiling

- **WHEN** a passphrase contains exactly 4,096 UTF-8 bytes
- **THEN** standard and KMP-compatible derivation accept it when the mnemonic is valid
- **WHEN** it contains 4,097 UTF-8 bytes
- **THEN** both reject it before normalization or PBKDF2 without exposing the value

#### Scenario: create_seed_kmp is unavailable without the feature

- **WHEN** the crate is compiled without `--features kmp-compat`
- **THEN** `create_seed_kmp` SHALL NOT be present on `MnemonicHelper`

#### Scenario: Random mnemonic creation reports entropy failure

- **WHEN** the injected entropy provider fails
- **THEN** random mnemonic and random seed creation SHALL return
  `Error::SecureRandomFailure` without panic or mnemonic construction

#### Scenario: All standard entropy sizes map to exact word counts

- **WHEN** entropy of 16, 20, 24, 28 or 32 bytes is converted
- **THEN** the result SHALL contain 12, 15, 18, 21 or 24 valid English words,
  respectively

#### Scenario: Non-standard entropy fails before dependency work

- **WHEN** empty, undersized, oversized or non-32-bit-aligned entropy is
  converted through the existing infallible API
- **THEN** the result SHALL be an empty vector without invoking mnemonic construction

#### Scenario: Word count and checksum are validated

- **WHEN** empty, unsupported-count, unknown-word or checksum-invalid input is
  validated or used for either seed function
- **THEN** validation SHALL be false and derivation SHALL return only the
  stable redacted `crypto.mnemonic_invalid` error

#### Scenario: Standard Unicode passphrases are NFKD-equivalent

- **WHEN** canonically composed and decomposed representations of the same
  Unicode passphrase are used with a valid English mnemonic
- **THEN** `create_seed` SHALL return the same byte-exact published/independent
  BIP-39 seed and owned normalization buffers SHALL be zeroizing

#### Scenario: Dependency remains a private implementation detail

- **WHEN** the generated public API and feature-disabled graph are inspected
- **THEN** no `bip39`, `Mnemonic`, dependency error or dependency formatting
  type SHALL be public and the package SHALL be absent without derivation

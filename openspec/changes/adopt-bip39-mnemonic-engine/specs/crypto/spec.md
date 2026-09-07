## MODIFIED Requirements

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
length. Mnemonic sentences and standard passphrases SHALL use NFKD before seed
derivation. Any normalized text owned by the implementation and every fixed
seed temporary SHALL be zeroized on drop. Dependency mnemonic/error/formatter
types SHALL remain private.

The KMP path SHALL share strict normalized mnemonic validation but SHALL keep
the legacy passphrase bytes and unprefixed salt unchanged. Its local PBKDF2
mechanics SHALL remain isolated behind `kmp-compat`; the standard path SHALL use
the adopted BIP-39 engine.

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

#### Scenario: All standard entropy sizes map to exact word counts

- **WHEN** entropy of 16, 20, 24, 28 or 32 bytes is converted
- **THEN** the result SHALL contain 12, 15, 18, 21 or 24 valid English words,
  respectively

#### Scenario: Non-standard entropy fails without panic

- **WHEN** empty, undersized, oversized or non-32-bit-aligned entropy is
  converted through the existing infallible API
- **THEN** the result SHALL be an empty vector

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

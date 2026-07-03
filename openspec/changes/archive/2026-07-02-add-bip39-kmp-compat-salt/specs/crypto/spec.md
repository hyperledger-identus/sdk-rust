## MODIFIED Requirements

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

### Requirement: Feature-gated with all-on default

The crate SHALL expose cargo features `ed25519`, `x25519`, `secp256k1`, `secp256r1`, `hash`, `hex`, `base64`, `jwk`, `derivation`, and `kmp-compat`, with `default` enabling all of them **except** `kmp-compat` (no `securerandom` feature — the `SecureRandom` port trait is zero-dependency and always available; no `wasm` feature — that is an `identus-adapters-entropy` feature, not a crypto one). `kmp-compat` SHALL be opt-in (off by default) and SHALL gate the KMP-interop surface (`create_seed_kmp`); it is introduced as an empty gate (no new dependency) and is shared with the future ed25519-bip32 interop change, which will later extend it with `dep:ed25519-bip32`. Feature-to-dependency edges SHALL mirror neoprism's gating (e.g. `ed25519 = ["jwk","dep:ed25519-dalek"]`). A `wasm` feature (gating the `getrandom`-backed `SecureRandom` adapter) is **not** introduced in this change; it is deferred to the future TS/wasm binding change and lives in `identus-adapters-entropy`.

#### Scenario: Default features compile the full surface

- **WHEN** `cargo build -p identus-crypto` is run with default features
- **THEN** all curve, hashing, derivation, and `SecureRandom`-port modules SHALL compile and be available, and the KMP-interop surface (`create_seed_kmp`) SHALL NOT be present

#### Scenario: Default features compile on wasm32

- **WHEN** `cargo build -p identus-crypto --target wasm32-unknown-unknown` is run with default features
- **THEN** the build SHALL succeed (crypto has no `ring` dependency; the wasm limitation is confined to `identus-adapters-entropy`)

#### Scenario: A minimal feature subset compiles

- **WHEN** `cargo build -p identus-crypto --no-default-features --features ed25519` is run
- **THEN** the build SHALL succeed with only the ed25519 (+jwk) surface

#### Scenario: The kmp-compat feature is opt-in

- **WHEN** `cargo build -p identus-crypto --features kmp-compat` is run
- **THEN** the build SHALL succeed and `create_seed_kmp` SHALL be present on `MnemonicHelper`

#### Scenario: kmp-compat introduces no new external dependency

- **WHEN** the `kmp-compat` feature is enabled alone (`--no-default-features --features kmp-compat` plus any required base)
- **THEN** the build SHALL succeed without pulling any crate not already required by the default feature set (the feature is an empty gate reusing existing `pbkdf2`/`hmac`/`sha2`)
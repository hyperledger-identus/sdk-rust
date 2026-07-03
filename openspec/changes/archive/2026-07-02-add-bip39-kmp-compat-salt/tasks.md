## 1. Cargo feature gate

- [x] 1.1 Add the `kmp-compat = []` feature to `crates/crypto/Cargo.toml` (empty gate, no new dependency)
- [x] 1.2 Confirm `kmp-compat` is NOT in the `default` feature list
- [x] 1.3 Verify `cargo build -p identus-crypto` (default features) still compiles and does NOT expose `create_seed_kmp`

## 2. Canonical default passphrase fix

- [x] 2.1 Change `DEFAULT_PASSPHRASE` in `crates/crypto/src/derivation/mnemonic.rs` from `"AtalaPrism"` to `""`
- [x] 2.2 Update the `create_random_seed` doc comment to reflect the standard `""` default (salt `"mnemonic"`) instead of `"AtalaPrism"`
- [x] 2.3 Update any existing test asserting a `"mnemonicAtalaPrism"` / `"AtalaPrism"`-derived seed from `create_random_seed` to assert the standard empty-passphrase seed instead

## 3. Shared validation/PBKDF2 core

- [x] 3.1 Extract a private helper that performs mnemonic validation + PBKDF2-HMAC-SHA512 given a pre-built salt string, returning `Result<Vec<u8>, Error>`
- [x] 3.2 Refactor `create_seed` to build `salt = format!("{SALT_PREFIX}{passphrase}")` and delegate to the helper
- [x] 3.3 Confirm `create_seed` behavior is unchanged (existing BIP-39 test vectors still pass)

## 4. KMP-compat seed derivation

- [x] 4.1 Add `create_seed_kmp(mnemonics: &[String], passphrase: &str) -> Result<Vec<u8>, Error>` gated by `#[cfg(feature = "kmp-compat")]`, building `salt = passphrase.to_string()` (no `"mnemonic"` prefix) and delegating to the shared helper
- [x] 4.2 Add a doc comment stating: opt-in KMP-interop escape hatch for legacy PRISM wallet import; passphrase is required (no default); use `create_seed` for new wallets
- [x] 4.3 Confirm `create_seed_kmp` has no default passphrase parameter (explicit `&str` required)

## 5. Tests — canonical default

- [x] 5.1 Add/keep a test asserting `create_random_seed` derives with salt `"mnemonic"` (passphrase `""`), NOT `"mnemonicAtalaPrism"`
- [x] 5.2 Keep the published BIP-39 test-vector test for `create_seed` green (standard salt)

## 6. Tests — KMP-compat variant (feature-gated)

- [x] 6.1 Add a test (behind `#[cfg(feature = "kmp-compat")]`) asserting `create_seed_kmp` produces the same seed as KMP `apollo`'s `MnemonicHelper.createSeed` for a known mnemonic + passphrase (no `"mnemonic"` prefix in salt)
- [x] 6.2 Add a test asserting `create_seed_kmp(words, "")` matches the `cloud-agent` legacy path (salt `""`)
- [x] 6.3 Add a test asserting `create_seed_kmp` rejects an invalid mnemonic with `Error::MnemonicInvalid`
- [x] 6.4 Add a build assertion that `create_seed_kmp` is absent when `kmp-compat` is disabled — a `trybuild` `compile_fail` case (`tests/ui/kmp_create_seed_absent.rs` calling `create_seed_kmp`) driven by a `#[cfg(not(feature = "kmp-compat"))]` test in `derivation.rs`

## 7. Validation

- [x] 7.1 `cargo fmt` clean
- [x] 7.2 `cargo clippy` clean with and without `--features kmp-compat`
- [x] 7.3 `cargo test -p identus-crypto` green with default features
- [x] 7.4 `cargo test -p identus-crypto --features kmp-compat` green
- [x] 7.5 `cargo build -p identus-crypto --target wasm32-unknown-unknown` succeeds with default features (no new dep, wasm-clean)
- [x] 7.6 `openspec validate add-bip39-kmp-compat-salt` passes

## 8. CI enforcement of the `kmp-compat` feature

The Nix flake checks (`rust-test`, `rust-clippy`) run with default features
only, so the `#[cfg(feature = "kmp-compat")]` surface (`create_seed_kmp` +
its tests) is never compiled by `nix flake check`. Tasks 7.2/7.4 are satisfied
manually but not enforced. Add parallel, feature-scoped checks (NOT
`--all-features`, which would also pull `ring`/`deterministic` from
`identus-adapters-entropy` and lose the default-surface coverage).

- [x] 8.1 Add `nix/checks/rust-test-kmp-compat.nix` — `cargoNextest` with `cargoBuildFeatures = [ "kmp-compat" ]` (scoped to `identus-crypto`; no-op on crates that don't define the feature)
- [x] 8.2 Add `nix/checks/rust-clippy-kmp-compat.nix` — `cargoClippy` with the same feature, `-D warnings`
- [x] 8.3 Wire both into `nix/checks/default.nix`
- [x] 8.4 `nix build .#checks.x86_64-linux.rust-test-kmp-compat` succeeds and runs the `kmp_compat::*` tests
- [x] 8.5 `nix build .#checks.x86_64-linux.rust-clippy-kmp-compat` succeeds
- [x] 8.6 Confirm default-feature `rust-test`/`rust-clippy` still pass (default surface + `kmp_create_seed_is_absent_without_feature` coverage retained)
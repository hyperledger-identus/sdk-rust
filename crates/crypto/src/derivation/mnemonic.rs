//! Standards-correct BIP-39 mnemonic support behind the Identus facade.
//!
//! The dependency mnemonic is transient and never crosses this module. Its
//! word-revealing formatting surfaces are deliberately not exposed.

use std::borrow::Cow;

use bip39::{Language, Mnemonic};
#[cfg(feature = "kmp-compat")]
use pbkdf2::pbkdf2_hmac;
#[cfg(feature = "kmp-compat")]
use sha2::Sha512;
use zeroize::Zeroizing;

use crate::error::Error;
use crate::securerandom::SecureRandom;

#[cfg(feature = "kmp-compat")]
const PBKDF2_ITERATIONS: u32 = 2048;
#[cfg(feature = "kmp-compat")]
const PBKDF2_DK_LEN: usize = 64;
const DEFAULT_PASSPHRASE: &str = "";
const ENTROPY_BYTES_24_WORDS: usize = 32;

/// The standard BIP-39 English wordlist.
#[must_use]
pub fn wordlist() -> Vec<&'static str> {
    Language::English.word_list().to_vec()
}

/// BIP-39 mnemonic helper.
pub struct MnemonicHelper;

impl MnemonicHelper {
    /// Validate the BIP-39 English word count, word membership and checksum.
    #[must_use]
    pub fn is_valid_mnemonic_code(mnemonics: &[String]) -> bool {
        Self::parse_mnemonic(mnemonics).is_ok()
    }

    /// Create a random 24-word mnemonic using the injected [`SecureRandom`].
    pub fn create_random_mnemonics(rng: &mut impl SecureRandom) -> Result<Vec<String>, Error> {
        let mut entropy = Zeroizing::new([0u8; ENTROPY_BYTES_24_WORDS]);
        rng.fill_bytes(entropy.as_mut())?;
        Ok(Self::to_mnemonic_code(entropy.as_ref()))
    }

    /// Convert BIP-39 entropy into English mnemonic words.
    ///
    /// This signature remains infallible for compatibility. Entropy lengths
    /// other than 16, 20, 24, 28 or 32 bytes return an empty vector.
    #[must_use]
    pub fn to_mnemonic_code(entropy: &[u8]) -> Vec<String> {
        Mnemonic::from_entropy(entropy)
            .map(|mnemonic| mnemonic.words().map(str::to_owned).collect())
            .unwrap_or_default()
    }

    /// Derive a 64-byte standard BIP-39 seed.
    ///
    /// The mnemonic and passphrase are NFKD-normalized. Dependency errors are
    /// collapsed to the stable, redacted [`Error::MnemonicInvalid`] contract.
    pub fn create_seed(mnemonics: &[String], passphrase: &str) -> Result<Vec<u8>, Error> {
        let mnemonic = Self::parse_mnemonic(mnemonics)?;
        let seed = Self::with_normalized(passphrase, |normalized_passphrase| {
            Zeroizing::new(mnemonic.to_seed_normalized(normalized_passphrase))
        });
        Ok(seed.to_vec())
    }

    /// Derive a 64-byte seed using Apollo's legacy KMP salt.
    ///
    /// This opt-in import path deliberately uses the exact passphrase bytes as
    /// salt, with no `"mnemonic"` prefix and no passphrase normalization. Use
    /// [`create_seed`](Self::create_seed) for standards-compliant wallets.
    #[cfg(feature = "kmp-compat")]
    pub fn create_seed_kmp(mnemonics: &[String], passphrase: &str) -> Result<Vec<u8>, Error> {
        let mnemonic = Self::parse_mnemonic(mnemonics)?;
        let phrase = Zeroizing::new(mnemonic.words().collect::<Vec<_>>().join(" "));
        let mut seed = Zeroizing::new([0u8; PBKDF2_DK_LEN]);
        pbkdf2_hmac::<Sha512>(
            phrase.as_bytes(),
            passphrase.as_bytes(),
            PBKDF2_ITERATIONS,
            seed.as_mut(),
        );
        Ok(seed.to_vec())
    }

    /// Create a random mnemonic and derive its standard empty-passphrase seed.
    pub fn create_random_seed(rng: &mut impl SecureRandom) -> Result<Vec<u8>, Error> {
        let mnemonics = Zeroizing::new(Self::create_random_mnemonics(rng)?);
        Self::create_seed(&mnemonics, DEFAULT_PASSPHRASE)
    }

    fn parse_mnemonic(mnemonics: &[String]) -> Result<Mnemonic, Error> {
        let phrase = Zeroizing::new(mnemonics.join(" "));
        Self::with_normalized(&phrase, |normalized| {
            Mnemonic::parse_in_normalized(Language::English, normalized)
        })
        .map_err(|_| Error::MnemonicInvalid)
    }

    fn with_normalized<T>(input: &str, operation: impl FnOnce(&str) -> T) -> T {
        let mut normalized = Cow::Borrowed(input);
        Mnemonic::normalize_utf8_cow(&mut normalized);
        match normalized {
            Cow::Borrowed(value) => operation(value),
            Cow::Owned(value) => {
                let value = Zeroizing::new(value);
                operation(value.as_str())
            }
        }
    }
}

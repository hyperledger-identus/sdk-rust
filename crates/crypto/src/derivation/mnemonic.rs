//! BIP39 mnemonic helper (ported from the KMP `MnemonicHelper`), with an
//! English wordlist. `create_seed` uses standard BIP39 PBKDF2-HMAC-SHA512
//! (salt = `"mnemonic" + passphrase`) so it matches the published BIP39 test
//! vectors; `create_random_mnemonics` takes an injected [`SecureRandom`].

use pbkdf2::pbkdf2_hmac;
use sha2::Sha512;

use crate::error::Error;
use crate::securerandom::SecureRandom;

const PBKDF2_ITERATIONS: u32 = 2048;
const PBKDF2_DK_LEN: usize = 64;
const DEFAULT_PASSPHRASE: &str = "";
const SALT_PREFIX: &str = "mnemonic";
const ENTROPY_BYTES_24_WORDS: usize = 32;

/// The BIP39 English wordlist as a slice of words (2048 entries).
pub fn wordlist() -> Vec<&'static str> {
    super::wordlist::ENGLISH_WORDLIST.to_vec()
}

/// BIP39 mnemonic helper.
pub struct MnemonicHelper;

impl MnemonicHelper {
    /// Validate that every word in `mnemonics` is in the English wordlist.
    #[must_use]
    pub fn is_valid_mnemonic_code(mnemonics: &[String]) -> bool {
        let words = super::wordlist::ENGLISH_WORDLIST;
        mnemonics.iter().all(|w| words.contains(&w.as_str()))
    }

    /// Create a random 24-word mnemonic using the injected [`SecureRandom`].
    pub fn create_random_mnemonics(rng: &mut impl SecureRandom) -> Vec<String> {
        let entropy = rng.generate_seed(ENTROPY_BYTES_24_WORDS);
        Self::to_mnemonic_code(&entropy)
    }

    /// Convert raw entropy into a mnemonic word list (BIP39).
    pub fn to_mnemonic_code(entropy: &[u8]) -> Vec<String> {
        if entropy.is_empty() || entropy.len() % 4 != 0 {
            // Mirrors the KMP `Exception` on bad entropy length.
            return Vec::new();
        }
        let words = super::wordlist::ENGLISH_WORDLIST;
        let hash = crate::hash::sha256(entropy);
        let checksum_bits = entropy.len() / 4;
        let total_bits = entropy.len() * 8 + checksum_bits;

        let bit = |i: usize| -> bool {
            if i < entropy.len() * 8 {
                let byte = entropy[i / 8];
                (byte >> (7 - (i % 8))) & 1 == 1
            } else {
                let ci = i - entropy.len() * 8;
                let check = hash.as_array()[0] >> (8 - checksum_bits);
                (check >> (checksum_bits - 1 - ci)) & 1 == 1
            }
        };

        let nwords = total_bits / 11;
        let mut result = Vec::with_capacity(nwords);
        for w in 0..nwords {
            let mut index: usize = 0;
            for j in 0..11 {
                index <<= 1;
                if bit(w * 11 + j) {
                    index |= 1;
                }
            }
            result.push(words[index].to_string());
        }
        result
    }

    /// Derive a 64-byte BIP39 seed from `mnemonics` and `passphrase`
    /// (PBKDF2-HMAC-SHA512, salt = `"mnemonic" + passphrase`, 2048 iterations).
    /// Errors with [`Error::MnemonicInvalid`] if any word is not in the
    /// wordlist.
    pub fn create_seed(mnemonics: &[String], passphrase: &str) -> Result<Vec<u8>, Error> {
        Self::derive_seed(mnemonics, &format!("{SALT_PREFIX}{passphrase}"))
    }

    /// Derive a 64-byte seed from `mnemonics` and `passphrase` using the
    /// **KMP salt** (`passphrase` with no `"mnemonic"` prefix), for one-way
    /// legacy PRISM wallet import.
    ///
    /// This is an opt-in KMP-interop escape hatch gated behind the
    /// `kmp-compat` Cargo feature. The `passphrase` is **required** (no
    /// default) — state the source wallet's passphrase explicitly
    /// (`""` for `cloud-agent`, `"AtalaPrism"` for KMP-default, or the
    /// user's value). Use [`create_seed`](Self::create_seed) for new wallets.
    ///
    /// Errors with [`Error::MnemonicInvalid`] if any word is not in the
    /// wordlist.
    #[cfg(feature = "kmp-compat")]
    pub fn create_seed_kmp(mnemonics: &[String], passphrase: &str) -> Result<Vec<u8>, Error> {
        Self::derive_seed(mnemonics, passphrase)
    }

    /// Convenience: create a random mnemonic and derive its seed with the
    /// standard default passphrase (`""`, i.e. salt `"mnemonic"`).
    pub fn create_random_seed(rng: &mut impl SecureRandom) -> Vec<u8> {
        let mnemonics = Self::create_random_mnemonics(rng);
        Self::create_seed(&mnemonics, DEFAULT_PASSPHRASE)
            .expect("a freshly-created mnemonic is valid")
    }

    /// Shared validation + PBKDF2-HMAC-SHA512 core. The only per-variant input
    /// is the pre-built `salt` string; the mnemonic is validated and the seed
    /// is derived identically for both the standard and KMP salt variants.
    fn derive_seed(mnemonics: &[String], salt: &str) -> Result<Vec<u8>, Error> {
        if !Self::is_valid_mnemonic_code(mnemonics) {
            return Err(Error::MnemonicInvalid);
        }
        let mnemonic_string = mnemonics.join(" ");
        let mut dk = [0u8; PBKDF2_DK_LEN];
        pbkdf2_hmac::<Sha512>(
            mnemonic_string.as_bytes(),
            salt.as_bytes(),
            PBKDF2_ITERATIONS,
            &mut dk,
        );
        Ok(dk.to_vec())
    }
}

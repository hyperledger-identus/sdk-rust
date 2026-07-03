//! Derivation tests: BIP32 (secp256k1), SLIP-0010 (ed25519), BIP39, and
//! Ed25519→X25519 conversion — all against published known vectors.

mod common;

use std::str::FromStr;

use identus_crypto::Base64UrlStrNoPad;
use identus_crypto::convert::ConvertEd25519;
use identus_crypto::derivation::{EdHDKey, HDKey, MnemonicHelper};
use identus_crypto::ed25519::Ed25519PrivateKey;

const BIP32_SEED: [u8; 16] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
];

const SAMPLE_32: [u8; 32] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
];

// ---------------------------------------------------------------------------
// BIP32 secp256k1 — published BIP-32 test vector 1
// ---------------------------------------------------------------------------

#[test]
fn bip32_master_matches_published_vector() {
    let master = HDKey::init_from_seed(&BIP32_SEED).unwrap();
    assert_eq!(
        hex::encode(master.private_key),
        "e8f32e723decf4051aefac8e2c93c9c5b214313817cdb01a1494b917c8436b35"
    );
    assert_eq!(
        hex::encode(master.chain_code),
        "873dff81c02f525623fd1fe5167eac3a55a049de3d314bb42ee227ffed37d508"
    );
}

#[test]
fn bip32_derive_m0h_matches_published_vector() {
    let master = HDKey::init_from_seed(&BIP32_SEED).unwrap();
    let child = master.derive("m/0'").unwrap();
    assert_eq!(
        hex::encode(child.private_key),
        "edb2e14f9ee77d26dd93b4ecede8d16ed408ce149b6cd80b0715a2d911a0afea"
    );
    assert_eq!(
        hex::encode(child.chain_code),
        "47fdacbd0f1097043b78c63c20c34ef4ed9a111d980047ad16282c7ae6236141"
    );
}

#[test]
fn bip32_non_hardened_child_is_unsupported() {
    let master = HDKey::init_from_seed(&BIP32_SEED).unwrap();
    assert!(master.derive("m/0").is_err());
}

#[test]
fn bip32_invalid_path_is_rejected() {
    let master = HDKey::init_from_seed(&BIP32_SEED).unwrap();
    assert!(master.derive("x/0").is_err());
    assert!(master.derive("m/abc").is_err());
}

// ---------------------------------------------------------------------------
// KMP (apollo) compatibility — secp256k1 `HDKey` "prism" vector
// ---------------------------------------------------------------------------
// Vectors ported verbatim (as base64url-no-pad, matching the KMP encoding)
// from `apollo`'s `HDKeyTest.kt` `setup()`. The secp256k1 hardened-derivation
// path is shared with this crate's `HDKey`, so the "prism" vector must
// reproduce byte-for-byte — this locks in cross-implementation compatibility.
//
// KMP's Ed25519 `EdHDKey` (Khovratovich/Cardano, not SLIP-0010) and BIP39
// `MnemonicHelper` (non-standard PBKDF2 salt) vectors are intentionally not
// ported; both are documented divergences.
const KMP_PRISM_SEED_B64URL: &str =
    "e8uNN7LRH5mEUcxa7FhxDAgWGLh8P94WEOD0jUdaJ2mSU1o02u-Lzao50elV32XvYT0ux9jWuBVECpFAz2ckKw";
const KMP_PRISM_MASTER_PRIV_B64URL: &str = "96ViMAl0_N1Xm5RJesQxC2NvxhNc4ZkwPyVevZ4akDI";
const KMP_PRISM_M_0_0_0_PRIV_B64URL: &str = "xURclKhT6as1Tb9vg4AJRRLPAMWb9dYTTthDvXEKjMc";

#[test]
fn kmp_apollo_prism_master_matches() {
    let seed = Base64UrlStrNoPad::from_str(KMP_PRISM_SEED_B64URL)
        .unwrap()
        .to_bytes();
    let master = HDKey::init_from_seed(&seed).unwrap();
    assert_eq!(
        master.private_key.to_vec(),
        Base64UrlStrNoPad::from_str(KMP_PRISM_MASTER_PRIV_B64URL)
            .unwrap()
            .to_bytes(),
        "KMP prism master private key must match apollo HDKeyTest"
    );
}

#[test]
fn kmp_apollo_prism_derive_m_0h_0h_0h_matches() {
    let seed = Base64UrlStrNoPad::from_str(KMP_PRISM_SEED_B64URL)
        .unwrap()
        .to_bytes();
    let master = HDKey::init_from_seed(&seed).unwrap();
    let derived = master.derive("m/0'/0'/0'").unwrap();
    assert_eq!(
        derived.private_key.to_vec(),
        Base64UrlStrNoPad::from_str(KMP_PRISM_M_0_0_0_PRIV_B64URL)
            .unwrap()
            .to_bytes(),
        "KMP prism m/0'/0'/0' private key must match apollo HDKeyTest"
    );
}

// ---------------------------------------------------------------------------
// SLIP-0010 ed25519 — published SLIP-0010 ed25519 test vectors
// ---------------------------------------------------------------------------

#[test]
fn slip0010_master_matches_published_vector() {
    let master = EdHDKey::init_from_seed(&BIP32_SEED).unwrap();
    assert_eq!(
        hex::encode(master.private_key),
        "2b4be7f19ee27bbf30c667b642d5f4aa69fd169872f8fc3059c08ebae2eb19e7"
    );
    assert_eq!(
        hex::encode(master.chain_code),
        "90046a93de5380a72b5e45010748567d5ea02bbf6522f979e05c0d8d8ca9fffb"
    );
}

#[test]
fn slip0010_derive_m0h_matches_published_vector() {
    let master = EdHDKey::init_from_seed(&BIP32_SEED).unwrap();
    let child = master.derive("m/0'").unwrap();
    assert_eq!(
        hex::encode(child.private_key),
        "68e0fe46dfb67e368c75379acec591dad19df3cde26e63b93a8e704f1dade7a3"
    );
    assert_eq!(
        hex::encode(child.chain_code),
        "8b59aa11380b624e81507a27fedda59fea6d0b779a778918a2fd3590e16e9c69"
    );
}

#[test]
fn slip0010_non_hardened_child_is_unsupported() {
    let master = EdHDKey::init_from_seed(&BIP32_SEED).unwrap();
    assert!(master.derive("m/0").is_err());
}

// ---------------------------------------------------------------------------
// BIP39 — published BIP-39 test vector (abandon...about / TREZOR)
// ---------------------------------------------------------------------------

#[test]
fn bip39_create_seed_matches_published_vector() {
    let mnemonics: Vec<String> = [
        "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon",
        "abandon", "abandon", "abandon", "about",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let seed = MnemonicHelper::create_seed(&mnemonics, "TREZOR").unwrap();
    assert_eq!(
        hex::encode(&seed),
        "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04"
    );
}

#[test]
fn bip39_invalid_mnemonic_is_rejected() {
    let bad: Vec<String> = ["notaword", "alsobad"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert!(!MnemonicHelper::is_valid_mnemonic_code(&bad));
    let err = MnemonicHelper::create_seed(&bad, "pass").unwrap_err();
    assert_eq!(
        err.to_identus_error().code().as_str(),
        "crypto.mnemonic_invalid"
    );
}

#[test]
fn bip39_create_random_mnemonics_is_valid_and_deterministic() {
    use common::DetRandom;
    let mut rng = DetRandom::new();
    let words = MnemonicHelper::create_random_mnemonics(&mut rng);
    assert_eq!(words.len(), 24);
    assert!(MnemonicHelper::is_valid_mnemonic_code(&words));
    // Deterministic: same rng state reproduces the same words.
    let mut rng2 = DetRandom::new();
    let words2 = MnemonicHelper::create_random_mnemonics(&mut rng2);
    assert_eq!(words, words2);
    let _seed = MnemonicHelper::create_seed(&words, "").unwrap();
}

// ---------------------------------------------------------------------------
// BIP39 — `create_random_seed` uses the standard default passphrase ("")
// ---------------------------------------------------------------------------

#[test]
fn bip39_create_random_seed_uses_standard_empty_passphrase() {
    use common::DetRandom;
    // Two identical rng states: one to make the mnemonic, one to replay it
    // through `create_random_seed` (which consumes the same amount of rng).
    let mut rng_a = DetRandom::new();
    let mut rng_b = DetRandom::new();
    let words = MnemonicHelper::create_random_mnemonics(&mut rng_a);
    let from_convenience = MnemonicHelper::create_random_seed(&mut rng_b);
    // The convenience must derive with salt "mnemonic" (passphrase ""),
    // not the legacy "mnemonicAtalaPrism" hybrid.
    let standard = MnemonicHelper::create_seed(&words, "").unwrap();
    assert_eq!(from_convenience, standard);
    let legacy = MnemonicHelper::create_seed(&words, "AtalaPrism").unwrap();
    assert_ne!(from_convenience, legacy);
}

// ---------------------------------------------------------------------------
// KMP-compat seed derivation (`kmp-compat` feature) — legacy PRISM import
// ---------------------------------------------------------------------------
// KMP `apollo`'s `createSeed` uses salt = `passphrase` with NO `"mnemonic"`
// prefix (a KMP bug). `create_seed_kmp` reproduces that salt for one-way
// import of legacy PRISM wallets (KMP `apollo`, `cloud-agent`).
#[cfg(feature = "kmp-compat")]
mod kmp_compat {
    use super::*;

    const ABANDON_ABOUT: [&str; 12] = [
        "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon",
        "abandon", "abandon", "abandon", "about",
    ];

    fn words() -> Vec<String> {
        ABANDON_ABOUT.iter().map(|s| s.to_string()).collect()
    }

    // salt = "TREZOR" (no prefix) — matches KMP apollo `createSeed(words, "TREZOR")`.
    #[test]
    fn kmp_create_seed_matches_kmp_apollo_salt() {
        let seed = MnemonicHelper::create_seed_kmp(&words(), "TREZOR").unwrap();
        assert_eq!(
            hex::encode(&seed),
            "1a5996fb39022bfe53a809fce13981c7d76c1c4027ac5a8259ac865aa42dd7eb36054cfd297aaff20871ca22003a38c48515bb65b38f8a31664bfd22db8c842e"
        );
    }

    // salt = "" (no prefix) — matches cloud-agent legacy `createSeed(words, "")`.
    #[test]
    fn kmp_create_seed_matches_cloud_agent_empty_passphrase() {
        let seed = MnemonicHelper::create_seed_kmp(&words(), "").unwrap();
        assert_eq!(
            hex::encode(&seed),
            "a6b8f37ae15bbce7ffaf883dfae55ef847dd71816c4b510a554c95ccc291989e0a9c8e99455de874b71fee38ac262433ba15a58fe3851a2fbf14b424f3522fa3"
        );
    }

    #[test]
    fn kmp_create_seed_rejects_invalid_mnemonic() {
        let bad: Vec<String> = ["notaword", "alsobad"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let err = MnemonicHelper::create_seed_kmp(&bad, "").unwrap_err();
        assert_eq!(
            err.to_identus_error().code().as_str(),
            "crypto.mnemonic_invalid"
        );
    }
}

// Assert `create_seed_kmp` is absent without `kmp-compat`: a plain `#[test]`
// can't observe non-existence (referencing the symbol fails to compile when
// the gate is correct, not referencing it passes vacuously), so `trybuild`
// `compile_fail` treats the non-compilation as a passing assertion. The driver
// is `#[cfg(not(feature = "kmp-compat"))]` so it's skipped in the kmp-compat
// CI job (where the case would compile and `compile_fail` would unexpectedly
// succeed). Same pattern as `crates/derive/tests/trybuild.rs`.
#[cfg(not(feature = "kmp-compat"))]
#[test]
fn kmp_create_seed_is_absent_without_feature() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/kmp_create_seed_absent.rs");
}

// ---------------------------------------------------------------------------
// Ed25519 → X25519 conversion — known vector
// ---------------------------------------------------------------------------

#[test]
fn convert_ed25519_to_x25519_matches_known_vector() {
    let sk = Ed25519PrivateKey::from_slice(&SAMPLE_32).unwrap();
    let xsk = ConvertEd25519::convert_secret_key_to_x25519(sk.to_bytes().as_slice()).unwrap();
    assert_eq!(
        hex::encode(xsk.to_bytes()),
        "70788f1a0cea001a2631dae5d05dbd062008d5b30f50b9e29beb2a7822289044"
    );
}

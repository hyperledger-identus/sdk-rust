//! Derivation tests: BIP32 (secp256k1), SLIP-0010 (ed25519), BIP39, and
//! Ed25519→X25519 conversion — all against published known vectors.

mod common;

use std::str::FromStr;

use identus_crypto::Base64UrlStrNoPad;
use identus_crypto::convert::ConvertEd25519;
use identus_crypto::derivation::{EdHDKey, HDKey, MnemonicHelper};
use identus_crypto::ed25519::Ed25519PrivateKey;

const SAMPLE_32: [u8; 32] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
];

// Vector 1 seed (16 bytes) — the canonical BIP-32 test vector 1 seed.
const BIP32_SEED: [u8; 16] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
];

// Vector 2 seed (64 bytes).
const BIP32_V2_SEED: [u8; 64] = [
    0xff, 0xfc, 0xf9, 0xf6, 0xf3, 0xf0, 0xed, 0xea, 0xe7, 0xe4, 0xe1, 0xde, 0xdb, 0xd8, 0xd5, 0xd2,
    0xcf, 0xcc, 0xc9, 0xc6, 0xc3, 0xc0, 0xbd, 0xba, 0xb7, 0xb4, 0xb1, 0xae, 0xab, 0xa8, 0xa5, 0xa2,
    0x9f, 0x9c, 0x99, 0x96, 0x93, 0x90, 0x8d, 0x8a, 0x87, 0x84, 0x81, 0x7e, 0x7b, 0x78, 0x75, 0x72,
    0x6f, 0x6c, 0x69, 0x66, 0x63, 0x60, 0x5d, 0x5a, 0x57, 0x54, 0x51, 0x4e, 0x4b, 0x48, 0x45, 0x42,
];

// Vector 3 seed (64 bytes) — tests retention of leading zeros.
const BIP32_V3_SEED: [u8; 64] = [
    0x4b, 0x38, 0x15, 0x41, 0x58, 0x3b, 0xe4, 0x42, 0x33, 0x46, 0xc6, 0x43, 0x85, 0x0d, 0xa4, 0xb3,
    0x20, 0xe4, 0x6a, 0x87, 0xae, 0x3d, 0x2a, 0x4e, 0x6d, 0xa1, 0x1e, 0xba, 0x81, 0x9c, 0xd4, 0xac,
    0xba, 0x45, 0xd2, 0x39, 0x31, 0x9a, 0xc1, 0x4f, 0x86, 0x3b, 0x8d, 0x5a, 0xb5, 0xa0, 0xd0, 0xc6,
    0x4d, 0x2e, 0x8a, 0x1e, 0x7d, 0x14, 0x57, 0xdf, 0x2e, 0x5a, 0x3c, 0x51, 0xc7, 0x32, 0x35, 0xbe,
];

// Vector 4 seed (32 bytes) — tests retention of leading zeros.
const BIP32_V4_SEED: [u8; 32] = [
    0x3d, 0xdd, 0x56, 0x02, 0x28, 0x58, 0x99, 0xa9, 0x46, 0x11, 0x45, 0x06, 0x15, 0x7c, 0x79, 0x97,
    0xe5, 0x44, 0x45, 0x28, 0xf3, 0x00, 0x3f, 0x61, 0x34, 0x71, 0x21, 0x47, 0xdb, 0x19, 0xb6, 0x78,
];

// ---------------------------------------------------------------------------
// BIP32 secp256k1 — published BIP-32 test vectors (hardened-only paths)
// ---------------------------------------------------------------------------
// Our `HDKey` only supports hardened derivation (matching the KMP port), so we
// exercise every fully-hardened path from all four published BIP-32 vectors:
//   - Vector 1 (seed `000102...0e0f`): `m`, `m/0'` — deeper nodes mix in
//     non-hardened indices, which our API rejects (see
//     `bip32_non_hardened_child_is_unsupported`).
//   - Vector 2 (seed `fffcf9...4243`): `m` — the chain starts with `m/0`
//     (non-hardened), so only the master is reachable.
//   - Vector 3 (seed `4b3815...3235be`): `m`, `m/0'` — leading-zero retention.
//   - Vector 4 (seed `3ddd56...b678`): `m`, `m/0'`, `m/0'/1'` — leading-zero
//     retention in a hardened child.
// Vectors 2/3/4 use different seeds than vector 1, so an independent master +
// derivation check guards against a vector-1-only fluke (e.g. a seed whose
// master HMAC happens to coincide). Raw (priv, chain) values are decoded from
// the spec's base58check `xprv` strings. Vector 5 (invalid serialized keys) is
// about xprv/xpub parsing, not derivation, so it is out of scope for `HDKey`.

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

#[test]
fn bip32_v2_master_matches_published_vector() {
    let master = HDKey::init_from_seed(&BIP32_V2_SEED).unwrap();
    assert_eq!(
        hex::encode(master.private_key),
        "4b03d6fc340455b363f51020ad3ecca4f0850280cf436c70c727923f6db46c3e"
    );
    assert_eq!(
        hex::encode(master.chain_code),
        "60499f801b896d83179a4374aeb7822aaeaceaa0db1f85ee3e904c4defbd9689"
    );
}

#[test]
fn bip32_v3_master_and_m0h_match_published_vector() {
    let master = HDKey::init_from_seed(&BIP32_V3_SEED).unwrap();
    // Vector 3 master private key starts with 0x00 — checks leading-zero retention.
    assert_eq!(
        hex::encode(master.private_key),
        "00ddb80b067e0d4993197fe10f2657a844a384589847602d56f0c629c81aae32"
    );
    assert_eq!(
        hex::encode(master.chain_code),
        "01d28a3e53cffa419ec122c968b3259e16b65076495494d97cae10bbfec3c36f"
    );
    let child = master.derive("m/0'").unwrap();
    assert_eq!(
        hex::encode(child.private_key),
        "491f7a2eebc7b57028e0d3faa0acda02e75c33b03c48fb288c41e2ea44e1daef"
    );
    assert_eq!(
        hex::encode(child.chain_code),
        "e5fea12a97b927fc9dc3d2cb0d1ea1cf50aa5a1fdc1f933e8906bb38df3377bd"
    );
}

#[test]
fn bip32_v4_master_m0h_m0h1h_match_published_vector() {
    // Vector 4 is a fully-hardened multi-level path (m/0'/1') — the strongest
    // independent check that hardened child derivation is correct across seeds.
    let master = HDKey::init_from_seed(&BIP32_V4_SEED).unwrap();
    assert_eq!(
        hex::encode(master.private_key),
        "12c0d59c7aa3a10973dbd3f478b65f2516627e3fe61e00c345be9a477ad2e215"
    );
    assert_eq!(
        hex::encode(master.chain_code),
        "d0c8a1f6edf2500798c3e0b54f1b56e45f6d03e6076abd36e5e2f54101e44ce6"
    );
    let m0h = master.derive("m/0'").unwrap();
    // m/0' private key starts with 0x00 — checks leading-zero retention in a child.
    assert_eq!(
        hex::encode(m0h.private_key),
        "00d948e9261e41362a688b916f297121ba6bfb2274a3575ac0e456551dfd7f7e"
    );
    assert_eq!(
        hex::encode(m0h.chain_code),
        "cdc0f06456a14876c898790e0b3b1a41c531170aec69da44ff7b7265bfe7743b"
    );
    // Derive the full path from the master (not from `m0h`): `derive` re-applies
    // the whole path from `self`, so `m0h.derive("m/0'/1'")` would yield
    // m/0'/0'/1'. Use the master as the anchor for multi-level paths.
    let m0h1h = master.derive("m/0'/1'").unwrap();
    assert_eq!(
        hex::encode(m0h1h.private_key),
        "3a2086edd7d9df86c3487a5905a1712a9aa664bce8cc268141e07549eaa8661d"
    );
    assert_eq!(
        hex::encode(m0h1h.chain_code),
        "a48ee6674c5264a237703fd383bccd9fad4d9378ac98ab05e6e7029b06360c0d"
    );
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
// SLIP-0010 publishes two ed25519 vectors (seed `000102...0e0f` and
// `fffcf9...4243`), each a 5-level hardened chain. ed25519 is hardened-only,
// so every published node is reachable. Below, every published (priv, chain)
// node along both chains is asserted — not just the master and the final deep
// node — so a regression at any depth is caught.

fn assert_edhd_node(master: &EdHDKey, path: &str, priv_hex: &str, chain_hex: &str) {
    let node = master.derive(path).unwrap();
    assert_eq!(
        hex::encode(node.private_key),
        priv_hex,
        "SLIP-0010 {path} private key must match the published vector"
    );
    assert_eq!(
        hex::encode(node.chain_code),
        chain_hex,
        "SLIP-0010 {path} chain code must match the published vector"
    );
}

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

// Vector 1 (seed `000102...0e0f`): every published node along
// m/0'/1'/2'/2'/1000000000'.
#[test]
fn slip0010_v1_full_chain_matches_published_vector_at_every_depth() {
    let master = EdHDKey::init_from_seed(&BIP32_SEED).unwrap();
    assert_edhd_node(
        &master,
        "m/0'/1'",
        "b1d0bad404bf35da785a64ca1ac54b2617211d2777696fbffaf208f746ae84f2",
        "a320425f77d1b5c2505a6b1b27382b37368ee640e3557c315416801243552f14",
    );
    assert_edhd_node(
        &master,
        "m/0'/1'/2'",
        "92a5b23c0b8a99e37d07df3fb9966917f5d06e02ddbd909c7e184371463e9fc9",
        "2e69929e00b5ab250f49c3fb1c12f252de4fed2c1db88387094a0f8c4c9ccd6c",
    );
    assert_edhd_node(
        &master,
        "m/0'/1'/2'/2'",
        "30d1dc7e5fc04c31219ab25a27ae00b50f6fd66622f6e9c913253d6511d1e662",
        "8f6d87f93d750e0efccda017d662a1b31a266e4a6f5993b15f5c1f07f74dd5cc",
    );
    assert_edhd_node(
        &master,
        "m/0'/1'/2'/2'/1000000000'",
        "8f94d394a8e8fd6b1bc2f3f49f5c47e385281d5c17e65324b0f62483e37e8793",
        "68789923a0cac2cd5a29172a475fe9e0fb14cd6adb5ad98a3fa70333e7afa230",
    );
}

// Vector 2 (seed `fffcf9...4243`): master + every published node along
// m/0'/2147483647'/1'/2147483646'/2' — an independent cross-seed check.
#[test]
fn slip0010_v2_full_chain_matches_published_vector_at_every_depth() {
    let master = EdHDKey::init_from_seed(&BIP32_V2_SEED).unwrap();
    assert_eq!(
        hex::encode(master.private_key),
        "171cb88b1b3c1db25add599712e36245d75bc65a1a5c9e18d76f9f2b1eab4012"
    );
    assert_eq!(
        hex::encode(master.chain_code),
        "ef70a74db9c3a5af931b5fe73ed8e1a53464133654fd55e7a66f8570b8e33c3b"
    );
    assert_edhd_node(
        &master,
        "m/0'",
        "1559eb2bbec5790b0c65d8693e4d0875b1747f4970ae8b650486ed7470845635",
        "0b78a3226f915c082bf118f83618a618ab6dec793752624cbeb622acb562862d",
    );
    assert_edhd_node(
        &master,
        "m/0'/2147483647'",
        "ea4f5bfe8694d8bb74b7b59404632fd5968b774ed545e810de9c32a4fb4192f4",
        "138f0b2551bcafeca6ff2aa88ba8ed0ed8de070841f0c4ef0165df8181eaad7f",
    );
    assert_edhd_node(
        &master,
        "m/0'/2147483647'/1'",
        "3757c7577170179c7868353ada796c839135b3d30554bbb74a4b1e4a5a58505c",
        "73bd9fff1cfbde33a1b846c27085f711c0fe2d66fd32e139d3ebc28e5a4a6b90",
    );
    assert_edhd_node(
        &master,
        "m/0'/2147483647'/1'/2147483646'",
        "5837736c89570de861ebc173b1086da4f505d4adb387c6a1b1342d5e4ac9ec72",
        "0902fe8a29f9140480a00ef244bd183e8a13288e4412d8389d140aac1794825a",
    );
    assert_edhd_node(
        &master,
        "m/0'/2147483647'/1'/2147483646'/2'",
        "551d333177df541ad876a60ea71f00447931c0a9da16f227c11ea080d7391b8d",
        "5d70af781f3a37b829f0d060924d5e960bdc02e85423494afc0b1a41bbe196d4",
    );
}

// ---------------------------------------------------------------------------
// BIP39 — published BIP-39 test vectors (passphrase "TREZOR")
// ---------------------------------------------------------------------------
// The BIP-39 spec references the canonical 24-vector English suite from
// trezor/python-mnemonic `vectors.json`, all derived with passphrase "TREZOR"
// and the standard salt `"mnemonic" + passphrase`. Vector 1 (abandon...about)
// is asserted individually above as a named anchor; the test below asserts
// every published vector so any wordlist/entropy-length/PBKDF2 regression is
// caught (entropy lengths 128/160/192/224/256, leading-zero retention, etc.).

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

// The full published English suite (trezor/python-mnemonic `vectors.json`),
// passphrase "TREZOR" for every entry. Covers entropy lengths 128/160/192/
// 224/256, all-zero / all-0xff / mixed entropy, and 12/15/18/21/24-word
// mnemonics. Raw (entropy, mnemonic, seed) tuples are copied verbatim.
#[test]
fn bip39_create_seed_matches_all_published_trezor_vectors() {
    let vectors: &[(&str, &str, &str)] = &[
        (
            "00000000000000000000000000000000",
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04",
        ),
        (
            "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
            "legal winner thank year wave sausage worth useful legal winner thank yellow",
            "2e8905819b8723fe2c1d161860e5ee1830318dbf49a83bd451cfb8440c28bd6fa457fe1296106559a3c80937a1c1069be3a3a5bd381ee6260e8d9739fce1f607",
        ),
        (
            "80808080808080808080808080808080",
            "letter advice cage absurd amount doctor acoustic avoid letter advice cage above",
            "d71de856f81a8acc65e6fc851a38d4d7ec216fd0796d0a6827a3ad6ed5511a30fa280f12eb2e47ed2ac03b5c462a0358d18d69fe4f985ec81778c1b370b652a8",
        ),
        (
            "ffffffffffffffffffffffffffffffff",
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
            "ac27495480225222079d7be181583751e86f571027b0497b5b5d11218e0a8a13332572917f0f8e5a589620c6f15b11c61dee327651a14c34e18231052e48c069",
        ),
        (
            "000000000000000000000000000000000000000000000000",
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon agent",
            "035895f2f481b1b0f01fcf8c289c794660b289981a78f8106447707fdd9666ca06da5a9a565181599b79f53b844d8a71dd9f439c52a3d7b3e8a79c906ac845fa",
        ),
        (
            "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
            "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal will",
            "f2b94508732bcbacbcc020faefecfc89feafa6649a5491b8c952cede496c214a0c7b3c392d168748f2d4a612bada0753b52a1c7ac53c1e93abd5c6320b9e95dd",
        ),
        (
            "808080808080808080808080808080808080808080808080",
            "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter always",
            "107d7c02a5aa6f38c58083ff74f04c607c2d2c0ecc55501dadd72d025b751bc27fe913ffb796f841c49b1d33b610cf0e91d3aa239027f5e99fe4ce9e5088cd65",
        ),
        (
            "ffffffffffffffffffffffffffffffffffffffffffffffff",
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo when",
            "0cd6e5d827bb62eb8fc1e262254223817fd068a74b5b449cc2f667c3f1f985a76379b43348d952e2265b4cd129090758b3e3c2c49103b5051aac2eaeb890a528",
        ),
        (
            "0000000000000000000000000000000000000000000000000000000000000000",
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art",
            "bda85446c68413707090a52022edd26a1c9462295029f2e60cd7c4f2bbd3097170af7a4d73245cafa9c3cca8d561a7c3de6f5d4a10be8ed2a5e608d68f92fcc8",
        ),
        (
            "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
            "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth title",
            "bc09fca1804f7e69da93c2f2028eb238c227f2e9dda30cd63699232578480a4021b146ad717fbb7e451ce9eb835f43620bf5c514db0f8add49f5d121449d3e87",
        ),
        (
            "8080808080808080808080808080808080808080808080808080808080808080",
            "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic bless",
            "c0c519bd0e91a2ed54357d9d1ebef6f5af218a153624cf4f2da911a0ed8f7a09e2ef61af0aca007096df430022f7a2b6fb91661a9589097069720d015e4e982f",
        ),
        (
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote",
            "dd48c104698c30cfe2b6142103248622fb7bb0ff692eebb00089b32d22484e1613912f0a5b694407be899ffd31ed3992c456cdf60f5d4564b8ba3f05a69890ad",
        ),
        (
            "9e885d952ad362caeb4efe34a8e91bd2",
            "ozone drill grab fiber curtain grace pudding thank cruise elder eight picnic",
            "274ddc525802f7c828d8ef7ddbcdc5304e87ac3535913611fbbfa986d0c9e5476c91689f9c8a54fd55bd38606aa6a8595ad213d4c9c9f9aca3fb217069a41028",
        ),
        (
            "6610b25967cdcca9d59875f5cb50b0ea75433311869e930b",
            "gravity machine north sort system female filter attitude volume fold club stay feature office ecology stable narrow fog",
            "628c3827a8823298ee685db84f55caa34b5cc195a778e52d45f59bcf75aba68e4d7590e101dc414bc1bbd5737666fbbef35d1f1903953b66624f910feef245ac",
        ),
        (
            "68a79eaca2324873eacc50cb9c6eca8cc68ea5d936f98787c60c7ebc74e6ce7c",
            "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length",
            "64c87cde7e12ecf6704ab95bb1408bef047c22db4cc7491c4271d170a1b213d20b385bc1588d9c7b38f1b39d415665b8a9030c9ec653d75e65f847d8fc1fc440",
        ),
        (
            "c0ba5a8e914111210f2bd131f3d5e08d",
            "scheme spot photo card baby mountain device kick cradle pact join borrow",
            "ea725895aaae8d4c1cf682c1bfd2d358d52ed9f0f0591131b559e2724bb234fca05aa9c02c57407e04ee9dc3b454aa63fbff483a8b11de949624b9f1831a9612",
        ),
        (
            "6d9be1ee6ebd27a258115aad99b7317b9c8d28b6d76431c3",
            "horn tenant knee talent sponsor spell gate clip pulse soap slush warm silver nephew swap uncle crack brave",
            "fd579828af3da1d32544ce4db5c73d53fc8acc4ddb1e3b251a31179cdb71e853c56d2fcb11aed39898ce6c34b10b5382772db8796e52837b54468aeb312cfc3d",
        ),
        (
            "9f6a2878b2520799a44ef18bc7df394e7061a224d2c33cd015b157d746869863",
            "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside",
            "72be8e052fc4919d2adf28d5306b5474b0069df35b02303de8c1729c9538dbb6fc2d731d5f832193cd9fb6aeecbc469594a70e3dd50811b5067f3b88b28c3e8d",
        ),
        (
            "23db8160a31d3e0dca3688ed941adbf3",
            "cat swing flag economy stadium alone churn speed unique patch report train",
            "deb5f45449e615feff5640f2e49f933ff51895de3b4381832b3139941c57b59205a42480c52175b6efcffaa58a2503887c1e8b363a707256bdd2b587b46541f5",
        ),
        (
            "8197a4a47f0425faeaa69deebc05ca29c0a5b5cc76ceacc0",
            "light rule cinnamon wrap drastic word pride squirrel upgrade then income fatal apart sustain crack supply proud access",
            "4cbdff1ca2db800fd61cae72a57475fdc6bab03e441fd63f96dabd1f183ef5b782925f00105f318309a7e9c3ea6967c7801e46c8a58082674c860a37b93eda02",
        ),
        (
            "066dca1a2bb7e8a1db2832148ce9933eea0f3ac9548d793112d9a95c9407efad",
            "all hour make first leader extend hole alien behind guard gospel lava path output census museum junior mass reopen famous sing advance salt reform",
            "26e975ec644423f4a4c4f4215ef09b4bd7ef924e85d1d17c4cf3f136c2863cf6df0a475045652c57eb5fb41513ca2a2d67722b77e954b4b3fc11f7590449191d",
        ),
        (
            "f30f8c1da665478f49b001d94c5fc452",
            "vessel ladder alter error federal sibling chat ability sun glass valve picture",
            "2aaa9242daafcee6aa9d7269f17d4efe271e1b9a529178d7dc139cd18747090bf9d60295d0ce74309a78852a9caadf0af48aae1c6253839624076224374bc63f",
        ),
        (
            "c10ec20dc3cd9f652c7fac2f1230f7a3c828389a14392f05",
            "scissors invite lock maple supreme raw rapid void congress muscle digital elegant little brisk hair mango congress clump",
            "7b4a10be9d98e6cba265566db7f136718e1398c71cb581e1b2f464cac1ceedf4f3e274dc270003c670ad8d02c4558b2f8e39edea2775c9e232c7cb798b069e88",
        ),
        (
            "f585c11aec520db57dd353c69554b21a89b20fb0650966fa0a9d6f74fd989d8f",
            "void come effort suffer camp survey warrior heavy shoot primary clutch crush open amazing screen patrol group space point ten exist slush involve unfold",
            "01f5bced59dec48e362f2c45b5de68b9fd6c92c6634f44d6d40aab69056506f0e35524a518034ddc1192e1dacd32c1ed3eaa3c3b131c88ed8e7e54c49a5d0998",
        ),
    ];

    for (entropy, mnemonic, seed_hex) in vectors {
        let words: Vec<String> = mnemonic.split(' ').map(|s| s.to_string()).collect();
        let seed = MnemonicHelper::create_seed(&words, "TREZOR")
            .unwrap_or_else(|e| panic!("create_seed failed for entropy {entropy}: {e}"));
        assert_eq!(
            hex::encode(&seed),
            *seed_hex,
            "BIP-39 seed mismatch for entropy {entropy}"
        );
    }
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

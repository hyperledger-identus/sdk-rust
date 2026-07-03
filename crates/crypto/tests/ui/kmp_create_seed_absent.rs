use identus_crypto::derivation::MnemonicHelper;

// `create_seed_kmp` must NOT exist on the canonical surface (no `kmp-compat`
// feature). If this compiles, the `#[cfg(feature = "kmp-compat")]` gate on
// `create_seed_kmp` was dropped and the KMP quirk leaked onto the default
// surface — the trybuild `compile_fail` in `derivation.rs` will fail.
fn _x(words: &[String], passphrase: &str) {
    let _ = MnemonicHelper::create_seed_kmp(words, passphrase);
}

fn main() {}

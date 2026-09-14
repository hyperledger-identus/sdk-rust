use identus_crypto::derivation::HDKey;

fn main() {
    let key = HDKey::init_from_seed(&[0x42; 32]).unwrap();
    let secret = key.expose_private_key();
    let _ = secret.clone();
}

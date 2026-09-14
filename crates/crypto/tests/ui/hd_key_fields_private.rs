use identus_crypto::derivation::{EdHDKey, HDKey};

fn inspect(hd: &HDKey, ed: &EdHDKey) {
    let _ = hd.private_key;
    let _ = hd.chain_code;
    let _ = ed.private_key;
    let _ = ed.chain_code;
}

fn main() {}

use identus_crypto::Base64UrlStrNoPad;

fn main() {
    let _ = Base64UrlStrNoPad::from(&[0_u8][..]);
}

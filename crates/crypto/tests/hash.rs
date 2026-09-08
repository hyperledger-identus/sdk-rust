use identus_crypto::{sha256, sha512};

#[test]
fn sha2_empty_message_known_answers() {
    assert_eq!(
        hex::encode(sha256([]).as_array()),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        hex::encode(sha512([]).as_array()),
        concat!(
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce",
            "47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
        )
    );
}

#[cfg(feature = "derivation")]
#[test]
fn hmac_sha512_matches_rfc4231_case_1() {
    let actual = identus_crypto::hash::hmac_sha512(&[0x0b; 20], b"Hi There");
    assert_eq!(
        hex::encode(actual),
        concat!(
            "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cde",
            "daa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854"
        )
    );
}

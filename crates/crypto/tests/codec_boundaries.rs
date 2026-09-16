use std::str::FromStr;

use identus_core::{CapabilityId, ErrorKind};
use identus_crypto::{Base64UrlStrNoPad, HexStr, MAX_CRYPTO_TEXT_BYTES};

#[test]
fn hex_byte_construction_is_exact_bounded_and_canonical() {
    let exact = vec![0xab; MAX_CRYPTO_TEXT_BYTES / 2];
    let encoded = HexStr::try_from_bytes(&exact).expect("exact hex byte boundary");
    assert_eq!(encoded.as_str(), "ab".repeat(exact.len()));
    assert_eq!(encoded.to_bytes(), exact);

    let parsed = HexStr::from_str(encoded.as_str()).expect("canonical hex text");
    assert_eq!(parsed, encoded);

    let rejected = HexStr::try_from_bytes(vec![0x5a; MAX_CRYPTO_TEXT_BYTES / 2 + 1])
        .expect_err("one byte above the hex boundary");
    assert_eq!(
        rejected.to_string(),
        "unable to parse key: hex input exceeds the 4096-byte limit (got 4098 bytes)"
    );
    assert_static_bridge(rejected);
}

#[test]
fn base64url_byte_construction_is_exact_bounded_and_canonical() {
    let exact = vec![0u8; (MAX_CRYPTO_TEXT_BYTES / 4) * 3];
    let encoded = Base64UrlStrNoPad::try_from_bytes(&exact).expect("exact base64url byte boundary");
    assert_eq!(encoded.as_str().len(), MAX_CRYPTO_TEXT_BYTES);
    assert_eq!(encoded.to_bytes(), exact);

    let parsed = Base64UrlStrNoPad::from_str(encoded.as_str()).expect("canonical base64url text");
    assert_eq!(parsed, encoded);

    let rejected =
        Base64UrlStrNoPad::try_from_bytes(vec![0x5a; (MAX_CRYPTO_TEXT_BYTES / 4) * 3 + 1])
            .expect_err("one byte above the base64url boundary");
    assert_eq!(
        rejected.to_string(),
        "unable to parse key: base64url input exceeds the 4096-byte limit (got 4098 bytes)"
    );
    assert_static_bridge(rejected);
}

#[test]
fn every_public_byte_ownership_form_uses_the_same_checked_boundary() {
    let bytes = [0x66, 0x6f, 0x6f];

    let hex = [
        HexStr::try_from(bytes.as_slice()).expect("hex slice"),
        HexStr::try_from(bytes.to_vec()).expect("hex vector"),
        HexStr::try_from(bytes).expect("hex array"),
        HexStr::try_from(&bytes).expect("hex borrowed array"),
    ];
    assert!(hex.iter().all(|value| value.as_str() == "666f6f"));

    let base64url = [
        Base64UrlStrNoPad::try_from(bytes.as_slice()).expect("base64url slice"),
        Base64UrlStrNoPad::try_from(bytes.to_vec()).expect("base64url vector"),
        Base64UrlStrNoPad::try_from(bytes).expect("base64url array"),
        Base64UrlStrNoPad::try_from(&bytes).expect("base64url borrowed array"),
    ];
    assert!(base64url.iter().all(|value| value.as_str() == "Zm9v"));
}

#[test]
fn removed_infallible_codec_conversions_stay_absent() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/hex_infallible_from_absent.rs");
    tests.compile_fail("tests/ui/base64url_infallible_from_absent.rs");
}

fn assert_static_bridge(error: identus_crypto::Error) {
    let bridged = error.to_identus_error();
    assert_eq!(bridged.code().as_str(), "crypto.key_parsing");
    assert_eq!(bridged.kind(), ErrorKind::InvalidInput);
    assert_eq!(bridged.capability(), Some(CapabilityId::new("crypto")));
    assert_eq!(
        bridged.to_string(),
        "crypto.key_parsing: key parsing failed"
    );
}

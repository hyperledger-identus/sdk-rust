#![cfg(feature = "cose")]

use std::time::Instant;

use identus_core::{CapabilityId, ErrorKind};
use identus_crypto::{
    CoseCoordinate, CoseCurve, CoseEcY, CoseKeyError, CoseKeyType, MAX_COSE_ADDITIONAL_PARAMETERS,
    MAX_COSE_KEY_BYTES, PublicKeyCose,
};

const X: [u8; 32] = [0x11; 32];
const Y: [u8; 32] = [0x22; 32];
const PRIVATE_SENTINEL: &[u8] = b"never-print-private-cbor";

fn push_uint(encoded: &mut Vec<u8>, major: u8, value: u64) {
    let prefix = major << 5;
    match value {
        0..=23 => encoded.push(prefix | value as u8),
        24..=255 => encoded.extend([prefix | 24, value as u8]),
        256..=65_535 => {
            encoded.push(prefix | 25);
            encoded.extend((value as u16).to_be_bytes());
        }
        _ => panic!("test helper only encodes small unsigned values"),
    }
}

fn push_map_len(encoded: &mut Vec<u8>, len: usize) {
    push_uint(encoded, 5, len as u64);
}

fn push_bytes(encoded: &mut Vec<u8>, value: &[u8]) {
    push_uint(encoded, 2, value.len() as u64);
    encoded.extend(value);
}

fn okp_fixture(curve: u8, x: &[u8]) -> Vec<u8> {
    let mut encoded = vec![0xa3, 0x01, 0x01, 0x20, curve, 0x21];
    push_bytes(&mut encoded, x);
    encoded
}

fn ec_fixture(curve: u8, x: &[u8], y: CoseEcY) -> Vec<u8> {
    let mut encoded = vec![0xa4, 0x01, 0x02, 0x20, curve, 0x21];
    push_bytes(&mut encoded, x);
    encoded.push(0x22);
    match y {
        CoseEcY::Coordinate(value) => push_bytes(&mut encoded, &value),
        CoseEcY::Sign(false) => encoded.push(0xf4),
        CoseEcY::Sign(true) => encoded.push(0xf5),
    }
    encoded
}

#[test]
fn assigned_okp_and_ec2_profiles_match_registered_values() {
    for (curve, assigned) in [(CoseCurve::Ed25519, 6), (CoseCurve::X25519, 4)] {
        let parsed = PublicKeyCose::from_cbor(&okp_fixture(assigned, &X)).expect("valid OKP");
        assert_eq!(parsed.key_type(), CoseKeyType::Okp);
        assert_eq!(parsed.curve(), curve);
        assert_eq!(parsed.x(), &X);
        assert_eq!(parsed.y(), None);
    }

    for (curve, assigned) in [(CoseCurve::P256, 1), (CoseCurve::Secp256k1, 8)] {
        let encoded = ec_fixture(assigned, &X, CoseEcY::Coordinate(Y));
        let parsed = PublicKeyCose::from_cbor(&encoded).expect("valid EC2");
        assert_eq!(parsed.key_type(), CoseKeyType::Ec2);
        assert_eq!(parsed.curve(), curve);
        assert_eq!(parsed.x(), &X);
        assert_eq!(parsed.y(), Some(CoseEcY::Coordinate(Y)));
    }
}

#[test]
fn registered_text_profile_normalizes_to_assigned_integers() {
    let mut encoded = vec![0xa3, 0x01, 0x63];
    encoded.extend(b"OKP");
    encoded.extend([0x20, 0x67]);
    encoded.extend(b"Ed25519");
    encoded.push(0x21);
    push_bytes(&mut encoded, &X);

    let parsed = PublicKeyCose::from_cbor(&encoded).expect("registered text profile");
    assert_eq!(parsed.key_type(), CoseKeyType::Okp);
    assert_eq!(parsed.curve(), CoseCurve::Ed25519);
    assert_eq!(parsed.to_cbor().expect("deterministic"), okp_fixture(6, &X));
}

#[test]
fn compressed_ec2_sign_is_preserved() {
    for sign in [false, true] {
        let expected = ec_fixture(1, &X, CoseEcY::Sign(sign));
        let parsed = PublicKeyCose::from_cbor(&expected).expect("compressed EC2");
        assert_eq!(parsed.y(), Some(CoseEcY::Sign(sign)));
        assert_eq!(parsed.to_cbor().expect("deterministic"), expected);
    }
}

#[test]
fn deterministic_encoding_sorts_and_retains_public_extensions() {
    let mut input = Vec::new();
    push_map_len(&mut input, 5);
    input.extend([0x18, 0x2a, 0xa2, 0x02, 0xf6, 0x01, 0xf5]);
    input.push(0x21);
    push_bytes(&mut input, &X);
    input.extend([0x20, 0x06, 0x02, 0x42, 0x01, 0x02, 0x01, 0x01]);

    let parsed = PublicKeyCose::from_cbor(&input).expect("key with extensions");
    assert_eq!(parsed.additional_parameter_count(), 1);
    let first = parsed.to_cbor().expect("deterministic encoding");
    assert_eq!(first, parsed.to_cbor().expect("repeat encoding"));
    assert_eq!(PublicKeyCose::from_cbor(&first).expect("reparse"), parsed);

    let expected_prefix = [
        0xa5, 0x01, 0x01, 0x02, 0x42, 0x01, 0x02, 0x20, 0x06, 0x21, 0x58, 0x20,
    ];
    assert_eq!(&first[..expected_prefix.len()], expected_prefix);
    assert_eq!(
        &first[first.len() - 7..],
        [0x18, 0x2a, 0xa2, 0x01, 0xf5, 0x02, 0xf6]
    );
}

#[test]
fn private_material_is_rejected_and_redacted() {
    let mut encoded = okp_fixture(6, &X);
    encoded[0] = 0xa4;
    encoded.push(0x23);
    push_bytes(&mut encoded, PRIVATE_SENTINEL);

    let error = PublicKeyCose::from_cbor(&encoded).expect_err("private d must fail");
    assert_eq!(error, CoseKeyError::PrivateKeyMaterial);
    let rendered = format!("{error:?} {error} {}", error.to_identus_error());
    assert!(!rendered.contains("never-print-private-cbor"));
    let identus = error.to_identus_error();
    assert_eq!(identus.code().as_str(), "crypto.invalid_cose_key");
    assert_eq!(identus.kind(), ErrorKind::InvalidInput);
    assert_eq!(identus.capability(), Some(CapabilityId::new("crypto")));

    let private_with_unsupported_kty = [0xa2, 0x01, 0x04, 0x23, 0x41, 0x01];
    assert_eq!(
        PublicKeyCose::from_cbor(&private_with_unsupported_kty),
        Err(CoseKeyError::PrivateKeyMaterial)
    );
}

#[test]
fn incompatible_and_missing_shapes_are_rejected() {
    let incompatible = ec_fixture(6, &X, CoseEcY::Coordinate(Y));
    assert_eq!(
        PublicKeyCose::from_cbor(&incompatible),
        Err(CoseKeyError::IncompatibleProfile {
            key_type: CoseKeyType::Ec2,
            curve: CoseCurve::Ed25519,
        })
    );

    let mut missing_y = vec![0xa3, 0x01, 0x02, 0x20, 0x01, 0x21];
    push_bytes(&mut missing_y, &X);
    assert_eq!(
        PublicKeyCose::from_cbor(&missing_y),
        Err(CoseKeyError::MissingYCoordinate)
    );

    let mut unexpected_y = okp_fixture(6, &X);
    unexpected_y[0] = 0xa4;
    unexpected_y.extend([0x22, 0xf4]);
    assert_eq!(
        PublicKeyCose::from_cbor(&unexpected_y),
        Err(CoseKeyError::UnexpectedYCoordinate)
    );
}

#[test]
fn coordinate_types_and_widths_are_rejected() {
    let invalid_type = [0xa3, 0x01, 0x01, 0x20, 0x06, 0x21, 0x61, b'x'];
    assert_eq!(
        PublicKeyCose::from_cbor(&invalid_type),
        Err(CoseKeyError::InvalidCoordinateType {
            coordinate: CoseCoordinate::X,
        })
    );

    let short = okp_fixture(6, &[0x11; 31]);
    assert_eq!(
        PublicKeyCose::from_cbor(&short),
        Err(CoseKeyError::InvalidCoordinateLength {
            coordinate: CoseCoordinate::X,
            expected: 32,
            actual: 31,
        })
    );

    let mut invalid_y = ec_fixture(1, &X, CoseEcY::Sign(false));
    *invalid_y.last_mut().expect("y value") = 0xf6;
    assert_eq!(
        PublicKeyCose::from_cbor(&invalid_y),
        Err(CoseKeyError::InvalidCoordinateType {
            coordinate: CoseCoordinate::Y,
        })
    );
}

#[test]
fn parser_rejects_tags_trailing_data_and_oversized_inputs() {
    let valid = okp_fixture(6, &X);

    let mut tagged = vec![0xd8, 0x65];
    tagged.extend(&valid);
    assert_eq!(
        PublicKeyCose::from_cbor(&tagged),
        Err(CoseKeyError::ExpectedMap)
    );

    let mut trailing = valid;
    trailing.push(0x00);
    assert_eq!(
        PublicKeyCose::from_cbor(&trailing),
        Err(CoseKeyError::TrailingData)
    );

    let oversized = vec![0u8; MAX_COSE_KEY_BYTES + 1];
    assert_eq!(
        PublicKeyCose::from_cbor(&oversized),
        Err(CoseKeyError::InputTooLarge {
            max: MAX_COSE_KEY_BYTES,
            actual: MAX_COSE_KEY_BYTES + 1,
        })
    );
}

#[test]
fn parser_rejects_duplicate_top_level_and_nested_map_keys() {
    let mut duplicate = vec![0xa4, 0x01, 0x01, 0x01, 0x01, 0x20, 0x06, 0x21];
    push_bytes(&mut duplicate, &X);
    assert_eq!(
        PublicKeyCose::from_cbor(&duplicate),
        Err(CoseKeyError::DuplicateMapKey)
    );

    let mut nested = okp_fixture(6, &X);
    nested[0] = 0xa4;
    nested.extend([0x18, 0x2a, 0xa2, 0x01, 0xf5, 0x01, 0xf4]);
    assert_eq!(
        PublicKeyCose::from_cbor(&nested),
        Err(CoseKeyError::DuplicateMapKey)
    );
}

#[test]
fn parser_rejects_parameter_and_nesting_exhaustion() {
    let additional = MAX_COSE_ADDITIONAL_PARAMETERS + 1;
    let mut crowded = Vec::new();
    push_map_len(&mut crowded, 3 + additional);
    crowded.extend([0x01, 0x01, 0x20, 0x06, 0x21]);
    push_bytes(&mut crowded, &X);
    for label in 100..100 + additional as u64 {
        push_uint(&mut crowded, 0, label);
        crowded.push(0xf6);
    }
    assert_eq!(
        PublicKeyCose::from_cbor(&crowded),
        Err(CoseKeyError::TooManyParameters {
            max: MAX_COSE_ADDITIONAL_PARAMETERS,
            actual: additional,
        })
    );

    let mut nested = okp_fixture(6, &X);
    nested[0] = 0xa4;
    nested.extend([0x18, 0x2a]);
    nested.extend(std::iter::repeat_n(0x81, 24));
    nested.push(0xf6);
    assert_eq!(
        PublicKeyCose::from_cbor(&nested),
        Err(CoseKeyError::InvalidCbor)
    );
}

#[test]
fn floating_point_extensions_are_rejected() {
    let mut encoded = okp_fixture(6, &X);
    encoded[0] = 0xa4;
    encoded.extend([0x18, 0x2a, 0xf9, 0x3e, 0x00]);
    assert_eq!(
        PublicKeyCose::from_cbor(&encoded),
        Err(CoseKeyError::FloatingPointValue)
    );
}

#[test]
fn constructors_enforce_curve_family() {
    assert!(PublicKeyCose::new_okp(CoseCurve::Ed25519, X).is_ok());
    assert!(PublicKeyCose::new_ec(CoseCurve::P256, X, Y).is_ok());
    assert!(matches!(
        PublicKeyCose::new_okp(CoseCurve::P256, X),
        Err(CoseKeyError::IncompatibleProfile { .. })
    ));
    assert!(matches!(
        PublicKeyCose::new_ec(CoseCurve::X25519, X, Y),
        Err(CoseKeyError::IncompatibleProfile { .. })
    ));
}

#[cfg(feature = "jwk")]
#[test]
fn full_coordinate_jwk_conversion_preserves_key_material() {
    use identus_crypto::{JwkCurve, PublicKeyJwk};

    for jwk in [
        PublicKeyJwk::new_okp(JwkCurve::Ed25519, X).expect("JWK"),
        PublicKeyJwk::new_okp(JwkCurve::X25519, X).expect("JWK"),
        PublicKeyJwk::new_ec(JwkCurve::P256, X, Y).expect("JWK"),
        PublicKeyJwk::new_ec(JwkCurve::Secp256k1, X, Y).expect("JWK"),
    ] {
        let cose = PublicKeyCose::try_from(&jwk).expect("JWK to COSE");
        let roundtrip = PublicKeyJwk::try_from(&cose).expect("COSE to JWK");
        assert_eq!(roundtrip.kty(), jwk.kty());
        assert_eq!(roundtrip.crv(), jwk.crv());
        assert_eq!(roundtrip.x(), jwk.x());
        assert_eq!(roundtrip.y(), jwk.y());
    }
}

#[cfg(feature = "jwk")]
#[test]
fn compressed_ec2_to_jwk_is_explicitly_rejected() {
    use identus_crypto::PublicKeyJwk;

    let compressed =
        PublicKeyCose::new_ec_compressed(CoseCurve::P256, X, true).expect("compressed COSE key");
    assert_eq!(
        PublicKeyJwk::try_from(&compressed),
        Err(CoseKeyError::CompressedCoordinate)
    );
}

#[cfg(feature = "ed25519")]
#[test]
fn ed25519_encoder_preserves_public_bytes() {
    use identus_crypto::{Ed25519PrivateKey, EncodeCose};

    let public = Ed25519PrivateKey::from_slice(&[1u8; 32])
        .expect("private key")
        .to_public_key();
    let cose = public.encode_cose();
    assert_eq!(cose.curve(), CoseCurve::Ed25519);
    assert_eq!(cose.x(), &public.0.to_bytes());
}

#[cfg(feature = "x25519")]
#[test]
fn x25519_encoder_preserves_public_bytes() {
    use identus_crypto::{EncodeCose, X25519PrivateKey};

    let public = X25519PrivateKey::from_slice(&[2u8; 32])
        .expect("private key")
        .to_public_key();
    let cose = public.encode_cose();
    assert_eq!(cose.curve(), CoseCurve::X25519);
    assert_eq!(cose.x(), &public.0.to_bytes());
}

#[cfg(feature = "secp256r1")]
#[test]
fn p256_encoder_preserves_public_coordinates() {
    use identus_crypto::{EncodeCose, P256PrivateKey};

    let public = P256PrivateKey::from_slice(&[3u8; 32])
        .expect("private key")
        .to_public_key();
    let (x, y) = public.curve_point();
    let cose = public.encode_cose();
    assert_eq!(cose.curve(), CoseCurve::P256);
    assert_eq!(cose.x(), &x);
    assert_eq!(cose.y(), Some(CoseEcY::Coordinate(y)));
}

#[cfg(feature = "secp256k1")]
#[test]
fn secp256k1_encoder_preserves_public_coordinates() {
    use identus_crypto::{EncodeCose, Secp256k1PrivateKey};

    let public = Secp256k1PrivateKey::from_slice(&[4u8; 32])
        .expect("private key")
        .to_public_key();
    let point = public.curve_point();
    let cose = public.encode_cose();
    assert_eq!(cose.curve(), CoseCurve::Secp256k1);
    assert_eq!(cose.x(), &point.x);
    assert_eq!(cose.y(), Some(CoseEcY::Coordinate(point.y)));
}

#[test]
#[ignore = "manual release-mode throughput observation"]
fn release_mode_encode_parse_throughput_observation() {
    let key = PublicKeyCose::new_okp(CoseCurve::Ed25519, X).expect("public key");
    let encoded = key.to_cbor().expect("encoding");
    let iterations = 50_000u32;
    let started = Instant::now();
    for _ in 0..iterations {
        let parsed = PublicKeyCose::from_cbor(std::hint::black_box(&encoded)).expect("parse");
        std::hint::black_box(parsed.to_cbor().expect("encode"));
    }
    let elapsed = started.elapsed();
    eprintln!(
        "COSE key encode+parse: {iterations} iterations in {elapsed:?} ({:.0} ops/s)",
        f64::from(iterations) / elapsed.as_secs_f64()
    );
}

use std::collections::BTreeMap;

use identus_core::{CapabilityId, ErrorKind};
use identus_crypto::{JwkCoordinate, JwkCurve, JwkError, JwkKeyType, PublicKeyJwk};
use serde_json::{Value, json};

const RFC_8037_ED25519_X: &str = "11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo";
const PRIVATE_SENTINEL: &str = "never-print-this-private-value";

fn zero_coordinate() -> String {
    "A".repeat(43)
}

#[test]
fn rfc_8037_appendix_a2_public_key_is_exact() {
    let source = json!({
        "kty": "OKP",
        "crv": "Ed25519",
        "x": RFC_8037_ED25519_X,
    });
    let jwk: PublicKeyJwk = serde_json::from_value(source.clone()).expect("RFC JWK");

    assert_eq!(jwk.kty(), JwkKeyType::Okp);
    assert_eq!(jwk.crv(), JwkCurve::Ed25519);
    assert_eq!(jwk.x().as_str(), RFC_8037_ED25519_X);
    assert!(jwk.y().is_none());
    assert_eq!(serde_json::to_value(jwk).expect("serialize"), source);
}

#[test]
fn supported_profiles_roundtrip_with_exact_wire_names() {
    for (kty, crv, y) in [
        ("OKP", "Ed25519", None),
        ("OKP", "X25519", None),
        ("EC", "P-256", Some(zero_coordinate())),
        ("EC", "secp256k1", Some(zero_coordinate())),
    ] {
        let mut source = json!({"kty": kty, "crv": crv, "x": zero_coordinate()});
        if let Some(y) = y {
            source["y"] = Value::String(y);
        }
        let parsed: PublicKeyJwk = serde_json::from_value(source.clone()).expect("valid profile");
        assert_eq!(serde_json::to_value(parsed).expect("serialize"), source);
    }
}

#[test]
fn public_extensions_survive_without_policy_interpretation() {
    let source = json!({
        "kty": "OKP",
        "crv": "Ed25519",
        "x": zero_coordinate(),
        "kid": "did:example:123#key-1",
        "https://example.test/jwk-policy": {"tier": 2, "active": true},
    });
    let parsed: PublicKeyJwk = serde_json::from_value(source.clone()).expect("extended JWK");

    assert_eq!(parsed.extensions().len(), 2);
    assert_eq!(serde_json::to_value(parsed).expect("serialize"), source);
}

#[test]
fn constructor_rejects_incompatible_profiles_and_shapes() {
    let x = zero_coordinate();
    assert_eq!(
        PublicKeyJwk::from_parts(
            JwkKeyType::Ec,
            JwkCurve::Ed25519,
            &x,
            Some(&x),
            BTreeMap::new(),
        ),
        Err(JwkError::IncompatibleProfile {
            key_type: JwkKeyType::Ec,
            curve: JwkCurve::Ed25519,
        })
    );
    assert_eq!(
        PublicKeyJwk::from_parts(JwkKeyType::Ec, JwkCurve::P256, &x, None, BTreeMap::new(),),
        Err(JwkError::MissingYCoordinate)
    );
    assert_eq!(
        PublicKeyJwk::from_parts(
            JwkKeyType::Okp,
            JwkCurve::Ed25519,
            &x,
            Some(&x),
            BTreeMap::new(),
        ),
        Err(JwkError::UnexpectedYCoordinate)
    );
}

#[test]
fn serde_rejects_incompatible_profiles_and_shapes() {
    for source in [
        json!({"kty":"EC", "crv":"Ed25519", "x":zero_coordinate(), "y":zero_coordinate()}),
        json!({"kty":"EC", "crv":"P-256", "x":zero_coordinate()}),
        json!({"kty":"OKP", "crv":"Ed25519", "x":zero_coordinate(), "y":zero_coordinate()}),
        json!({"kty":"OKP", "crv":"Ed25519"}),
        json!({"kty":"RSA", "crv":"Ed25519", "x":zero_coordinate()}),
        json!({"kty":"OKP", "crv":"Ed448", "x":zero_coordinate()}),
    ] {
        assert!(serde_json::from_value::<PublicKeyJwk>(source).is_err());
    }
}

#[test]
fn profile_specific_constructors_reject_the_other_key_family() {
    assert!(matches!(
        PublicKeyJwk::new_okp(JwkCurve::P256, [0; 32]),
        Err(JwkError::IncompatibleProfile { .. })
    ));
    assert!(matches!(
        PublicKeyJwk::new_ec(JwkCurve::Ed25519, [0; 32], [0; 32]),
        Err(JwkError::IncompatibleProfile { .. })
    ));
}

#[test]
fn constructor_rejects_noncanonical_and_wrong_width_coordinates() {
    for (value, expected) in [
        (
            format!("{}=", zero_coordinate()),
            JwkError::InvalidCoordinateEncoding {
                coordinate: JwkCoordinate::X,
            },
        ),
        (
            format!("{}+", "A".repeat(42)),
            JwkError::InvalidCoordinateEncoding {
                coordinate: JwkCoordinate::X,
            },
        ),
        (
            format!("{}B", "A".repeat(42)),
            JwkError::InvalidCoordinateEncoding {
                coordinate: JwkCoordinate::X,
            },
        ),
        (
            "A".repeat(42),
            JwkError::InvalidCoordinateLength {
                coordinate: JwkCoordinate::X,
                expected: 32,
                actual: 31,
            },
        ),
        (
            "A".repeat(44),
            JwkError::InvalidCoordinateLength {
                coordinate: JwkCoordinate::X,
                expected: 32,
                actual: 33,
            },
        ),
    ] {
        assert_eq!(
            PublicKeyJwk::from_parts(
                JwkKeyType::Okp,
                JwkCurve::Ed25519,
                &value,
                None,
                BTreeMap::new(),
            ),
            Err(expected),
            "input length {}",
            value.len()
        );
    }
}

#[test]
fn serde_rejects_noncanonical_and_wrong_width_coordinates() {
    for x in [
        format!("{}=", zero_coordinate()),
        format!("{}+", "A".repeat(42)),
        format!("{}B", "A".repeat(42)),
        "A".repeat(42),
        "A".repeat(44),
    ] {
        let source = json!({"kty":"OKP", "crv":"Ed25519", "x":x});
        assert!(serde_json::from_value::<PublicKeyJwk>(source).is_err());
    }
}

#[test]
fn ec_y_coordinate_uses_the_same_canonical_width_gate() {
    let error = PublicKeyJwk::from_parts(
        JwkKeyType::Ec,
        JwkCurve::P256,
        &zero_coordinate(),
        Some(&"A".repeat(42)),
        BTreeMap::new(),
    )
    .expect_err("31-byte y");
    assert_eq!(
        error,
        JwkError::InvalidCoordinateLength {
            coordinate: JwkCoordinate::Y,
            expected: 32,
            actual: 31,
        }
    );
}

#[test]
fn private_and_reserved_members_are_rejected_without_value_leaks() {
    let wire = format!(
        r#"{{"kty":"OKP","crv":"Ed25519","x":"{}","d":"{}"}}"#,
        zero_coordinate(),
        PRIVATE_SENTINEL
    );
    let serde_error = serde_json::from_str::<PublicKeyJwk>(&wire).expect_err("private d");
    assert!(!serde_error.to_string().contains(PRIVATE_SENTINEL));

    let mut private = BTreeMap::new();
    private.insert("d".to_owned(), json!(PRIVATE_SENTINEL));
    assert_eq!(
        PublicKeyJwk::from_parts(
            JwkKeyType::Okp,
            JwkCurve::Ed25519,
            &zero_coordinate(),
            None,
            private,
        ),
        Err(JwkError::PrivateKeyMaterial)
    );

    let mut reserved = BTreeMap::new();
    reserved.insert("x".to_owned(), json!("shadow"));
    assert_eq!(
        PublicKeyJwk::from_parts(
            JwkKeyType::Okp,
            JwkCurve::Ed25519,
            &zero_coordinate(),
            None,
            reserved,
        ),
        Err(JwkError::ReservedExtension)
    );
}

#[test]
fn duplicate_structural_wire_members_are_rejected() {
    let duplicate = format!(
        r#"{{"kty":"OKP","kty":"EC","crv":"Ed25519","x":"{}"}}"#,
        zero_coordinate()
    );
    assert!(serde_json::from_str::<PublicKeyJwk>(&duplicate).is_err());
}

#[test]
fn jwk_error_bridge_is_stable_and_redacted() {
    let error = JwkError::InvalidCoordinateLength {
        coordinate: JwkCoordinate::X,
        expected: 32,
        actual: 31,
    };
    let identus = error.to_identus_error();

    assert_eq!(identus.code().as_str(), "crypto.invalid_jwk");
    assert_eq!(identus.kind(), ErrorKind::InvalidInput);
    assert_eq!(identus.capability(), Some(CapabilityId::new("crypto")));
    assert_eq!(
        identus.to_string(),
        "crypto.invalid_jwk: invalid public JSON Web Key"
    );
    assert!(!identus.to_string().contains("31"));
    assert!(!identus.to_string().contains("32"));
}

#[test]
fn coordinate_parse_error_never_echoes_the_rejected_value() {
    let rejected = "secret-looking-coordinate+";
    let error = PublicKeyJwk::from_parts(
        JwkKeyType::Okp,
        JwkCurve::Ed25519,
        rejected,
        None,
        BTreeMap::new(),
    )
    .expect_err("invalid alphabet");

    assert!(!error.to_string().contains(rejected));
    assert!(!error.to_identus_error().to_string().contains(rejected));
}

use std::collections::BTreeMap;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_crypto::{
    Ed25519PrivateKey, EncodeJwk, JwkCurve, JwkKeyType, PublicKeyJwk, X25519PrivateKey,
};
use identus_did_jwk::{DidJwk, Error, KeyUse};
use serde_json::Value;

#[test]
fn pinned_normative_examples_match_the_selected_profile() {
    let packet: Value = serde_json::from_str(include_str!(
        "../../../docs/conformance/fixtures/did-jwk/v1/vectors.json"
    ))
    .unwrap();
    for case in packet["cases"].as_array().unwrap() {
        let parsed = DidJwk::parse(case["input"]["literal"].as_str().unwrap()).unwrap();
        assert_eq!(
            curve_name(parsed.public_key().crv()),
            case["expected"]["fields"]["curve"]
        );
        assert_eq!(
            relationship_count(&parsed),
            usize::try_from(
                case["expected"]["fields"]["relationshipCount"]
                    .as_u64()
                    .unwrap()
            )
            .unwrap()
        );
    }
}

#[test]
fn alternate_ordering_and_whitespace_preserve_the_exact_did() {
    let jwk = Ed25519PrivateKey::from_slice(&[17; 32])
        .unwrap()
        .to_public_key()
        .encode_jwk();
    let json = format!(
        "{{ \"x\" : \"{}\", \"crv\" : \"Ed25519\", \"note\": {{\"b\":2,\"a\":1}}, \"kty\" : \"OKP\" }}",
        jwk.x()
    );
    let did = format!("did:jwk:{}", URL_SAFE_NO_PAD.encode(json));
    let parsed = DidJwk::parse(&did).unwrap();
    assert_eq!(parsed.did().as_str(), did);
    assert_eq!(parsed.public_key().extensions()["note"]["a"], 1);
    assert_ne!(
        DidJwk::create(parsed.public_key().clone()).unwrap().did(),
        parsed.did()
    );
}

#[test]
fn explicit_signature_and_encryption_uses_project_separately() {
    let signing = Ed25519PrivateKey::from_slice(&[19; 32])
        .unwrap()
        .to_public_key()
        .encode_jwk();
    let signing = with_use(&signing, "sig");
    let signing = DidJwk::create(signing).unwrap();
    assert_eq!(signing.key_use(), KeyUse::Signature);
    assert_eq!(relationship_count(&signing), 4);

    let agreement = X25519PrivateKey::from_slice(&[20; 32])
        .unwrap()
        .to_public_key()
        .encode_jwk();
    let agreement = with_use(&agreement, "enc");
    let agreement = DidJwk::create(agreement).unwrap();
    assert_eq!(agreement.key_use(), KeyUse::Encryption);
    assert_eq!(relationship_count(&agreement), 1);
}

#[test]
fn constructor_parser_property_covers_public_material() {
    for seed in 1u8..=64 {
        let jwk = Ed25519PrivateKey::from_slice(&[seed; 32])
            .unwrap()
            .to_public_key()
            .encode_jwk();
        let created = DidJwk::create(jwk).unwrap();
        assert_eq!(DidJwk::parse(created.did().as_str()).unwrap(), created);

        let jwk = X25519PrivateKey::from_slice(&[seed; 32])
            .unwrap()
            .to_public_key()
            .encode_jwk();
        let created = DidJwk::create(jwk).unwrap();
        assert_eq!(DidJwk::parse(created.did().as_str()).unwrap(), created);
    }
}

#[test]
fn creation_and_parsing_share_structural_and_projection_limits() {
    let jwk = Ed25519PrivateKey::from_slice(&[23; 32])
        .unwrap()
        .to_public_key()
        .encode_jwk();

    let mut nested = Value::Null;
    for _ in 0..15 {
        nested = Value::Array(vec![nested]);
    }
    let nested = with_extension(&jwk, "meta", nested);
    assert_eq!(DidJwk::create(nested), Err(Error::ResourceLimit));

    let wide = with_extension(&jwk, "meta", Value::Array(vec![Value::Null; 129]));
    assert_eq!(DidJwk::create(wide.clone()), Err(Error::Projection));
    let json = serde_json::to_vec(&wide).unwrap();
    let did = format!("did:jwk:{}", URL_SAFE_NO_PAD.encode(json));
    assert_eq!(DidJwk::parse(&did), Err(Error::Projection));

    let mut extensions = BTreeMap::new();
    extensions.insert("note".to_owned(), Value::String("x".repeat(16_384)));
    let oversized_weak = PublicKeyJwk::from_parts(
        JwkKeyType::Okp,
        JwkCurve::Ed25519,
        &URL_SAFE_NO_PAD.encode([0u8; 32]),
        None,
        extensions,
    )
    .unwrap();
    assert_eq!(DidJwk::create(oversized_weak), Err(Error::ResourceLimit));
}

#[test]
fn duplicate_private_key_ops_and_noncanonical_carriers_fail_closed() {
    let cases = [
        (
            r#"{"kty":"OKP","kty":"OKP","crv":"Ed25519","x":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}"#,
            Error::DuplicateMember,
        ),
        (
            r#"{"kty":"OKP","crv":"Ed25519","x":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","d":"secret"}"#,
            Error::PrivateKeyMaterial,
        ),
        (
            r#"{"kty":"OKP","crv":"Ed25519","x":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","key_ops":["verify"]}"#,
            Error::UnsupportedProfile,
        ),
        (
            r#"{"kty":"OKP","crv":"Ed25519","x":"AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE","y":null}"#,
            Error::UnsupportedProfile,
        ),
    ];
    for (json, expected) in cases {
        let did = format!("did:jwk:{}", URL_SAFE_NO_PAD.encode(json));
        assert_eq!(DidJwk::parse(&did), Err(expected));
    }
    assert_eq!(DidJwk::parse("did:jwk:QQ=="), Err(Error::MalformedEncoding));
    assert_eq!(DidJwk::parse("did:jwk:A"), Err(Error::ResourceLimit));
}

fn with_use(jwk: &PublicKeyJwk, value: &str) -> PublicKeyJwk {
    with_extension(jwk, "use", Value::String(value.to_owned()))
}

fn with_extension(jwk: &PublicKeyJwk, name: &str, value: Value) -> PublicKeyJwk {
    let mut extensions = BTreeMap::new();
    extensions.insert(name.to_owned(), value);
    PublicKeyJwk::from_parts(
        match jwk.crv() {
            JwkCurve::Ed25519 | JwkCurve::X25519 => JwkKeyType::Okp,
            JwkCurve::P256 | JwkCurve::Secp256k1 => JwkKeyType::Ec,
        },
        jwk.crv(),
        jwk.x().as_str(),
        jwk.y().map(|value| value.as_str()),
        extensions,
    )
    .unwrap()
}

fn relationship_count(value: &DidJwk) -> usize {
    let document = value.resolve().unwrap();
    document.authentication().map_or(0, <[_]>::len)
        + document.assertion_method().map_or(0, <[_]>::len)
        + document.key_agreement().map_or(0, <[_]>::len)
        + document.capability_invocation().map_or(0, <[_]>::len)
        + document.capability_delegation().map_or(0, <[_]>::len)
}

fn curve_name(curve: JwkCurve) -> &'static str {
    match curve {
        JwkCurve::Ed25519 => "Ed25519",
        JwkCurve::X25519 => "X25519",
        JwkCurve::P256 => "P-256",
        JwkCurve::Secp256k1 => "secp256k1",
    }
}

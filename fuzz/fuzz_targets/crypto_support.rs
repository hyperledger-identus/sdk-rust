// The two fuzz binaries intentionally compile different subsets of this shared
// invariant module.
#![allow(dead_code)]

use std::{borrow::Cow, sync::Once};

use identus_crypto::{
    CoseEcY, CoseKeyError, CoseKeyType, JwkKeyType, MAX_COSE_ADDITIONAL_PARAMETERS,
    MAX_COSE_KEY_BYTES, PublicKeyCose, PublicKeyJwk,
};

static COSE_BOUNDARY_PROBE: Once = Once::new();

pub(crate) fn fuzz_public_jwk(data: &[u8]) {
    let Ok(jwk) = serde_json::from_slice::<PublicKeyJwk>(data) else {
        return;
    };

    assert_jwk_invariants(&jwk);

    let encoded =
        serde_json::to_vec(&jwk).unwrap_or_else(|_| panic!("accepted public JWK must serialize"));
    let reparsed = serde_json::from_slice::<PublicKeyJwk>(&encoded)
        .unwrap_or_else(|_| panic!("serialized public JWK must deserialize"));
    assert!(reparsed == jwk, "public JWK serde round trip disagrees");
    assert_jwk_invariants(&reparsed);

    let cose = PublicKeyCose::try_from(&jwk)
        .unwrap_or_else(|_| panic!("accepted public JWK must convert to COSE Key"));
    let round_trip = PublicKeyJwk::try_from(&cose)
        .unwrap_or_else(|_| panic!("JWK-derived COSE Key must convert to JWK"));
    assert_same_public_material(&jwk, &round_trip);
}

pub(crate) fn fuzz_public_cose(data: &[u8]) {
    assert_cose_resource_boundary();
    let Some(encoded) = decode_cose_seed(data) else {
        return;
    };
    let Ok(cose) = PublicKeyCose::from_cbor(&encoded) else {
        return;
    };

    assert_cose_invariants(&cose);

    let normalized = cose
        .to_cbor()
        .unwrap_or_else(|_| panic!("accepted public COSE Key must serialize"));
    assert!(
        normalized.len() <= MAX_COSE_KEY_BYTES,
        "serialized public COSE Key exceeded the parser bound"
    );
    let reparsed = PublicKeyCose::from_cbor(&normalized)
        .unwrap_or_else(|_| panic!("serialized public COSE Key must deserialize"));
    assert!(reparsed == cose, "public COSE Key round trip disagrees");
    let reencoded = reparsed
        .to_cbor()
        .unwrap_or_else(|_| panic!("reparsed public COSE Key must serialize"));
    assert!(
        reencoded == normalized,
        "public COSE Key encoding is not deterministic"
    );

    match cose.y() {
        Some(CoseEcY::Sign(_)) => assert!(
            matches!(
                PublicKeyJwk::try_from(&cose),
                Err(CoseKeyError::CompressedCoordinate)
            ),
            "compressed EC2 key must fail JWK conversion explicitly"
        ),
        Some(CoseEcY::Coordinate(_)) | None => {
            let jwk = PublicKeyJwk::try_from(&cose)
                .unwrap_or_else(|_| panic!("full-coordinate COSE Key must convert to JWK"));
            assert_jwk_invariants(&jwk);
            let round_trip = PublicKeyCose::try_from(&jwk)
                .unwrap_or_else(|_| panic!("COSE-derived JWK must convert to COSE Key"));
            assert_same_cose_material(&cose, &round_trip);
        }
    }
}

fn decode_cose_seed(data: &[u8]) -> Option<Cow<'_, [u8]>> {
    let Some(encoded) = data.strip_prefix(b"hex:") else {
        return Some(Cow::Borrowed(data));
    };
    let encoded = std::str::from_utf8(encoded).ok()?.trim();
    hex::decode(encoded).ok().map(Cow::Owned)
}

fn assert_cose_resource_boundary() {
    COSE_BOUNDARY_PROBE.call_once(|| {
        let exact = vec![0_u8; MAX_COSE_KEY_BYTES];
        let over = vec![0_u8; MAX_COSE_KEY_BYTES + 1];
        let _ = PublicKeyCose::from_cbor(&exact);
        assert!(
            matches!(
                PublicKeyCose::from_cbor(&over),
                Err(CoseKeyError::InputTooLarge { max, actual })
                    if max == MAX_COSE_KEY_BYTES && actual == MAX_COSE_KEY_BYTES + 1
            ),
            "COSE Key over-limit input was not rejected at the resource boundary"
        );
    });
}

fn assert_jwk_invariants(jwk: &PublicKeyJwk) {
    assert!(jwk.kty() == jwk.crv().key_type(), "JWK profile disagrees");
    assert_coordinate(jwk.x().as_str(), jwk.x().to_bytes().len());
    match (jwk.kty(), jwk.y()) {
        (JwkKeyType::Ec, Some(y)) => assert_coordinate(y.as_str(), y.to_bytes().len()),
        (JwkKeyType::Okp, None) => {}
        _ => panic!("JWK coordinate shape disagrees with key type"),
    }
    assert!(
        jwk.extensions()
            .keys()
            .all(|name| !matches!(name.as_str(), "d" | "kty" | "crv" | "x" | "y")),
        "JWK retained a private or structural extension"
    );

    let first = jwk.thumbprint_sha256();
    let second = jwk.thumbprint_sha256();
    assert!(first == second, "JWK thumbprint is not deterministic");
    assert!(
        first.as_bytes().len() == 32,
        "JWK thumbprint length drifted"
    );
    let encoded = first.to_base64url();
    assert!(
        encoded.as_str().len() == 43 && !encoded.as_str().contains('='),
        "JWK thumbprint is not canonical unpadded base64url"
    );
}

fn assert_coordinate(encoded: &str, decoded_len: usize) {
    assert!(decoded_len == 32, "public-key coordinate length drifted");
    assert!(
        encoded.len() == 43 && !encoded.contains('='),
        "public-key coordinate is not canonical unpadded base64url"
    );
}

fn assert_cose_invariants(cose: &PublicKeyCose) {
    assert!(
        cose.key_type() == cose.curve().key_type(),
        "COSE Key profile disagrees"
    );
    assert!(cose.x().len() == 32, "COSE Key x length drifted");
    assert!(
        cose.additional_parameter_count() <= MAX_COSE_ADDITIONAL_PARAMETERS,
        "COSE Key retained too many additional parameters"
    );
    match (cose.key_type(), cose.y()) {
        (CoseKeyType::Okp, None) => {}
        (CoseKeyType::Ec2, Some(CoseEcY::Coordinate(y))) => {
            assert!(y.len() == 32, "COSE Key y length drifted");
        }
        (CoseKeyType::Ec2, Some(CoseEcY::Sign(_))) => {}
        _ => panic!("COSE Key coordinate shape disagrees with key type"),
    }
}

fn assert_same_public_material(left: &PublicKeyJwk, right: &PublicKeyJwk) {
    assert!(
        left.kty() == right.kty()
            && left.crv() == right.crv()
            && left.x() == right.x()
            && left.y() == right.y(),
        "JWK/COSE conversion changed public key material"
    );
}

fn assert_same_cose_material(left: &PublicKeyCose, right: &PublicKeyCose) {
    assert!(
        left.key_type() == right.key_type()
            && left.curve() == right.curve()
            && left.x() == right.x()
            && left.y() == right.y(),
        "COSE/JWK conversion changed public key material"
    );
}

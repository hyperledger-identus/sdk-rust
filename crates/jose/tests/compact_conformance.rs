use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use identus_core::IdentusError;
use identus_crypto::{JwkCurve, PublicKeyJwk};
use identus_jose::{
    JoseError, JwsKeyReference, JwsLimits, JwsSigningInput, MAX_X5C_CERTIFICATES, ProtectedHeader,
    UnverifiedCompactJws,
};

const RFC_HEADER: &str = "eyJ0eXAiOiJKV1QiLA0KICJhbGciOiJIUzI1NiJ9";
const RFC_PAYLOAD: &str = "eyJpc3MiOiJqb2UiLA0KICJleHAiOjEzMDA4MTkzODAsDQogImh0dHA6Ly9leGFtcGxlLmNvbS9pc19yb290Ijp0cnVlfQ";
const RFC_SIGNATURE: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";

fn compact_from_raw(header: &[u8], payload: &[u8], signature: &[u8]) -> String {
    format!(
        "{}.{}.{}",
        URL_SAFE_NO_PAD.encode(header),
        URL_SAFE_NO_PAD.encode(payload),
        URL_SAFE_NO_PAD.encode(signature)
    )
}

fn proof_header() -> ProtectedHeader {
    ProtectedHeader::new(
        "EdDSA",
        Some("openid4vci-proof+jwt"),
        Some("did:example:holder#key-1"),
        JwsLimits::default(),
    )
    .expect("valid proof-shaped header")
}

#[test]
fn preserves_the_exact_rfc_7515_example() {
    let compact = format!("{RFC_HEADER}.{RFC_PAYLOAD}.{RFC_SIGNATURE}");
    let parsed = UnverifiedCompactJws::parse(&compact, JwsLimits::default()).expect("RFC example");

    assert_eq!(parsed.compact(), compact);
    assert_eq!(
        parsed.signing_input(),
        format!("{RFC_HEADER}.{RFC_PAYLOAD}").as_bytes()
    );
    assert_eq!(parsed.protected_header().algorithm(), "HS256");
    assert_eq!(parsed.protected_header().type_(), Some("JWT"));
    assert_eq!(parsed.protected_header().key_id(), None);
    assert_eq!(URL_SAFE_NO_PAD.encode(parsed.payload()), RFC_PAYLOAD);
    assert_eq!(URL_SAFE_NO_PAD.encode(parsed.signature()), RFC_SIGNATURE);
}

#[test]
fn staged_encoding_supports_oxid_and_portal_shapes_without_policy() {
    let payload = br#"{"aud":"https://issuer.example","iat":1701960444,"nonce":"fresh"}"#;
    let prepared = JwsSigningInput::new(proof_header(), payload.to_vec(), JwsLimits::default())
        .expect("prepare Oxid-shaped proof");
    let signing_input = prepared.as_bytes().to_vec();
    let oxid = prepared
        .attach_signature(vec![0x5a; 64])
        .expect("attach external signature");
    let parsed = UnverifiedCompactJws::parse(oxid.compact(), JwsLimits::default())
        .expect("parse Oxid-shaped proof");
    assert_eq!(parsed.signing_input(), signing_input);
    assert_eq!(parsed.payload(), payload);
    assert_eq!(parsed.signature(), [0x5a; 64]);
    assert_eq!(parsed.protected_header(), &proof_header());

    let portal_header = ProtectedHeader::new(
        "EdDSA",
        None,
        Some("did:example:holder#authentication-1"),
        JwsLimits::default(),
    )
    .expect("valid Portal-shaped header");
    let portal = JwsSigningInput::new(
        portal_header,
        br#"{"iss":"did:example:holder","sub":"did:example:holder"}"#.to_vec(),
        JwsLimits::default(),
    )
    .expect("prepare Portal-shaped token")
    .attach_signature(vec![0xa5; 64])
    .expect("attach Portal-shaped signature");
    assert_eq!(
        UnverifiedCompactJws::parse(portal.compact(), JwsLimits::default())
            .expect("parse Portal-shaped token")
            .protected_header()
            .type_(),
        None
    );
}

#[test]
fn accepts_empty_payload_but_rejects_empty_required_segments() {
    let empty_payload = JwsSigningInput::new(proof_header(), Vec::new(), JwsLimits::default())
        .expect("empty payload is valid")
        .attach_signature(vec![1])
        .expect("non-empty signature");
    assert_eq!(empty_payload.payload(), b"");
    assert!(empty_payload.compact().contains(".."));
    assert_eq!(
        UnverifiedCompactJws::parse(empty_payload.compact(), JwsLimits::default())
            .expect("empty payload parses")
            .payload(),
        b""
    );

    assert_eq!(
        UnverifiedCompactJws::parse(".eA.eA", JwsLimits::default()),
        Err(JoseError::InvalidCompactStructure)
    );
    assert_eq!(
        UnverifiedCompactJws::parse("e30.eA.", JwsLimits::default()),
        Err(JoseError::InvalidCompactStructure)
    );
    assert_eq!(
        JwsSigningInput::new(proof_header(), Vec::new(), JwsLimits::default())
            .expect("prepared")
            .attach_signature(Vec::new()),
        Err(JoseError::EmptySignature)
    );
}

#[test]
fn rejects_invalid_compact_structure_and_segment_encodings() {
    for invalid in ["no-dots", "a.b", "a.b.c.d"] {
        assert_eq!(
            UnverifiedCompactJws::parse(invalid, JwsLimits::default()),
            Err(JoseError::InvalidCompactStructure)
        );
    }

    let valid_header = URL_SAFE_NO_PAD.encode(br#"{"alg":"EdDSA"}"#);
    for invalid in [
        format!("{valid_header}.eA==.eA"),
        format!("{valid_header}.e A.eA"),
        format!("{valid_header}.e+A.eA"),
        format!("{valid_header}.a.eA"),
    ] {
        assert_eq!(
            UnverifiedCompactJws::parse(&invalid, JwsLimits::default()),
            Err(JoseError::NonCanonicalBase64Url)
        );
    }
}

#[test]
fn rejects_invalid_closed_protected_headers() {
    let cases: &[(&[u8], JoseError)] = &[
        (
            br#"{"alg":"EdDSA","alg":"ES256"}"#,
            JoseError::DuplicateProtectedHeader,
        ),
        (
            br#"{"alg":"EdDSA","unsupported":{}}"#,
            JoseError::UnknownProtectedHeader,
        ),
        (br#"{"typ":"JWT"}"#, JoseError::MissingAlgorithm),
        (br#"{"alg":"none"}"#, JoseError::InvalidHeaderValue),
        (br#"{"alg":7}"#, JoseError::InvalidHeaderValue),
        (
            br#"{"alg":"EdDSA","typ":"\u0085"}"#,
            JoseError::InvalidHeaderValue,
        ),
        (br#"[]"#, JoseError::InvalidProtectedHeader),
        (
            b"{\"alg\":\"EdDSA\"} true",
            JoseError::InvalidProtectedHeader,
        ),
        (&[0xff], JoseError::InvalidProtectedHeader),
    ];
    for (header, expected) in cases {
        let compact = compact_from_raw(header, b"payload", b"signature");
        assert_eq!(
            UnverifiedCompactJws::parse(&compact, JwsLimits::default()),
            Err(*expected)
        );
    }

    for algorithm in ["", "none", "not visible", "\n", &"a".repeat(65)] {
        assert_eq!(
            ProtectedHeader::new(algorithm, None, None, JwsLimits::default()),
            Err(JoseError::InvalidHeaderValue)
        );
    }
    for value in ["", "control\n", "control\u{0085}"] {
        assert_eq!(
            ProtectedHeader::new("EdDSA", Some(value), None, JwsLimits::default()),
            Err(JoseError::InvalidHeaderValue)
        );
        assert_eq!(
            ProtectedHeader::new("EdDSA", None, Some(value), JwsLimits::default()),
            Err(JoseError::InvalidHeaderValue)
        );
    }
    let tiny_string_limit = JwsLimits::new(128, 64, 32, 32, 8).expect("tiny string limit");
    assert_eq!(
        ProtectedHeader::new(
            "ES256",
            None,
            Some("borrowed-key-id-too-large"),
            tiny_string_limit
        ),
        Err(JoseError::InvalidHeaderValue)
    );
}

#[test]
fn protected_headers_round_trip_one_exclusive_public_key_reference() {
    let jwk = PublicKeyJwk::new_okp(JwkCurve::Ed25519, [7; 32]).expect("public JWK");
    let references = [
        JwsKeyReference::KeyId("did:example:holder#key-1".to_owned()),
        JwsKeyReference::Jwk(jwk),
        JwsKeyReference::X5c(vec!["AQID".to_owned(), "BAUG".to_owned()]),
    ];

    for key_reference in references {
        let header = ProtectedHeader::with_key_reference(
            "Ed25519",
            Some("openid4vci-proof+jwt"),
            Some(key_reference),
            JwsLimits::default(),
        )
        .expect("bounded key reference");
        let compact = JwsSigningInput::new(header.clone(), b"{}".to_vec(), JwsLimits::default())
            .expect("signing input")
            .attach_signature(vec![1; 64])
            .expect("compact");
        let parsed = UnverifiedCompactJws::parse(compact.compact(), JwsLimits::default())
            .expect("parse emitted header");
        assert_eq!(parsed.protected_header(), &header);
    }
}

#[test]
fn protected_headers_reject_ambiguous_or_unsafe_key_references() {
    let coordinate = URL_SAFE_NO_PAD.encode([7; 32]);
    let cases = [
        (
            format!(
                r#"{{"alg":"Ed25519","kid":"key-1","jwk":{{"kty":"OKP","crv":"Ed25519","x":"{coordinate}"}}}}"#
            ),
            JoseError::AmbiguousKeyReference,
        ),
        (
            format!(
                r#"{{"alg":"Ed25519","jwk":{{"kty":"OKP","crv":"Ed25519","x":"{coordinate}","d":"private"}}}}"#
            ),
            JoseError::InvalidHeaderValue,
        ),
        (
            r#"{"alg":"Ed25519","x5c":[]}"#.to_owned(),
            JoseError::InvalidHeaderValue,
        ),
        (
            r#"{"alg":"Ed25519","x5c":["not base64!"]}"#.to_owned(),
            JoseError::InvalidHeaderValue,
        ),
        (
            format!(
                r#"{{"alg":"Ed25519","x5c":[{}]}}"#,
                [r#""AQID""#; MAX_X5C_CERTIFICATES + 1].join(",")
            ),
            JoseError::InvalidHeaderValue,
        ),
    ];

    for (header, expected) in cases {
        let compact = compact_from_raw(header.as_bytes(), b"{}", &[1; 64]);
        assert_eq!(
            UnverifiedCompactJws::parse(&compact, JwsLimits::default()),
            Err(expected)
        );
    }

    assert_eq!(
        ProtectedHeader::with_key_reference(
            "Ed25519",
            None,
            Some(JwsKeyReference::X5c(Vec::new())),
            JwsLimits::default(),
        ),
        Err(JoseError::InvalidHeaderValue)
    );
    assert_eq!(
        ProtectedHeader::with_key_reference(
            "Ed25519",
            None,
            Some(JwsKeyReference::X5c(vec![
                "AQID".to_owned();
                MAX_X5C_CERTIFICATES + 1
            ])),
            JwsLimits::default(),
        ),
        Err(JoseError::InvalidHeaderValue)
    );
    let header_smaller_than_string =
        JwsLimits::new(65_536, 8, 64, 64, 1_024).expect("separate header bounds");
    assert_eq!(
        ProtectedHeader::with_key_reference(
            "Ed25519",
            None,
            Some(JwsKeyReference::X5c(vec!["AQIDAQIDAQID".to_owned()])),
            header_smaller_than_string,
        ),
        Err(JoseError::InvalidHeaderValue)
    );
}

#[test]
fn protected_header_serialization_stops_at_the_configured_byte_limit() {
    let mut extensions = BTreeMap::new();
    extensions.insert("large-public-metadata".to_owned(), json_string(16_384));
    let jwk = PublicKeyJwk::from_parts(
        identus_crypto::JwkKeyType::Okp,
        JwkCurve::Ed25519,
        &URL_SAFE_NO_PAD.encode([7; 32]),
        None,
        extensions,
    )
    .expect("structurally valid public JWK");
    let limits = JwsLimits::new(65_536, 256, 64, 64, 2_048).expect("limits");
    let header = ProtectedHeader::with_key_reference(
        "Ed25519",
        Some("openid4vci-proof+jwt"),
        Some(JwsKeyReference::Jwk(jwk)),
        limits,
    )
    .expect("header shape");

    assert_eq!(
        JwsSigningInput::new(header, b"{}".to_vec(), limits),
        Err(JoseError::ProtectedHeaderTooLarge)
    );
}

fn json_string(length: usize) -> serde_json::Value {
    serde_json::Value::String("x".repeat(length))
}

#[test]
fn enforces_each_configured_limit_at_the_boundary() {
    for invalid in [
        (0, 1, 1, 1, 1),
        (1, 0, 1, 1, 1),
        (1, 1, 0, 1, 1),
        (1, 1, 1, 0, 1),
        (1, 1, 1, 1, 0),
    ] {
        assert_eq!(
            JwsLimits::new(invalid.0, invalid.1, invalid.2, invalid.3, invalid.4),
            Err(JoseError::InvalidLimits)
        );
    }
    let defaults = JwsLimits::default();
    assert_eq!(defaults.max_compact_bytes(), 65_536);
    assert_eq!(defaults.max_protected_header_bytes(), 4_096);
    assert_eq!(defaults.max_payload_bytes(), 49_152);
    assert_eq!(defaults.max_signature_bytes(), 1_024);
    assert_eq!(defaults.max_header_string_bytes(), 2_048);

    let roomy = JwsLimits::new(4_096, 256, 3, 2, 8).expect("positive limits");
    let header =
        ProtectedHeader::new("EdDSA", Some("12345678"), None, roomy).expect("exact string bound");
    let exact = JwsSigningInput::new(header, vec![1, 2, 3], roomy)
        .expect("exact payload bound")
        .attach_signature(vec![4, 5])
        .expect("exact signature bound");
    assert!(UnverifiedCompactJws::parse(exact.compact(), roomy).is_ok());

    assert_eq!(
        ProtectedHeader::new("EdDSA", Some("123456789"), None, roomy),
        Err(JoseError::InvalidHeaderValue)
    );
    let algorithm_too_large = JwsLimits::new(4_096, 256, 3, 2, 4).expect("limits");
    assert_eq!(
        ProtectedHeader::new("EdDSA", None, None, algorithm_too_large),
        Err(JoseError::InvalidHeaderValue)
    );
    assert_eq!(
        JwsSigningInput::new(
            ProtectedHeader::new("EdDSA", None, None, roomy).expect("header"),
            vec![0; 4],
            roomy,
        ),
        Err(JoseError::PayloadTooLarge)
    );
    assert_eq!(
        JwsSigningInput::new(
            ProtectedHeader::new("EdDSA", None, None, roomy).expect("header"),
            Vec::new(),
            roomy,
        )
        .expect("prepared")
        .attach_signature(vec![0; 3]),
        Err(JoseError::SignatureTooLarge)
    );

    let compact = compact_from_raw(br#"{"alg":"EdDSA"}"#, b"1234", b"12");
    assert_eq!(
        UnverifiedCompactJws::parse(&compact, roomy),
        Err(JoseError::PayloadTooLarge)
    );
    let compact = compact_from_raw(br#"{"alg":"EdDSA"}"#, b"123", b"123");
    assert_eq!(
        UnverifiedCompactJws::parse(&compact, roomy),
        Err(JoseError::SignatureTooLarge)
    );
    let tiny_header = JwsLimits::new(4_096, 1, 64, 64, 64).expect("limits");
    assert_eq!(
        UnverifiedCompactJws::parse(exact.compact(), tiny_header),
        Err(JoseError::ProtectedHeaderTooLarge)
    );
    let tiny_compact = JwsLimits::new(8, 256, 64, 64, 64).expect("limits");
    assert_eq!(
        UnverifiedCompactJws::parse(exact.compact(), tiny_compact),
        Err(JoseError::CompactTooLarge)
    );

    let exact_compact =
        JwsLimits::new(exact.compact().len(), 256, 3, 2, 8).expect("exact compact limit");
    assert!(UnverifiedCompactJws::parse(exact.compact(), exact_compact).is_ok());
    let below_compact =
        JwsLimits::new(exact.compact().len() - 1, 256, 3, 2, 8).expect("below compact limit");
    assert_eq!(
        UnverifiedCompactJws::parse(exact.compact(), below_compact),
        Err(JoseError::CompactTooLarge)
    );

    let minimum_input = JwsSigningInput::new(
        ProtectedHeader::new("EdDSA", None, None, defaults).expect("header"),
        Vec::new(),
        defaults,
    )
    .expect("unbounded signing input");
    let minimum_compact_len = minimum_input.as_bytes().len() + 3;
    let exact_minimum = JwsLimits::new(minimum_compact_len, 256, 3, 2, 8).expect("limits");
    let exact_minimum_input = JwsSigningInput::new(
        ProtectedHeader::new("EdDSA", None, None, exact_minimum).expect("header"),
        Vec::new(),
        exact_minimum,
    )
    .expect("smallest signature fits");
    assert_eq!(
        exact_minimum_input
            .attach_signature(vec![0])
            .expect("one-byte signature")
            .compact()
            .len(),
        minimum_compact_len
    );
    let unusable_compact = JwsLimits::new(minimum_compact_len - 1, 256, 3, 2, 8).expect("limits");
    assert_eq!(
        JwsSigningInput::new(
            ProtectedHeader::new("EdDSA", None, None, unusable_compact).expect("header"),
            Vec::new(),
            unusable_compact,
        ),
        Err(JoseError::CompactTooLarge)
    );

    let raw_header = br#"{"alg":"EdDSA"}"#;
    let header_compact = compact_from_raw(raw_header, b"123", b"12");
    let exact_header =
        JwsLimits::new(4_096, raw_header.len(), 3, 2, 8).expect("exact header limit");
    assert!(UnverifiedCompactJws::parse(&header_compact, exact_header).is_ok());
    let below_header =
        JwsLimits::new(4_096, raw_header.len() - 1, 3, 2, 8).expect("below header limit");
    assert_eq!(
        UnverifiedCompactJws::parse(&header_compact, below_header),
        Err(JoseError::ProtectedHeaderTooLarge)
    );
}

#[test]
fn deterministic_bounded_matrix_round_trips_exact_bytes() {
    let payload_lengths = [0, 1, 2, 3, 31, 32, 33, 255, 1_024];
    let signature_lengths = [1, 2, 3, 63, 64, 65, 127];
    for payload_len in payload_lengths {
        for signature_len in signature_lengths {
            let payload = (0..payload_len)
                .map(|index| (index % 251) as u8)
                .collect::<Vec<_>>();
            let signature = (0..signature_len)
                .map(|index| (255 - index % 251) as u8)
                .collect::<Vec<_>>();
            let prepared =
                JwsSigningInput::new(proof_header(), payload.clone(), JwsLimits::default())
                    .expect("bounded input");
            let signing_input = prepared.as_bytes().to_vec();
            let encoded = prepared
                .attach_signature(signature.clone())
                .expect("bounded signature");
            let parsed = UnverifiedCompactJws::parse(encoded.compact(), JwsLimits::default())
                .expect("canonical output parses");
            assert_eq!(parsed.payload(), payload);
            assert_eq!(parsed.signature(), signature);
            assert_eq!(parsed.signing_input(), signing_input);
            assert_eq!(parsed.compact(), encoded.compact());
        }
    }
}

#[test]
fn diagnostics_are_static_and_redact_content() {
    let key_canary = "kid-canary-never-render";
    let payload_canary = b"payload-canary-never-render";
    let signature_canary = b"signature-canary-never-render";
    let header = ProtectedHeader::new(
        "EdDSA",
        Some("proof+jwt"),
        Some(key_canary),
        JwsLimits::default(),
    )
    .expect("header");
    let prepared = JwsSigningInput::new(
        header.clone(),
        payload_canary.to_vec(),
        JwsLimits::default(),
    )
    .expect("prepared");
    let prepared_debug = format!("{prepared:?}");
    let header_debug = format!("{header:?}");
    let parsed = prepared
        .attach_signature(signature_canary.to_vec())
        .expect("attached");
    let parsed_debug = format!("{parsed:?}");

    for rendered in [&prepared_debug, &header_debug, &parsed_debug] {
        assert!(!rendered.contains(key_canary));
        assert!(!rendered.contains("payload-canary"));
        assert!(!rendered.contains("signature-canary"));
    }
    for error in [
        JoseError::InvalidCompactStructure,
        JoseError::NonCanonicalBase64Url,
        JoseError::InvalidProtectedHeader,
        JoseError::InvalidHeaderValue,
    ] {
        let rendered = format!("{error:?} {error}");
        let bridged: IdentusError = error.into();
        let bridged_rendered = format!("{bridged:?} {bridged}");
        assert!(!rendered.contains("canary"));
        assert!(!bridged_rendered.contains("canary"));
        assert_eq!(bridged.capability().expect("capability").as_str(), "jose");
        assert!(bridged.code().as_str().starts_with("jose."));
    }
}

#[test]
#[ignore = "release-only diagnostic; informational, not a correctness threshold"]
fn parse_throughput_diagnostic() {
    let compact = JwsSigningInput::new(
        proof_header(),
        br#"{"aud":"https://issuer.example","iat":1701960444,"nonce":"fresh"}"#.to_vec(),
        JwsLimits::default(),
    )
    .expect("prepared")
    .attach_signature(vec![0x5a; 64])
    .expect("signed");
    let iterations = 100_000_u32;
    let started = Instant::now();
    for _ in 0..iterations {
        let parsed =
            UnverifiedCompactJws::parse(black_box(compact.compact()), JwsLimits::default())
                .expect("parse diagnostic fixture");
        assert_eq!(parsed.signature().len(), 64);
        black_box(parsed);
    }
    let elapsed = started.elapsed();
    let operations_per_second = f64::from(iterations) / elapsed.as_secs_f64();
    eprintln!(
        "jws compact parse: {iterations} iterations in {elapsed:?} ({operations_per_second:.0} ops/s)"
    );
}

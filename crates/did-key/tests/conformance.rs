use identus_crypto::{
    Ed25519PrivateKey, Ed25519PublicKey, P256PrivateKey, Secp256k1PrivateKey, X25519PrivateKey,
};
use identus_did_key::{DidKey, Error, PublicKey};

#[test]
fn attributed_vector_packet_matches_selected_profile() {
    let packet: serde_json::Value = serde_json::from_str(include_str!(
        "../../../docs/conformance/fixtures/did-key/v1/vectors.json"
    ))
    .unwrap();
    let cases = packet["cases"].as_array().unwrap();
    for case in cases {
        let input = case["input"]["literal"].as_str().unwrap();
        let expected = case["expected"]["kind"].as_str().unwrap();
        match (DidKey::parse(input), expected) {
            (Ok(parsed), "success") => {
                let curve = match parsed.public_key() {
                    PublicKey::Ed25519(_) => "Ed25519",
                    PublicKey::X25519(_) => "X25519",
                    PublicKey::P256(_) => "P-256",
                    PublicKey::Secp256k1(_) => "secp256k1",
                    _ => panic!("fixture reached an unsupported future key profile"),
                };
                assert_eq!(curve, case["expected"]["fields"]["curve"]);
                let expected_count = case["expected"]["fields"]["relationshipCount"]
                    .as_u64()
                    .unwrap();
                let document = parsed.resolve().unwrap();
                let relationship_count = document.authentication().map_or(0, <[_]>::len)
                    + document.assertion_method().map_or(0, <[_]>::len)
                    + document.key_agreement().map_or(0, <[_]>::len)
                    + document.capability_invocation().map_or(0, <[_]>::len)
                    + document.capability_delegation().map_or(0, <[_]>::len);
                assert_eq!(u64::try_from(relationship_count).unwrap(), expected_count);
            }
            (Err(Error::MalformedEncoding), "error") => {
                assert_eq!(case["expected"]["redacted"], true);
            }
            (actual, _) => panic!("unexpected vector outcome: {actual:?}"),
        }
    }
}

#[test]
fn pinned_ed25519_example_round_trips() {
    let value = "did:key:z6MkiToqovww7vYtxm1xNM15u9JzqzUFZ1k7s7MazYJUyAxv";
    let parsed = DidKey::parse(value).expect("pinned Ed25519 example");
    assert!(matches!(parsed.public_key(), PublicKey::Ed25519(_)));
    assert_eq!(parsed.did().as_str(), value);
    assert_eq!(DidKey::create(parsed.public_key().clone()).unwrap(), parsed);
}

#[test]
fn direct_x25519_projects_only_key_agreement() {
    let value = "did:key:z6LSj72tK8brWgZja8NLRwfiAcBXQWZJL6n1AP1zFjvVUtoA";
    let document = DidKey::parse(value).unwrap().resolve().unwrap();
    assert_eq!(document.key_agreement().unwrap().len(), 1);
    assert!(document.authentication().is_none());
    assert!(document.assertion_method().is_none());
    assert!(document.capability_invocation().is_none());
    assert!(document.capability_delegation().is_none());
}

#[test]
fn signing_keys_project_four_relationships() {
    let keys = [
        PublicKey::from(
            Ed25519PrivateKey::from_slice(&[7; 32])
                .unwrap()
                .to_public_key(),
        ),
        PublicKey::from(
            P256PrivateKey::from_slice(&[8; 32])
                .unwrap()
                .to_public_key(),
        ),
        PublicKey::from(
            Secp256k1PrivateKey::from_slice(&[9; 32])
                .unwrap()
                .to_public_key(),
        ),
    ];
    for key in keys {
        let document = DidKey::create(key).unwrap().resolve().unwrap();
        assert_eq!(document.authentication().unwrap().len(), 1);
        assert_eq!(document.assertion_method().unwrap().len(), 1);
        assert_eq!(document.capability_invocation().unwrap().len(), 1);
        assert_eq!(document.capability_delegation().unwrap().len(), 1);
        assert!(document.key_agreement().is_none());
    }
}

#[test]
fn constructor_parser_property_covers_deterministic_public_material() {
    for seed in 1u8..=64 {
        let ed = Ed25519PrivateKey::from_slice(&[seed; 32])
            .unwrap()
            .to_public_key();
        let created = DidKey::create(ed).unwrap();
        assert_eq!(DidKey::parse(created.did().as_str()).unwrap(), created);

        let x = X25519PrivateKey::from_slice(&[seed; 32])
            .unwrap()
            .to_public_key();
        let created = DidKey::create(x).unwrap();
        assert_eq!(DidKey::parse(created.did().as_str()).unwrap(), created);
    }
}

#[test]
fn rejects_aliased_and_weak_ed25519_keys() {
    let mut non_canonical = [0xff; 32];
    non_canonical[0] = 0xf0;
    non_canonical[31] = 0x7f;
    let non_canonical_key = Ed25519PublicKey::from_slice(&non_canonical).unwrap();
    assert_eq!(
        DidKey::create(non_canonical_key),
        Err(Error::InvalidPublicKey)
    );

    let mut fingerprint = vec![0xed, 0x01];
    fingerprint.extend(non_canonical);
    let did = format!("did:key:z{}", bs58::encode(fingerprint).into_string());
    assert_eq!(DidKey::parse(&did), Err(Error::InvalidPublicKey));

    let mut identity = [0; 32];
    identity[0] = 1;
    let identity_key = Ed25519PublicKey::from_slice(&identity).unwrap();
    assert_eq!(DidKey::create(identity_key), Err(Error::InvalidPublicKey));

    let mut fingerprint = vec![0xed, 0x01];
    fingerprint.extend(identity);
    let did = format!("did:key:z{}", bs58::encode(fingerprint).into_string());
    assert_eq!(DidKey::parse(&did), Err(Error::InvalidPublicKey));
}

#[test]
fn rejects_wrong_method_encoding_codec_length_and_point() {
    assert_eq!(
        DidKey::parse("did:web:example.com"),
        Err(Error::WrongMethod)
    );
    assert_eq!(DidKey::parse("did:key:uAA"), Err(Error::MalformedEncoding));
    assert_eq!(DidKey::parse("did:key:z"), Err(Error::MalformedEncoding));

    let unsupported = format!("did:key:z{}", bs58::encode([0x01, 0x00]).into_string());
    assert_eq!(DidKey::parse(&unsupported), Err(Error::UnsupportedCodec));

    let bad_length = format!(
        "did:key:z{}",
        bs58::encode([0xed, 0x01, 0x00]).into_string()
    );
    assert_eq!(DidKey::parse(&bad_length), Err(Error::InvalidPublicKey));

    let mut invalid_point = vec![0x80, 0x24];
    invalid_point.extend([0; 33]);
    let invalid_point = format!("did:key:z{}", bs58::encode(invalid_point).into_string());
    assert_eq!(DidKey::parse(&invalid_point), Err(Error::InvalidPublicKey));

    let oversized = format!("did:key:z{}", "1".repeat(128));
    assert_eq!(DidKey::parse(&oversized), Err(Error::ResourceLimit));
}

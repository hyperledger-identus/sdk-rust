#![no_main]

use identus_did::{
    DidDocument, MAX_DID_DOCUMENT_BYTES, VerificationMethod, VerificationRelationship,
};
use libfuzzer_sys::fuzz_target;

const PRIVATE_JWK_MEMBERS: [&str; 8] = ["d", "p", "q", "dp", "dq", "qi", "oth", "k"];

fuzz_target!(|data: &[u8]| {
    let parsed = DidDocument::from_json_slice(data);
    if data.len() > MAX_DID_DOCUMENT_BYTES {
        assert!(parsed.is_err());
        return;
    }

    let Ok(document) = parsed else {
        return;
    };

    let serialized = serde_json::to_vec(&document).expect("validated document must serialize");
    let reparsed = DidDocument::from_json_slice(&serialized)
        .expect("serialized validated document must reparse");
    assert_eq!(document, reparsed);
    assert_eq!(
        serialized,
        serde_json::to_vec(&reparsed).expect("reparsed document must serialize")
    );

    assert_public_methods(document.verification_methods().unwrap_or_default());
    for relationships in [
        document.authentication(),
        document.assertion_method(),
        document.key_agreement(),
        document.capability_invocation(),
        document.capability_delegation(),
    ]
    .into_iter()
    .flatten()
    {
        for relationship in relationships {
            if let VerificationRelationship::Embedded(method) = relationship {
                assert_public_method(method);
            }
        }
    }
});

fn assert_public_methods(methods: &[VerificationMethod]) {
    for method in methods {
        assert_public_method(method);
    }
}

fn assert_public_method(method: &VerificationMethod) {
    if let Some(jwk) = method.public_key_jwk() {
        for private_member in PRIVATE_JWK_MEMBERS {
            assert!(!jwk.contains_key(private_member));
        }
    }
}

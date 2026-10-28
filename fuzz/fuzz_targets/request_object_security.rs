#![no_main]

use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits, REQUEST_OBJECT_JWT_TYPE,
    REQUEST_OBJECT_MEDIA_TYPE, RequestObjectValidationLimits, RequestUriRetrievalInput,
    RequestUriRetrievalLimits,
};
use libfuzzer_sys::fuzz_target;
use serde_json::{Value, json};

mod oid4vp_support;

const PRIVATE: [u8; 32] = [0x71; 32];
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";

fuzz_target!(|data: &[u8]| {
    let Some((&selector, bytes)) = data.split_first() else {
        return;
    };
    let arbitrary = String::from_utf8_lossy(bytes);
    let mut request = json!({
        "client_id": "decentralized_identifier:did:example:verifier",
    });
    let object = request.as_object_mut().expect("request object");
    if selector == b'S' {
        object.insert(
            "aud".to_owned(),
            json!(identus_oid4vp::STATIC_DISCOVERY_AUDIENCE),
        );
        object.insert("iat".to_owned(), json!(oid4vp_support::NOW_SECONDS));
        object.insert("jti".to_owned(), json!("static-seed"));
    } else {
        object.insert("aud".to_owned(), audience(selector, &arbitrary));
        object.insert("iat".to_owned(), numeric_date(selector >> 3, bytes));
        if selector & 0x40 != 0 {
            object.insert("exp".to_owned(), numeric_date(selector >> 1, bytes));
        }
        if selector & 0x20 != 0 {
            object.insert("nbf".to_owned(), numeric_date(selector >> 2, bytes));
        }
        if selector & 0x10 != 0 {
            let jwt_id = if selector & 0x08 == 0 {
                Value::String(arbitrary.into_owned())
            } else {
                json!(selector)
            };
            object.insert("jti".to_owned(), jwt_id);
        }
    }
    let payload = serde_json::to_vec(&request).expect("JSON");
    let private = Ed25519PrivateKey::from_slice(&PRIVATE).expect("fixed private key");
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("header");
    let Ok(input) = JwsSigningInput::new(header, payload, JwsLimits::default()) else {
        return;
    };
    let compact = input
        .sign_with(&Ed25519Signer::new(&private))
        .expect("fixed signer")
        .compact()
        .to_owned();
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("invocation");
    let Ok(object) = reference
        .prepare_retrieval(
            RequestUriRetrievalInput::get(),
            RequestUriRetrievalLimits::default(),
        )
        .expect("retrieval")
        .bind_response(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            RequestObjectValidationLimits::default(),
        )
    else {
        return;
    };
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key");
    if let Ok(verified) = object.verify(&SignatureSuiteRegistry::recommended(), &key) {
        let _ = oid4vp_support::secure(verified);
    }
});

fn audience(selector: u8, arbitrary: &str) -> Value {
    match selector & 0x07 {
        0 => json!(identus_oid4vp::STATIC_DISCOVERY_AUDIENCE),
        1 => json!(arbitrary),
        2 => json!([identus_oid4vp::STATIC_DISCOVERY_AUDIENCE, arbitrary]),
        3 => json!([]),
        4 => json!([identus_oid4vp::STATIC_DISCOVERY_AUDIENCE, selector]),
        5 => json!(selector),
        6 => Value::Null,
        _ => json!([arbitrary]),
    }
}

fn numeric_date(selector: u8, bytes: &[u8]) -> Value {
    let mut encoded = [0_u8; 8];
    let retained = bytes.len().min(encoded.len());
    encoded[..retained].copy_from_slice(&bytes[..retained]);
    let arbitrary = u64::from_le_bytes(encoded);
    match selector & 0x07 {
        0 => json!(oid4vp_support::NOW_SECONDS),
        1 => json!(arbitrary),
        2 => json!(u64::MAX),
        3 => json!(oid4vp_support::NOW_SECONDS + 31),
        4 => json!(1.5),
        5 => json!(-1),
        6 => json!(arbitrary.to_string()),
        _ => Value::Null,
    }
}

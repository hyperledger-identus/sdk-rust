#![no_main]

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_jose::{
    Ed25519Signer, JweLimits, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey,
    ProtectedHeader, SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, CredentialFormat, CredentialFormatCapability,
    CredentialFormatCapabilityError, DcqlLimits, REQUEST_OBJECT_JWT_TYPE,
    REQUEST_OBJECT_MEDIA_TYPE, RequestObjectValidationLimits, RequestUriRetrievalInput,
    RequestUriRetrievalLimits, TransactionDataCapability, TransactionDataCapabilityError,
    TransactionDataLimits,
};
use libfuzzer_sys::fuzz_target;
use serde_json::{Value, json};

mod oid4vp_support;

const PRIVATE: [u8; 32] = [0x71; 32];
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";

struct Format;
impl CredentialFormatCapability for Format {
    fn format(&self) -> CredentialFormat {
        CredentialFormat::DcSdJwt
    }

    fn validate_metadata(&self, metadata: &Value) -> Result<(), CredentialFormatCapabilityError> {
        metadata
            .as_object()
            .filter(|value| value.is_empty())
            .map(|_| ())
            .ok_or(CredentialFormatCapabilityError::InvalidMetadata)
    }

    fn supports_transaction_type(&self, transaction_type: &str) -> bool {
        transaction_type == "fuzz"
    }
}

struct Transaction;
impl TransactionDataCapability for Transaction {
    fn transaction_type(&self) -> &str {
        "fuzz"
    }

    fn validate(&self, value: &Value) -> Result<(), TransactionDataCapabilityError> {
        value
            .as_object()
            .map(|_| ())
            .ok_or(TransactionDataCapabilityError::Invalid)
    }
}

fuzz_target!(|data: &[u8]| {
    let Some((&selector, bytes)) = data.split_first() else {
        return;
    };
    let transaction = match selector & 0x03 {
        0 => String::from_utf8_lossy(bytes).into_owned(),
        1 => URL_SAFE_NO_PAD.encode(bytes),
        2 => URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&json!({
                "type": "fuzz",
                "credential_ids": [String::from_utf8_lossy(bytes)],
            }))
            .expect("JSON"),
        ),
        _ => URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&json!({
                "type": "fuzz",
                "credential_ids": ["credential"],
                "data": String::from_utf8_lossy(bytes),
            }))
            .expect("JSON"),
        ),
    };
    let request = json!({
        "client_id": "decentralized_identifier:did:example:verifier",
        "aud": identus_oid4vp::STATIC_DISCOVERY_AUDIENCE,
        "iat": oid4vp_support::NOW_SECONDS,
        "response_type": "vp_token",
        "response_mode": "direct_post",
        "nonce": "fuzz-nonce",
        "response_uri": "https://verifier.example/response",
        "dcql_query": {"credentials": [{
            "id": "credential", "format": "dc+sd-jwt", "meta": {}
        }]},
        "transaction_data": [transaction]
    });
    let private = Ed25519PrivateKey::from_slice(&PRIVATE).expect("fixed key");
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("header");
    let Ok(input) = JwsSigningInput::new(
        header,
        serde_json::to_vec(&request).expect("JSON"),
        JwsLimits::default(),
    ) else {
        return;
    };
    let compact = input
        .sign_with(&Ed25519Signer::new(&private))
        .expect("sign")
        .compact()
        .to_owned();
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("invocation");
    let Ok(unverified) = reference
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
    let Ok(verified) = unverified.verify(&SignatureSuiteRegistry::recommended(), &key) else {
        return;
    };
    let Some(secured) = oid4vp_support::secure(verified) else {
        return;
    };
    let Some(authorized) = oid4vp_support::authorize(secured) else {
        return;
    };
    let format = Format;
    let transaction = Transaction;
    let _ = authorized.into_authorization_request_with_transaction_capabilities(
        AuthorizationRequestValidationLimits::default(),
        DcqlLimits::default(),
        JweLimits::default(),
        TransactionDataLimits::default(),
        &[&format],
        &[&transaction],
    );
});

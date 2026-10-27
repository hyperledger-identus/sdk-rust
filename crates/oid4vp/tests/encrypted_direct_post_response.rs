use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, P256PrivateKey};
use identus_jose::{
    CompactJwe, Ed25519Signer, JweEncrypter, JweEncryptionError, JweEncryptionRequest, JweLimits,
    JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, AuthorizationResponseMode, DcqlLimits,
    DirectPostPresentation, DirectPostQueryResult, DirectPostResponseLimits, Oid4vpError,
    REQUEST_OBJECT_JWT_TYPE, REQUEST_OBJECT_MEDIA_TYPE, RequestObjectValidationLimits,
    RequestUriRetrievalInput, RequestUriRetrievalLimits,
};
use serde_json::{Value, json};

const REQUEST_SIGNING_KEY: [u8; 32] = [0x71; 32];
const RESPONSE_KEY: [u8; 32] = [0x42; 32];
const CLIENT_ID: &str = "decentralized_identifier:did:example:verifier";
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";
const PACKET: &str =
    include_str!("../../../docs/conformance/fixtures/oid4vp-encrypted-direct-post/v1/vectors.json");

fn compact_response(key_id: &str) -> String {
    let point = P256PrivateKey::from_slice(&[0x24; 32])
        .expect("ephemeral key")
        .to_public_key()
        .encode_uncompressed();
    let header = json!({
        "alg": "ECDH-ES",
        "enc": "A128GCM",
        "kid": key_id,
        "epk": {
            "kty": "EC",
            "crv": "P-256",
            "x": URL_SAFE_NO_PAD.encode(&point[1..33]),
            "y": URL_SAFE_NO_PAD.encode(&point[33..65])
        }
    });
    format!(
        "{}..{}.{}.{}",
        URL_SAFE_NO_PAD.encode(serde_json::to_vec(&header).expect("header")),
        URL_SAFE_NO_PAD.encode([1_u8; 12]),
        URL_SAFE_NO_PAD.encode(b"ciphertext"),
        URL_SAFE_NO_PAD.encode([2_u8; 16])
    )
}

fn signed(payload: &[u8]) -> String {
    let private = Ed25519PrivateKey::from_slice(&REQUEST_SIGNING_KEY).expect("key");
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#request-key"),
        JwsLimits::default(),
    )
    .expect("header");
    JwsSigningInput::new(header, payload.to_vec(), JwsLimits::default())
        .expect("input")
        .sign_with(&Ed25519Signer::new(&private))
        .expect("signature")
        .compact()
        .to_owned()
}

fn encrypted_request() -> Value {
    let point = P256PrivateKey::from_slice(&RESPONSE_KEY)
        .expect("key")
        .to_public_key()
        .encode_uncompressed();
    json!({
        "client_id": CLIENT_ID,
        "response_type": "vp_token",
        "response_mode": "direct_post.jwt",
        "nonce": "fresh-nonce_1",
        "response_uri": "https://verifier.example/response",
        "state": "opaque-state",
        "client_metadata": {
            "jwks": {"keys": [{
                "kty": "EC",
                "crv": "P-256",
                "alg": "ECDH-ES",
                "use": "enc",
                "key_ops": ["deriveKey"],
                "kid": "response-key",
                "x": URL_SAFE_NO_PAD.encode(&point[1..33]),
                "y": URL_SAFE_NO_PAD.encode(&point[33..65])
            }]},
            "encrypted_response_enc_values_supported": ["A128GCM"]
        },
        "dcql_query": {"credentials": [{
            "id": "degree",
            "format": "dc+sd-jwt",
            "meta": {}
        }]}
    })
}

fn plain_request() -> Value {
    let mut request = encrypted_request();
    request["response_mode"] = json!("direct_post");
    request
        .as_object_mut()
        .expect("request object")
        .remove("client_metadata");
    request
}

fn validated(
    request: &Value,
    jwe_limits: JweLimits,
) -> identus_oid4vp::ValidatedAuthorizationRequest {
    let payload = serde_json::to_vec(request).expect("request JSON");
    let compact = signed(&payload);
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("invocation");
    let object = reference
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
        .expect("binding");
    let public = Ed25519PrivateKey::from_slice(&REQUEST_SIGNING_KEY)
        .expect("key")
        .to_public_key()
        .encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("verification key");
    object
        .verify(&SignatureSuiteRegistry::recommended(), &key)
        .expect("verified")
        .into_authorization_request_with_jwe_limits(
            AuthorizationRequestValidationLimits::default(),
            DcqlLimits::default(),
            jwe_limits,
        )
        .expect("validated")
}

fn result(limits: DirectPostResponseLimits) -> DirectPostQueryResult {
    let presentation =
        DirectPostPresentation::string("eyJ2cCI6dHJ1ZX0", limits).expect("presentation");
    DirectPostQueryResult::new("degree", vec![presentation], limits).expect("result")
}

#[derive(Debug, PartialEq, Eq)]
struct Seen {
    key_id: Option<String>,
    key: [u8; 65],
    algorithm: &'static str,
    encryption: &'static str,
    plaintext: Vec<u8>,
    maximum: usize,
}

struct RecordingEncrypter {
    calls: AtomicUsize,
    seen: Mutex<Option<Seen>>,
}

struct WrongKeyIdEncrypter;

impl JweEncrypter for WrongKeyIdEncrypter {
    fn encrypt(&self, _request: JweEncryptionRequest) -> Result<CompactJwe, JweEncryptionError> {
        CompactJwe::new(compact_response("different-key"), 8_192)
    }
}

impl RecordingEncrypter {
    fn new() -> Self {
        Self {
            calls: AtomicUsize::new(0),
            seen: Mutex::new(None),
        }
    }
}

impl JweEncrypter for RecordingEncrypter {
    fn encrypt(&self, request: JweEncryptionRequest) -> Result<CompactJwe, JweEncryptionError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let key_id = request.recipient().expose_key_id().unwrap_or("missing");
        *self.seen.lock().expect("seen") = Some(Seen {
            key_id: request.recipient().expose_key_id().map(str::to_owned),
            key: request.recipient().expose_public_sec1(),
            algorithm: request.key_management_algorithm().as_str(),
            encryption: request.content_encryption_algorithm().as_str(),
            plaintext: request.expose_sensitive_plaintext().to_vec(),
            maximum: request.max_compact_bytes(),
        });
        CompactJwe::new(compact_response(key_id), 8_192)
    }
}

#[test]
fn e2e_verified_request_selects_key_encrypts_once_and_emits_only_response() {
    let packet: Value = serde_json::from_str(PACKET).expect("packet");
    let case = &packet["cases"][0];
    assert_eq!(
        case["id"],
        "oid4vp.encrypted-direct-post-response.final-p256-a128gcm"
    );
    let jwe_limits = JweLimits::new(4_096, 1_024, 64).expect("limits");
    let response_limits = DirectPostResponseLimits::default();
    let request = validated(&encrypted_request(), jwe_limits);
    assert_eq!(
        request.response_mode(),
        AuthorizationResponseMode::DirectPostJwt
    );
    let encrypter = RecordingEncrypter::new();
    let response = request
        .into_encrypted_direct_post_response(
            vec![result(response_limits)],
            response_limits,
            jwe_limits,
            &encrypter,
        )
        .expect("response");

    assert_eq!(encrypter.calls.load(Ordering::SeqCst), 1);
    let seen = encrypter.seen.lock().expect("seen");
    let seen = seen.as_ref().expect("request captured");
    assert_eq!(seen.key_id.as_deref(), Some("response-key"));
    assert_eq!(seen.algorithm, "ECDH-ES");
    assert_eq!(seen.encryption, "A128GCM");
    assert_eq!(seen.maximum, 1_024);
    assert_eq!(
        seen.plaintext,
        br#"{"vp_token":{"degree":["eyJ2cCI6dHJ1ZX0"]},"state":"opaque-state"}"#
    );
    assert_eq!(
        response.expose_sensitive_target(),
        "https://verifier.example/response"
    );
    let compact = compact_response("response-key");
    assert_eq!(
        response.expose_sensitive_form_body(),
        format!("response={compact}")
    );
    assert_eq!(
        response.expose_sensitive_form_body(),
        case["expected"]["fields"]["serialized"]
            .as_str()
            .expect("serialized")
    );
    assert!(!response.expose_sensitive_form_body().contains("vp_token"));
    assert!(
        !response
            .expose_sensitive_form_body()
            .contains("opaque-state")
    );
    assert!(response.state_present());
    assert_eq!(response.query_result_count(), 1);
    assert!(!format!("{response:?}").contains(&compact));
}

#[test]
fn metadata_rejects_private_ambiguous_and_unsupported_keys() {
    let limits = JweLimits::default();
    let mut private = encrypted_request();
    private["client_metadata"]["jwks"]["keys"][0]["d"] = json!("private");
    assert_eq!(
        validation_error(&private, limits),
        Oid4vpError::InvalidRequestObjectPayload
    );

    let mut ambiguous = encrypted_request();
    let duplicate = ambiguous["client_metadata"]["jwks"]["keys"][0].clone();
    ambiguous["client_metadata"]["jwks"]["keys"]
        .as_array_mut()
        .expect("keys")
        .push(duplicate);
    assert_eq!(
        validation_error(&ambiguous, limits),
        Oid4vpError::InvalidRequestObjectPayload
    );

    let mut unsupported = encrypted_request();
    unsupported["client_metadata"]["encrypted_response_enc_values_supported"] = json!(["A256GCM"]);
    assert_eq!(
        validation_error(&unsupported, limits),
        Oid4vpError::UnsupportedResponseMode
    );

    for (member, value) in [("use", json!("sig")), ("key_ops", json!(["sign"]))] {
        let mut incompatible = encrypted_request();
        incompatible["client_metadata"]["jwks"]["keys"][0][member] = value;
        assert_eq!(
            validation_error(&incompatible, limits),
            Oid4vpError::InvalidRequestObjectPayload
        );
    }

    for algorithm in [None, Some("RSA-OAEP")] {
        let mut incompatible = encrypted_request();
        let key = incompatible["client_metadata"]["jwks"]["keys"][0]
            .as_object_mut()
            .expect("key object");
        match algorithm {
            Some(value) => {
                key.insert("alg".to_owned(), json!(value));
            }
            None => {
                key.remove("alg");
            }
        }
        assert_eq!(
            validation_error(&incompatible, limits),
            Oid4vpError::InvalidRequestObjectPayload
        );
    }

    let mut too_many_keys = encrypted_request();
    let key = too_many_keys["client_metadata"]["jwks"]["keys"][0].clone();
    too_many_keys["client_metadata"]["jwks"]["keys"] = Value::Array(vec![key; 17]);
    assert_eq!(
        validation_error(&too_many_keys, limits),
        Oid4vpError::InvalidRequestObjectPayload
    );

    let mut too_many_enc_values = encrypted_request();
    let mut enc_values: Vec<_> = (0..16).map(|index| json!(format!("enc-{index}"))).collect();
    enc_values.push(json!("A128GCM"));
    too_many_enc_values["client_metadata"]["encrypted_response_enc_values_supported"] =
        Value::Array(enc_values);
    assert_eq!(
        validation_error(&too_many_enc_values, limits),
        Oid4vpError::InvalidRequestObjectPayload
    );

    let mut default_enc = encrypted_request();
    default_enc["client_metadata"]
        .as_object_mut()
        .expect("metadata object")
        .remove("encrypted_response_enc_values_supported");
    assert_eq!(
        validated(&default_enc, limits).response_mode(),
        AuthorizationResponseMode::DirectPostJwt
    );

    let mut no_key_id = encrypted_request();
    no_key_id["client_metadata"]["jwks"]["keys"][0]
        .as_object_mut()
        .expect("key object")
        .remove("kid");
    assert_eq!(
        validated(&no_key_id, limits).response_mode(),
        AuthorizationResponseMode::DirectPostJwt
    );
}

fn validation_error(request: &Value, limits: JweLimits) -> Oid4vpError {
    let payload = serde_json::to_vec(request).expect("JSON");
    let compact = signed(&payload);
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("invocation");
    let object = reference
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
        .expect("binding");
    let public = Ed25519PrivateKey::from_slice(&REQUEST_SIGNING_KEY)
        .expect("key")
        .to_public_key()
        .encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("key");
    object
        .verify(&SignatureSuiteRegistry::recommended(), &key)
        .expect("verified")
        .into_authorization_request_with_jwe_limits(
            AuthorizationRequestValidationLimits::default(),
            DcqlLimits::default(),
            limits,
        )
        .unwrap_err()
}

#[test]
fn stop_boundaries_precede_encryption_and_plain_mode_cannot_use_protected_constructor() {
    let jwe_limits = JweLimits::new(4_096, 1_024, 64).expect("limits");
    let encrypter = RecordingEncrypter::new();
    let narrow_form = DirectPostResponseLimits::new(1, 32, 1, 128, 8, 32, 512, 64).expect("limits");
    assert_eq!(
        validated(&encrypted_request(), jwe_limits)
            .into_encrypted_direct_post_response(
                vec![result(narrow_form)],
                narrow_form,
                jwe_limits,
                &encrypter,
            )
            .unwrap_err(),
        Oid4vpError::DirectPostFormTooLarge
    );
    assert_eq!(encrypter.calls.load(Ordering::SeqCst), 0);

    let response_limits = DirectPostResponseLimits::default();
    assert_eq!(
        validated(&plain_request(), jwe_limits)
            .into_encrypted_direct_post_response(
                vec![result(response_limits)],
                response_limits,
                jwe_limits,
                &encrypter,
            )
            .unwrap_err(),
        Oid4vpError::ResponseModeMismatch
    );
    assert_eq!(encrypter.calls.load(Ordering::SeqCst), 0);

    assert_eq!(
        validated(&encrypted_request(), jwe_limits)
            .into_direct_post_response(vec![result(response_limits)], response_limits)
            .unwrap_err(),
        Oid4vpError::ResponseModeMismatch
    );

    assert_eq!(
        validated(&encrypted_request(), jwe_limits)
            .into_encrypted_direct_post_response(
                vec![result(response_limits)],
                response_limits,
                jwe_limits,
                &WrongKeyIdEncrypter,
            )
            .unwrap_err(),
        Oid4vpError::ResponseEncryptionFailed
    );
}

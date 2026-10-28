use std::{hint::black_box, time::Instant};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, CredentialFormat, CredentialFormatCapability,
    CredentialFormatCapabilityError, DcqlLimits, DirectPostAcknowledgementLimits,
    DirectPostErrorCode, DirectPostErrorDetails, DirectPostErrorResponseLimits,
    DirectPostPresentation, DirectPostQueryResult, DirectPostResponseLimits,
    REQUEST_OBJECT_JWT_TYPE, REQUEST_OBJECT_MEDIA_TYPE, RedirectResponseLimits,
    RequestObjectValidationLimits, RequestUriRetrievalInput, RequestUriRetrievalLimits,
    SelectedCredentialInput, TransactionDataCapability, TransactionDataCapabilityError,
    TransactionDataLimits, ValidatedAuthorizationRequest, VerifierAuthorizationEvidence,
};
use serde_json::{Value, json};

mod support;

const SAMPLES: usize = 21;
const ITERATIONS: usize = 20;
const PRIVATE_BYTES: [u8; 32] = [0x71; 32];
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";

fn main() {
    measure("ordinary.end-to-end", || construct("compact"));
    let boundary = "p".repeat(60 * 1_024);
    measure("boundary.end-to-end", || construct(black_box(&boundary)));
    measure("error.ordinary.end-to-end", || {
        construct_error("unsupported client_id_prefix")
    });
    let error_boundary = "e".repeat(1_024);
    measure("error.boundary.end-to-end", || {
        construct_error(black_box(&error_boundary))
    });
    measure("fragment.ordinary.end-to-end", || {
        construct_redirect("fragment", "compact")
    });
    measure("fragment.boundary.end-to-end", || {
        construct_redirect("fragment", black_box(&boundary))
    });
    measure(
        "acknowledgement.ordinary.end-to-end",
        construct_acknowledgement,
    );
    measure("transaction.ordinary.end-to-end", || {
        construct_transaction(32)
    });
    measure("transaction.boundary.end-to-end", || {
        construct_transaction(4_096)
    });
    println!("oid4vp-direct-post-benchmark allocation_observation=not_available");
}

fn construct(presentation: &str) {
    let request = validated_request();
    let limits = DirectPostResponseLimits::default();
    let result = DirectPostQueryResult::new(
        "degree",
        vec![DirectPostPresentation::string(presentation, limits).unwrap()],
        limits,
    )
    .unwrap();
    black_box(
        request
            .into_direct_post_response(vec![result], limits)
            .unwrap(),
    );
}

fn construct_error(description: &str) {
    let request = validated_request();
    let limits = DirectPostErrorResponseLimits::default();
    let details = DirectPostErrorDetails::new(Some(description), None, limits).unwrap();
    black_box(
        request
            .into_direct_post_error_response(DirectPostErrorCode::InvalidRequest, details, limits)
            .unwrap(),
    );
}

fn construct_redirect(mode: &str, presentation: &str) {
    let request =
        validated_request_for(mode, "redirect_uri", "openid://wallet/callback#screen=done");
    let limits = DirectPostResponseLimits::default();
    let result = DirectPostQueryResult::new(
        "degree",
        vec![DirectPostPresentation::string(presentation, limits).unwrap()],
        limits,
    )
    .unwrap();
    black_box(
        request
            .into_redirect_response(vec![result], limits, RedirectResponseLimits::default())
            .unwrap(),
    );
}

fn construct_acknowledgement() {
    let request = validated_request();
    let limits = DirectPostResponseLimits::default();
    let result = DirectPostQueryResult::new(
        "degree",
        vec![DirectPostPresentation::string("compact", limits).unwrap()],
        limits,
    )
    .unwrap();
    let response = request
        .into_direct_post_response(vec![result], limits)
        .unwrap();
    black_box(
        response
            .bind_endpoint_acknowledgement(
                200,
                Some("application/json"),
                Some("no-store"),
                br#"{"redirect_uri":"openid://wallet/done?response_code=opaque"}"#,
                DirectPostAcknowledgementLimits::default(),
            )
            .unwrap(),
    );
}

struct SdJwtFormat;

impl CredentialFormatCapability for SdJwtFormat {
    fn format(&self) -> CredentialFormat {
        CredentialFormat::DcSdJwt
    }

    fn validate_metadata(&self, metadata: &Value) -> Result<(), CredentialFormatCapabilityError> {
        metadata
            .get("vct_values")
            .and_then(Value::as_array)
            .filter(|values| !values.is_empty())
            .map(|_| ())
            .ok_or(CredentialFormatCapabilityError::InvalidMetadata)
    }

    fn supports_transaction_type(&self, transaction_type: &str) -> bool {
        transaction_type == "payment_data"
    }
}

struct PaymentData;

impl TransactionDataCapability for PaymentData {
    fn transaction_type(&self) -> &str {
        "payment_data"
    }

    fn validate(&self, value: &Value) -> Result<(), TransactionDataCapabilityError> {
        value
            .get("amount")
            .and_then(Value::as_u64)
            .filter(|amount| *amount > 0)
            .map(|_| ())
            .ok_or(TransactionDataCapabilityError::Invalid)
    }
}

fn construct_transaction(memo_len: usize) {
    let transaction = json!({
        "type": "payment_data",
        "credential_ids": ["degree"],
        "amount": 42,
        "memo": "m".repeat(memo_len),
    });
    let encoded = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&transaction).unwrap());
    let mut request = base_request(
        "direct_post",
        "response_uri",
        "https://verifier.example/response",
    );
    request["dcql_query"]["credentials"][0]["meta"] = json!({"vct_values": ["urn:example:degree"]});
    request["transaction_data"] = json!([encoded]);
    let format = SdJwtFormat;
    let transaction = PaymentData;
    let request = authorize_request(request)
        .into_authorization_request_with_transaction_capabilities(
            AuthorizationRequestValidationLimits::default(),
            DcqlLimits::default(),
            identus_jose::JweLimits::default(),
            TransactionDataLimits::default(),
            &[&format],
            &[&transaction],
        )
        .unwrap();
    let (request, bindings) = request
        .bind_transaction_data(&[SelectedCredentialInput::new(
            "degree",
            CredentialFormat::DcSdJwt,
        )])
        .unwrap();
    black_box(bindings);
    let limits = DirectPostResponseLimits::default();
    let result = DirectPostQueryResult::new(
        "degree",
        vec![DirectPostPresentation::string("compact", limits).unwrap()],
        limits,
    )
    .unwrap();
    black_box(
        request
            .into_direct_post_response(vec![result], limits)
            .unwrap(),
    );
}

fn validated_request() -> ValidatedAuthorizationRequest {
    validated_request_for(
        "direct_post",
        "response_uri",
        "https://verifier.example/response",
    )
}

fn validated_request_for(
    mode: &str,
    destination_name: &str,
    destination: &str,
) -> ValidatedAuthorizationRequest {
    let request = base_request(mode, destination_name, destination);
    authorize_request(request)
        .into_authorization_request(
            AuthorizationRequestValidationLimits::default(),
            DcqlLimits::default(),
        )
        .unwrap()
}

fn base_request(mode: &str, destination_name: &str, destination: &str) -> Value {
    let mut request = json!({
        "client_id": "decentralized_identifier:did:example:verifier",
        "aud": identus_oid4vp::STATIC_DISCOVERY_AUDIENCE,
        "iat": support::NOW_SECONDS - 120,
        "response_type": "vp_token",
        "response_mode": mode,
        "nonce": "fresh-nonce_1",
        "state": "state-123",
        "dcql_query": {"credentials": [{"id": "degree", "format": "dc+sd-jwt", "meta": {}}]}
    });
    request
        .as_object_mut()
        .unwrap()
        .insert(destination_name.to_owned(), json!(destination));
    request
}

fn authorize_request(request: Value) -> VerifierAuthorizationEvidence {
    let private = Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).unwrap();
    let payload = serde_json::to_vec(&request).unwrap();
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .unwrap();
    let compact = JwsSigningInput::new(header, payload, JwsLimits::default())
        .unwrap()
        .sign_with(&Ed25519Signer::new(&private))
        .unwrap()
        .compact()
        .to_owned();
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .unwrap();
    let unverified = reference
        .prepare_retrieval(
            RequestUriRetrievalInput::get(),
            RequestUriRetrievalLimits::default(),
        )
        .unwrap()
        .bind_response(
            200,
            Some(REQUEST_OBJECT_MEDIA_TYPE),
            compact.as_bytes(),
            RequestObjectValidationLimits::default(),
        )
        .unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    support::authorize(support::secure(
        unverified
            .verify(&SignatureSuiteRegistry::recommended(), &key)
            .unwrap(),
    ))
}

fn measure(name: &str, mut operation: impl FnMut()) {
    for _ in 0..ITERATIONS {
        operation();
    }
    let mut observations = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let started = Instant::now();
        for _ in 0..ITERATIONS {
            operation();
        }
        observations.push(started.elapsed().as_nanos() / ITERATIONS as u128);
    }
    observations.sort_unstable();
    println!(
        "oid4vp-direct-post-benchmark operation={} median_ns_per_iteration={} samples={} iterations_per_sample={}",
        black_box(name),
        observations[SAMPLES / 2],
        SAMPLES,
        ITERATIONS
    );
}

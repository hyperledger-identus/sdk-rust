use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, PublicKeyJwk};
use identus_jose::{
    Ed25519Signer, JweLimits, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey,
    ProtectedHeader, SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits,
    AuthorizationRequestValidationLimits, AuthorizationResponseMode, AuthorizationResponseType,
    CredentialFormat, CredentialFormatCapability, CredentialFormatCapabilityError, DcqlClaimError,
    DcqlCredential, DcqlLimits, DcqlPathComponent, DirectPostErrorCode, DirectPostErrorDetails,
    DirectPostErrorResponseLimits, DirectPostPresentation, DirectPostQueryResult,
    DirectPostResponseLimits, Oid4vpError, REQUEST_OBJECT_JWT_TYPE, REQUEST_OBJECT_MEDIA_TYPE,
    RequestObjectValidationLimits, RequestUriRetrievalInput, RequestUriRetrievalLimits,
    SelectedCredentialInput, TransactionDataCapability, TransactionDataCapabilityError,
    TransactionDataLimits, ValidatedAuthorizationRequest,
};
use serde_json::{Value, json};

mod support;

const PRIVATE_BYTES: [u8; 32] = [0x71; 32];
const CLIENT_ID: &str = "decentralized_identifier:did:example:verifier";
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";
const CANARY: &str = "VERIFIER_SECRET_CANARY";
const TRANSACTION_FORMAT_PACKET: &str =
    include_str!("../../../docs/conformance/fixtures/oid4vp-transaction-format/v1/vectors.json");

fn private_key() -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_slice(&PRIVATE_BYTES).expect("fixed private key")
}

fn public_key() -> PublicKeyJwk {
    private_key().to_public_key().encode_jwk()
}

fn signed(payload: &[u8]) -> String {
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:verifier#key-1"),
        JwsLimits::default(),
    )
    .expect("fixed protected header");
    JwsSigningInput::new(header, payload.to_vec(), JwsLimits::default())
        .expect("bounded input")
        .sign_with(&Ed25519Signer::new(&private_key()))
        .expect("fixed signature")
        .compact()
        .to_owned()
}

fn valid_request() -> Value {
    json!({
        "client_id": CLIENT_ID,
        "aud": identus_oid4vp::STATIC_DISCOVERY_AUDIENCE,
        "iat": support::ISSUED_AT_SECONDS,
        "response_type": "vp_token",
        "response_mode": "direct_post",
        "nonce": "fresh-nonce_1",
        "response_uri": "https://verifier.example/response",
        "dcql_query": {
            "credentials": [{
                "id": "pid",
                "format": "dc+sd-jwt",
                "meta": {}
            }]
        }
    })
}

fn verified_request(request: &Value) -> identus_oid4vp::VerifiedRequestObject {
    let mut request = request.clone();
    let object = request.as_object_mut().expect("request object");
    object.insert(
        "aud".to_owned(),
        json!(identus_oid4vp::STATIC_DISCOVERY_AUDIENCE),
    );
    object.insert("iat".to_owned(), json!(support::ISSUED_AT_SECONDS));
    let payload = serde_json::to_vec(&request).expect("fixed JSON");
    let compact = signed(&payload);
    let AuthorizationRequestInvocation::Referenced(reference) =
        AuthorizationRequestInvocation::parse(
            REFERENCE,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("fixed invocation");
    let unverified = reference
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
        .expect("bound request object");
    let public = public_key();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("bound key");
    unverified
        .verify(&SignatureSuiteRegistry::recommended(), &key)
        .expect("verified request object")
}

fn validate(
    request: &Value,
    routing_limits: AuthorizationRequestValidationLimits,
) -> Result<ValidatedAuthorizationRequest, Oid4vpError> {
    support::authorize(support::secure(verified_request(request)))
        .into_authorization_request(routing_limits, DcqlLimits::default())
}

#[test]
fn valid_direct_post_request_preserves_composed_evidence() {
    let request = validate(
        &valid_request(),
        AuthorizationRequestValidationLimits::default(),
    )
    .expect("valid authorization request");

    assert_eq!(request.response_type(), AuthorizationResponseType::VpToken);
    assert_eq!(
        request.response_mode(),
        AuthorizationResponseMode::DirectPost
    );
    assert_eq!(request.algorithm(), JwsAlgorithm::Ed25519);
    assert_eq!(
        request.protected_header().key_id(),
        Some("did:example:verifier#key-1")
    );
    assert_eq!(request.expose_sensitive_client_id(), CLIENT_ID);
    assert_eq!(request.expose_sensitive_wallet_nonce(), None);
    assert_eq!(request.expose_sensitive_nonce(), "fresh-nonce_1");
    assert_eq!(request.expose_sensitive_state(), None);
    assert_eq!(
        request.expose_sensitive_response_uri(),
        Some("https://verifier.example/response")
    );
    assert_eq!(request.dcql_query().credential_query_count(), 1);
}

#[test]
fn optional_state_is_bounded_visible_ascii_and_retained_exactly() {
    let mut request = valid_request();
    request["state"] = json!("opaque state~1");
    let validated =
        validate(&request, AuthorizationRequestValidationLimits::default()).expect("valid state");
    assert_eq!(validated.expose_sensitive_state(), Some("opaque state~1"));

    for invalid in [json!(""), json!("line\nbreak"), json!("💣"), json!(7)] {
        let mut request = valid_request();
        request["state"] = invalid;
        assert_eq!(
            validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
            Oid4vpError::InvalidState
        );
    }

    let mut request = valid_request();
    request["state"] = json!("abc");
    let limits = AuthorizationRequestValidationLimits::default()
        .with_max_state_bytes(2)
        .unwrap();
    assert_eq!(
        validate(&request, limits).unwrap_err(),
        Oid4vpError::StateTooLarge
    );
}

#[test]
fn unsupported_and_missing_response_fields_are_explicit() {
    let mut request = valid_request();
    request.as_object_mut().unwrap().remove("response_type");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::MissingResponseType
    );

    let mut request = valid_request();
    request["response_type"] = json!("code");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::UnsupportedResponseType
    );

    let mut request = valid_request();
    request.as_object_mut().unwrap().remove("response_mode");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::MissingResponseMode
    );

    let mut request = valid_request();
    request["response_mode"] = json!("direct_post.jwt");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::InvalidRequestObjectPayload
    );
}

#[test]
fn authorization_nonce_uses_final_grammar_and_its_own_limit() {
    let mut request = valid_request();
    request.as_object_mut().unwrap().remove("nonce");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::MissingAuthorizationNonce
    );

    for nonce in [
        json!(""),
        json!("contains space"),
        json!("non-ascii-💣"),
        json!(7),
    ] {
        let mut request = valid_request();
        request["nonce"] = nonce;
        assert_eq!(
            validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
            Oid4vpError::InvalidAuthorizationNonce
        );
    }

    let mut request = valid_request();
    request["nonce"] = json!("abc");
    let limits = AuthorizationRequestValidationLimits::new(2, 4_096).unwrap();
    assert_eq!(
        validate(&request, limits).unwrap_err(),
        Oid4vpError::AuthorizationNonceTooLarge
    );
}

#[test]
fn direct_post_destination_is_bounded_and_unambiguous() {
    let mut request = valid_request();
    request.as_object_mut().unwrap().remove("response_uri");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::MissingResponseUri
    );

    let mut request = valid_request();
    request["redirect_uri"] = json!("https://verifier.example/callback");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::ConflictingResponseDestination
    );

    let mut request = valid_request();
    request["response_uri"] = json!("https://verifier.example/response");
    let limits = AuthorizationRequestValidationLimits::new(256, 8).unwrap();
    assert_eq!(
        validate(&request, limits).unwrap_err(),
        Oid4vpError::ResponseUriTooLarge
    );
}

#[test]
fn response_uri_rejects_non_https_authority_and_fragment_shapes() {
    for response_uri in [
        "",
        "/relative",
        "http://verifier.example/response",
        "https:///response",
        "https://user@verifier.example/response",
        "https://verifier.example/response#fragment",
    ] {
        let mut request = valid_request();
        request["response_uri"] = json!(response_uri);
        assert_eq!(
            validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
            Oid4vpError::UnsafeResponseUri,
            "unexpected result for {response_uri}"
        );
    }
}

#[test]
fn validation_order_precedes_dcql_and_is_map_order_independent() {
    let mut request = json!({
        "dcql_query": {"credentials": []},
        "response_uri": "http://unsafe.example",
        "redirect_uri": "https://verifier.example/callback",
        "nonce": "contains space",
        "response_mode": "jwt",
        "response_type": "code",
        "client_id": CLIENT_ID
    });
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::UnsupportedResponseType
    );

    request["response_type"] = json!("vp_token");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::UnsupportedResponseMode
    );

    request["response_mode"] = json!("direct_post");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::InvalidAuthorizationNonce
    );

    request["nonce"] = json!("valid-nonce");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::ConflictingResponseDestination
    );

    request.as_object_mut().unwrap().remove("redirect_uri");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::UnsafeResponseUri
    );

    request["response_uri"] = json!("https://verifier.example/response");
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::InvalidDcqlQuery
    );
}

#[test]
fn query_only_transition_remains_compatible_without_routing_fields() {
    let request = json!({
        "client_id": CLIENT_ID,
        "dcql_query": {
            "credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {}}]
        }
    });
    let query = support::secure(verified_request(&request))
        .into_dcql_query(DcqlLimits::default())
        .expect("existing query-only transition");
    assert_eq!(query.credential_query_count(), 1);
}

#[test]
fn debug_and_errors_do_not_disclose_routing_values() {
    let mut request = valid_request();
    request["nonce"] = json!(CANARY);
    request["response_uri"] = json!(format!("https://verifier.example/{CANARY}"));
    request["dcql_query"]["credentials"][0]["id"] = json!(CANARY);
    let validated = validate(&request, AuthorizationRequestValidationLimits::default())
        .expect("valid canary-bearing request");
    assert!(!format!("{validated:?}").contains(CANARY));

    let mut invalid = valid_request();
    invalid["nonce"] = json!(format!("{CANARY} value"));
    let error = validate(&invalid, AuthorizationRequestValidationLimits::default()).unwrap_err();
    assert!(!format!("{error}").contains(CANARY));
    assert!(!format!("{error:?}").contains(CANARY));
}

#[test]
fn routing_limits_require_positive_values() {
    assert_eq!(
        AuthorizationRequestValidationLimits::new(0, 1),
        Err(Oid4vpError::InvalidLimits)
    );
    assert_eq!(
        AuthorizationRequestValidationLimits::new(1, 0),
        Err(Oid4vpError::InvalidLimits)
    );
    assert_eq!(
        AuthorizationRequestValidationLimits::default().with_max_state_bytes(0),
        Err(Oid4vpError::InvalidLimits)
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

struct SdJwtCredential {
    id: &'static str,
    vct: &'static str,
}

impl DcqlCredential for SdJwtCredential {
    fn expose_sensitive_id(&self) -> &str {
        self.id
    }

    fn format(&self) -> &str {
        "dc+sd-jwt"
    }

    fn has_cryptographic_holder_binding(&self) -> bool {
        true
    }

    fn satisfies_format_metadata(&self, metadata: &serde_json::Map<String, Value>) -> bool {
        metadata
            .get("vct_values")
            .and_then(Value::as_array)
            .is_some_and(|values| values.iter().any(|value| value.as_str() == Some(self.vct)))
    }

    fn resolve_claim(&self, _path: &[DcqlPathComponent]) -> Result<Vec<Value>, DcqlClaimError> {
        Err(DcqlClaimError::Missing)
    }
}

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

struct MdocFormat;

impl CredentialFormatCapability for MdocFormat {
    fn format(&self) -> CredentialFormat {
        CredentialFormat::MsoMdoc
    }

    fn validate_metadata(&self, metadata: &Value) -> Result<(), CredentialFormatCapabilityError> {
        metadata
            .get("doctype_value")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .map(|_| ())
            .ok_or(CredentialFormatCapabilityError::InvalidMetadata)
    }

    fn supports_transaction_type(&self, transaction_type: &str) -> bool {
        transaction_type == "payment_data"
    }
}

struct GenericFormat;

impl CredentialFormatCapability for GenericFormat {
    fn format(&self) -> CredentialFormat {
        CredentialFormat::extension("vc+example", 64).expect("fixed extension identifier")
    }

    fn validate_metadata(&self, metadata: &Value) -> Result<(), CredentialFormatCapabilityError> {
        (metadata.get("profile").and_then(Value::as_str) == Some("interop"))
            .then_some(())
            .ok_or(CredentialFormatCapabilityError::InvalidMetadata)
    }

    fn supports_transaction_type(&self, transaction_type: &str) -> bool {
        transaction_type == "generic_extension"
    }
}

struct GenericTransaction;

impl TransactionDataCapability for GenericTransaction {
    fn transaction_type(&self) -> &str {
        "generic_extension"
    }

    fn validate(&self, value: &Value) -> Result<(), TransactionDataCapabilityError> {
        (value.get("purpose").and_then(Value::as_str) == Some("interop"))
            .then_some(())
            .ok_or(TransactionDataCapabilityError::Invalid)
    }
}

fn assert_transaction_format_vector(
    input: &Value,
    format_capability: &dyn CredentialFormatCapability,
    transaction_capability: &dyn TransactionDataCapability,
    selected_format: CredentialFormat,
) {
    let query_id = input["queryId"].as_str().expect("query ID");
    let format = input["format"].as_str().expect("format");
    let encoded = input["encoded"].as_str().expect("encoded transaction");
    let mut request = valid_request();
    request["dcql_query"] = json!({
        "credentials": [{
            "id": query_id,
            "format": format,
            "meta": input["meta"].clone()
        }]
    });
    request["transaction_data"] = json!([encoded]);
    let validated = support::authorize(support::secure(verified_request(&request)))
        .into_authorization_request_with_transaction_capabilities(
            AuthorizationRequestValidationLimits::default(),
            DcqlLimits::default(),
            JweLimits::default(),
            TransactionDataLimits::default(),
            &[format_capability],
            &[transaction_capability],
        )
        .expect("vector validates through ports");
    let (_, bindings) = validated
        .bind_transaction_data(&[SelectedCredentialInput::new(query_id, selected_format)])
        .expect("vector selection binds");
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].expose_sensitive_query_id(), query_id);
    assert_eq!(bindings[0].expose_sensitive_encoded(), encoded);
    assert_eq!(
        bindings[0].transaction_type(),
        input["transactionType"].as_str().expect("transaction type")
    );
}

#[test]
fn transaction_format_packet_covers_sd_jwt_mdoc_and_generic_extension_ports() {
    let packet: Value = serde_json::from_str(TRANSACTION_FORMAT_PACKET).expect("valid packet");
    let cases = packet["cases"].as_array().expect("packet cases");
    assert_eq!(cases.len(), 3);
    let sd_jwt = SdJwtFormat;
    let mdoc = MdocFormat;
    let generic_format = GenericFormat;
    let payment = PaymentData;
    let generic_transaction = GenericTransaction;

    for case in cases {
        let input: Value = serde_json::from_str(
            case["input"]["literal"]
                .as_str()
                .expect("literal vector input"),
        )
        .expect("JSON vector input");
        assert_eq!(case["expected"]["fields"]["serialized"], input["encoded"]);
        match input["format"].as_str().expect("format") {
            "dc+sd-jwt" => assert_transaction_format_vector(
                &input,
                &sd_jwt,
                &payment,
                CredentialFormat::DcSdJwt,
            ),
            "mso_mdoc" => {
                assert_transaction_format_vector(&input, &mdoc, &payment, CredentialFormat::MsoMdoc)
            }
            "vc+example" => assert_transaction_format_vector(
                &input,
                &generic_format,
                &generic_transaction,
                CredentialFormat::extension("vc+example", 64).unwrap(),
            ),
            other => panic!("unexpected vector format: {other}"),
        }
    }
}

#[test]
fn transaction_data_flows_from_signed_request_to_selected_format_input() {
    let mut request = valid_request();
    request["dcql_query"]["credentials"][0]["meta"] =
        json!({"vct_values": ["urn:example:payment-credential"]});
    let transaction = json!({
        "type": "payment_data",
        "credential_ids": ["pid"],
        "amount": 42
    });
    let encoded = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&transaction).unwrap());
    request["transaction_data"] = json!([encoded]);

    let format = SdJwtFormat;
    let transaction = PaymentData;
    let validate = || {
        support::authorize(support::secure(verified_request(&request)))
            .into_authorization_request_with_transaction_capabilities(
                AuthorizationRequestValidationLimits::default(),
                DcqlLimits::default(),
                JweLimits::default(),
                TransactionDataLimits::default(),
                &[&format],
                &[&transaction],
            )
            .expect("supported transaction request")
    };

    let response_limits = DirectPostResponseLimits::default();
    let result = DirectPostQueryResult::new(
        "pid",
        vec![DirectPostPresentation::string("presentation", response_limits).unwrap()],
        response_limits,
    )
    .unwrap();
    assert_eq!(
        validate()
            .into_direct_post_response(vec![result], response_limits)
            .unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );

    let validated = validate();

    assert_eq!(validated.transaction_data_count(), 1);
    let (validated, bindings) = validated
        .bind_transaction_data(&[SelectedCredentialInput::new(
            "pid",
            CredentialFormat::DcSdJwt,
        )])
        .expect("selected format binding");
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].transaction_type(), "payment_data");
    assert_eq!(bindings[0].expose_sensitive_encoded(), encoded);
    assert!(!format!("{validated:?}").contains(&encoded));
    assert!(!format!("{:?}", bindings[0]).contains(&encoded));

    let result = DirectPostQueryResult::new(
        "pid",
        vec![DirectPostPresentation::string("presentation", response_limits).unwrap()],
        response_limits,
    )
    .unwrap();
    validated
        .into_direct_post_response(vec![result], response_limits)
        .expect("bound request can construct response material");

    assert_eq!(
        validate()
            .bind_transaction_data(&[SelectedCredentialInput::new(
                "pid",
                CredentialFormat::MsoMdoc,
            )])
            .unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );
}

#[test]
fn format_metadata_filters_dcql_candidates_after_request_validation() {
    let mut request = valid_request();
    request["dcql_query"]["credentials"][0]["meta"] =
        json!({"vct_values": ["urn:example:required"]});
    let format = SdJwtFormat;
    let validated = support::authorize(support::secure(verified_request(&request)))
        .into_authorization_request_with_transaction_capabilities(
            AuthorizationRequestValidationLimits::default(),
            DcqlLimits::default(),
            JweLimits::default(),
            TransactionDataLimits::default(),
            &[&format],
            &[],
        )
        .expect("format metadata validates");

    let evaluation = validated
        .dcql_query()
        .evaluate(&[
            SdJwtCredential {
                id: "wrong-vct",
                vct: "urn:example:other",
            },
            SdJwtCredential {
                id: "matching-vct",
                vct: "urn:example:required",
            },
        ])
        .expect("bounded evaluation");
    assert!(evaluation.is_satisfiable());
    assert_eq!(evaluation.matches()[0].candidates().len(), 1);
    assert_eq!(
        evaluation.matches()[0].candidates()[0].expose_sensitive_credential_id(),
        "matching-vct"
    );
}

#[test]
fn transaction_bindings_cover_every_selected_reference_and_match_success_results() {
    let mut request = valid_request();
    request["dcql_query"] = json!({
        "credentials": [
            {
                "id": "pid",
                "format": "dc+sd-jwt",
                "meta": {"vct_values": ["urn:example:pid"]}
            },
            {
                "id": "address",
                "format": "dc+sd-jwt",
                "meta": {"vct_values": ["urn:example:address"]}
            }
        ],
        "credential_sets": [{"options": [["pid"], ["address"]]}]
    });
    let encoded = URL_SAFE_NO_PAD.encode(
        serde_json::to_vec(&json!({
            "type": "payment_data",
            "credential_ids": ["pid", "address"],
            "amount": 42
        }))
        .unwrap(),
    );
    request["transaction_data"] = json!([encoded]);

    let format = SdJwtFormat;
    let transaction = PaymentData;
    let validate = || {
        support::authorize(support::secure(verified_request(&request)))
            .into_authorization_request_with_transaction_capabilities(
                AuthorizationRequestValidationLimits::default(),
                DcqlLimits::default(),
                JweLimits::default(),
                TransactionDataLimits::default(),
                &[&format],
                &[&transaction],
            )
            .expect("supported transaction request")
    };
    let response_limits = DirectPostResponseLimits::default();
    let presentation = || {
        DirectPostPresentation::string("presentation", response_limits)
            .expect("bounded presentation")
    };
    let result = |query_id| {
        DirectPostQueryResult::new(query_id, vec![presentation()], response_limits)
            .expect("bounded query result")
    };

    let (validated, bindings) = validate()
        .bind_transaction_data(&[
            SelectedCredentialInput::new("pid", CredentialFormat::DcSdJwt),
            SelectedCredentialInput::new("address", CredentialFormat::DcSdJwt),
        ])
        .expect("both referenced selections bind");
    assert_eq!(bindings.len(), 2);
    assert_eq!(bindings[0].expose_sensitive_query_id(), "pid");
    assert_eq!(bindings[1].expose_sensitive_query_id(), "address");
    assert_eq!(bindings[0].expose_sensitive_encoded(), encoded);
    assert_eq!(bindings[1].expose_sensitive_encoded(), encoded);
    assert_eq!(
        validated
            .into_direct_post_response(vec![result("address")], response_limits)
            .unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );

    let (validated, bindings) = validate()
        .bind_transaction_data(&[SelectedCredentialInput::new(
            "pid",
            CredentialFormat::DcSdJwt,
        )])
        .expect("one alternative binds");
    assert_eq!(bindings.len(), 1);
    validated
        .into_direct_post_response(vec![result("pid")], response_limits)
        .expect("response carries the selected transaction binding");

    assert_eq!(
        validate()
            .bind_transaction_data(&[SelectedCredentialInput::new(
                "unknown",
                CredentialFormat::DcSdJwt,
            )])
            .unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );
    assert_eq!(
        validate()
            .bind_transaction_data(&[
                SelectedCredentialInput::new("pid", CredentialFormat::DcSdJwt),
                SelectedCredentialInput::new("address", CredentialFormat::DcSdJwt),
                SelectedCredentialInput::new("extra", CredentialFormat::DcSdJwt),
            ])
            .unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );

    let (validated, _) = validate()
        .bind_transaction_data(&[SelectedCredentialInput::new(
            "pid",
            CredentialFormat::DcSdJwt,
        )])
        .expect("one alternative binds");
    assert_eq!(
        validated
            .into_direct_post_response(vec![result("pid"), result("address")], response_limits,)
            .unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );

    validate()
        .into_direct_post_error_response(
            DirectPostErrorCode::InvalidTransactionData,
            DirectPostErrorDetails::empty(),
            DirectPostErrorResponseLimits::default(),
        )
        .expect("protocol error does not require successful credential binding");
}

#[test]
fn nonempty_metadata_and_transaction_semantics_fail_closed_without_ports() {
    let mut request = valid_request();
    request["dcql_query"]["credentials"][0]["meta"] = json!({"vct_values": ["x"]});
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::InvalidDcqlQuery
    );

    let mut request = valid_request();
    let encoded = URL_SAFE_NO_PAD.encode(
        serde_json::to_vec(&json!({
            "type": "unknown",
            "credential_ids": ["pid"]
        }))
        .unwrap(),
    );
    request["transaction_data"] = json!([encoded]);
    assert_eq!(
        validate(&request, AuthorizationRequestValidationLimits::default()).unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );
}

#[test]
fn transaction_data_rejects_bad_encoding_references_and_resource_policy() {
    let format = SdJwtFormat;
    let transaction = PaymentData;

    for value in [json!("not+base64"), json!("e30=")] {
        let mut request = valid_request();
        request["transaction_data"] = json!([value]);
        assert_eq!(
            support::authorize(support::secure(verified_request(&request)))
                .into_authorization_request_with_transaction_capabilities(
                    AuthorizationRequestValidationLimits::default(),
                    DcqlLimits::default(),
                    JweLimits::default(),
                    TransactionDataLimits::default(),
                    &[&format],
                    &[&transaction],
                )
                .unwrap_err(),
            Oid4vpError::InvalidTransactionData
        );
    }

    let duplicate_member = URL_SAFE_NO_PAD
        .encode(br#"{"type":"payment_data","type":"other","credential_ids":["pid"],"amount":42}"#);
    let mut request = valid_request();
    request["transaction_data"] = json!([duplicate_member]);
    assert_eq!(
        support::authorize(support::secure(verified_request(&request)))
            .into_authorization_request_with_transaction_capabilities(
                AuthorizationRequestValidationLimits::default(),
                DcqlLimits::default(),
                JweLimits::default(),
                TransactionDataLimits::default(),
                &[&format],
                &[&transaction],
            )
            .unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );

    let bad_reference = URL_SAFE_NO_PAD.encode(
        serde_json::to_vec(&json!({
            "type": "payment_data",
            "credential_ids": ["absent"],
            "amount": 42
        }))
        .unwrap(),
    );
    let mut request = valid_request();
    request["transaction_data"] = json!([bad_reference]);
    assert_eq!(
        support::authorize(support::secure(verified_request(&request)))
            .into_authorization_request_with_transaction_capabilities(
                AuthorizationRequestValidationLimits::default(),
                DcqlLimits::default(),
                JweLimits::default(),
                TransactionDataLimits::default(),
                &[&format],
                &[&transaction],
            )
            .unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );

    let generic = GenericFormat;
    let incompatible_reference = URL_SAFE_NO_PAD.encode(
        serde_json::to_vec(&json!({
            "type": "payment_data",
            "credential_ids": ["pid", "generic"],
            "amount": 42
        }))
        .unwrap(),
    );
    let mut request = valid_request();
    request["dcql_query"]["credentials"] = json!([
        {"id": "pid", "format": "dc+sd-jwt", "meta": {}},
        {"id": "generic", "format": "vc+example", "meta": {"profile": "interop"}}
    ]);
    request["transaction_data"] = json!([incompatible_reference]);
    assert_eq!(
        support::authorize(support::secure(verified_request(&request)))
            .into_authorization_request_with_transaction_capabilities(
                AuthorizationRequestValidationLimits::default(),
                DcqlLimits::default(),
                JweLimits::default(),
                TransactionDataLimits::default(),
                &[&format, &generic],
                &[&transaction],
            )
            .unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );

    assert_eq!(
        TransactionDataLimits::new(0, 1, 1, 1, 1, 1, 1, 1),
        Err(Oid4vpError::InvalidLimits)
    );
    assert_eq!(
        CredentialFormat::extension("", 64).unwrap_err(),
        Oid4vpError::InvalidDcqlQuery
    );
    assert_eq!(
        CredentialFormat::extension("vc+example", 0).unwrap_err(),
        Oid4vpError::InvalidLimits
    );
    assert_eq!(
        CredentialFormat::extension("vc+example", 4).unwrap_err(),
        Oid4vpError::InvalidDcqlQuery
    );

    let excessive_formats = vec![
        &format as &dyn CredentialFormatCapability;
        DcqlLimits::default().max_collection_items() + 1
    ];
    assert_eq!(
        support::authorize(support::secure(verified_request(&valid_request())))
            .into_authorization_request_with_capabilities(
                AuthorizationRequestValidationLimits::default(),
                DcqlLimits::default(),
                &excessive_formats,
            )
            .unwrap_err(),
        Oid4vpError::InvalidDcqlQuery
    );

    let mut request = valid_request();
    request["transaction_data"] = json!([URL_SAFE_NO_PAD.encode(
        serde_json::to_vec(&json!({
            "type": "payment_data",
            "credential_ids": ["pid"],
            "amount": 42
        }))
        .unwrap()
    )]);
    let limits = TransactionDataLimits::default();
    let excessive_transactions =
        vec![&transaction as &dyn TransactionDataCapability; limits.max_items() + 1];
    assert_eq!(
        support::authorize(support::secure(verified_request(&request)))
            .into_authorization_request_with_transaction_capabilities(
                AuthorizationRequestValidationLimits::default(),
                DcqlLimits::default(),
                JweLimits::default(),
                limits,
                &[&format],
                &excessive_transactions,
            )
            .unwrap_err(),
        Oid4vpError::InvalidTransactionData
    );
}

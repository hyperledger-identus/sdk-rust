use identus_crypto::{Ed25519PrivateKey, EncodeJwk, PublicKeyJwk};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits, DcqlClaimError,
    DcqlCredential, DcqlLimits, DcqlPathComponent, Oid4vpError, REQUEST_OBJECT_JWT_TYPE,
    REQUEST_OBJECT_MEDIA_TYPE, RequestObjectValidationLimits, RequestUriRetrievalInput,
    RequestUriRetrievalLimits, ValidatedDcqlQuery,
};
use serde_json::{Value, json};

const PRIVATE_BYTES: [u8; 32] = [0x63; 32];
const CLIENT_ID: &str = "decentralized_identifier:did:example:verifier";
const REFERENCE: &str = "openid4vp:?client_id=decentralized_identifier%3Adid%3Aexample%3Averifier&request_uri=https%3A%2F%2Fverifier.example%2Frequest";

struct JsonCredential {
    id: &'static str,
    format: &'static str,
    claims: Value,
    holder_bound: bool,
}

impl DcqlCredential for JsonCredential {
    fn expose_sensitive_id(&self) -> &str {
        self.id
    }

    fn format(&self) -> &str {
        self.format
    }

    fn has_cryptographic_holder_binding(&self) -> bool {
        self.holder_bound
    }

    fn resolve_claim(&self, path: &[DcqlPathComponent]) -> Result<Vec<Value>, DcqlClaimError> {
        let mut selected = vec![&self.claims];
        for component in path {
            let mut next = Vec::new();
            for value in selected {
                if let Some(key) = component.expose_sensitive_key() {
                    let object = value.as_object().ok_or(DcqlClaimError::TypeMismatch)?;
                    if let Some(value) = object.get(key) {
                        next.push(value);
                    }
                } else if let Some(index) = component.index() {
                    let array = value.as_array().ok_or(DcqlClaimError::TypeMismatch)?;
                    if let Some(value) = usize::try_from(index)
                        .ok()
                        .and_then(|index| array.get(index))
                    {
                        next.push(value);
                    }
                } else if component.is_all() {
                    next.extend(value.as_array().ok_or(DcqlClaimError::TypeMismatch)?.iter());
                } else {
                    return Err(DcqlClaimError::Malformed);
                }
            }
            selected = next;
        }
        if selected.is_empty() {
            Err(DcqlClaimError::Missing)
        } else {
            Ok(selected.into_iter().cloned().collect())
        }
    }
}

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

fn parse_query(query: Value, limits: DcqlLimits) -> Result<ValidatedDcqlQuery, Oid4vpError> {
    parse_request(json!({"client_id": CLIENT_ID, "dcql_query": query}), limits)
}

fn parse_request(request: Value, limits: DcqlLimits) -> Result<ValidatedDcqlQuery, Oid4vpError> {
    let payload = serde_json::to_vec(&request).expect("fixed JSON");
    let compact = signed(&payload);
    let invocation = AuthorizationRequestInvocation::parse(
        REFERENCE,
        AuthorizationRequestInvocationLimits::default(),
    )
    .expect("fixed invocation");
    let identus_oid4vp::AuthorizationRequestInvocation::Referenced(reference) = invocation;
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
        .expect("bound object");
    let public = public_key();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("bound key");
    unverified
        .verify(&SignatureSuiteRegistry::recommended(), &key)
        .expect("verified object")
        .into_dcql_query(limits)
}

#[test]
fn valid_query_selects_only_a_complete_bound_credential() {
    let query = parse_query(
        json!({
            "credentials": [{
                "id": "pid",
                "format": "dc+sd-jwt",
                "meta": {},
                "claims": [{"id": "adult", "path": ["adult"], "values": [true]}]
            }]
        }),
        DcqlLimits::default(),
    )
    .expect("valid query");
    assert_eq!(query.credential_query_count(), 1);
    assert_eq!(query.claim_query_count(), 1);

    let credentials = [
        JsonCredential {
            id: "complete",
            format: "dc+sd-jwt",
            claims: json!({"adult": true}),
            holder_bound: true,
        },
        JsonCredential {
            id: "missing",
            format: "dc+sd-jwt",
            claims: json!({"adult": "true"}),
            holder_bound: true,
        },
        JsonCredential {
            id: "unbound",
            format: "dc+sd-jwt",
            claims: json!({"adult": true}),
            holder_bound: false,
        },
    ];
    let result = query.evaluate(&credentials).expect("bounded evaluation");
    assert!(result.is_satisfiable());
    assert_eq!(result.matches().len(), 1);
    let query_match = &result.matches()[0];
    assert_eq!(query_match.expose_sensitive_query_id(), "pid");
    assert_eq!(query_match.candidates().len(), 1);
    assert_eq!(
        query_match.candidates()[0].expose_sensitive_credential_id(),
        "complete"
    );
    assert_eq!(query_match.candidates()[0].claims().len(), 1);
    assert_eq!(result.combinations().len(), 1);
    assert_eq!(result.dropped().count(), 0);
}

#[test]
fn credential_sets_and_combination_caps_are_preserved() {
    let limits = DcqlLimits::new(16_384, 64, 256, 32, 256, 65_536, 1).unwrap();
    let query = parse_query(
        json!({
            "credentials": [
                {"id": "pid", "format": "dc+sd-jwt", "meta": {}},
                {"id": "mdl", "format": "mso_mdoc", "meta": {}}
            ],
            "credential_sets": [{"options": [["pid"], ["mdl"]], "required": true}]
        }),
        limits,
    )
    .expect("valid set query");
    let credentials = [
        JsonCredential {
            id: "pid-a",
            format: "dc+sd-jwt",
            claims: json!({}),
            holder_bound: true,
        },
        JsonCredential {
            id: "pid-b",
            format: "dc+sd-jwt",
            claims: json!({}),
            holder_bound: true,
        },
    ];
    let result = query.evaluate(&credentials).expect("bounded evaluation");
    assert!(result.is_satisfiable());
    assert_eq!(result.combinations().len(), 1);
    assert_eq!(result.dropped().count(), 1);
    assert!(result.dropped().is_exact());
}

#[test]
fn sdk_facade_rejects_candidate_tolerance_and_reference_gaps() {
    let invalid = [
        json!({"credentials": []}),
        json!({"credentials": [{"id": "pid", "format": "dc+sd-jwt"}]}),
        json!({"credentials": [{"id": "pid.dot", "format": "dc+sd-jwt", "meta": {}}]}),
        json!({"credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {"vct_values": ["x"]}}]}),
        json!({"credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {}, "trusted_authorities": []}]}),
        json!({"credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {}, "claims": []}]}),
        json!({"credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {}, "claims": [{"path": []}]}]}),
        json!({"credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {}, "claims": [{"path": ["age"], "values": []}]}]}),
        json!({"credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {}}], "credential_sets": [{"options": [["unknown"]]}]}),
        json!({"credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {}}], "credential_sets": [{"options": [["pid"]], "purpose": true}]}),
    ];
    for query in invalid {
        assert_eq!(
            parse_query(query, DcqlLimits::default()).unwrap_err(),
            Oid4vpError::InvalidDcqlQuery
        );
    }

    assert_eq!(
        parse_request(json!({"client_id": CLIENT_ID}), DcqlLimits::default()).unwrap_err(),
        Oid4vpError::MissingDcqlQuery
    );
    assert_eq!(
        parse_request(
            json!({
                "client_id": CLIENT_ID,
                "scope": "example",
                "dcql_query": {"credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {}}]}
            }),
            DcqlLimits::default()
        )
        .unwrap_err(),
        Oid4vpError::UnsupportedDcqlScope
    );
}

#[test]
fn resource_work_and_diagnostics_are_bounded_and_redacted() {
    for limits in [
        DcqlLimits::new(0, 1, 1, 1, 1, 1, 1),
        DcqlLimits::new(1, 0, 1, 1, 1, 1, 1),
        DcqlLimits::new(1, 1, 0, 1, 1, 1, 1),
        DcqlLimits::new(1, 1, 1, 0, 1, 1, 1),
        DcqlLimits::new(1, 1, 1, 1, 0, 1, 1),
        DcqlLimits::new(1, 1, 1, 1, 1, 0, 1),
        DcqlLimits::new(1, 1, 1, 1, 1, 1, 0),
    ] {
        assert_eq!(limits.unwrap_err(), Oid4vpError::InvalidLimits);
    }

    let tiny_query = DcqlLimits::new(8, 64, 256, 32, 256, 65_536, 1).unwrap();
    assert_eq!(
        parse_query(
            json!({"credentials": [{"id": "pid", "format": "dc+sd-jwt", "meta": {}}]}),
            tiny_query
        )
        .unwrap_err(),
        Oid4vpError::DcqlQueryTooLarge
    );

    let tiny_work = DcqlLimits::new(16_384, 64, 256, 32, 4, 1, 4).unwrap();
    let query = parse_query(
        json!({"credentials": [{"id": "SECRET_QUERY", "format": "dc+sd-jwt", "meta": {}}]}),
        tiny_work,
    )
    .expect("valid bounded query");
    let credentials = [
        JsonCredential {
            id: "SECRET_CREDENTIAL",
            format: "dc+sd-jwt",
            claims: json!({}),
            holder_bound: true,
        },
        JsonCredential {
            id: "SECOND_SECRET_CREDENTIAL",
            format: "dc+sd-jwt",
            claims: json!({}),
            holder_bound: true,
        },
    ];
    assert_eq!(
        query.evaluate(&credentials).unwrap_err(),
        Oid4vpError::DcqlWorkLimitExceeded
    );

    let query = parse_query(
        json!({"credentials": [{"id": "SECRET_QUERY", "format": "dc+sd-jwt", "meta": {}}]}),
        DcqlLimits::default(),
    )
    .expect("valid query");
    let result = query.evaluate(&credentials).expect("evaluation");
    for rendered in [format!("{query:?}"), format!("{result:?}")] {
        assert!(!rendered.contains("SECRET_QUERY"));
        assert!(!rendered.contains("SECRET_CREDENTIAL"));
        assert!(!rendered.contains("SECOND_SECRET_CREDENTIAL"));
    }
}

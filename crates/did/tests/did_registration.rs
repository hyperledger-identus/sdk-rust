use std::{
    collections::BTreeMap,
    future::Future,
    hint::black_box,
    pin::{Pin, pin},
    sync::{
        Arc, Barrier, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
    time::Instant,
};

use identus_core::ErrorKind;
use identus_did::{
    CancelRegistrationRequest, ContinueRegistrationRequest, CreateRegistrationRequest,
    DeactivateRegistrationRequest, Did, DidDocument, DidDocumentMetadata, DidDocumentOperation,
    DidMethod, DidMethodBinding, DidMethodRegistry, DidRegistrar, DidRegistrationErrorKind,
    DidRegistrationFuture, DidRegistrationResult, DidRegistrationState, DidResolutionFuture,
    DidResolver, Error, InternalSecretPolicy, MAX_DID_REGISTRATION_BYTES, MAX_REGISTRATION_ITEMS,
    MAX_REGISTRATION_WAIT_MILLIS, RegistrationAction, RegistrationActionId,
    RegistrationActionResponse, RegistrationContinuation, RegistrationError,
    RegistrationFailureCode, RegistrationIdempotencyKey, RegistrationJob, RegistrationJobId,
    RegistrationOperationName, RegistrationPublicData, RegistrationRequest,
    RegistrationSecretHandle, RegistrationSecretMode, ResolutionOptions, UpdateRegistrationRequest,
};
use serde_json::{Value, json};

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn method(value: &str) -> DidMethod {
    DidMethod::parse(value).unwrap()
}

fn key(value: &str) -> RegistrationIdempotencyKey {
    RegistrationIdempotencyKey::parse(value).unwrap()
}

fn create(method_name: &str, key_value: &str) -> RegistrationRequest {
    RegistrationRequest::Create(
        CreateRegistrationRequest::new(
            method(method_name),
            None,
            None,
            RegistrationPublicData::empty(),
            RegistrationSecretMode::ClientManaged,
            key(key_value),
        )
        .unwrap(),
    )
}

fn finished(method_name: &str, did: Did) -> DidRegistrationResult {
    DidRegistrationResult::new(
        method(method_name),
        None,
        DidRegistrationState::Finished {
            did,
            document: None,
            secret_handles: Vec::new(),
        },
        RegistrationPublicData::empty(),
        DidDocumentMetadata::empty(),
    )
    .unwrap()
}

#[test]
fn opaque_identifiers_are_bounded_and_redacted() {
    let id = RegistrationJobId::parse("wallet-secret-job-token").unwrap();
    assert_eq!(id.as_str(), "wallet-secret-job-token");
    assert!(!format!("{id:?}").contains("wallet-secret-job-token"));
    assert!(RegistrationActionId::parse("").is_err());
    assert!(RegistrationSecretHandle::parse(" leading").is_err());
    assert!(RegistrationOperationName::parse("line\nbreak").is_err());
    assert!(RegistrationFailureCode::parse(&"x".repeat(257)).is_err());

    let code = RegistrationFailureCode::standard(DidRegistrationErrorKind::Conflict);
    assert_eq!(code.kind(), Some(DidRegistrationErrorKind::Conflict));
    assert_eq!(code.as_str(), "conflict");
}

#[test]
fn public_data_roundtrips_but_rejects_private_material_recursively() {
    let data = RegistrationPublicData::new(BTreeMap::from([
        ("network".to_owned(), json!("preprod")),
        ("proof".to_owned(), json!({"type": "Ed25519Signature2020"})),
    ]))
    .unwrap();
    let encoded = serde_json::to_string(&data).unwrap();
    assert_eq!(
        serde_json::from_str::<RegistrationPublicData>(&encoded).unwrap(),
        data
    );
    assert!(format!("{data:?}").contains("property_count"));
    assert!(!format!("{data:?}").contains("preprod"));

    for value in [
        json!({"secret": "bytes"}),
        json!({"nested": {"private_key_multibase": "bytes"}}),
        json!({"key": {"kty": "EC", "d": "bytes"}}),
        json!({"decrypted-payload": "bytes"}),
    ] {
        let map = value.as_object().unwrap().clone().into_iter().collect();
        assert!(matches!(
            RegistrationPublicData::new(map),
            Err(Error::InvalidRegistration(
                RegistrationError::PrivateMaterial
            ))
        ));
    }

    assert!(matches!(
        RegistrationPublicData::new(BTreeMap::from([("jobId".to_owned(), json!("shadow"))])),
        Err(Error::InvalidRegistration(
            RegistrationError::ReservedProperty
        ))
    ));
}

#[test]
fn public_data_enforces_raw_and_structural_resource_bounds() {
    assert!(matches!(
        RegistrationPublicData::from_json_slice(&vec![b' '; MAX_DID_REGISTRATION_BYTES + 1]),
        Err(Error::InvalidRegistration(RegistrationError::TooLarge))
    ));
    assert!(matches!(
        RegistrationPublicData::from_json_str("[]"),
        Err(Error::InvalidRegistration(RegistrationError::MalformedJson))
    ));

    let properties = (0..65)
        .map(|index| (format!("p{index}"), Value::Null))
        .collect();
    assert!(matches!(
        RegistrationPublicData::new(properties),
        Err(Error::InvalidRegistration(
            RegistrationError::TooManyProperties
        ))
    ));
    assert!(
        RegistrationPublicData::new(BTreeMap::from([(
            "items".to_owned(),
            Value::Array(vec![Value::Null; MAX_REGISTRATION_ITEMS + 1]),
        )]))
        .is_err()
    );
    assert!(
        RegistrationPublicData::new(BTreeMap::from([(
            "large".to_owned(),
            Value::String("x".repeat(65_537)),
        )]))
        .is_err()
    );
    assert!(
        RegistrationPublicData::new(BTreeMap::from([(
            "text".to_owned(),
            Value::String("line\nbreak".to_owned()),
        )]))
        .is_err()
    );
    assert!(
        RegistrationPublicData::new(BTreeMap::from([(
            "nested".to_owned(),
            Value::Object(
                (0..65)
                    .map(|index| (format!("p{index}"), Value::Null))
                    .collect(),
            ),
        )]))
        .is_err()
    );

    let wide_tree = (0..33)
        .map(|index| {
            (
                format!("a{index}"),
                Value::Array(vec![Value::Null; MAX_REGISTRATION_ITEMS]),
            )
        })
        .collect();
    assert!(matches!(
        RegistrationPublicData::new(wide_tree),
        Err(Error::InvalidRegistration(RegistrationError::TooManyNodes))
    ));

    let mut nested = Value::Null;
    for _ in 0..33 {
        nested = json!({"nested": nested});
    }
    assert!(RegistrationPublicData::new(BTreeMap::from([("root".to_owned(), nested)])).is_err());
}

#[test]
fn public_accessors_preserve_values_while_all_debug_surfaces_redact_payloads() {
    let parsed = RegistrationPublicData::from_json_slice(br#"{"network":"preview"}"#).unwrap();
    assert_eq!(parsed.as_map()["network"], "preview");
    assert_eq!(parsed.clone().into_map()["network"], "preview");

    let action_id = RegistrationActionId::parse("sensitive-action").unwrap();
    let action = RegistrationAction::new(
        action_id.clone(),
        RegistrationOperationName::parse("sensitive-operation").unwrap(),
        parsed.clone(),
    );
    assert_eq!(action.name().as_str(), "sensitive-operation");
    assert_eq!(action.data(), &parsed);
    assert!(!format!("{action:?}").contains("sensitive-operation"));
    let response = RegistrationActionResponse::new(action_id.clone(), parsed.clone());
    assert_eq!(response.data(), &parsed);
    assert!(!format!("{response:?}").contains("sensitive-action"));

    let job = RegistrationJob::new(
        method("prism"),
        RegistrationJobId::parse("sensitive-job").unwrap(),
        RegistrationContinuation::Action(action_id),
    );
    assert_eq!(job.method().as_str(), "prism");
    assert!(!format!("{job:?}").contains("sensitive-job"));
    assert_eq!(
        RegistrationJobId::parse("owned-job").unwrap().into_string(),
        "owned-job"
    );
    assert_eq!(
        RegistrationActionId::parse("owned-action")
            .unwrap()
            .into_string(),
        "owned-action"
    );
    assert_eq!(
        RegistrationSecretHandle::parse("owned-handle")
            .unwrap()
            .into_string(),
        "owned-handle"
    );
    assert_eq!(
        RegistrationOperationName::parse("owned-name")
            .unwrap()
            .into_string(),
        "owned-name"
    );
    assert_eq!(
        RegistrationFailureCode::parse("owned-code")
            .unwrap()
            .into_string(),
        "owned-code"
    );
    assert_eq!(
        RegistrationIdempotencyKey::parse("owned-key")
            .unwrap()
            .into_string(),
        "owned-key"
    );

    let document = DidDocument::builder(Did::parse("did:prism:requested").unwrap())
        .build()
        .unwrap();
    let create_request = CreateRegistrationRequest::new(
        method("prism"),
        Some(document.id().clone()),
        Some(document.clone()),
        parsed.clone(),
        RegistrationSecretMode::External(
            RegistrationSecretHandle::parse("custody-handle").unwrap(),
        ),
        key("create-accessors"),
    )
    .unwrap();
    assert_eq!(create_request.document(), Some(&document));
    assert_eq!(create_request.options(), &parsed);
    assert!(matches!(
        create_request.secret_mode(),
        RegistrationSecretMode::External(_)
    ));
    assert_eq!(
        create_request.idempotency_key().as_str(),
        "create-accessors"
    );

    let update = UpdateRegistrationRequest::new(
        document.id().clone(),
        vec![DidDocumentOperation::SetDidDocument(Box::new(
            document.clone(),
        ))],
        parsed.clone(),
        RegistrationSecretMode::ClientManaged,
        key("update-accessors"),
    )
    .unwrap();
    assert_eq!(update.options(), &parsed);
    assert!(matches!(
        update.secret_mode(),
        RegistrationSecretMode::ClientManaged
    ));
    assert_eq!(update.idempotency_key().as_str(), "update-accessors");

    let deactivate = DeactivateRegistrationRequest::new(
        document.id().clone(),
        parsed.clone(),
        RegistrationSecretMode::ClientManaged,
        key("deactivate-accessors"),
    )
    .unwrap();
    assert_eq!(deactivate.method().as_str(), "prism");
    assert_eq!(deactivate.options(), &parsed);
    assert!(matches!(
        deactivate.secret_mode(),
        RegistrationSecretMode::ClientManaged
    ));
    assert_eq!(
        deactivate.idempotency_key().as_str(),
        "deactivate-accessors"
    );

    let continue_request =
        ContinueRegistrationRequest::new(job.clone(), Some(response), key("continue-accessors"))
            .unwrap();
    assert!(continue_request.response().is_some());
    assert_eq!(
        continue_request.idempotency_key().as_str(),
        "continue-accessors"
    );
    let cancel_request = CancelRegistrationRequest::new(job, key("cancel-accessors"));
    assert_eq!(
        cancel_request.idempotency_key().as_str(),
        "cancel-accessors"
    );

    for request in [
        RegistrationRequest::Create(create_request),
        RegistrationRequest::Update(update),
        RegistrationRequest::Deactivate(deactivate),
        RegistrationRequest::Continue(continue_request),
        RegistrationRequest::Cancel(cancel_request),
    ] {
        assert!(!format!("{request:?}").contains("preview"));
        assert!(!request.idempotency_key().as_str().is_empty());
    }
}

#[test]
fn secret_policy_never_silently_discards_generated_capability() {
    assert!(matches!(
        InternalSecretPolicy::new(false, false),
        Err(Error::InvalidRegistration(
            RegistrationError::InvalidSecretPolicy
        ))
    ));
    let store = InternalSecretPolicy::new(true, false).unwrap();
    assert!(store.store_generated());
    assert!(!store.return_handle());
    let returned = InternalSecretPolicy::new(false, true).unwrap();
    assert!(!returned.store_generated());
    assert!(returned.return_handle());
}

#[test]
fn creation_enforces_requested_did_and_document_method_identity() {
    let prism = Did::parse("did:prism:123").unwrap();
    let midnight_document = DidDocument::builder(Did::parse("did:midnight:123").unwrap())
        .build()
        .unwrap();
    let request = CreateRegistrationRequest::new(
        method("prism"),
        Some(prism.clone()),
        None,
        RegistrationPublicData::empty(),
        RegistrationSecretMode::ClientManaged,
        key("create-1"),
    )
    .unwrap();
    assert_eq!(request.requested_did(), Some(&prism));
    assert_eq!(request.method().as_str(), "prism");

    assert!(
        CreateRegistrationRequest::new(
            method("prism"),
            None,
            Some(midnight_document),
            RegistrationPublicData::empty(),
            RegistrationSecretMode::ClientManaged,
            key("create-2"),
        )
        .is_err()
    );

    let secret_shaped_document = DidDocument::builder(Did::parse("did:prism:secret-doc").unwrap())
        .extensions(BTreeMap::from([(
            "privateKeyBase58".to_owned(),
            json!("bytes"),
        )]))
        .build()
        .unwrap();
    assert!(matches!(
        CreateRegistrationRequest::new(
            method("prism"),
            None,
            Some(secret_shaped_document),
            RegistrationPublicData::empty(),
            RegistrationSecretMode::ClientManaged,
            key("create-3"),
        ),
        Err(Error::InvalidRegistration(
            RegistrationError::PrivateMaterial
        ))
    ));
}

#[test]
fn updates_preserve_order_and_reject_empty_or_mismatched_sets() {
    let did = Did::parse("did:prism:123").unwrap();
    let add =
        RegistrationPublicData::new(BTreeMap::from([("service".to_owned(), json!(1))])).unwrap();
    let remove =
        RegistrationPublicData::new(BTreeMap::from([("service".to_owned(), json!(0))])).unwrap();
    let request = UpdateRegistrationRequest::new(
        did.clone(),
        vec![
            DidDocumentOperation::AddToDidDocument(add.clone()),
            DidDocumentOperation::RemoveFromDidDocument(remove.clone()),
        ],
        RegistrationPublicData::empty(),
        RegistrationSecretMode::ClientManaged,
        key("update-1"),
    )
    .unwrap();
    assert_eq!(request.method().as_str(), "prism");
    assert_eq!(
        request.operations(),
        &[
            DidDocumentOperation::AddToDidDocument(add),
            DidDocumentOperation::RemoveFromDidDocument(remove),
        ]
    );

    assert!(
        UpdateRegistrationRequest::new(
            did.clone(),
            Vec::new(),
            RegistrationPublicData::empty(),
            RegistrationSecretMode::ClientManaged,
            key("update-2"),
        )
        .is_err()
    );
    let other = DidDocument::builder(Did::parse("did:prism:other").unwrap())
        .build()
        .unwrap();
    assert!(
        UpdateRegistrationRequest::new(
            did,
            vec![DidDocumentOperation::SetDidDocument(Box::new(other))],
            RegistrationPublicData::empty(),
            RegistrationSecretMode::ClientManaged,
            key("update-3"),
        )
        .is_err()
    );
}

#[test]
fn continuation_requires_exact_action_response_or_pure_wait_poll() {
    let action_id = RegistrationActionId::parse("sign-1").unwrap();
    let action_job = RegistrationJob::new(
        method("prism"),
        RegistrationJobId::parse("job-1").unwrap(),
        RegistrationContinuation::Action(action_id.clone()),
    );
    assert!(ContinueRegistrationRequest::new(action_job.clone(), None, key("continue-1")).is_err());
    assert!(
        ContinueRegistrationRequest::new(
            action_job.clone(),
            Some(RegistrationActionResponse::new(
                RegistrationActionId::parse("sign-2").unwrap(),
                RegistrationPublicData::empty(),
            )),
            key("continue-2"),
        )
        .is_err()
    );
    assert!(
        ContinueRegistrationRequest::new(
            action_job,
            Some(RegistrationActionResponse::new(
                action_id,
                RegistrationPublicData::empty(),
            )),
            key("continue-3"),
        )
        .is_ok()
    );

    let wait_job = RegistrationJob::new(
        method("prism"),
        RegistrationJobId::parse("job-2").unwrap(),
        RegistrationContinuation::Wait,
    );
    assert!(ContinueRegistrationRequest::new(wait_job.clone(), None, key("poll-1")).is_ok());
    assert!(
        ContinueRegistrationRequest::new(
            wait_job,
            Some(RegistrationActionResponse::new(
                RegistrationActionId::parse("invented").unwrap(),
                RegistrationPublicData::empty(),
            )),
            key("poll-2"),
        )
        .is_err()
    );
}

#[test]
fn result_construction_enforces_terminal_job_and_nonterminal_correlation() {
    let prism = method("prism");
    let did = Did::parse("did:prism:123").unwrap();
    let wait_job = RegistrationJob::new(
        prism.clone(),
        RegistrationJobId::parse("job-1").unwrap(),
        RegistrationContinuation::Wait,
    );
    assert!(
        DidRegistrationResult::new(
            prism.clone(),
            Some(wait_job.clone()),
            DidRegistrationState::Finished {
                did: did.clone(),
                document: None,
                secret_handles: Vec::new(),
            },
            RegistrationPublicData::empty(),
            DidDocumentMetadata::empty(),
        )
        .is_err()
    );
    assert!(
        DidRegistrationResult::new(
            prism.clone(),
            None,
            DidRegistrationState::Wait {
                did: Some(did.clone()),
                retry_after_millis: Some(1),
            },
            RegistrationPublicData::empty(),
            DidDocumentMetadata::empty(),
        )
        .is_err()
    );
    assert!(
        DidRegistrationResult::new(
            prism.clone(),
            Some(wait_job.clone()),
            DidRegistrationState::Wait {
                did: Some(did.clone()),
                retry_after_millis: Some(MAX_REGISTRATION_WAIT_MILLIS + 1),
            },
            RegistrationPublicData::empty(),
            DidDocumentMetadata::empty(),
        )
        .is_err()
    );
    assert!(
        DidRegistrationResult::new(
            prism,
            Some(wait_job),
            DidRegistrationState::Wait {
                did: Some(did),
                retry_after_millis: Some(MAX_REGISTRATION_WAIT_MILLIS),
            },
            RegistrationPublicData::empty(),
            DidDocumentMetadata::empty(),
        )
        .is_ok()
    );

    let excessive_handles = (0..=MAX_REGISTRATION_ITEMS)
        .map(|index| RegistrationSecretHandle::parse(&format!("handle-{index}")).unwrap())
        .collect();
    assert!(matches!(
        DidRegistrationResult::new(
            method("prism"),
            None,
            DidRegistrationState::Finished {
                did: Did::parse("did:prism:123").unwrap(),
                document: None,
                secret_handles: excessive_handles,
            },
            RegistrationPublicData::empty(),
            DidDocumentMetadata::empty(),
        ),
        Err(Error::InvalidRegistration(RegistrationError::TooManyItems))
    ));

    let secret_metadata = DidDocumentMetadata::builder()
        .extensions(BTreeMap::from([("secretKey".to_owned(), json!("bytes"))]))
        .build()
        .unwrap();
    assert!(matches!(
        DidRegistrationResult::new(
            method("prism"),
            None,
            DidRegistrationState::Finished {
                did: Did::parse("did:prism:123").unwrap(),
                document: None,
                secret_handles: Vec::new(),
            },
            RegistrationPublicData::empty(),
            secret_metadata,
        ),
        Err(Error::InvalidRegistration(
            RegistrationError::PrivateMaterial
        ))
    ));
}

#[test]
fn action_results_and_request_validation_bind_method_did_and_job() {
    let action_id = RegistrationActionId::parse("sign-1").unwrap();
    let job = RegistrationJob::new(
        method("prism"),
        RegistrationJobId::parse("job-1").unwrap(),
        RegistrationContinuation::Action(action_id.clone()),
    );
    let action = RegistrationAction::new(
        action_id,
        RegistrationOperationName::parse("signPayload").unwrap(),
        RegistrationPublicData::empty(),
    );
    let result = DidRegistrationResult::new(
        method("prism"),
        Some(job.clone()),
        DidRegistrationState::Action {
            did: Some(Did::parse("did:prism:123").unwrap()),
            action,
        },
        RegistrationPublicData::empty(),
        DidDocumentMetadata::empty(),
    )
    .unwrap();
    let request = RegistrationRequest::Continue(
        ContinueRegistrationRequest::new(
            job,
            Some(RegistrationActionResponse::new(
                RegistrationActionId::parse("sign-1").unwrap(),
                RegistrationPublicData::empty(),
            )),
            key("continue-1"),
        )
        .unwrap(),
    );
    result.validate_for_request(&request).unwrap();

    let other_job = RegistrationJob::new(
        method("prism"),
        RegistrationJobId::parse("job-other").unwrap(),
        RegistrationContinuation::Wait,
    );
    let wait = DidRegistrationResult::new(
        method("prism"),
        Some(other_job),
        DidRegistrationState::Wait {
            did: None,
            retry_after_millis: None,
        },
        RegistrationPublicData::empty(),
        DidDocumentMetadata::empty(),
    )
    .unwrap();
    assert!(wait.validate_for_request(&request).is_err());
}

fn result(
    method_name: &str,
    job: Option<RegistrationJob>,
    state: DidRegistrationState,
) -> DidRegistrationResult {
    DidRegistrationResult::new(
        method(method_name),
        job,
        state,
        RegistrationPublicData::empty(),
        DidDocumentMetadata::empty(),
    )
    .unwrap()
}

struct PrismShapeRegistrar;

impl DidRegistrar for PrismShapeRegistrar {
    fn execute<'a>(&'a self, request: &'a RegistrationRequest) -> DidRegistrationFuture<'a> {
        Box::pin(async move {
            let did = match request {
                RegistrationRequest::Create(request) => request
                    .requested_did()
                    .cloned()
                    .unwrap_or_else(|| Did::parse("did:prism:created").unwrap()),
                RegistrationRequest::Update(request) => request.did().clone(),
                RegistrationRequest::Deactivate(request) => request.did().clone(),
                RegistrationRequest::Continue(_) | RegistrationRequest::Cancel(_) => {
                    return DidRegistrationResult::standard_failure(
                        request.method().clone(),
                        DidRegistrationErrorKind::FeatureNotSupported,
                    );
                }
            };
            finished("prism", did)
        })
    }
}

struct MidnightShapeRegistrar;

impl DidRegistrar for MidnightShapeRegistrar {
    fn execute<'a>(&'a self, request: &'a RegistrationRequest) -> DidRegistrationFuture<'a> {
        Box::pin(async move {
            match request {
                RegistrationRequest::Create(_) => {
                    let action_id = RegistrationActionId::parse("prove-controller").unwrap();
                    let job = RegistrationJob::new(
                        method("midnight"),
                        RegistrationJobId::parse("create-job").unwrap(),
                        RegistrationContinuation::Action(action_id.clone()),
                    );
                    result(
                        "midnight",
                        Some(job),
                        DidRegistrationState::Action {
                            did: None,
                            action: RegistrationAction::new(
                                action_id,
                                RegistrationOperationName::parse("signPayload").unwrap(),
                                RegistrationPublicData::empty(),
                            ),
                        },
                    )
                }
                RegistrationRequest::Update(request) => {
                    wait_for("update-job", Some(request.did().clone()), Some(250))
                }
                RegistrationRequest::Deactivate(request) => {
                    wait_for("deactivate-job", Some(request.did().clone()), None)
                }
                RegistrationRequest::Continue(request) => match request.job().id().as_str() {
                    "create-job"
                        if matches!(
                            request.job().continuation(),
                            RegistrationContinuation::Action(_)
                        ) =>
                    {
                        wait_for(
                            "create-job",
                            Some(Did::parse("did:midnight:created").unwrap()),
                            Some(100),
                        )
                    }
                    "create-job" => {
                        finished("midnight", Did::parse("did:midnight:created").unwrap())
                    }
                    "update-job" => {
                        finished("midnight", Did::parse("did:midnight:subject").unwrap())
                    }
                    "deactivate-job" => {
                        finished("midnight", Did::parse("did:midnight:subject").unwrap())
                    }
                    _ => DidRegistrationResult::standard_failure(
                        method("midnight"),
                        DidRegistrationErrorKind::InternalError,
                    ),
                },
                RegistrationRequest::Cancel(_) => DidRegistrationResult::standard_failure(
                    method("midnight"),
                    DidRegistrationErrorKind::Cancelled,
                ),
            }
        })
    }
}

fn wait_for(
    job_id: &str,
    did: Option<Did>,
    retry_after_millis: Option<u64>,
) -> DidRegistrationResult {
    let job = RegistrationJob::new(
        method("midnight"),
        RegistrationJobId::parse(job_id).unwrap(),
        RegistrationContinuation::Wait,
    );
    result(
        "midnight",
        Some(job),
        DidRegistrationState::Wait {
            did,
            retry_after_millis,
        },
    )
}

#[test]
fn prism_and_midnight_shapes_cover_immediate_and_multistep_lifecycles() {
    let prism: Arc<dyn DidRegistrar> = Arc::new(PrismShapeRegistrar);
    let create_prism = create("prism", "prism-create");
    block_on(prism.execute(&create_prism))
        .validate_for_request(&create_prism)
        .unwrap();
    let prism_did = Did::parse("did:prism:subject").unwrap();
    let update_prism = RegistrationRequest::Update(
        UpdateRegistrationRequest::new(
            prism_did.clone(),
            vec![DidDocumentOperation::MethodSpecific {
                name: RegistrationOperationName::parse("publishUpdate").unwrap(),
                data: None,
            }],
            RegistrationPublicData::empty(),
            RegistrationSecretMode::ClientManaged,
            key("prism-update"),
        )
        .unwrap(),
    );
    block_on(prism.execute(&update_prism))
        .validate_for_request(&update_prism)
        .unwrap();
    let deactivate_prism = RegistrationRequest::Deactivate(
        DeactivateRegistrationRequest::new(
            prism_did,
            RegistrationPublicData::empty(),
            RegistrationSecretMode::ClientManaged,
            key("prism-deactivate"),
        )
        .unwrap(),
    );
    block_on(prism.execute(&deactivate_prism))
        .validate_for_request(&deactivate_prism)
        .unwrap();

    let midnight: Arc<dyn DidRegistrar> = Arc::new(MidnightShapeRegistrar);
    let create_midnight = create("midnight", "midnight-create");
    let action_result = block_on(midnight.execute(&create_midnight));
    action_result
        .validate_for_request(&create_midnight)
        .unwrap();
    let action_job = action_result.job().unwrap().clone();
    let action_id = match action_result.state() {
        DidRegistrationState::Action { action, .. } => action.id().clone(),
        _ => panic!("create should require a controller proof"),
    };
    let action_response = RegistrationRequest::Continue(
        ContinueRegistrationRequest::new(
            action_job,
            Some(RegistrationActionResponse::new(
                action_id,
                RegistrationPublicData::empty(),
            )),
            key("midnight-create-action"),
        )
        .unwrap(),
    );
    let wait_result = block_on(midnight.execute(&action_response));
    wait_result.validate_for_request(&action_response).unwrap();
    let poll = RegistrationRequest::Continue(
        ContinueRegistrationRequest::new(
            wait_result.job().unwrap().clone(),
            None,
            key("midnight-create-poll"),
        )
        .unwrap(),
    );
    let created = block_on(midnight.execute(&poll));
    created.validate_for_request(&poll).unwrap();
    assert!(created.state().is_terminal());

    let midnight_did = Did::parse("did:midnight:subject").unwrap();
    let update_midnight = RegistrationRequest::Update(
        UpdateRegistrationRequest::new(
            midnight_did.clone(),
            vec![DidDocumentOperation::MethodSpecific {
                name: RegistrationOperationName::parse("submitUpdate").unwrap(),
                data: None,
            }],
            RegistrationPublicData::empty(),
            RegistrationSecretMode::ClientManaged,
            key("midnight-update"),
        )
        .unwrap(),
    );
    let update_wait = block_on(midnight.execute(&update_midnight));
    update_wait.validate_for_request(&update_midnight).unwrap();
    let update_poll = RegistrationRequest::Continue(
        ContinueRegistrationRequest::new(
            update_wait.job().unwrap().clone(),
            None,
            key("midnight-update-poll"),
        )
        .unwrap(),
    );
    block_on(midnight.execute(&update_poll))
        .validate_for_request(&update_poll)
        .unwrap();

    let deactivate_midnight = RegistrationRequest::Deactivate(
        DeactivateRegistrationRequest::new(
            midnight_did,
            RegistrationPublicData::empty(),
            RegistrationSecretMode::ClientManaged,
            key("midnight-deactivate"),
        )
        .unwrap(),
    );
    let deactivate_wait = block_on(midnight.execute(&deactivate_midnight));
    deactivate_wait
        .validate_for_request(&deactivate_midnight)
        .unwrap();
    let deactivate_poll = RegistrationRequest::Continue(
        ContinueRegistrationRequest::new(
            deactivate_wait.job().unwrap().clone(),
            None,
            key("midnight-deactivate-poll"),
        )
        .unwrap(),
    );
    block_on(midnight.execute(&deactivate_poll))
        .validate_for_request(&deactivate_poll)
        .unwrap();
}

struct NeverResolver;

impl DidResolver for NeverResolver {
    fn resolve<'a>(
        &'a self,
        _did: &'a Did,
        _options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async { panic!("registration dispatch must not call resolution") })
    }
}

struct FixedRegistrar;

impl DidRegistrar for FixedRegistrar {
    fn execute<'a>(&'a self, request: &'a RegistrationRequest) -> DidRegistrationFuture<'a> {
        Box::pin(async move {
            finished(
                request.method().as_str(),
                Did::parse(&format!("did:{}:created", request.method())).unwrap(),
            )
        })
    }
}

#[test]
fn registry_dispatches_exactly_and_reports_independent_support() {
    let prism = DidMethodBinding::new(method("prism"), Arc::new(NeverResolver))
        .with_registrar(Arc::new(FixedRegistrar));
    let midnight = DidMethodBinding::new(method("midnight"), Arc::new(NeverResolver));
    let registry = DidMethodRegistry::builder()
        .register(prism)
        .unwrap()
        .register(midnight)
        .unwrap()
        .build();
    assert!(registry.supports_registration(&method("prism")));
    assert!(!registry.supports_registration(&method("midnight")));

    let result = block_on(registry.execute(&create("prism", "create-1")));
    assert_eq!(result.state().did().unwrap().as_str(), "did:prism:created");
    let unsupported = block_on(registry.execute(&create("midnight", "create-2")));
    assert_eq!(
        match unsupported.state() {
            DidRegistrationState::Failed { code, .. } => code.kind(),
            _ => None,
        },
        Some(DidRegistrationErrorKind::FeatureNotSupported)
    );
    let unknown = block_on(registry.execute(&create("prismx", "create-3")));
    assert_eq!(
        match unknown.state() {
            DidRegistrationState::Failed { code, .. } => code.kind(),
            _ => None,
        },
        Some(DidRegistrationErrorKind::MethodNotSupported)
    );
}

#[test]
fn object_safe_registry_dispatch_is_send_sync_under_concurrency() {
    let registry: Arc<dyn DidRegistrar> = Arc::new(
        DidMethodRegistry::builder()
            .register(
                DidMethodBinding::new(method("prism"), Arc::new(NeverResolver))
                    .with_registrar(Arc::new(FixedRegistrar)),
            )
            .unwrap()
            .build(),
    );
    let barrier = Arc::new(Barrier::new(17));
    let handles = (0..16)
        .map(|index| {
            let registry = Arc::clone(&registry);
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                let request = create("prism", &format!("create-{index}"));
                barrier.wait();
                let result = block_on(registry.execute(&request));
                result.validate_for_request(&request).unwrap();
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
#[ignore = "manual release-mode performance diagnostic"]
fn registration_registry_dispatch_throughput_diagnostic() {
    const ITERATIONS: usize = 100_000;
    let registry: Arc<dyn DidRegistrar> = Arc::new(
        DidMethodRegistry::builder()
            .register(
                DidMethodBinding::new(method("prism"), Arc::new(NeverResolver))
                    .with_registrar(Arc::new(FixedRegistrar)),
            )
            .unwrap()
            .build(),
    );
    let request = create("prism", "performance-diagnostic");

    let started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(block_on(registry.execute(black_box(&request))));
    }
    let elapsed = started.elapsed();
    println!(
        "dispatched {ITERATIONS} DID Registration requests in {elapsed:?} ({:.0} requests/s)",
        ITERATIONS as f64 / elapsed.as_secs_f64()
    );
}

#[derive(Default)]
struct IdempotentRegistrar {
    requests: Mutex<BTreeMap<String, (RegistrationRequest, DidRegistrationResult)>>,
}

impl DidRegistrar for IdempotentRegistrar {
    fn execute<'a>(&'a self, request: &'a RegistrationRequest) -> DidRegistrationFuture<'a> {
        Box::pin(async move {
            let mut requests = self.requests.lock().unwrap();
            let idempotency_key = request.idempotency_key().as_str().to_owned();
            if let Some((canonical, result)) = requests.get(&idempotency_key) {
                return if canonical == request {
                    result.clone()
                } else {
                    DidRegistrationResult::standard_failure(
                        request.method().clone(),
                        DidRegistrationErrorKind::Conflict,
                    )
                };
            }
            let result = finished(
                request.method().as_str(),
                Did::parse(&format!("did:{}:created", request.method())).unwrap(),
            );
            requests.insert(idempotency_key, (request.clone(), result.clone()));
            result
        })
    }
}

#[test]
fn adapter_contract_can_replay_identical_input_and_conflict_on_key_reuse() {
    let registrar = IdempotentRegistrar::default();
    let original = create("prism", "same-key");
    let first = block_on(registrar.execute(&original));
    let replay = block_on(registrar.execute(&original));
    assert_eq!(first, replay);

    let changed = RegistrationRequest::Create(
        CreateRegistrationRequest::new(
            method("prism"),
            None,
            None,
            RegistrationPublicData::new(BTreeMap::from([("network".to_owned(), json!("other"))]))
                .unwrap(),
            RegistrationSecretMode::ClientManaged,
            key("same-key"),
        )
        .unwrap(),
    );
    let conflict = block_on(registrar.execute(&changed));
    assert!(matches!(
        conflict.state(),
        DidRegistrationState::Failed { code, .. }
            if code.kind() == Some(DidRegistrationErrorKind::Conflict)
    ));
}

struct PendingObservation {
    drops: Arc<AtomicUsize>,
}

impl Future for PendingObservation {
    type Output = DidRegistrationResult;

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for PendingObservation {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

struct CancellationRegistrar {
    observation_drops: Arc<AtomicUsize>,
    cancellation_requests: Arc<AtomicUsize>,
}

impl DidRegistrar for CancellationRegistrar {
    fn execute<'a>(&'a self, request: &'a RegistrationRequest) -> DidRegistrationFuture<'a> {
        if matches!(request, RegistrationRequest::Cancel(_)) {
            self.cancellation_requests.fetch_add(1, Ordering::SeqCst);
            return Box::pin(std::future::ready(DidRegistrationResult::standard_failure(
                request.method().clone(),
                DidRegistrationErrorKind::Cancelled,
            )));
        }
        Box::pin(PendingObservation {
            drops: Arc::clone(&self.observation_drops),
        })
    }
}

#[test]
fn dropping_observation_does_not_fabricate_explicit_cancellation() {
    let drops = Arc::new(AtomicUsize::new(0));
    let cancellations = Arc::new(AtomicUsize::new(0));
    let registrar = CancellationRegistrar {
        observation_drops: Arc::clone(&drops),
        cancellation_requests: Arc::clone(&cancellations),
    };
    let create_request = create("prism", "create-1");
    drop(registrar.execute(&create_request));
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(cancellations.load(Ordering::SeqCst), 0);

    let job = RegistrationJob::new(
        method("prism"),
        RegistrationJobId::parse("job-1").unwrap(),
        RegistrationContinuation::Wait,
    );
    let cancel = RegistrationRequest::Cancel(CancelRegistrationRequest::new(job, key("cancel-1")));
    let result = block_on(registrar.execute(&cancel));
    assert_eq!(cancellations.load(Ordering::SeqCst), 1);
    assert!(matches!(
        result.state(),
        DidRegistrationState::Failed { code, .. }
            if code.kind() == Some(DidRegistrationErrorKind::Cancelled)
    ));
}

#[test]
fn errors_and_debug_views_do_not_expose_caller_values() {
    let error = RegistrationJobId::parse("\nwallet-secret").unwrap_err();
    let public = error.to_identus_error();
    assert_eq!(public.code().as_str(), "did.invalid_registration");
    assert_eq!(public.kind(), ErrorKind::InvalidInput);
    assert!(!public.to_string().contains("wallet-secret"));

    let request = create("secretmethod", "secret-idempotency-key");
    let debug = format!("{request:?}");
    assert!(debug.contains("secretmethod"));
    assert!(!debug.contains("secret-idempotency-key"));
}

use std::{hint::black_box, str::FromStr, time::Instant};

use identus_core::ErrorKind;
use identus_credentials::{
    CredentialClaimPath, CredentialClaimPathSegment, CredentialEntityId, CredentialFormat,
    CredentialSchemaId, CredentialType,
};
use identus_presentations::{
    GeneratedPresentation, MAX_GENERATED_PRESENTATION_ARTIFACTS, MAX_GENERATED_PRESENTATION_BYTES,
    MAX_PRESENTATION_ARTIFACT_BINDINGS, MAX_PRESENTATION_ARTIFACT_BYTES,
    MAX_PRESENTATION_CANDIDATE_CLAIMS, MAX_PRESENTATION_CANDIDATES,
    MAX_PRESENTATION_CHALLENGE_BYTES, MAX_PRESENTATION_CREDENTIAL_HANDLE_BYTES,
    MAX_PRESENTATION_DISCLOSURE_SELECTIONS, MAX_PRESENTATION_FILTER_VALUES,
    MAX_PRESENTATION_PURPOSE_BYTES, MAX_PRESENTATION_QUERY_CLAIMS, MAX_PRESENTATION_QUERY_ID_BYTES,
    MAX_PRESENTATION_REQUEST_QUERIES, MAX_PRESENTATION_SELECTION_CLAIMS, PresentationArtifact,
    PresentationArtifactBinding, PresentationCandidateSet, PresentationChallenge,
    PresentationClaimIntent, PresentationClaimRequest, PresentationCredentialCandidate,
    PresentationCredentialFilters, PresentationCredentialHandle, PresentationCredentialQuery,
    PresentationCredentialSelection, PresentationDisclosurePlan, PresentationError,
    PresentationPurpose, PresentationQueryId, PresentationRequest, PresentationSelectedClaim,
};

fn query_id(value: &str) -> PresentationQueryId {
    PresentationQueryId::parse(value).expect("valid query id")
}

fn format(value: &str) -> CredentialFormat {
    CredentialFormat::parse(value).expect("valid format")
}

fn entity(value: &str) -> CredentialEntityId {
    CredentialEntityId::parse(value).expect("valid entity id")
}

fn credential_type(value: &str) -> CredentialType {
    CredentialType::parse(value).expect("valid credential type")
}

fn schema(value: &str) -> CredentialSchemaId {
    CredentialSchemaId::parse(value).expect("valid schema id")
}

fn path(segments: &[&str]) -> CredentialClaimPath {
    CredentialClaimPath::new(
        segments
            .iter()
            .map(|segment| CredentialClaimPathSegment::parse(segment).expect("valid path segment"))
            .collect(),
    )
    .expect("valid path")
}

fn claim(
    segments: &[&str],
    intent: PresentationClaimIntent,
    required: bool,
) -> PresentationClaimRequest {
    PresentationClaimRequest::new(path(segments), intent, required)
}

fn unrestricted_query(
    id: &str,
    format_name: &str,
    claims: Vec<PresentationClaimRequest>,
) -> PresentationCredentialQuery {
    PresentationCredentialQuery::new(
        query_id(id),
        format(format_name),
        false,
        true,
        PresentationCredentialFilters::unrestricted(),
        claims,
    )
    .expect("valid query")
}

fn request(queries: Vec<PresentationCredentialQuery>) -> PresentationRequest {
    PresentationRequest::new(
        entity("https://verifier.example"),
        Some(PresentationPurpose::parse("Prove eligibility").unwrap()),
        Some(PresentationChallenge::from_text("nonce-123").unwrap()),
        queries,
    )
    .expect("valid request")
}

fn candidate(
    query: &str,
    handle: &str,
    format_name: &str,
    claims: Vec<CredentialClaimPath>,
) -> PresentationCredentialCandidate {
    PresentationCredentialCandidate::new(
        query_id(query),
        PresentationCredentialHandle::from_text(handle).unwrap(),
        format(format_name),
        claims,
    )
    .expect("valid candidate")
}

fn selected_claim(segments: &[&str], intent: PresentationClaimIntent) -> PresentationSelectedClaim {
    PresentationSelectedClaim::new(path(segments), intent)
}

fn selection(
    query: &str,
    handle: &str,
    claims: Vec<PresentationSelectedClaim>,
) -> PresentationCredentialSelection {
    PresentationCredentialSelection::new(
        query_id(query),
        PresentationCredentialHandle::from_text(handle).unwrap(),
        claims,
    )
    .expect("valid credential selection")
}

fn binding(query: &str, handle: &str) -> PresentationArtifactBinding {
    PresentationArtifactBinding::new(
        query_id(query),
        PresentationCredentialHandle::from_text(handle).unwrap(),
    )
}

fn artifact(
    format_name: &str,
    bindings: Vec<PresentationArtifactBinding>,
    bytes: Vec<u8>,
) -> PresentationArtifact {
    PresentationArtifact::new(format(format_name), bindings, bytes).expect("valid artifact")
}

fn multi_selection_fixture(
    count: usize,
    format_name: &str,
) -> (PresentationRequest, PresentationDisclosurePlan) {
    let request = request(vec![
        PresentationCredentialQuery::new(
            query_id("query"),
            format(format_name),
            true,
            true,
            PresentationCredentialFilters::unrestricted(),
            Vec::new(),
        )
        .unwrap(),
    ]);
    let candidates = PresentationCandidateSet::new(
        &request,
        (0..count)
            .map(|index| {
                candidate(
                    "query",
                    &format!("credential-{index}"),
                    format_name,
                    Vec::new(),
                )
            })
            .collect(),
    )
    .unwrap();
    let plan = PresentationDisclosurePlan::new(
        &request,
        &candidates,
        (0..count)
            .map(|index| selection("query", &format!("credential-{index}"), Vec::new()))
            .collect(),
    )
    .unwrap();
    (request, plan)
}

#[test]
fn scalar_roles_preserve_exact_boundaries_and_representation() {
    let maximum_query = format!("q{}", "x".repeat(MAX_PRESENTATION_QUERY_ID_BYTES - 1));
    assert_eq!(
        PresentationQueryId::parse(&maximum_query).unwrap().as_str(),
        maximum_query
    );

    let maximum_purpose = "p".repeat(MAX_PRESENTATION_PURPOSE_BYTES);
    assert_eq!(
        PresentationPurpose::parse(&maximum_purpose)
            .unwrap()
            .as_str(),
        maximum_purpose
    );

    let challenge_text = "c".repeat(MAX_PRESENTATION_CHALLENGE_BYTES);
    let challenge = PresentationChallenge::from_text(&challenge_text).unwrap();
    assert_eq!(challenge.as_text(), Some(challenge_text.as_str()));
    assert_eq!(challenge.as_bytes(), challenge_text.as_bytes());

    let challenge_bytes = vec![0xff; MAX_PRESENTATION_CHALLENGE_BYTES];
    let challenge = PresentationChallenge::from_bytes(challenge_bytes.clone()).unwrap();
    assert_eq!(challenge.as_text(), None);
    assert_eq!(challenge.as_bytes(), challenge_bytes);

    let handle_bytes = vec![0; MAX_PRESENTATION_CREDENTIAL_HANDLE_BYTES];
    let handle = PresentationCredentialHandle::from_bytes(handle_bytes.clone()).unwrap();
    assert_eq!(handle.as_text(), None);
    assert_eq!(handle.as_bytes(), handle_bytes);
}

#[test]
fn scalar_roles_reject_invalid_input_without_echoing_it() {
    for invalid in [
        "",
        "-leading",
        "white space",
        "query/slash",
        "query\nnewline",
        "é",
    ] {
        let error = PresentationQueryId::parse(invalid).expect_err("invalid query id");
        assert_eq!(error, PresentationError::InvalidQueryId);
        if !invalid.is_empty() {
            assert!(!error.to_string().contains(invalid));
        }
    }
    assert_eq!(
        PresentationQueryId::parse(&"q".repeat(MAX_PRESENTATION_QUERY_ID_BYTES + 1)),
        Err(PresentationError::InvalidQueryId)
    );

    for invalid in ["", " padded", "padded ", "line\nbreak"] {
        assert_eq!(
            PresentationPurpose::parse(invalid),
            Err(PresentationError::InvalidPurpose)
        );
        assert_eq!(
            PresentationChallenge::from_text(invalid),
            Err(PresentationError::InvalidChallenge)
        );
        assert_eq!(
            PresentationCredentialHandle::from_text(invalid),
            Err(PresentationError::InvalidCredentialHandle)
        );
    }
    assert_eq!(
        PresentationPurpose::parse(&"p".repeat(MAX_PRESENTATION_PURPOSE_BYTES + 1)),
        Err(PresentationError::InvalidPurpose)
    );
    assert_eq!(
        PresentationChallenge::from_bytes(Vec::new()),
        Err(PresentationError::InvalidChallenge)
    );
    assert_eq!(
        PresentationCredentialHandle::from_bytes(vec![
            0;
            MAX_PRESENTATION_CREDENTIAL_HANDLE_BYTES + 1
        ]),
        Err(PresentationError::InvalidCredentialHandle)
    );
}

#[test]
fn claim_intents_and_requests_are_value_free() {
    for intent in [
        PresentationClaimIntent::Reveal,
        PresentationClaimIntent::Predicate,
    ] {
        assert_eq!(
            PresentationClaimIntent::from_str(intent.as_str()),
            Ok(intent)
        );
        assert_eq!(intent.to_string(), intent.as_str());
    }
    assert_eq!(
        PresentationClaimIntent::parse("age_over"),
        Err(PresentationError::InvalidClaimIntent)
    );

    let request = claim(
        &["credentialSubject", "dateOfBirth"],
        PresentationClaimIntent::Predicate,
        true,
    );
    assert_eq!(request.path().segments().len(), 2);
    assert_eq!(request.intent(), PresentationClaimIntent::Predicate);
    assert!(request.required());
}

#[test]
fn filters_distinguish_unrestricted_from_bounded_unique_values() {
    let unrestricted = PresentationCredentialFilters::unrestricted();
    assert_eq!(unrestricted.accepted_issuers(), None);
    assert_eq!(unrestricted.accepted_types(), None);
    assert_eq!(unrestricted.accepted_schemas(), None);

    let filters = PresentationCredentialFilters::new(
        Some(
            (0..MAX_PRESENTATION_FILTER_VALUES)
                .map(|index| entity(&format!("did:example:issuer-{index}")))
                .collect(),
        ),
        Some(vec![credential_type("ExampleCredential")]),
        Some(vec![schema("https://example.test/schema")]),
    )
    .expect("bounded filters");
    assert_eq!(
        filters.accepted_issuers().unwrap().len(),
        MAX_PRESENTATION_FILTER_VALUES
    );

    assert_eq!(
        PresentationCredentialFilters::new(Some(Vec::new()), None, None),
        Err(PresentationError::InvalidQueryFilters)
    );
    assert_eq!(
        PresentationCredentialFilters::new(
            None,
            Some(
                (0..=MAX_PRESENTATION_FILTER_VALUES)
                    .map(|index| credential_type(&format!("Type{index}")))
                    .collect()
            ),
            None,
        ),
        Err(PresentationError::InvalidQueryFilters)
    );
    assert_eq!(
        PresentationCredentialFilters::new(
            Some(vec![entity("did:example:one"), entity("did:example:one")]),
            None,
            None,
        ),
        Err(PresentationError::DuplicateIssuerFilter)
    );
    assert_eq!(
        PresentationCredentialFilters::new(
            None,
            Some(vec![credential_type("Type"), credential_type("Type")]),
            None,
        ),
        Err(PresentationError::DuplicateTypeFilter)
    );
    assert_eq!(
        PresentationCredentialFilters::new(
            None,
            None,
            Some(vec![schema("schema:one"), schema("schema:one")]),
        ),
        Err(PresentationError::DuplicateSchemaFilter)
    );
}

#[test]
fn dcql_midnight_and_dummy_queries_share_one_request() {
    let dcql = PresentationCredentialQuery::new(
        query_id("identity_card"),
        format("dc+sd-jwt"),
        false,
        true,
        PresentationCredentialFilters::new(
            Some(vec![entity("https://issuer.example")]),
            Some(vec![credential_type(
                "https://credentials.example/identity_credential",
            )]),
            None,
        )
        .unwrap(),
        vec![
            claim(&["last_name"], PresentationClaimIntent::Reveal, true),
            claim(&["postal_code"], PresentationClaimIntent::Reveal, false),
        ],
    )
    .unwrap();
    let midnight = unrestricted_query(
        "age_proof",
        "midnight_cbor_phase1",
        vec![claim(
            &["credentialSubject", "dateOfBirth"],
            PresentationClaimIntent::Predicate,
            true,
        )],
    );
    let dummy = PresentationCredentialQuery::new(
        query_id("future-format"),
        format("example+future"),
        true,
        false,
        PresentationCredentialFilters::new(
            None,
            None,
            Some(vec![schema("urn:example:schema:future")]),
        )
        .unwrap(),
        Vec::new(),
    )
    .unwrap();
    let request = request(vec![dcql, midnight, dummy]);

    assert_eq!(request.queries().len(), 3);
    assert_eq!(request.queries()[0].id().as_str(), "identity_card");
    assert_eq!(request.queries()[0].format().as_str(), "dc+sd-jwt");
    assert!(!request.queries()[0].multiple());
    assert!(request.queries()[0].requires_holder_binding());
    assert_eq!(request.queries()[1].claims().len(), 1);
    assert!(request.queries()[2].multiple());
    assert_eq!(request.verifier().as_str(), "https://verifier.example");
    assert_eq!(request.purpose().unwrap().as_str(), "Prove eligibility");
    assert_eq!(request.challenge().unwrap().as_text(), Some("nonce-123"));
}

#[test]
fn query_claim_collections_are_bounded_and_unique() {
    let maximum = (0..MAX_PRESENTATION_QUERY_CLAIMS)
        .map(|index| {
            claim(
                &[&format!("claim{index}")],
                PresentationClaimIntent::Reveal,
                index == 0,
            )
        })
        .collect();
    assert_eq!(
        unrestricted_query("maximum", "dc+sd-jwt", maximum)
            .claims()
            .len(),
        MAX_PRESENTATION_QUERY_CLAIMS
    );

    let oversized = (0..=MAX_PRESENTATION_QUERY_CLAIMS)
        .map(|index| {
            claim(
                &[&format!("claim{index}")],
                PresentationClaimIntent::Reveal,
                false,
            )
        })
        .collect();
    assert_eq!(
        PresentationCredentialQuery::new(
            query_id("oversized"),
            format("example"),
            false,
            false,
            PresentationCredentialFilters::unrestricted(),
            oversized,
        ),
        Err(PresentationError::InvalidQueryClaims)
    );

    let duplicate = claim(&["same"], PresentationClaimIntent::Reveal, true);
    assert_eq!(
        PresentationCredentialQuery::new(
            query_id("duplicate"),
            format("example"),
            false,
            false,
            PresentationCredentialFilters::unrestricted(),
            vec![duplicate.clone(), duplicate],
        ),
        Err(PresentationError::DuplicateQueryClaim)
    );
}

#[test]
fn request_query_collections_are_non_empty_bounded_and_unique() {
    assert_eq!(
        PresentationRequest::new(entity("did:example:verifier"), None, None, Vec::new()),
        Err(PresentationError::InvalidRequestQueries)
    );

    let maximum = (0..MAX_PRESENTATION_REQUEST_QUERIES)
        .map(|index| unrestricted_query(&format!("query-{index}"), "example", Vec::new()))
        .collect();
    assert_eq!(
        PresentationRequest::new(entity("did:example:verifier"), None, None, maximum)
            .unwrap()
            .queries()
            .len(),
        MAX_PRESENTATION_REQUEST_QUERIES
    );

    let oversized = (0..=MAX_PRESENTATION_REQUEST_QUERIES)
        .map(|index| unrestricted_query(&format!("query-{index}"), "example", Vec::new()))
        .collect();
    assert_eq!(
        PresentationRequest::new(entity("did:example:verifier"), None, None, oversized),
        Err(PresentationError::InvalidRequestQueries)
    );

    let repeated = unrestricted_query("same", "example", Vec::new());
    assert_eq!(
        PresentationRequest::new(
            entity("did:example:verifier"),
            None,
            None,
            vec![repeated.clone(), repeated],
        ),
        Err(PresentationError::DuplicateQueryId)
    );
}

#[test]
fn candidate_sets_validate_complete_dcql_and_midnight_shapes() {
    let last_name = path(&["last_name"]);
    let postal_code = path(&["postal_code"]);
    let birth_date = path(&["credentialSubject", "dateOfBirth"]);
    let request = request(vec![
        unrestricted_query(
            "identity_card",
            "dc+sd-jwt",
            vec![
                PresentationClaimRequest::new(
                    last_name.clone(),
                    PresentationClaimIntent::Reveal,
                    true,
                ),
                PresentationClaimRequest::new(postal_code, PresentationClaimIntent::Reveal, false),
            ],
        ),
        unrestricted_query(
            "age_proof",
            "midnight_cbor_phase1",
            vec![PresentationClaimRequest::new(
                birth_date.clone(),
                PresentationClaimIntent::Predicate,
                true,
            )],
        ),
    ]);

    let set = PresentationCandidateSet::new(
        &request,
        vec![
            candidate("identity_card", "local-vc-1", "dc+sd-jwt", vec![last_name]),
            candidate(
                "age_proof",
                "local-vc-2",
                "midnight_cbor_phase1",
                vec![birth_date],
            ),
        ],
    )
    .expect("valid candidates");
    assert_eq!(set.as_slice().len(), 2);
    assert_eq!(
        set.as_slice()[0].credential_handle().as_text(),
        Some("local-vc-1")
    );
    assert_eq!(set.as_slice()[1].query_id().as_str(), "age_proof");

    let no_match = PresentationCandidateSet::new(&request, Vec::new()).unwrap();
    assert!(no_match.into_vec().is_empty());
}

#[test]
fn candidate_construction_enforces_claim_bounds_and_uniqueness() {
    let maximum = (0..MAX_PRESENTATION_CANDIDATE_CLAIMS)
        .map(|index| path(&[&format!("claim{index}")]))
        .collect();
    assert_eq!(
        candidate("query", "handle", "example", maximum)
            .satisfiable_claims()
            .len(),
        MAX_PRESENTATION_CANDIDATE_CLAIMS
    );

    let oversized = (0..=MAX_PRESENTATION_CANDIDATE_CLAIMS)
        .map(|index| path(&[&format!("claim{index}")]))
        .collect();
    assert_eq!(
        PresentationCredentialCandidate::new(
            query_id("query"),
            PresentationCredentialHandle::from_text("handle").unwrap(),
            format("example"),
            oversized,
        ),
        Err(PresentationError::InvalidCandidateClaims)
    );
    let same = path(&["same"]);
    assert_eq!(
        PresentationCredentialCandidate::new(
            query_id("query"),
            PresentationCredentialHandle::from_text("handle").unwrap(),
            format("example"),
            vec![same.clone(), same],
        ),
        Err(PresentationError::DuplicateCandidateClaim)
    );
}

#[test]
fn candidate_sets_reject_cross_query_format_and_claim_mismatches() {
    let required = path(&["required"]);
    let optional = path(&["optional"]);
    let query = unrestricted_query(
        "query",
        "dc+sd-jwt",
        vec![
            PresentationClaimRequest::new(required.clone(), PresentationClaimIntent::Reveal, true),
            PresentationClaimRequest::new(optional, PresentationClaimIntent::Reveal, false),
        ],
    );
    let request = request(vec![query]);

    let unknown = candidate("other", "handle", "dc+sd-jwt", vec![required.clone()]);
    assert_eq!(
        PresentationCandidateSet::new(&request, vec![unknown]),
        Err(PresentationError::UnknownCandidateQuery)
    );

    let wrong_format = candidate("query", "handle", "mso_mdoc", vec![required.clone()]);
    assert_eq!(
        PresentationCandidateSet::new(&request, vec![wrong_format]),
        Err(PresentationError::CandidateFormatMismatch)
    );

    let unrequested = candidate(
        "query",
        "handle",
        "dc+sd-jwt",
        vec![required.clone(), path(&["secret_extra"])],
    );
    assert_eq!(
        PresentationCandidateSet::new(&request, vec![unrequested]),
        Err(PresentationError::CandidateUnrequestedClaim)
    );

    let incomplete = candidate("query", "handle", "dc+sd-jwt", Vec::new());
    assert_eq!(
        PresentationCandidateSet::new(&request, vec![incomplete]),
        Err(PresentationError::CandidateMissingRequiredClaim)
    );

    let repeated = candidate("query", "handle", "dc+sd-jwt", vec![required]);
    assert_eq!(
        PresentationCandidateSet::new(&request, vec![repeated.clone(), repeated]),
        Err(PresentationError::DuplicateCandidate)
    );
}

#[test]
fn candidate_set_rejects_only_oversized_collections() {
    let request = request(vec![unrestricted_query("query", "example", Vec::new())]);
    let candidates = (0..MAX_PRESENTATION_CANDIDATES)
        .map(|index| candidate("query", &format!("handle-{index}"), "example", Vec::new()))
        .collect();
    assert_eq!(
        PresentationCandidateSet::new(&request, candidates)
            .unwrap()
            .as_slice()
            .len(),
        MAX_PRESENTATION_CANDIDATES
    );

    let oversized = (0..=MAX_PRESENTATION_CANDIDATES)
        .map(|index| candidate("query", &format!("handle-{index}"), "example", Vec::new()))
        .collect();
    assert_eq!(
        PresentationCandidateSet::new(&request, oversized),
        Err(PresentationError::InvalidCandidates)
    );
}

#[test]
fn disclosure_plan_unifies_dcql_midnight_and_dummy_selections() {
    let family_name = path(&["family_name"]);
    let postal_code = path(&["postal_code"]);
    let birth_date = path(&["credentialSubject", "dateOfBirth"]);
    let request = request(vec![
        unrestricted_query(
            "identity_card",
            "dc+sd-jwt",
            vec![
                PresentationClaimRequest::new(
                    family_name.clone(),
                    PresentationClaimIntent::Reveal,
                    true,
                ),
                PresentationClaimRequest::new(
                    postal_code.clone(),
                    PresentationClaimIntent::Reveal,
                    false,
                ),
            ],
        ),
        PresentationCredentialQuery::new(
            query_id("age_proof"),
            format("midnight_cbor_phase1"),
            true,
            true,
            PresentationCredentialFilters::unrestricted(),
            vec![PresentationClaimRequest::new(
                birth_date.clone(),
                PresentationClaimIntent::Predicate,
                true,
            )],
        )
        .unwrap(),
        unrestricted_query("future_format", "example+future", Vec::new()),
    ]);
    let candidates = PresentationCandidateSet::new(
        &request,
        vec![
            candidate(
                "identity_card",
                "credential-1",
                "dc+sd-jwt",
                vec![family_name, postal_code],
            ),
            candidate(
                "age_proof",
                "credential-2",
                "midnight_cbor_phase1",
                vec![birth_date.clone()],
            ),
            candidate(
                "age_proof",
                "credential-3",
                "midnight_cbor_phase1",
                vec![birth_date],
            ),
            candidate(
                "future_format",
                "credential-4",
                "example+future",
                Vec::new(),
            ),
        ],
    )
    .unwrap();
    let plan = PresentationDisclosurePlan::new(
        &request,
        &candidates,
        vec![
            selection(
                "identity_card",
                "credential-1",
                vec![
                    selected_claim(&["family_name"], PresentationClaimIntent::Reveal),
                    selected_claim(&["postal_code"], PresentationClaimIntent::Reveal),
                ],
            ),
            selection(
                "age_proof",
                "credential-2",
                vec![selected_claim(
                    &["credentialSubject", "dateOfBirth"],
                    PresentationClaimIntent::Predicate,
                )],
            ),
            selection(
                "age_proof",
                "credential-3",
                vec![selected_claim(
                    &["credentialSubject", "dateOfBirth"],
                    PresentationClaimIntent::Predicate,
                )],
            ),
            selection("future_format", "credential-4", Vec::new()),
        ],
    )
    .expect("valid disclosure plan");

    assert_eq!(plan.as_slice().len(), 4);
    assert_eq!(
        plan.as_slice()[0].credential_handle().as_text(),
        Some("credential-1")
    );
    assert_eq!(plan.as_slice()[0].selected_claims().len(), 2);
    assert_eq!(
        plan.as_slice()[1].selected_claims()[0].intent(),
        PresentationClaimIntent::Predicate
    );
    assert!(plan.as_slice()[3].selected_claims().is_empty());
    assert_eq!(plan.into_vec().len(), 4);
}

#[test]
fn credential_selection_claims_are_bounded_and_unique_by_path() {
    let maximum = (0..MAX_PRESENTATION_SELECTION_CLAIMS)
        .map(|index| selected_claim(&[&format!("claim{index}")], PresentationClaimIntent::Reveal))
        .collect();
    assert_eq!(
        selection("query", "handle", maximum)
            .selected_claims()
            .len(),
        MAX_PRESENTATION_SELECTION_CLAIMS
    );

    let oversized = (0..=MAX_PRESENTATION_SELECTION_CLAIMS)
        .map(|index| selected_claim(&[&format!("claim{index}")], PresentationClaimIntent::Reveal))
        .collect();
    assert_eq!(
        PresentationCredentialSelection::new(
            query_id("query"),
            PresentationCredentialHandle::from_text("handle").unwrap(),
            oversized,
        ),
        Err(PresentationError::InvalidSelectionClaims)
    );

    let same_path = path(&["same"]);
    assert_eq!(
        PresentationCredentialSelection::new(
            query_id("query"),
            PresentationCredentialHandle::from_text("handle").unwrap(),
            vec![
                PresentationSelectedClaim::new(same_path.clone(), PresentationClaimIntent::Reveal,),
                PresentationSelectedClaim::new(same_path, PresentationClaimIntent::Predicate),
            ],
        ),
        Err(PresentationError::DuplicateSelectionClaim)
    );
}

#[test]
fn disclosure_plan_collection_is_non_empty_bounded_and_unique() {
    let request = request(vec![
        PresentationCredentialQuery::new(
            query_id("query"),
            format("example"),
            true,
            false,
            PresentationCredentialFilters::unrestricted(),
            Vec::new(),
        )
        .unwrap(),
    ]);
    let candidates = PresentationCandidateSet::new(
        &request,
        (0..MAX_PRESENTATION_DISCLOSURE_SELECTIONS)
            .map(|index| candidate("query", &format!("handle-{index}"), "example", Vec::new()))
            .collect(),
    )
    .unwrap();
    assert_eq!(
        PresentationDisclosurePlan::new(
            &request,
            &candidates,
            (0..MAX_PRESENTATION_DISCLOSURE_SELECTIONS)
                .map(|index| selection("query", &format!("handle-{index}"), Vec::new()))
                .collect(),
        )
        .unwrap()
        .as_slice()
        .len(),
        MAX_PRESENTATION_DISCLOSURE_SELECTIONS
    );
    assert_eq!(
        PresentationDisclosurePlan::new(&request, &candidates, Vec::new()),
        Err(PresentationError::InvalidDisclosureSelections)
    );
    let oversized_candidates = PresentationCandidateSet::new(
        &request,
        (0..=MAX_PRESENTATION_DISCLOSURE_SELECTIONS)
            .map(|index| candidate("query", &format!("extra-{index}"), "example", Vec::new()))
            .take(MAX_PRESENTATION_CANDIDATES)
            .collect(),
    )
    .unwrap();
    let oversized = (0..=MAX_PRESENTATION_DISCLOSURE_SELECTIONS)
        .map(|index| selection("query", &format!("extra-{index}"), Vec::new()))
        .collect();
    assert_eq!(
        PresentationDisclosurePlan::new(&request, &oversized_candidates, oversized),
        Err(PresentationError::InvalidDisclosureSelections)
    );

    let one_candidate = PresentationCandidateSet::new(
        &request,
        vec![candidate("query", "same", "example", Vec::new())],
    )
    .unwrap();
    let repeated = selection("query", "same", Vec::new());
    assert_eq!(
        PresentationDisclosurePlan::new(&request, &one_candidate, vec![repeated.clone(), repeated],),
        Err(PresentationError::DuplicateDisclosureSelection)
    );
}

#[test]
fn disclosure_plan_rejects_unknown_query_candidate_and_claim_mismatches() {
    let required = path(&["required"]);
    let optional = path(&["optional"]);
    let request = request(vec![unrestricted_query(
        "query",
        "example",
        vec![
            PresentationClaimRequest::new(required.clone(), PresentationClaimIntent::Reveal, true),
            PresentationClaimRequest::new(optional, PresentationClaimIntent::Predicate, false),
        ],
    )]);
    let candidates = PresentationCandidateSet::new(
        &request,
        vec![candidate("query", "available", "example", vec![required])],
    )
    .unwrap();

    assert_eq!(
        PresentationDisclosurePlan::new(
            &request,
            &candidates,
            vec![selection("unknown", "available", Vec::new())],
        ),
        Err(PresentationError::UnknownSelectionQuery)
    );
    assert_eq!(
        PresentationDisclosurePlan::new(
            &request,
            &candidates,
            vec![selection(
                "query",
                "missing",
                vec![selected_claim(
                    &["required"],
                    PresentationClaimIntent::Reveal,
                )],
            )],
        ),
        Err(PresentationError::UnknownSelectionCandidate)
    );
    assert_eq!(
        PresentationDisclosurePlan::new(
            &request,
            &candidates,
            vec![selection(
                "query",
                "available",
                vec![selected_claim(
                    &["unrequested"],
                    PresentationClaimIntent::Reveal,
                )],
            )],
        ),
        Err(PresentationError::SelectionUnrequestedClaim)
    );
    assert_eq!(
        PresentationDisclosurePlan::new(
            &request,
            &candidates,
            vec![selection(
                "query",
                "available",
                vec![selected_claim(
                    &["required"],
                    PresentationClaimIntent::Predicate,
                )],
            )],
        ),
        Err(PresentationError::SelectionClaimIntentMismatch)
    );
    assert_eq!(
        PresentationDisclosurePlan::new(
            &request,
            &candidates,
            vec![selection(
                "query",
                "available",
                vec![
                    selected_claim(&["required"], PresentationClaimIntent::Reveal),
                    selected_claim(&["optional"], PresentationClaimIntent::Predicate),
                ],
            )],
        ),
        Err(PresentationError::SelectionUnavailableClaim)
    );
    assert_eq!(
        PresentationDisclosurePlan::new(
            &request,
            &candidates,
            vec![selection("query", "available", Vec::new())],
        ),
        Err(PresentationError::SelectionMissingRequiredClaim)
    );
}

#[test]
fn disclosure_plan_enforces_query_coverage_and_multiplicity() {
    let request = request(vec![
        unrestricted_query("single", "example", Vec::new()),
        unrestricted_query("second", "example", Vec::new()),
    ]);
    let candidates = PresentationCandidateSet::new(
        &request,
        vec![
            candidate("single", "one", "example", Vec::new()),
            candidate("single", "two", "example", Vec::new()),
            candidate("second", "one", "example", Vec::new()),
        ],
    )
    .unwrap();

    assert_eq!(
        PresentationDisclosurePlan::new(
            &request,
            &candidates,
            vec![selection("single", "one", Vec::new())],
        ),
        Err(PresentationError::MissingQuerySelection)
    );
    assert_eq!(
        PresentationDisclosurePlan::new(
            &request,
            &candidates,
            vec![
                selection("single", "one", Vec::new()),
                selection("single", "two", Vec::new()),
                selection("second", "one", Vec::new()),
            ],
        ),
        Err(PresentationError::QueryMultiplicityExceeded)
    );

    let same_handle_plan = PresentationDisclosurePlan::new(
        &request,
        &candidates,
        vec![
            selection("single", "one", Vec::new()),
            selection("second", "one", Vec::new()),
        ],
    )
    .expect("one handle may satisfy distinct query ids");
    assert_eq!(same_handle_plan.as_slice().len(), 2);
}

#[test]
fn disclosure_plan_revalidates_candidates_against_the_exact_request() {
    let first_request = request(vec![unrestricted_query(
        "query",
        "example",
        vec![claim(&["first"], PresentationClaimIntent::Reveal, true)],
    )]);
    let candidates = PresentationCandidateSet::new(
        &first_request,
        vec![candidate(
            "query",
            "credential",
            "example",
            vec![path(&["first"])],
        )],
    )
    .unwrap();
    let second_request = request(vec![unrestricted_query(
        "query",
        "example",
        vec![claim(&["second"], PresentationClaimIntent::Reveal, true)],
    )]);

    assert_eq!(
        PresentationDisclosurePlan::new(
            &second_request,
            &candidates,
            vec![selection(
                "query",
                "credential",
                vec![selected_claim(&["second"], PresentationClaimIntent::Reveal,)],
            )],
        ),
        Err(PresentationError::CandidateRequestMismatch)
    );

    let filtered_query = PresentationCredentialQuery::new(
        query_id("query"),
        format("example"),
        false,
        false,
        PresentationCredentialFilters::new(
            Some(vec![entity("did:example:restricted-issuer")]),
            None,
            None,
        )
        .unwrap(),
        vec![claim(&["first"], PresentationClaimIntent::Reveal, true)],
    )
    .unwrap();
    let filtered_request = request(vec![filtered_query]);
    assert_eq!(
        PresentationDisclosurePlan::new(
            &filtered_request,
            &candidates,
            vec![selection(
                "query",
                "credential",
                vec![selected_claim(&["first"], PresentationClaimIntent::Reveal,)],
            )],
        ),
        Err(PresentationError::CandidateRequestMismatch)
    );
}

#[test]
fn generated_presentation_unifies_per_credential_and_aggregate_artifacts() {
    let family_name = path(&["family_name"]);
    let birth_date = path(&["credentialSubject", "dateOfBirth"]);
    let request = request(vec![
        unrestricted_query(
            "identity_card",
            "dc+sd-jwt",
            vec![PresentationClaimRequest::new(
                family_name.clone(),
                PresentationClaimIntent::Reveal,
                true,
            )],
        ),
        PresentationCredentialQuery::new(
            query_id("age_proof"),
            format("midnight_cbor_phase1"),
            true,
            true,
            PresentationCredentialFilters::unrestricted(),
            vec![PresentationClaimRequest::new(
                birth_date.clone(),
                PresentationClaimIntent::Predicate,
                true,
            )],
        )
        .unwrap(),
        unrestricted_query("future_format", "example+future", Vec::new()),
    ]);
    let candidates = PresentationCandidateSet::new(
        &request,
        vec![
            candidate(
                "identity_card",
                "credential-1",
                "dc+sd-jwt",
                vec![family_name],
            ),
            candidate(
                "age_proof",
                "credential-2",
                "midnight_cbor_phase1",
                vec![birth_date.clone()],
            ),
            candidate(
                "age_proof",
                "credential-3",
                "midnight_cbor_phase1",
                vec![birth_date],
            ),
            candidate(
                "future_format",
                "credential-4",
                "example+future",
                Vec::new(),
            ),
        ],
    )
    .unwrap();
    let plan = PresentationDisclosurePlan::new(
        &request,
        &candidates,
        vec![
            selection(
                "identity_card",
                "credential-1",
                vec![selected_claim(
                    &["family_name"],
                    PresentationClaimIntent::Reveal,
                )],
            ),
            selection(
                "age_proof",
                "credential-2",
                vec![selected_claim(
                    &["credentialSubject", "dateOfBirth"],
                    PresentationClaimIntent::Predicate,
                )],
            ),
            selection(
                "age_proof",
                "credential-3",
                vec![selected_claim(
                    &["credentialSubject", "dateOfBirth"],
                    PresentationClaimIntent::Predicate,
                )],
            ),
            selection("future_format", "credential-4", Vec::new()),
        ],
    )
    .unwrap();
    let artifacts = vec![
        artifact(
            "example+future",
            vec![binding("future_format", "credential-4")],
            b"dummy-presentation".to_vec(),
        ),
        artifact(
            "midnight_cbor_phase1",
            vec![
                binding("age_proof", "credential-2"),
                binding("age_proof", "credential-3"),
            ],
            vec![0xd8, 0x18, 0x2a],
        ),
        artifact(
            "dc+sd-jwt",
            vec![binding("identity_card", "credential-1")],
            b"eyJhbGciOi...~kb-jwt".to_vec(),
        ),
    ];
    let artifact_vector_address = artifacts.as_ptr();

    let generated = GeneratedPresentation::new(&request, plan, artifacts).unwrap();
    assert_eq!(generated.artifacts().as_ptr(), artifact_vector_address);
    assert_eq!(generated.artifacts()[0].format().as_str(), "example+future");
    assert_eq!(
        generated.artifacts()[1].bindings().len(),
        2,
        "one Midnight artifact may aggregate two selected credentials"
    );
    assert_eq!(generated.artifacts()[2].as_bytes(), b"eyJhbGciOi...~kb-jwt");

    let receipt = generated.receipt_input();
    assert_eq!(receipt.verifier().as_str(), "https://verifier.example");
    assert_eq!(
        receipt.purpose().map(PresentationPurpose::as_str),
        Some("Prove eligibility")
    );
    assert_eq!(receipt.entries().len(), 4);
    assert_eq!(receipt.entries()[0].query_id().as_str(), "identity_card");
    assert_eq!(
        receipt.entries()[0].credential_handle().as_text(),
        Some("credential-1")
    );
    assert_eq!(receipt.entries()[0].format().as_str(), "dc+sd-jwt");
    assert_eq!(receipt.entries()[1].query_id().as_str(), "age_proof");
    assert_eq!(
        receipt.entries()[1].selected_claims()[0].intent(),
        PresentationClaimIntent::Predicate
    );
    assert_eq!(receipt.entries()[3].query_id().as_str(), "future_format");
    assert_eq!(generated.into_artifacts().len(), 3);
}

#[test]
fn artifact_bindings_and_payload_are_bounded_and_zero_copy() {
    assert_eq!(
        PresentationArtifact::new(format("example"), Vec::new(), vec![1]),
        Err(PresentationError::InvalidArtifactBindings)
    );
    let oversized_bindings = (0..=MAX_PRESENTATION_ARTIFACT_BINDINGS)
        .map(|index| binding("query", &format!("credential-{index}")))
        .collect();
    assert_eq!(
        PresentationArtifact::new(format("example"), oversized_bindings, vec![1]),
        Err(PresentationError::InvalidArtifactBindings)
    );
    let repeated = binding("query", "credential");
    assert_eq!(
        PresentationArtifact::new(format("example"), vec![repeated.clone(), repeated], vec![1],),
        Err(PresentationError::DuplicateArtifactBinding)
    );
    assert_eq!(
        PresentationArtifact::new(
            format("example"),
            vec![binding("query", "credential")],
            Vec::new(),
        ),
        Err(PresentationError::InvalidArtifactPayload)
    );

    let maximum = vec![0xa5; MAX_PRESENTATION_ARTIFACT_BYTES];
    let allocation = maximum.as_ptr();
    let artifact = PresentationArtifact::new(
        format("example"),
        vec![binding("query", "credential")],
        maximum,
    )
    .unwrap();
    assert_eq!(artifact.as_bytes().as_ptr(), allocation);
    assert_eq!(artifact.as_bytes().len(), MAX_PRESENTATION_ARTIFACT_BYTES);
    assert_eq!(artifact.into_bytes().len(), MAX_PRESENTATION_ARTIFACT_BYTES);

    assert_eq!(
        PresentationArtifact::new(
            format("example"),
            vec![binding("query", "credential")],
            vec![0; MAX_PRESENTATION_ARTIFACT_BYTES + 1],
        ),
        Err(PresentationError::InvalidArtifactPayload)
    );
}

#[test]
fn generated_presentation_enforces_collection_and_total_byte_bounds() {
    let (request, plan) = multi_selection_fixture(1, "example");
    assert_eq!(
        GeneratedPresentation::new(&request, plan, Vec::new()),
        Err(PresentationError::InvalidGeneratedArtifacts)
    );

    let (request, plan) = multi_selection_fixture(1, "example");
    let repeated = artifact("example", vec![binding("query", "credential-0")], vec![1]);
    assert_eq!(
        GeneratedPresentation::new(
            &request,
            plan,
            vec![repeated; MAX_GENERATED_PRESENTATION_ARTIFACTS + 1],
        ),
        Err(PresentationError::InvalidGeneratedArtifacts)
    );

    let payload_size = 1_024 * 1_024;
    let (request, plan) = multi_selection_fixture(16, "example");
    let artifacts = (0..16)
        .map(|index| {
            artifact(
                "example",
                vec![binding("query", &format!("credential-{index}"))],
                vec![index as u8; payload_size],
            )
        })
        .collect();
    let generated = GeneratedPresentation::new(&request, plan, artifacts).unwrap();
    assert_eq!(
        generated
            .artifacts()
            .iter()
            .map(|artifact| artifact.as_bytes().len())
            .sum::<usize>(),
        MAX_GENERATED_PRESENTATION_BYTES
    );
    drop(generated);

    let (request, plan) = multi_selection_fixture(17, "example");
    let artifacts = (0..17)
        .map(|index| {
            artifact(
                "example",
                vec![binding("query", &format!("credential-{index}"))],
                vec![index as u8; payload_size],
            )
        })
        .collect();
    assert_eq!(
        GeneratedPresentation::new(&request, plan, artifacts),
        Err(PresentationError::ArtifactPayloadBudgetExceeded)
    );
}

#[test]
fn generated_presentation_rejects_request_format_and_binding_mismatches() {
    let (first_request, first_plan) = multi_selection_fixture(1, "example");
    let second_request = PresentationRequest::new(
        entity("https://another-verifier.example"),
        first_request.purpose().cloned(),
        first_request.challenge().cloned(),
        first_request.queries().to_vec(),
    )
    .unwrap();
    assert_eq!(
        GeneratedPresentation::new(
            &second_request,
            first_plan,
            vec![artifact(
                "example",
                vec![binding("query", "credential-0")],
                vec![1],
            )],
        ),
        Err(PresentationError::DisclosureRequestMismatch)
    );

    let (request, plan) = multi_selection_fixture(1, "example");
    assert_eq!(
        GeneratedPresentation::new(
            &request,
            plan,
            vec![artifact(
                "example",
                vec![binding("query", "unknown")],
                vec![1],
            )],
        ),
        Err(PresentationError::UnknownArtifactSelection)
    );

    let (request, plan) = multi_selection_fixture(1, "example");
    assert_eq!(
        GeneratedPresentation::new(
            &request,
            plan,
            vec![artifact(
                "other-format",
                vec![binding("query", "credential-0")],
                vec![1],
            )],
        ),
        Err(PresentationError::ArtifactFormatMismatch)
    );

    let (request, plan) = multi_selection_fixture(1, "example");
    assert_eq!(
        GeneratedPresentation::new(
            &request,
            plan,
            vec![
                artifact("example", vec![binding("query", "credential-0")], vec![1],),
                artifact("example", vec![binding("query", "credential-0")], vec![2],),
            ],
        ),
        Err(PresentationError::DuplicateGeneratedArtifactBinding)
    );

    let (request, plan) = multi_selection_fixture(2, "example");
    assert_eq!(
        GeneratedPresentation::new(
            &request,
            plan,
            vec![artifact(
                "example",
                vec![binding("query", "credential-0")],
                vec![1],
            )],
        ),
        Err(PresentationError::MissingArtifactSelection)
    );
}

#[test]
fn artifact_and_receipt_debug_redact_all_correlating_values() {
    let query_canary = "artifact-query-canary";
    let handle_canary = "artifact-handle-canary";
    let path_canary = "artifact-path-canary";
    let verifier_canary = "did:example:artifact-verifier-canary";
    let purpose_canary = "artifact-purpose-canary";
    let challenge_canary = "artifact-challenge-canary";
    let payload_canary = b"artifact-payload-canary";
    let request = PresentationRequest::new(
        entity(verifier_canary),
        Some(PresentationPurpose::parse(purpose_canary).unwrap()),
        Some(PresentationChallenge::from_text(challenge_canary).unwrap()),
        vec![unrestricted_query(
            query_canary,
            "example-safe-format",
            vec![claim(&[path_canary], PresentationClaimIntent::Reveal, true)],
        )],
    )
    .unwrap();
    let candidates = PresentationCandidateSet::new(
        &request,
        vec![candidate(
            query_canary,
            handle_canary,
            "example-safe-format",
            vec![path(&[path_canary])],
        )],
    )
    .unwrap();
    let plan = PresentationDisclosurePlan::new(
        &request,
        &candidates,
        vec![selection(
            query_canary,
            handle_canary,
            vec![selected_claim(
                &[path_canary],
                PresentationClaimIntent::Reveal,
            )],
        )],
    )
    .unwrap();
    let binding = binding(query_canary, handle_canary);
    let artifact = artifact(
        "example-safe-format",
        vec![binding.clone()],
        payload_canary.to_vec(),
    );
    let generated = GeneratedPresentation::new(&request, plan, vec![artifact.clone()]).unwrap();
    let receipt = generated.receipt_input();
    let renderings = [
        format!("{binding:?}"),
        format!("{artifact:?}"),
        format!("{generated:?}"),
        format!("{:?}", generated.disclosure_plan()),
        format!("{:?}", receipt.entries()[0]),
        format!("{receipt:?}"),
    ];
    for rendered in renderings {
        for canary in [
            query_canary,
            handle_canary,
            path_canary,
            verifier_canary,
            purpose_canary,
            challenge_canary,
            std::str::from_utf8(payload_canary).unwrap(),
        ] {
            assert!(!rendered.contains(canary), "leaked {canary}: {rendered}");
        }
    }
}

#[test]
fn direct_and_aggregate_debug_redact_correlating_values() {
    let query_canary = "query-canary";
    let purpose_canary = "purpose-canary";
    let challenge_canary = "challenge-canary";
    let handle_canary = "handle-canary";
    let verifier_canary = "did:example:verifier-canary";
    let issuer_canary = "did:example:issuer-canary";
    let schema_canary = "schema-canary";
    let path_canary = "claim-canary";

    let filters = PresentationCredentialFilters::new(
        Some(vec![entity(issuer_canary)]),
        None,
        Some(vec![schema(schema_canary)]),
    )
    .unwrap();
    let claim = claim(&[path_canary], PresentationClaimIntent::Reveal, true);
    let query = PresentationCredentialQuery::new(
        query_id(query_canary),
        format("example"),
        false,
        false,
        filters.clone(),
        vec![claim.clone()],
    )
    .unwrap();
    let request = PresentationRequest::new(
        entity(verifier_canary),
        Some(PresentationPurpose::parse(purpose_canary).unwrap()),
        Some(PresentationChallenge::from_text(challenge_canary).unwrap()),
        vec![query.clone()],
    )
    .unwrap();
    let candidate = PresentationCredentialCandidate::new(
        query_id(query_canary),
        PresentationCredentialHandle::from_text(handle_canary).unwrap(),
        format("example"),
        vec![path(&[path_canary])],
    )
    .unwrap();
    let set = PresentationCandidateSet::new(&request, vec![candidate.clone()]).unwrap();
    let selected =
        PresentationSelectedClaim::new(path(&[path_canary]), PresentationClaimIntent::Reveal);
    let selection = PresentationCredentialSelection::new(
        query_id(query_canary),
        PresentationCredentialHandle::from_text(handle_canary).unwrap(),
        vec![selected.clone()],
    )
    .unwrap();
    let plan = PresentationDisclosurePlan::new(&request, &set, vec![selection.clone()]).unwrap();

    let renderings = [
        format!("{:?}", query_id(query_canary)),
        format!("{:?}", PresentationPurpose::parse(purpose_canary).unwrap()),
        format!(
            "{:?}",
            PresentationChallenge::from_text(challenge_canary).unwrap()
        ),
        format!(
            "{:?}",
            PresentationCredentialHandle::from_text(handle_canary).unwrap()
        ),
        format!("{filters:?}"),
        format!("{claim:?}"),
        format!("{query:?}"),
        format!("{request:?}"),
        format!("{candidate:?}"),
        format!("{set:?}"),
        format!("{selected:?}"),
        format!("{selection:?}"),
        format!("{plan:?}"),
    ];
    for rendered in renderings {
        for canary in [
            query_canary,
            purpose_canary,
            challenge_canary,
            handle_canary,
            verifier_canary,
            issuer_canary,
            schema_canary,
            path_canary,
        ] {
            assert!(!rendered.contains(canary), "leaked {canary}: {rendered}");
        }
    }
}

#[test]
fn every_error_has_a_static_presentation_contract() {
    let cases = [
        (
            PresentationError::InvalidQueryId,
            "presentation.invalid_query_id",
        ),
        (
            PresentationError::InvalidPurpose,
            "presentation.invalid_purpose",
        ),
        (
            PresentationError::InvalidChallenge,
            "presentation.invalid_challenge",
        ),
        (
            PresentationError::InvalidCredentialHandle,
            "presentation.invalid_credential_handle",
        ),
        (
            PresentationError::InvalidClaimIntent,
            "presentation.invalid_claim_intent",
        ),
        (
            PresentationError::InvalidQueryFilters,
            "presentation.invalid_query_filters",
        ),
        (
            PresentationError::DuplicateIssuerFilter,
            "presentation.duplicate_issuer_filter",
        ),
        (
            PresentationError::DuplicateTypeFilter,
            "presentation.duplicate_type_filter",
        ),
        (
            PresentationError::DuplicateSchemaFilter,
            "presentation.duplicate_schema_filter",
        ),
        (
            PresentationError::InvalidQueryClaims,
            "presentation.invalid_query_claims",
        ),
        (
            PresentationError::DuplicateQueryClaim,
            "presentation.duplicate_query_claim",
        ),
        (
            PresentationError::InvalidRequestQueries,
            "presentation.invalid_request_queries",
        ),
        (
            PresentationError::DuplicateQueryId,
            "presentation.duplicate_query_id",
        ),
        (
            PresentationError::InvalidCandidateClaims,
            "presentation.invalid_candidate_claims",
        ),
        (
            PresentationError::DuplicateCandidateClaim,
            "presentation.duplicate_candidate_claim",
        ),
        (
            PresentationError::InvalidCandidates,
            "presentation.invalid_candidates",
        ),
        (
            PresentationError::DuplicateCandidate,
            "presentation.duplicate_candidate",
        ),
        (
            PresentationError::UnknownCandidateQuery,
            "presentation.unknown_candidate_query",
        ),
        (
            PresentationError::CandidateFormatMismatch,
            "presentation.candidate_format_mismatch",
        ),
        (
            PresentationError::CandidateUnrequestedClaim,
            "presentation.candidate_unrequested_claim",
        ),
        (
            PresentationError::CandidateMissingRequiredClaim,
            "presentation.candidate_missing_required_claim",
        ),
        (
            PresentationError::CandidateRequestMismatch,
            "presentation.candidate_request_mismatch",
        ),
        (
            PresentationError::InvalidSelectionClaims,
            "presentation.invalid_selection_claims",
        ),
        (
            PresentationError::DuplicateSelectionClaim,
            "presentation.duplicate_selection_claim",
        ),
        (
            PresentationError::InvalidDisclosureSelections,
            "presentation.invalid_disclosure_selections",
        ),
        (
            PresentationError::DuplicateDisclosureSelection,
            "presentation.duplicate_disclosure_selection",
        ),
        (
            PresentationError::UnknownSelectionQuery,
            "presentation.unknown_selection_query",
        ),
        (
            PresentationError::UnknownSelectionCandidate,
            "presentation.unknown_selection_candidate",
        ),
        (
            PresentationError::SelectionUnrequestedClaim,
            "presentation.selection_unrequested_claim",
        ),
        (
            PresentationError::SelectionClaimIntentMismatch,
            "presentation.selection_claim_intent_mismatch",
        ),
        (
            PresentationError::SelectionUnavailableClaim,
            "presentation.selection_unavailable_claim",
        ),
        (
            PresentationError::SelectionMissingRequiredClaim,
            "presentation.selection_missing_required_claim",
        ),
        (
            PresentationError::MissingQuerySelection,
            "presentation.missing_query_selection",
        ),
        (
            PresentationError::QueryMultiplicityExceeded,
            "presentation.query_multiplicity_exceeded",
        ),
        (
            PresentationError::DisclosureRequestMismatch,
            "presentation.disclosure_request_mismatch",
        ),
        (
            PresentationError::InvalidArtifactBindings,
            "presentation.invalid_artifact_bindings",
        ),
        (
            PresentationError::DuplicateArtifactBinding,
            "presentation.duplicate_artifact_binding",
        ),
        (
            PresentationError::InvalidArtifactPayload,
            "presentation.invalid_artifact_payload",
        ),
        (
            PresentationError::InvalidGeneratedArtifacts,
            "presentation.invalid_generated_artifacts",
        ),
        (
            PresentationError::ArtifactPayloadBudgetExceeded,
            "presentation.artifact_payload_budget_exceeded",
        ),
        (
            PresentationError::UnknownArtifactSelection,
            "presentation.unknown_artifact_selection",
        ),
        (
            PresentationError::ArtifactFormatMismatch,
            "presentation.artifact_format_mismatch",
        ),
        (
            PresentationError::DuplicateGeneratedArtifactBinding,
            "presentation.duplicate_generated_artifact_binding",
        ),
        (
            PresentationError::MissingArtifactSelection,
            "presentation.missing_artifact_selection",
        ),
        (
            PresentationError::InvalidLifecyclePhase,
            "presentation.invalid_lifecycle_phase",
        ),
        (
            PresentationError::InvalidTerminalOutcome,
            "presentation.invalid_terminal_outcome",
        ),
        (
            PresentationError::InvalidProtocolState,
            "presentation.invalid_protocol_state",
        ),
        (
            PresentationError::InvalidProtocolTransition,
            "presentation.invalid_protocol_transition",
        ),
    ];

    for (error, expected_code) in cases {
        let bridged = error.to_identus_error();
        assert_eq!(bridged.capability().unwrap().as_str(), "presentation");
        assert_eq!(bridged.kind(), ErrorKind::InvalidInput);
        assert_eq!(bridged.code().as_str(), expected_code);
        assert!(!bridged.public_message().is_empty());
        assert!(!error.to_string().contains("caller-secret"));
    }
}

#[test]
fn deterministic_untrusted_scalar_corpus_never_panics() {
    let mut corpus = vec![
        String::new(),
        " ".into(),
        "\0".into(),
        "\n".into(),
        "é".into(),
        "query".into(),
        "query.with-safe_chars:1".into(),
        "x".repeat(MAX_PRESENTATION_PURPOSE_BYTES + 1),
    ];
    corpus.extend((0_u8..=255).map(|byte| String::from_utf8_lossy(&[byte]).into_owned()));

    for value in corpus {
        let _ = black_box(PresentationQueryId::parse(&value));
        let _ = black_box(PresentationPurpose::parse(&value));
        let _ = black_box(PresentationChallenge::from_text(&value));
        let _ = black_box(PresentationCredentialHandle::from_text(&value));
    }
}

#[test]
#[ignore = "manual release-mode construction diagnostic"]
fn presentation_generation_and_receipt_input_throughput_diagnostic() {
    const ITERATIONS: usize = 250_000;
    let started = Instant::now();

    for index in 0..ITERATIONS {
        let requested = path(&["credentialSubject", "ageOver18"]);
        let query = unrestricted_query(
            "age_proof",
            "midnight_cbor_phase1",
            vec![PresentationClaimRequest::new(
                requested.clone(),
                PresentationClaimIntent::Predicate,
                true,
            )],
        );
        let request = PresentationRequest::new(
            entity("https://verifier.example"),
            None,
            Some(PresentationChallenge::from_bytes(index.to_le_bytes().to_vec()).unwrap()),
            vec![query],
        )
        .unwrap();
        let candidate = candidate(
            "age_proof",
            "credential-1",
            "midnight_cbor_phase1",
            vec![requested.clone()],
        );
        let candidates = PresentationCandidateSet::new(&request, vec![candidate]).unwrap();
        let selection = selection(
            "age_proof",
            "credential-1",
            vec![PresentationSelectedClaim::new(
                requested,
                PresentationClaimIntent::Predicate,
            )],
        );
        let plan = PresentationDisclosurePlan::new(&request, &candidates, vec![selection]).unwrap();
        let artifact = PresentationArtifact::new(
            format("midnight_cbor_phase1"),
            vec![binding("age_proof", "credential-1")],
            vec![0xd8, 0x18, 0x2a],
        )
        .unwrap();
        let generated = GeneratedPresentation::new(&request, plan, vec![artifact]).unwrap();
        let _ = black_box(generated.receipt_input());
    }

    let elapsed = started.elapsed();
    let throughput = ITERATIONS as f64 / elapsed.as_secs_f64();
    eprintln!(
        "validated {ITERATIONS} presentation generation/receipt-input flows in {elapsed:?} ({throughput:.0} flows/s)"
    );
}

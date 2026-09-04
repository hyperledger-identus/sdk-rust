use std::{hint::black_box, str::FromStr, time::Instant};

use identus_core::ErrorKind;
use identus_credentials::{
    CredentialClaimPath, CredentialClaimPathSegment, CredentialEntityId, CredentialFormat,
    CredentialSchemaId, CredentialType,
};
use identus_presentations::{
    MAX_PRESENTATION_CANDIDATE_CLAIMS, MAX_PRESENTATION_CANDIDATES,
    MAX_PRESENTATION_CHALLENGE_BYTES, MAX_PRESENTATION_CREDENTIAL_HANDLE_BYTES,
    MAX_PRESENTATION_FILTER_VALUES, MAX_PRESENTATION_PURPOSE_BYTES, MAX_PRESENTATION_QUERY_CLAIMS,
    MAX_PRESENTATION_QUERY_ID_BYTES, MAX_PRESENTATION_REQUEST_QUERIES, PresentationCandidateSet,
    PresentationChallenge, PresentationClaimIntent, PresentationClaimRequest,
    PresentationCredentialCandidate, PresentationCredentialFilters, PresentationCredentialHandle,
    PresentationCredentialQuery, PresentationError, PresentationPurpose, PresentationQueryId,
    PresentationRequest,
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
fn presentation_request_candidate_throughput_diagnostic() {
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
            vec![requested],
        );
        let _ = black_box(PresentationCandidateSet::new(&request, vec![candidate]).unwrap());
    }

    let elapsed = started.elapsed();
    let throughput = ITERATIONS as f64 / elapsed.as_secs_f64();
    eprintln!(
        "validated {ITERATIONS} presentation request/candidate pairs in {elapsed:?} ({throughput:.0} pairs/s)"
    );
}

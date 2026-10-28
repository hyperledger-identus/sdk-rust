use identus_core::UnixTimestampMillis;
use identus_credentials::{
    CredentialClaimPath, CredentialClaimPathSegment, CredentialEntityId, CredentialFormat,
};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, SecureRandom};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_presentations::{
    PresentationCandidateSet, PresentationClaimIntent, PresentationClaimRequest,
    PresentationCredentialCandidate, PresentationCredentialFilters, PresentationCredentialHandle,
    PresentationCredentialQuery, PresentationCredentialSelection, PresentationDisclosurePlan,
    PresentationQueryId, PresentationRequest, PresentationSelectedClaim,
};
use identus_sd_jwt::{
    ClaimPath, DecoyLocation, IssuancePlan, KeyBindingExpectations, PathSegment, SdJwtError,
    SdJwtLimits, SelectionPlan, UnverifiedSdJwt, issue, map_claim_path, map_disclosure_plan,
    parse_envelope,
};
use serde_json::json;

struct DeterministicEntropy(u8);
impl SecureRandom for DeterministicEntropy {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        for byte in output {
            *byte = self.0;
            self.0 = self.0.wrapping_add(1);
        }
        Ok(())
    }
}

fn generic_path(names: &[&str]) -> CredentialClaimPath {
    CredentialClaimPath::new(
        names
            .iter()
            .map(|s| CredentialClaimPathSegment::parse(s).unwrap())
            .collect(),
    )
    .unwrap()
}

fn disclosure_plan() -> PresentationDisclosurePlan {
    let path = generic_path(&["address", "city"]);
    let query_id = PresentationQueryId::parse("q").unwrap();
    let handle = PresentationCredentialHandle::from_text("local").unwrap();
    let format = CredentialFormat::parse("sd-jwt").unwrap();
    let request = PresentationRequest::new(
        CredentialEntityId::parse("https://verifier.example").unwrap(),
        None,
        None,
        vec![
            PresentationCredentialQuery::new(
                query_id.clone(),
                format.clone(),
                false,
                true,
                PresentationCredentialFilters::unrestricted(),
                vec![PresentationClaimRequest::new(
                    path.clone(),
                    PresentationClaimIntent::Reveal,
                    true,
                )],
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let candidates = PresentationCandidateSet::new(
        &request,
        vec![
            PresentationCredentialCandidate::new(
                query_id.clone(),
                handle.clone(),
                format,
                vec![path.clone()],
            )
            .unwrap(),
        ],
    )
    .unwrap();
    PresentationDisclosurePlan::new(
        &request,
        &candidates,
        vec![
            PresentationCredentialSelection::new(
                query_id,
                handle,
                vec![PresentationSelectedClaim::new(
                    path,
                    PresentationClaimIntent::Reveal,
                )],
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

#[test]
fn issue_store_select_bind_and_verify_offline() {
    let limits = SdJwtLimits::default();
    let issuer_private = Ed25519PrivateKey::from_slice(&[3; 32]).unwrap();
    let holder_private = Ed25519PrivateKey::from_slice(&[4; 32]).unwrap();
    let issuer_public = issuer_private.to_public_key().encode_jwk();
    let holder_public = holder_private.to_public_key().encode_jwk();
    let issuer_key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &issuer_public).unwrap();
    let holder_key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &holder_public).unwrap();
    let suites = SignatureSuiteRegistry::recommended();
    let path = |parts| ClaimPath::new(parts, limits).unwrap();
    let issued = issue(
        json!({"iss":"unique-issuer-123", "address":{"city":"Bratislava","street":"Main"}, "roles":["reader","writer"]}),
        &IssuancePlan {
            disclose: vec![
                path(vec![PathSegment::Member("address".into())]),
                path(vec![PathSegment::Member("address".into()), PathSegment::Member("city".into())]),
                path(vec![PathSegment::Member("roles".into()), PathSegment::Index(1)]),
            ],
            decoy_locations: vec![DecoyLocation::at(path(vec![PathSegment::Member("address".into())]))],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1), &Ed25519Signer::new(&issuer_private), limits,
    ).unwrap();
    assert_eq!(issued.disclosures().len(), 3);
    assert!(!format!("{issued:?} {:?}", issued.disclosures()).contains("Bratislava"));
    let envelope = issued.into_envelope().unwrap();
    let verified = parse_envelope(&envelope, limits)
        .unwrap()
        .verify_issuer(&suites, &issuer_key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert_eq!(verified.claims()["address"]["city"], "Bratislava");
    let plan = map_disclosure_plan(&disclosure_plan(), 0, limits).unwrap();
    let presentation = verified.select(&plan).unwrap();
    assert_eq!(presentation.receipt().disclosure_count, 2);
    let time = UnixTimestampMillis::new(1_700_000_000_000);
    let sd_hash =
        URL_SAFE_NO_PAD.encode(identus_crypto::sha256(presentation.as_str().as_bytes()).as_bytes());
    let kb_limits = JwsLimits::default();
    let kb_header = ProtectedHeader::new("Ed25519", Some("kb+jwt"), None, kb_limits).unwrap();
    let kb_payload = serde_json::to_vec(&json!({
        "aud": "verifier", "nonce": "nonce", "iat": time.whole_seconds(),
        "sd_hash": sd_hash, "iss": "optional-holder-issuer"
    }))
    .unwrap();
    let kb = JwsSigningInput::new(kb_header, kb_payload, kb_limits)
        .unwrap()
        .sign_with(&Ed25519Signer::new(&holder_private))
        .unwrap();
    let with_additional_claim = format!("{}{}", presentation.as_str(), kb.compact());
    UnverifiedSdJwt::parse(&with_additional_claim, limits)
        .unwrap()
        .verify_issuer(&suites, &issuer_key)
        .unwrap()
        .process_disclosures()
        .unwrap()
        .verify_key_binding(
            &suites,
            &holder_key,
            KeyBindingExpectations {
                audience: "verifier",
                nonce: "nonce",
                issued_at: time,
            },
        )
        .unwrap();
    let bound = presentation
        .bind(
            "verifier",
            "nonce",
            time,
            &Ed25519Signer::new(&holder_private),
        )
        .unwrap();
    let mut reordered: Vec<_> = bound.split('~').collect();
    reordered.swap(1, 2);
    let reordered = reordered.join("~");
    let mismatch = UnverifiedSdJwt::parse(&reordered, limits)
        .unwrap()
        .verify_issuer(&suites, &issuer_key)
        .unwrap()
        .process_disclosures()
        .unwrap()
        .verify_key_binding(
            &suites,
            &holder_key,
            KeyBindingExpectations {
                audience: "verifier",
                nonce: "nonce",
                issued_at: time,
            },
        );
    assert!(matches!(mismatch, Err(SdJwtError::KeyBindingHash)));
    let final_value = UnverifiedSdJwt::parse(&bound, limits)
        .unwrap()
        .verify_issuer(&suites, &issuer_key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert_eq!(final_value.claims()["roles"], json!(["reader"]));
    let final_value = final_value
        .verify_key_binding(
            &suites,
            &holder_key,
            KeyBindingExpectations {
                audience: "verifier",
                nonce: "nonce",
                issued_at: time,
            },
        )
        .unwrap();
    let receipt = final_value.receipt();
    assert!(receipt.issuer_signature && receipt.disclosures_processed && receipt.holder_binding);
    assert_eq!(receipt.disclosure_count, 2);
    let debug = format!("{receipt:?} {final_value:?}");
    for secret in [
        "Bratislava",
        "verifier",
        "nonce",
        "unique-issuer-123",
        "Main",
    ] {
        assert!(!debug.contains(secret));
    }
}

#[test]
fn selection_accepts_public_claims_and_includes_only_disclosure_ancestors() {
    let limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[3; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let path = |segments| ClaimPath::new(segments, limits).unwrap();
    let address = path(vec![PathSegment::Member("address".into())]);
    let street = path(vec![
        PathSegment::Member("address".into()),
        PathSegment::Member("street".into()),
    ]);
    let issued = issue(
        json!({"iss":"issuer", "address":{"street":"Main"}}),
        &IssuancePlan {
            disclose: vec![address],
            decoy_locations: vec![DecoyLocation::root()],
            salt_bytes: 16,
        },
        &mut DeterministicEntropy(1),
        &Ed25519Signer::new(&private),
        limits,
    )
    .unwrap();
    let verified = UnverifiedSdJwt::parse(issued.as_str(), limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();

    let public = verified
        .select(&SelectionPlan {
            paths: vec![path(vec![PathSegment::Member("iss".into())])],
        })
        .unwrap();
    assert_eq!(public.receipt().disclosure_count, 0);
    assert!(public.as_str().ends_with('~'));

    let nested = verified
        .select(&SelectionPlan {
            paths: vec![street],
        })
        .unwrap();
    assert_eq!(nested.receipt().disclosure_count, 1);
}

#[test]
fn generic_paths_preserve_hash_prefixed_object_member_names() {
    let limits = SdJwtLimits::default();
    for name in ["#0", "#foo"] {
        let mapped = map_claim_path(&generic_path(&[name]), limits).unwrap();
        assert!(matches!(
            mapped.segments(),
            [PathSegment::Member(member)] if member == name
        ));
    }
}
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

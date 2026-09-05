use std::hint::black_box;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use identus_crypto::{JwkCurve, PublicKeyJwk};
use identus_jose::{
    JoseError, JwsAlgorithm, JwsKeyReference, JwsLimits, JwsSigner, OID4VCI_PROOF_JWT_TYPE,
    Oid4vciProofJwtBuilder, Oid4vciProofJwtClaims, Oid4vciProofJwtClient, Oid4vciProofJwtLimits,
    SignerFailure,
};
use serde_json::{Value, json};

#[derive(Clone)]
struct RecordingSigner {
    algorithm: JwsAlgorithm,
    seen: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl RecordingSigner {
    fn new(algorithm: JwsAlgorithm) -> Self {
        Self {
            algorithm,
            seen: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn calls(&self) -> Vec<Vec<u8>> {
        self.seen.lock().expect("recording lock").clone()
    }
}

impl JwsSigner for RecordingSigner {
    fn algorithm(&self) -> JwsAlgorithm {
        self.algorithm
    }

    fn sign(&self, signing_input: &[u8]) -> Result<[u8; 64], SignerFailure> {
        self.seen
            .lock()
            .expect("recording lock")
            .push(signing_input.to_vec());
        Ok([0x5a; 64])
    }
}

fn identified_claims(limits: Oid4vciProofJwtLimits) -> Oid4vciProofJwtClaims {
    Oid4vciProofJwtClaims::new(
        Oid4vciProofJwtClient::identified("wallet-client", limits).expect("valid client"),
        "https://credential-issuer.example.com",
        1_701_960_444,
        Some("server-nonce".to_owned()),
        limits,
    )
    .expect("valid identified claims")
}

#[test]
fn identified_kid_proof_preserves_exact_signing_input_and_profile() {
    let limits = Oid4vciProofJwtLimits::default();
    let input = Oid4vciProofJwtBuilder::new(limits)
        .prepare(
            JwsAlgorithm::Ed25519,
            JwsKeyReference::KeyId("did:example:holder#authentication-1".to_owned()),
            identified_claims(limits),
        )
        .expect("prepare proof");
    let expected_signing_input = input.as_bytes().to_vec();
    let signer = RecordingSigner::new(JwsAlgorithm::Ed25519);
    let proof = input.sign_with(&signer).expect("sign proof");

    assert_eq!(signer.calls(), vec![expected_signing_input]);
    assert_eq!(
        proof.as_unverified().protected_header().type_(),
        Some(OID4VCI_PROOF_JWT_TYPE)
    );
    assert_eq!(
        proof.as_unverified().protected_header().key_id(),
        Some("did:example:holder#authentication-1")
    );
    assert_eq!(
        serde_json::from_slice::<Value>(proof.as_unverified().payload()).expect("claims JSON"),
        json!({
            "iss": "wallet-client",
            "aud": "https://credential-issuer.example.com",
            "iat": 1_701_960_444,
            "nonce": "server-nonce"
        })
    );
}

#[test]
fn anonymous_pre_authorized_proof_omits_issuer_by_construction() {
    let limits = Oid4vciProofJwtLimits::default();
    let claims = Oid4vciProofJwtClaims::new(
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        "https://issuer.example",
        1_700_000_000,
        Some("fresh".to_owned()),
        limits,
    )
    .expect("anonymous claims");
    let proof = Oid4vciProofJwtBuilder::new(limits)
        .prepare(
            JwsAlgorithm::LegacyEdDsa,
            JwsKeyReference::KeyId("did:midnight:fixture:holder#auth-1".to_owned()),
            claims,
        )
        .expect("Oxid-shaped proof")
        .sign_with(&RecordingSigner::new(JwsAlgorithm::LegacyEdDsa))
        .expect("sign proof");
    let payload = serde_json::from_slice::<Value>(proof.as_unverified().payload())
        .expect("anonymous payload");

    assert_eq!(
        payload,
        json!({
            "aud": "https://issuer.example",
            "iat": 1_700_000_000,
            "nonce": "fresh"
        })
    );
    assert!(payload.get("iss").is_none());
}

#[test]
fn inline_public_jwk_is_bound_to_the_selected_algorithm_before_signing() {
    let limits = Oid4vciProofJwtLimits::default();
    let ed25519 = PublicKeyJwk::new_okp(JwkCurve::Ed25519, [7; 32]).expect("Ed25519 JWK");
    let p256 = PublicKeyJwk::new_ec(JwkCurve::P256, [1; 32], [2; 32]).expect("P-256 JWK");
    let builder = Oid4vciProofJwtBuilder::new(limits);

    let input = builder
        .prepare(
            JwsAlgorithm::Ed25519,
            JwsKeyReference::Jwk(ed25519),
            identified_claims(limits),
        )
        .expect("matching inline JWK");
    assert_eq!(
        input
            .sign_with(&RecordingSigner::new(JwsAlgorithm::Ed25519))
            .expect("signed inline JWK")
            .as_unverified()
            .protected_header()
            .public_jwk()
            .expect("JWK")
            .crv(),
        JwkCurve::Ed25519
    );

    assert!(matches!(
        builder.prepare(
            JwsAlgorithm::Ed25519,
            JwsKeyReference::Jwk(p256),
            identified_claims(limits),
        ),
        Err(JoseError::InvalidVerificationKey)
    ));
}

#[test]
fn x5c_reference_is_bounded_but_not_misrepresented_as_trusted() {
    let limits = Oid4vciProofJwtLimits::default();
    let proof = Oid4vciProofJwtBuilder::new(limits)
        .prepare(
            JwsAlgorithm::Es256,
            JwsKeyReference::X5c(vec!["AQID".to_owned(), "BAUG".to_owned()]),
            identified_claims(limits),
        )
        .expect("bounded x5c")
        .sign_with(&RecordingSigner::new(JwsAlgorithm::Es256))
        .expect("signed x5c proof");

    assert_eq!(
        proof
            .as_unverified()
            .protected_header()
            .certificate_chain()
            .expect("x5c")
            .len(),
        2
    );
    assert!(format!("{proof:?}").starts_with("Oid4vciProofJwt"));
}

#[test]
fn invalid_claims_and_fixed_output_bounds_precede_external_signing() {
    let default_limits = Oid4vciProofJwtLimits::default();
    assert!(matches!(
        Oid4vciProofJwtClient::identified("", default_limits),
        Err(JoseError::InvalidProofClaims)
    ));
    let tiny_claim_limits =
        Oid4vciProofJwtLimits::new(JwsLimits::default(), 4).expect("tiny claim limit");
    assert!(matches!(
        Oid4vciProofJwtClient::identified("borrowed-client-too-large", tiny_claim_limits),
        Err(JoseError::InvalidProofClaims)
    ));
    for invalid in [
        Oid4vciProofJwtClaims::new(
            Oid4vciProofJwtClient::AnonymousPreAuthorized,
            "",
            0,
            None,
            default_limits,
        ),
        Oid4vciProofJwtClaims::new(
            Oid4vciProofJwtClient::AnonymousPreAuthorized,
            "https://issuer.example",
            0,
            Some("bad\nnonce".to_owned()),
            default_limits,
        ),
    ] {
        assert!(matches!(invalid, Err(JoseError::InvalidProofClaims)));
    }

    let builder = Oid4vciProofJwtBuilder::new(default_limits);
    let baseline = builder
        .prepare(
            JwsAlgorithm::Ed25519,
            JwsKeyReference::KeyId("key-1".to_owned()),
            identified_claims(default_limits),
        )
        .expect("baseline");
    let tight_jws = JwsLimits::new(baseline.as_bytes().len() + 3, 4_096, 49_152, 64, 2_048)
        .expect("tight JWS limits");
    let tight_limits = Oid4vciProofJwtLimits::new(tight_jws, 2_048).expect("proof limits");
    let input = Oid4vciProofJwtBuilder::new(tight_limits)
        .prepare(
            JwsAlgorithm::Ed25519,
            JwsKeyReference::KeyId("key-1".to_owned()),
            identified_claims(tight_limits),
        )
        .expect("one-byte minimum fits");
    let signer = RecordingSigner::new(JwsAlgorithm::Ed25519);

    assert!(matches!(
        input.sign_with(&signer),
        Err(JoseError::CompactTooLarge)
    ));
    assert!(signer.calls().is_empty());

    let small_payload_jws =
        JwsLimits::new(65_536, 4_096, 32, 1_024, 2_048).expect("small payload limit");
    let small_payload_limits =
        Oid4vciProofJwtLimits::new(small_payload_jws, 64).expect("proof limits");
    let claims = Oid4vciProofJwtClaims::new(
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        "audience-within-the-claim-bound",
        0,
        Some("nonce-within-the-claim-bound".to_owned()),
        small_payload_limits,
    )
    .expect("individually bounded claims");
    assert!(matches!(
        Oid4vciProofJwtBuilder::new(small_payload_limits).prepare(
            JwsAlgorithm::Ed25519,
            JwsKeyReference::KeyId("key-1".to_owned()),
            claims,
        ),
        Err(JoseError::PayloadTooLarge)
    ));
}

#[test]
fn algorithm_mismatch_and_diagnostics_do_not_leak_proof_values() {
    let canary = "proof-canary-never-render";
    let issued_at_canary = 1_725_689_123_i64;
    let limits = Oid4vciProofJwtLimits::default();
    let claims = Oid4vciProofJwtClaims::new(
        Oid4vciProofJwtClient::identified(canary, limits).expect("bounded client"),
        canary,
        issued_at_canary,
        Some(canary.to_owned()),
        limits,
    )
    .expect("claims");
    let claims_debug = format!("{claims:?}");
    let input = Oid4vciProofJwtBuilder::new(limits)
        .prepare(
            JwsAlgorithm::Ed25519,
            JwsKeyReference::KeyId(canary.to_owned()),
            claims,
        )
        .expect("prepared");
    let input_debug = format!("{input:?}");
    let signer = RecordingSigner::new(JwsAlgorithm::Es256);
    let error = input.sign_with(&signer).expect_err("algorithm mismatch");

    assert_eq!(error, JoseError::AlgorithmMismatch);
    assert!(signer.calls().is_empty());
    for rendered in [claims_debug, input_debug, format!("{error:?} {error}")] {
        assert!(!rendered.contains(canary));
        assert!(!rendered.contains(&issued_at_canary.to_string()));
    }
}

#[test]
#[ignore = "release-only diagnostic; informational, not a correctness threshold"]
fn proof_preparation_throughput_diagnostic() {
    let limits = Oid4vciProofJwtLimits::default();
    let builder = Oid4vciProofJwtBuilder::new(limits);
    let iterations = 100_000_u32;
    let started = Instant::now();
    for _ in 0..iterations {
        let input = builder
            .prepare(
                JwsAlgorithm::Ed25519,
                JwsKeyReference::KeyId("did:example:holder#key-1".to_owned()),
                identified_claims(limits),
            )
            .expect("prepare diagnostic proof");
        black_box(input);
    }
    let elapsed = started.elapsed();
    let operations_per_second = f64::from(iterations) / elapsed.as_secs_f64();
    eprintln!(
        "OID4VCI proof preparation: {iterations} iterations in {elapsed:?} ({operations_per_second:.0} ops/s)"
    );
}

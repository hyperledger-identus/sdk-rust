use identus_crypto::{Ed25519PrivateKey, EncodeJwk, SecureRandom};
use identus_jose::{Ed25519Signer, JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_sd_jwt::{
    ClaimPath, DecoyLocation, IssuancePlan, PathSegment, SdJwtLimits, SelectionPlan,
    UnverifiedSdJwt, issue,
};
use serde_json::{Value, json};

struct SeededEntropy(u64);

impl SecureRandom for SeededEntropy {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        for byte in output {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            *byte = self.0 as u8;
        }
        Ok(())
    }
}

fn path(limits: SdJwtLimits, segments: Vec<PathSegment>) -> ClaimPath {
    ClaimPath::new(segments, limits).unwrap()
}

fn verified(
    seed: u64,
    claims: Value,
    disclose: Vec<ClaimPath>,
) -> identus_sd_jwt::DisclosureVerifiedSdJwt {
    let limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[29; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let issued = issue(
        claims,
        &IssuancePlan {
            disclose,
            decoy_locations: vec![DecoyLocation::root()],
            salt_bytes: 16,
        },
        &mut SeededEntropy(seed.max(1)),
        &Ed25519Signer::new(&private),
        limits,
    )
    .unwrap();
    UnverifiedSdJwt::parse(issued.as_str(), limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap()
}

#[test]
fn deterministic_nested_roundtrip_and_minimal_closure_sweep() {
    let limits = SdJwtLimits::default();
    let address = path(limits, vec![PathSegment::Member("address".to_owned())]);
    let city = path(
        limits,
        vec![
            PathSegment::Member("address".to_owned()),
            PathSegment::Member("city".to_owned()),
        ],
    );
    let role = path(
        limits,
        vec![
            PathSegment::Member("roles".to_owned()),
            PathSegment::Index(1),
        ],
    );

    for seed in 1..=64 {
        let claims = json!({
            "iss": format!("issuer-{seed}"),
            "address": {"city": format!("city-{seed}"), "street": "Main"},
            "roles": ["reader", format!("writer-{seed}")]
        });
        let value = verified(
            seed,
            claims.clone(),
            vec![address.clone(), city.clone(), role.clone()],
        );
        assert_eq!(value.claims(), &claims);

        let first = value
            .select(&SelectionPlan {
                paths: vec![city.clone(), role.clone()],
            })
            .unwrap();
        let reordered = value
            .select(&SelectionPlan {
                paths: vec![role.clone(), city.clone()],
            })
            .unwrap();
        assert_eq!(first.as_str(), reordered.as_str());
        assert_eq!(first.receipt().disclosure_count, 3);

        let public = value
            .select(&SelectionPlan {
                paths: vec![path(limits, vec![PathSegment::Member("iss".to_owned())])],
            })
            .unwrap();
        assert_eq!(public.receipt().disclosure_count, 0);
    }
}

#[test]
fn array_paths_and_exact_salt_boundaries_roundtrip() {
    let limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[31; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let array_path = path(
        limits,
        vec![
            PathSegment::Member("items".to_owned()),
            PathSegment::Index(0),
        ],
    );

    for salt_bytes in [limits.salt_min_bytes, limits.salt_max_bytes] {
        let issued = issue(
            json!({"items":[{"nested":true}]}),
            &IssuancePlan {
                disclose: vec![array_path.clone()],
                decoy_locations: Vec::new(),
                salt_bytes,
            },
            &mut SeededEntropy(0xfeed_face),
            &Ed25519Signer::new(&private),
            limits,
        )
        .unwrap();
        let value = UnverifiedSdJwt::parse(issued.as_str(), limits)
            .unwrap()
            .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
            .unwrap()
            .process_disclosures()
            .unwrap();
        assert_eq!(value.claims(), &json!({"items":[{"nested":true}]}));
        assert_eq!(
            value
                .select(&SelectionPlan {
                    paths: vec![array_path.clone()],
                })
                .unwrap()
                .receipt()
                .disclosure_count,
            1
        );
    }
}

#[test]
fn partial_array_presentations_reindex_visible_disclosures() {
    let limits = SdJwtLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[31; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let item = |index| {
        path(
            limits,
            vec![
                PathSegment::Member("items".to_owned()),
                PathSegment::Index(index),
            ],
        )
    };
    let issued = issue(
        json!({"items":["hidden-first", "visible-second"]}),
        &IssuancePlan {
            disclose: vec![item(0), item(1)],
            decoy_locations: Vec::new(),
            salt_bytes: 16,
        },
        &mut SeededEntropy(0xfeed_face),
        &Ed25519Signer::new(&private),
        limits,
    )
    .unwrap();
    let full = UnverifiedSdJwt::parse(issued.as_str(), limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    let partial = full
        .select(&SelectionPlan {
            paths: vec![item(1)],
        })
        .unwrap();
    let materialized = UnverifiedSdJwt::parse(partial.as_str(), limits)
        .unwrap()
        .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
        .unwrap()
        .process_disclosures()
        .unwrap();
    assert_eq!(materialized.claims(), &json!({"items":["visible-second"]}));
    assert_eq!(
        materialized
            .select(&SelectionPlan {
                paths: vec![item(0)],
            })
            .unwrap()
            .receipt()
            .disclosure_count,
        1
    );
}

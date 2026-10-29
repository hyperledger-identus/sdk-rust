use std::{hint::black_box, time::Instant};

use identus_credentials::{
    CredentialStatusEvidence, CredentialStatusRevision, CredentialStatusValue,
};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, SecureRandom};
use identus_jose::{Ed25519Signer, JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_sd_jwt::{
    IssuanceOptions, IssuancePlan, SdJwtLimits, UnverifiedSdJwt, issue_with_options,
};
use identus_sd_jwt_vc::{
    ClaimDisclosureRule, ClaimMetadata, Draft22SdJwtVc, LegacyValidationInputs,
    LegacyVcSdJwtDraft03Adapter, ProfileValidationInputs, SdJwtVcLimits, StatusListBinding,
    StatusListEvidence, TypeMetadata, TypeMetadataChain, correlate_status_evidence, issue_draft22,
    parse_draft22_envelope,
};
use serde_json::json;

struct Entropy(u8);

impl SecureRandom for Entropy {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        for value in output {
            *value = self.0;
            self.0 = self.0.wrapping_add(1);
        }
        Ok(())
    }
}

fn measure(label: &str, iterations: usize, mut operation: impl FnMut()) {
    for _ in 0..500.min(iterations) {
        operation();
    }
    let started = Instant::now();
    for _ in 0..iterations {
        operation();
    }
    println!(
        "sd-jwt-vc {label} iterations={iterations} elapsed={:?}",
        started.elapsed()
    );
}

fn main() {
    let base_limits = SdJwtLimits::default();
    let profile_limits = SdJwtVcLimits::default();
    let private = Ed25519PrivateKey::from_slice(&[4; 32]).expect("private key");
    let public = private.to_public_key().encode_jwk();
    let verification_key =
        JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).expect("verification key");
    let signer = Ed25519Signer::new(&private);
    let plan = IssuancePlan {
        disclose: vec![],
        decoy_locations: vec![],
        salt_bytes: 16,
    };

    measure("ordinary-roundtrip", 101, || {
        let issued = issue_draft22(
            json!({
                "iss":"https://issuer.example",
                "vct":"urn:example:pid",
                "status":{"status_list":{"idx":7,"uri":"https://status.example/list"}}
            }),
            &plan,
            &mut Entropy(1),
            &signer,
            base_limits,
            profile_limits,
        )
        .expect("issue");
        let envelope = issued.into_envelope().expect("envelope");
        let base = parse_draft22_envelope(&envelope, base_limits)
            .expect("parse")
            .verify_issuer(&SignatureSuiteRegistry::recommended(), &verification_key)
            .expect("issuer signature")
            .process_disclosures()
            .expect("disclosures");
        black_box(Draft22SdJwtVc::validate(
            base,
            ProfileValidationInputs::default(),
            profile_limits,
        ))
        .expect("profile");
    });

    let type_prefix = "urn:example:";
    let boundary_type = format!(
        "{type_prefix}{}",
        "v".repeat(profile_limits.credential_type_bytes - type_prefix.len())
    );
    measure("exact-type-boundary", 101, || {
        black_box(issue_draft22(
            json!({"iss":"https://issuer.example", "vct":boundary_type}),
            &plan,
            &mut Entropy(1),
            &signer,
            base_limits,
            profile_limits,
        ))
        .expect("exact type boundary");
    });

    let oversized_type = format!(
        "{type_prefix}{}",
        "v".repeat(profile_limits.credential_type_bytes + 1 - type_prefix.len())
    );
    measure("oversized-early-rejection", 100_000, || {
        black_box(issue_draft22(
            json!({"iss":"https://issuer.example", "vct":oversized_type}),
            &plan,
            &mut Entropy(1),
            &signer,
            base_limits,
            profile_limits,
        ))
        .expect_err("oversized type");
    });

    let legacy = issue_with_options(
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid"}),
        &plan,
        IssuanceOptions {
            protected_type: Some("vc+sd-jwt"),
        },
        &mut Entropy(1),
        &signer,
        base_limits,
    )
    .expect("legacy fixture");
    measure("legacy-accepted-migration", 101, || {
        let base = UnverifiedSdJwt::parse(legacy.as_str(), base_limits)
            .expect("parse")
            .verify_issuer(&SignatureSuiteRegistry::recommended(), &verification_key)
            .expect("issuer signature")
            .process_disclosures()
            .expect("disclosures");
        black_box(LegacyVcSdJwtDraft03Adapter::validate(
            base,
            LegacyValidationInputs::default(),
            profile_limits,
        ))
        .expect("legacy common subset");
    });

    let canonical = issue_draft22(
        json!({"iss":"https://issuer.example", "vct":"urn:example:pid"}),
        &plan,
        &mut Entropy(1),
        &signer,
        base_limits,
        profile_limits,
    )
    .expect("canonical fixture");
    measure("legacy-wrong-identity-rejection", 101, || {
        let base = UnverifiedSdJwt::parse(canonical.as_str(), base_limits)
            .expect("parse")
            .verify_issuer(&SignatureSuiteRegistry::recommended(), &verification_key)
            .expect("issuer signature")
            .process_disclosures()
            .expect("disclosures");
        black_box(LegacyVcSdJwtDraft03Adapter::validate(
            base,
            LegacyValidationInputs::default(),
            profile_limits,
        ))
        .expect_err("canonical identity");
    });

    let legacy_status = issue_with_options(
        json!({
            "iss":"https://issuer.example",
            "vct":"urn:example:pid",
            "status":{"status_list":{"idx":7,"uri":"https://status.example/list"}}
        }),
        &plan,
        IssuanceOptions {
            protected_type: Some("vc+sd-jwt"),
        },
        &mut Entropy(1),
        &signer,
        base_limits,
    )
    .expect("legacy status fixture");
    measure("legacy-draft-feature-rejection", 101, || {
        let base = UnverifiedSdJwt::parse(legacy_status.as_str(), base_limits)
            .expect("parse")
            .verify_issuer(&SignatureSuiteRegistry::recommended(), &verification_key)
            .expect("issuer signature")
            .process_disclosures()
            .expect("disclosures");
        black_box(LegacyVcSdJwtDraft03Adapter::validate(
            base,
            LegacyValidationInputs::default(),
            profile_limits,
        ))
        .expect_err("draft-specific status");
    });

    let claim = ClaimMetadata::new(
        vec!["family_name".into()],
        true,
        ClaimDisclosureRule::Never,
        profile_limits,
    )
    .expect("claim metadata");
    measure("maximum-metadata-chain", 10_000, || {
        let values = (0..profile_limits.type_extension_depth)
            .map(|index| {
                let vct = format!("urn:example:type-{index}");
                let parent = (index + 1 < profile_limits.type_extension_depth)
                    .then(|| format!("urn:example:type-{}", index + 1));
                TypeMetadata::new(
                    &vct,
                    parent.as_deref(),
                    vec![claim.clone()],
                    64,
                    profile_limits,
                )
                .expect("metadata")
            })
            .collect();
        black_box(TypeMetadataChain::new(values, profile_limits)).expect("metadata chain");
    });

    let binding =
        StatusListBinding::new(7, "https://status.example/list", profile_limits).expect("binding");
    let evidence = StatusListEvidence::jwt(
        CredentialStatusEvidence::new(
            binding.generic().clone(),
            CredentialStatusValue::from_text("active").expect("status"),
            Some(CredentialStatusRevision::from_text("1").expect("revision")),
            None,
            None,
        )
        .expect("status evidence"),
    );
    measure("status-correlation", 100_000, || {
        black_box(correlate_status_evidence(&binding, &evidence)).expect("correlation");
    });
}

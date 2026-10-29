#![no_main]

use identus_credentials::{CredentialEnvelope, CredentialFormat, CredentialPayload};
use identus_crypto::{Ed25519PrivateKey, EncodeJwk, SecureRandom};
use identus_jose::{Ed25519Signer, JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry};
use identus_sd_jwt::{IssuanceOptions, IssuancePlan, SdJwtLimits, issue_with_options};
use identus_sd_jwt_vc::{
    ClaimDisclosureRule, ClaimMetadata, IssuerMetadataEvidence, SdJwtVcLimits,
    LegacyValidationInputs, LegacyVcSdJwtDraft03Adapter, StatusListBinding, TypeMetadata,
    TypeMetadataChain,
};
use libfuzzer_sys::fuzz_target;
use serde_json::Value;

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

fuzz_target!(|data: &[u8]| {
    let limits = SdJwtVcLimits::default();
    let text = std::str::from_utf8(data).unwrap_or_default();
    let split = data.len() / 2;
    let left = std::str::from_utf8(&data[..split]).unwrap_or_default();
    let right = std::str::from_utf8(&data[split..]).unwrap_or_default();
    let encoded_bytes = data.len().saturating_add(1);

    if let (Ok(format), Ok(payload)) = (
        CredentialFormat::parse("vc+sd-jwt"),
        CredentialPayload::new(data.to_vec()),
    ) {
        let envelope = CredentialEnvelope::new(format, payload);
        let _ = LegacyVcSdJwtDraft03Adapter::parse_envelope(
            &envelope,
            SdJwtLimits::default(),
        );
    }

    if let Ok(mut claims) = serde_json::from_slice::<Value>(data)
        && let Some(object) = claims.as_object_mut()
    {
        object
            .entry("iss")
            .or_insert_with(|| Value::String("https://issuer.example".into()));
        object
            .entry("vct")
            .or_insert_with(|| Value::String("urn:example:fuzz".into()));
        if let Ok(private) = Ed25519PrivateKey::from_slice(&[7; 32]) {
            let public = private.to_public_key().encode_jwk();
            if let Ok(key) = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public)
                && let Ok(issued) = issue_with_options(
                    claims,
                    &IssuancePlan {
                        disclose: vec![],
                        decoy_locations: vec![],
                        salt_bytes: 16,
                    },
                    IssuanceOptions {
                        protected_type: Some("vc+sd-jwt"),
                    },
                    &mut Entropy(1),
                    &Ed25519Signer::new(&private),
                    SdJwtLimits::default(),
                )
                && let Ok(verified) = identus_sd_jwt::UnverifiedSdJwt::parse(
                    issued.as_str(),
                    SdJwtLimits::default(),
                )
                .and_then(|value| {
                    value.verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
                })
                .and_then(|value| value.process_disclosures())
            {
                let _ = LegacyVcSdJwtDraft03Adapter::validate(
                    verified,
                    LegacyValidationInputs::default(),
                    limits,
                );
            }
        }
    }

    let _ = StatusListBinding::new(data.len() as u64, text, limits);
    let _ = IssuerMetadataEvidence::jwks_uri(left, right, vec![], encoded_bytes, limits);
    let claim = ClaimMetadata::new(
        vec![left.to_owned(), right.to_owned()],
        data.first().is_some_and(|value| value & 1 == 1),
        match data.first().map(|value| value % 3) {
            Some(0) => ClaimDisclosureRule::Always,
            Some(1) => ClaimDisclosureRule::Allowed,
            _ => ClaimDisclosureRule::Never,
        },
        limits,
    );
    if let Ok(claim) = claim
        && let Ok(metadata) = TypeMetadata::new(text, None, vec![claim], encoded_bytes, limits)
    {
        let _ = TypeMetadataChain::new(vec![metadata], limits);
    }
});

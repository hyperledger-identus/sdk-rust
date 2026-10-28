#![no_main]

use identus_crypto::{Ed25519PrivateKey, EncodeJwk, SecureRandom};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsVerificationKey, SignatureSuiteRegistry,
};
use identus_sd_jwt::{
    ClaimPath, DecoyLocation, IssuancePlan, PathSegment, SdJwtLimits, SelectionPlan,
    UnverifiedSdJwt, issue,
};
use libfuzzer_sys::fuzz_target;
use serde_json::json;

struct Entropy<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl SecureRandom for Entropy<'_> {
    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), identus_crypto::Error> {
        for byte in output {
            *byte = self.bytes.get(self.offset % self.bytes.len()).copied().unwrap_or(0);
            self.offset += 1;
        }
        Ok(())
    }
}

fuzz_target!(|data: &[u8]| {
    let limits = SdJwtLimits::default();
    if let Ok(text) = std::str::from_utf8(data) {
        let _ = UnverifiedSdJwt::parse(text, limits);
    }
    if data.is_empty() || data.len() > 16_384 {
        return;
    }

    let private = Ed25519PrivateKey::from_slice(&[43; 32]).unwrap();
    let public = private.to_public_key().encode_jwk();
    let key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &public).unwrap();
    let path = ClaimPath::new(vec![PathSegment::Member("hidden".to_owned())], limits).unwrap();
    let claims = json!({
        "public": data.len(),
        "hidden": URL_SAFE_NO_PAD.encode(data),
        "nested": {"flag": data[0] & 1 == 1}
    });
    let mut entropy = Entropy { bytes: data, offset: 0 };
    let issued = issue(
        claims.clone(),
        &IssuancePlan {
            disclose: vec![path.clone()],
            decoy_locations: if data[0] & 2 == 2 {
                vec![DecoyLocation::root()]
            } else {
                Vec::new()
            },
            salt_bytes: 16,
        },
        &mut entropy,
        &Ed25519Signer::new(&private),
        limits,
    );
    if let Ok(issued) = issued {
        let verified = UnverifiedSdJwt::parse(issued.as_str(), limits)
            .unwrap()
            .verify_issuer(&SignatureSuiteRegistry::recommended(), &key)
            .unwrap()
            .process_disclosures()
            .unwrap();
        assert_eq!(verified.claims(), &claims);
        let selected = verified
            .select(&SelectionPlan { paths: vec![path] })
            .unwrap();
        assert_eq!(selected.receipt().disclosure_count, 1);
    }
});

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

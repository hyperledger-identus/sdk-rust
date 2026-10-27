#![no_main]

use identus_crypto::{Ed25519PrivateKey, EncodeJwk};
use identus_did::{Did, DidUrl};
use identus_jose::{
    Ed25519Signer, JwsAlgorithm, JwsLimits, JwsSigningInput, JwsVerificationKey, ProtectedHeader,
    SignatureSuiteRegistry,
};
use identus_siopv2::{
    EventId, REQUEST_OBJECT_JWT_TYPE, SiopFlow, Siopv2Limits, UnverifiedSiopRequest,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = std::str::from_utf8(data) {
        let _ = UnverifiedSiopRequest::parse(value, Siopv2Limits::default());
        let _ = EventId::new(value, Siopv2Limits::default().max_event_id_bytes());
    }

    let request_private = Ed25519PrivateKey::from_slice(&[0x31; 32]).unwrap();
    let header = ProtectedHeader::new(
        "Ed25519",
        Some(REQUEST_OBJECT_JWT_TYPE),
        Some("did:example:rp#key-1"),
        JwsLimits::default(),
    )
    .unwrap();
    let Ok(input) = JwsSigningInput::new(header, data.to_vec(), JwsLimits::default()) else {
        return;
    };
    let compact = input
        .sign_with(&Ed25519Signer::new(&request_private))
        .unwrap();
    let request_public = request_private.to_public_key().encode_jwk();
    let request_key = JwsVerificationKey::new(JwsAlgorithm::Ed25519, &request_public).unwrap();
    let Ok(request) = UnverifiedSiopRequest::parse(compact.compact(), Siopv2Limits::default())
        .and_then(|request| {
            request.verify(
                &SignatureSuiteRegistry::recommended(),
                &request_key,
                1_800_000_000,
            )
        })
    else {
        return;
    };

    let subject_private = Ed25519PrivateKey::from_slice(&[0x52; 32]).unwrap();
    let mut flow = SiopFlow::new(request);
    if flow
        .approve(
            EventId::new("approval", 128).unwrap(),
            Did::parse("did:example:alice").unwrap(),
            DidUrl::parse("did:example:alice#auth-1").unwrap(),
            JwsAlgorithm::Ed25519,
            1_800_000_000,
            1_800_000_000,
            1_800_000_120,
            &Ed25519Signer::new(&subject_private),
        )
        .is_ok()
    {
        let _ = flow.delivered(EventId::new("delivery", 128).unwrap());
    }
});

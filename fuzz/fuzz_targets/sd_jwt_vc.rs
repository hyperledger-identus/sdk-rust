#![no_main]

use identus_sd_jwt_vc::{
    ClaimDisclosureRule, ClaimMetadata, IssuerMetadataEvidence, SdJwtVcLimits,
    StatusListBinding, TypeMetadata, TypeMetadataChain,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let limits = SdJwtVcLimits::default();
    let text = std::str::from_utf8(data).unwrap_or_default();
    let split = data.len() / 2;
    let left = std::str::from_utf8(&data[..split]).unwrap_or_default();
    let right = std::str::from_utf8(&data[split..]).unwrap_or_default();
    let encoded_bytes = data.len().saturating_add(1);

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

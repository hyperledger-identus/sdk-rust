use std::fmt;

/// Stable, redacted SD-JWT failure classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SdJwtError {
    InvalidLimits,
    ResourceLimit,
    CompactStructure,
    Encoding,
    Json,
    ReservedClaim,
    UnsupportedHashAlgorithm,
    DisclosureShape,
    DuplicateDisclosure,
    DigestCollision,
    AmbiguousReference,
    UnusedDisclosure,
    InvalidSelection,
    IssuerSignature,
    KeyBindingStructure,
    KeyBindingSignature,
    KeyBindingType,
    KeyBindingHash,
    KeyBindingAudience,
    KeyBindingNonce,
    KeyBindingTime,
    EffectRejected,
    EffectUnavailable,
    InternalInvariant,
}

impl SdJwtError {
    /// Return a stable code independent of private input.
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidLimits => "sd_jwt.invalid_limits",
            Self::ResourceLimit => "sd_jwt.resource_limit",
            Self::CompactStructure => "sd_jwt.invalid_compact",
            Self::Encoding => "sd_jwt.encoding",
            Self::Json => "sd_jwt.json",
            Self::ReservedClaim => "sd_jwt.reserved_claim",
            Self::UnsupportedHashAlgorithm => "sd_jwt.unsupported_hash_algorithm",
            Self::DisclosureShape => "sd_jwt.disclosure_shape",
            Self::DuplicateDisclosure => "sd_jwt.duplicate_disclosure",
            Self::DigestCollision => "sd_jwt.digest_collision",
            Self::AmbiguousReference => "sd_jwt.ambiguous_reference",
            Self::UnusedDisclosure => "sd_jwt.unused_disclosure",
            Self::InvalidSelection => "sd_jwt.invalid_selection",
            Self::IssuerSignature => "sd_jwt.issuer_signature",
            Self::KeyBindingStructure => "sd_jwt.key_binding_structure",
            Self::KeyBindingSignature => "sd_jwt.key_binding_signature",
            Self::KeyBindingType => "sd_jwt.key_binding_type",
            Self::KeyBindingHash => "sd_jwt.key_binding_hash",
            Self::KeyBindingAudience => "sd_jwt.key_binding_audience",
            Self::KeyBindingNonce => "sd_jwt.key_binding_nonce",
            Self::KeyBindingTime => "sd_jwt.key_binding_time",
            Self::EffectRejected => "sd_jwt.effect_rejected",
            Self::EffectUnavailable => "sd_jwt.effect_unavailable",
            Self::InternalInvariant => "sd_jwt.internal_invariant",
        }
    }
}

impl fmt::Display for SdJwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl std::error::Error for SdJwtError {}

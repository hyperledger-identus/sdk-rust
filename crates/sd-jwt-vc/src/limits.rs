use crate::SdJwtVcError;

/// Finite ceilings for draft-22 profile work and retained values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SdJwtVcLimits {
    pub credential_type_bytes: usize,
    pub additional_types: usize,
    pub issuer_identifier_bytes: usize,
    pub issuer_metadata_bytes: usize,
    pub jwk_set_keys: usize,
    pub type_metadata_bytes: usize,
    pub type_extension_depth: usize,
    pub claim_metadata_entries: usize,
    pub claim_path_segments: usize,
    pub status_bytes: usize,
    pub retained_output_bytes: usize,
    pub work_units: usize,
}

impl SdJwtVcLimits {
    pub fn validate(self) -> Result<Self, SdJwtVcError> {
        let values = [
            self.credential_type_bytes,
            self.additional_types,
            self.issuer_identifier_bytes,
            self.issuer_metadata_bytes,
            self.jwk_set_keys,
            self.type_metadata_bytes,
            self.type_extension_depth,
            self.claim_metadata_entries,
            self.claim_path_segments,
            self.status_bytes,
            self.retained_output_bytes,
            self.work_units,
        ];
        if values.contains(&0)
            || self.credential_type_bytes > self.retained_output_bytes
            || self.issuer_metadata_bytes > self.retained_output_bytes
            || self.type_metadata_bytes > self.retained_output_bytes
        {
            return Err(SdJwtVcError::InvalidLimits);
        }
        Ok(self)
    }
}

impl Default for SdJwtVcLimits {
    fn default() -> Self {
        Self {
            credential_type_bytes: 2_048,
            additional_types: 16,
            issuer_identifier_bytes: 2_048,
            issuer_metadata_bytes: 131_072,
            jwk_set_keys: 64,
            type_metadata_bytes: 262_144,
            type_extension_depth: 8,
            claim_metadata_entries: 256,
            claim_path_segments: 32,
            status_bytes: 16_384,
            retained_output_bytes: 524_288,
            work_units: 500_000,
        }
    }
}

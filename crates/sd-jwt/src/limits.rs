use crate::SdJwtError;

/// Public resource limits. Every entry point validates this profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SdJwtLimits {
    pub compact_bytes: usize,
    pub issuer_jws_bytes: usize,
    pub key_binding_jws_bytes: usize,
    pub disclosure_encoded_bytes: usize,
    pub disclosure_decoded_bytes: usize,
    pub disclosures: usize,
    pub decoys: usize,
    pub salt_min_bytes: usize,
    pub salt_max_bytes: usize,
    pub issuer_json_bytes: usize,
    pub json_members: usize,
    pub json_nodes: usize,
    pub json_depth: usize,
    pub selected_paths: usize,
    pub audience_nonce_bytes: usize,
    pub retained_output_bytes: usize,
    pub work_units: usize,
}

impl Default for SdJwtLimits {
    fn default() -> Self {
        Self {
            compact_bytes: 1_048_576,
            issuer_jws_bytes: 524_288,
            key_binding_jws_bytes: 65_536,
            disclosure_encoded_bytes: 131_072,
            disclosure_decoded_bytes: 98_304,
            disclosures: 256,
            decoys: 256,
            salt_min_bytes: 16,
            salt_max_bytes: 64,
            issuer_json_bytes: 524_288,
            json_members: 4_096,
            json_nodes: 16_384,
            json_depth: 32,
            selected_paths: 256,
            audience_nonce_bytes: 1_024,
            retained_output_bytes: 1_048_576,
            work_units: 1_000_000,
        }
    }
}

impl SdJwtLimits {
    /// Reject zero, inverted, and unsafe ceilings before use.
    pub fn validate(self) -> Result<Self, SdJwtError> {
        let values = [
            self.compact_bytes,
            self.issuer_jws_bytes,
            self.key_binding_jws_bytes,
            self.disclosure_encoded_bytes,
            self.disclosure_decoded_bytes,
            self.disclosures,
            self.decoys,
            self.salt_min_bytes,
            self.salt_max_bytes,
            self.issuer_json_bytes,
            self.json_members,
            self.json_nodes,
            self.json_depth,
            self.selected_paths,
            self.audience_nonce_bytes,
            self.retained_output_bytes,
            self.work_units,
        ];
        let ceilings = Self::default();
        let maximums = [
            ceilings.compact_bytes,
            ceilings.issuer_jws_bytes,
            ceilings.key_binding_jws_bytes,
            ceilings.disclosure_encoded_bytes,
            ceilings.disclosure_decoded_bytes,
            ceilings.disclosures,
            ceilings.decoys,
            ceilings.salt_max_bytes,
            ceilings.salt_max_bytes,
            ceilings.issuer_json_bytes,
            ceilings.json_members,
            ceilings.json_nodes,
            ceilings.json_depth,
            ceilings.selected_paths,
            ceilings.audience_nonce_bytes,
            ceilings.retained_output_bytes,
            ceilings.work_units,
        ];
        if values.contains(&0)
            || values
                .iter()
                .zip(maximums)
                .any(|(value, maximum)| *value > maximum)
            || self.salt_min_bytes < 16
            || self.issuer_jws_bytes > self.compact_bytes
            || self.key_binding_jws_bytes > self.compact_bytes
            || self.disclosure_encoded_bytes > self.compact_bytes
            || self.disclosure_decoded_bytes > self.disclosure_encoded_bytes
            || self.salt_min_bytes > self.salt_max_bytes
        {
            return Err(SdJwtError::InvalidLimits);
        }
        Ok(self)
    }
}

/// Checked aggregate charge: decoded bytes, JSON nodes/members, hashes,
/// graph edges, selection comparisons, and emitted bytes each cost units.
pub(crate) struct Work {
    left: usize,
}

impl Work {
    pub(crate) fn new(limits: SdJwtLimits) -> Self {
        Self {
            left: limits.work_units,
        }
    }
    pub(crate) fn charge(&mut self, amount: usize) -> Result<(), SdJwtError> {
        self.left = self
            .left
            .checked_sub(amount)
            .ok_or(SdJwtError::ResourceLimit)?;
        Ok(())
    }
}

//! Validated allocation limits for JWS Compact values.

use crate::JoseError;

/// Default maximum complete compact representation: 64 KiB.
pub const DEFAULT_MAX_COMPACT_BYTES: usize = 65_536;
/// Default maximum decoded protected header: 4 KiB.
pub const DEFAULT_MAX_PROTECTED_HEADER_BYTES: usize = 4_096;
/// Default maximum decoded payload: 48 KiB.
pub const DEFAULT_MAX_PAYLOAD_BYTES: usize = 49_152;
/// Default maximum decoded signature: 1 KiB.
pub const DEFAULT_MAX_SIGNATURE_BYTES: usize = 1_024;
/// Default maximum `typ` or `kid` value: 2 KiB.
pub const DEFAULT_MAX_HEADER_STRING_BYTES: usize = 2_048;

/// Positive allocation limits applied before and after JWS segment decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JwsLimits {
    max_compact_bytes: usize,
    max_protected_header_bytes: usize,
    max_payload_bytes: usize,
    max_signature_bytes: usize,
    max_header_string_bytes: usize,
}

impl JwsLimits {
    /// Construct an explicit positive limit set.
    pub const fn new(
        max_compact_bytes: usize,
        max_protected_header_bytes: usize,
        max_payload_bytes: usize,
        max_signature_bytes: usize,
        max_header_string_bytes: usize,
    ) -> Result<Self, JoseError> {
        if max_compact_bytes == 0
            || max_protected_header_bytes == 0
            || max_payload_bytes == 0
            || max_signature_bytes == 0
            || max_header_string_bytes == 0
        {
            return Err(JoseError::InvalidLimits);
        }
        Ok(Self {
            max_compact_bytes,
            max_protected_header_bytes,
            max_payload_bytes,
            max_signature_bytes,
            max_header_string_bytes,
        })
    }

    /// Maximum bytes in the complete encoded compact representation.
    pub const fn max_compact_bytes(self) -> usize {
        self.max_compact_bytes
    }

    /// Maximum bytes in the decoded protected header.
    pub const fn max_protected_header_bytes(self) -> usize {
        self.max_protected_header_bytes
    }

    /// Maximum bytes in the decoded payload.
    pub const fn max_payload_bytes(self) -> usize {
        self.max_payload_bytes
    }

    /// Maximum bytes in the decoded signature.
    pub const fn max_signature_bytes(self) -> usize {
        self.max_signature_bytes
    }

    /// Maximum bytes in a `typ` or `kid` protected-header string.
    pub const fn max_header_string_bytes(self) -> usize {
        self.max_header_string_bytes
    }
}

impl Default for JwsLimits {
    fn default() -> Self {
        Self {
            max_compact_bytes: DEFAULT_MAX_COMPACT_BYTES,
            max_protected_header_bytes: DEFAULT_MAX_PROTECTED_HEADER_BYTES,
            max_payload_bytes: DEFAULT_MAX_PAYLOAD_BYTES,
            max_signature_bytes: DEFAULT_MAX_SIGNATURE_BYTES,
            max_header_string_bytes: DEFAULT_MAX_HEADER_STRING_BYTES,
        }
    }
}

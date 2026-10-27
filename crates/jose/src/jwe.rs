//! Bounded, engine-neutral compact JWE encryption capability.

use std::fmt;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_core::{ErrorCode, ErrorKind, IdentusError};
use identus_crypto::P256PublicKey;
use serde::Deserialize;
use zeroize::Zeroizing;

use crate::CAPABILITY;

/// Default maximum plaintext: 64 KiB.
pub const DEFAULT_MAX_JWE_PLAINTEXT_BYTES: usize = 65_536;
/// Default maximum compact JWE: 128 KiB.
pub const DEFAULT_MAX_COMPACT_JWE_BYTES: usize = 131_072;
/// Default maximum recipient key identifier: 2 KiB.
pub const DEFAULT_MAX_JWE_KEY_ID_BYTES: usize = 2_048;

/// Resource policy for one compact JWE encryption operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JweLimits {
    max_plaintext_bytes: usize,
    max_compact_bytes: usize,
    max_key_id_bytes: usize,
}

impl JweLimits {
    /// Construct a positive resource policy.
    pub const fn new(
        max_plaintext_bytes: usize,
        max_compact_bytes: usize,
        max_key_id_bytes: usize,
    ) -> Result<Self, JweEncryptionError> {
        if max_plaintext_bytes == 0
            || max_compact_bytes == 0
            || max_key_id_bytes == 0
            || max_key_id_bytes > DEFAULT_MAX_JWE_KEY_ID_BYTES
        {
            return Err(JweEncryptionError::InvalidLimits);
        }
        Ok(Self {
            max_plaintext_bytes,
            max_compact_bytes,
            max_key_id_bytes,
        })
    }

    /// Maximum plaintext bytes accepted by the port.
    pub const fn max_plaintext_bytes(self) -> usize {
        self.max_plaintext_bytes
    }

    /// Maximum bytes in the returned compact JWE.
    pub const fn max_compact_bytes(self) -> usize {
        self.max_compact_bytes
    }

    /// Maximum bytes in an optional recipient key identifier.
    pub const fn max_key_id_bytes(self) -> usize {
        self.max_key_id_bytes
    }
}

impl Default for JweLimits {
    fn default() -> Self {
        Self {
            max_plaintext_bytes: DEFAULT_MAX_JWE_PLAINTEXT_BYTES,
            max_compact_bytes: DEFAULT_MAX_COMPACT_JWE_BYTES,
            max_key_id_bytes: DEFAULT_MAX_JWE_KEY_ID_BYTES,
        }
    }
}

/// Closed key-management algorithm set for the initial encryption port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JweKeyManagementAlgorithm {
    /// Direct key agreement using P-256 ECDH-ES.
    EcdhEs,
}

impl JweKeyManagementAlgorithm {
    /// Exact JOSE algorithm identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EcdhEs => "ECDH-ES",
        }
    }
}

/// Closed content-encryption algorithm set for the initial encryption port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JweContentEncryptionAlgorithm {
    /// AES-128 in Galois/Counter Mode.
    A128Gcm,
}

impl JweContentEncryptionAlgorithm {
    /// Exact JOSE content-encryption identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A128Gcm => "A128GCM",
        }
    }
}

/// Validated public P-256 recipient and optional bounded key identifier.
pub struct JweRecipientPublicKey {
    key: P256PublicKey,
    key_id: Option<Zeroizing<String>>,
}

impl JweRecipientPublicKey {
    /// Validate one pair of P-256 affine coordinates.
    pub fn p256_coordinates(
        x: &[u8; 32],
        y: &[u8; 32],
        key_id: Option<&str>,
        limits: JweLimits,
    ) -> Result<Self, JweEncryptionError> {
        let mut point = [0_u8; 65];
        point[0] = 4;
        point[1..33].copy_from_slice(x);
        point[33..].copy_from_slice(y);
        Self::p256(&point, key_id, limits)
    }

    /// Parse one uncompressed 65-byte P-256 SEC1 public point.
    pub fn p256(
        sec1_uncompressed: &[u8],
        key_id: Option<&str>,
        limits: JweLimits,
    ) -> Result<Self, JweEncryptionError> {
        if sec1_uncompressed.len() != 65 || sec1_uncompressed.first() != Some(&4) {
            return Err(JweEncryptionError::InvalidRecipientKey);
        }
        let key = P256PublicKey::from_slice(sec1_uncompressed)
            .map_err(|_| JweEncryptionError::InvalidRecipientKey)?;
        let key_id = key_id
            .map(|value| {
                if value.is_empty() || value.len() > limits.max_key_id_bytes() {
                    return Err(JweEncryptionError::InvalidKeyId);
                }
                if !value.bytes().all(|byte| matches!(byte, 0x21..=0x7e)) {
                    return Err(JweEncryptionError::InvalidKeyId);
                }
                Ok(Zeroizing::new(value.to_owned()))
            })
            .transpose()?;
        Ok(Self { key, key_id })
    }

    /// Copy the normalized uncompressed SEC1 public point for an adapter.
    pub fn expose_public_sec1(&self) -> [u8; 65] {
        self.key.encode_uncompressed()
    }

    /// Explicitly reveal the optional key identifier to an adapter.
    pub fn expose_key_id(&self) -> Option<&str> {
        self.key_id.as_ref().map(|value| value.as_str())
    }
}

impl fmt::Debug for JweRecipientPublicKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("JweRecipientPublicKey")
            .field("curve", &"P-256")
            .field("has_key_id", &self.key_id.is_some())
            .finish_non_exhaustive()
    }
}

/// One bounded request for an injected compact-JWE implementation.
pub struct JweEncryptionRequest {
    recipient: JweRecipientPublicKey,
    key_management: JweKeyManagementAlgorithm,
    content_encryption: JweContentEncryptionAlgorithm,
    plaintext: Zeroizing<Vec<u8>>,
    max_compact_bytes: usize,
}

impl JweEncryptionRequest {
    /// Construct a request after enforcing independent plaintext/output limits.
    pub fn new(
        recipient: JweRecipientPublicKey,
        key_management: JweKeyManagementAlgorithm,
        content_encryption: JweContentEncryptionAlgorithm,
        plaintext: &[u8],
        limits: JweLimits,
    ) -> Result<Self, JweEncryptionError> {
        if plaintext.is_empty() || plaintext.len() > limits.max_plaintext_bytes() {
            return Err(JweEncryptionError::PlaintextTooLarge);
        }
        if recipient
            .expose_key_id()
            .is_some_and(|value| value.len() > limits.max_key_id_bytes())
        {
            return Err(JweEncryptionError::InvalidKeyId);
        }
        Ok(Self {
            recipient,
            key_management,
            content_encryption,
            plaintext: Zeroizing::new(plaintext.to_vec()),
            max_compact_bytes: limits.max_compact_bytes(),
        })
    }

    /// Borrow the validated public recipient.
    pub const fn recipient(&self) -> &JweRecipientPublicKey {
        &self.recipient
    }

    /// Exact key-management algorithm.
    pub const fn key_management_algorithm(&self) -> JweKeyManagementAlgorithm {
        self.key_management
    }

    /// Exact content-encryption algorithm.
    pub const fn content_encryption_algorithm(&self) -> JweContentEncryptionAlgorithm {
        self.content_encryption
    }

    /// Explicitly reveal plaintext bytes to the selected adapter.
    pub fn expose_sensitive_plaintext(&self) -> &[u8] {
        &self.plaintext
    }

    /// Maximum compact result accepted from the adapter.
    pub const fn max_compact_bytes(&self) -> usize {
        self.max_compact_bytes
    }
}

impl fmt::Debug for JweEncryptionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("JweEncryptionRequest")
            .field("recipient", &self.recipient)
            .field("key_management", &self.key_management)
            .field("content_encryption", &self.content_encryption)
            .field("plaintext_bytes", &self.plaintext.len())
            .field("max_compact_bytes", &self.max_compact_bytes)
            .finish_non_exhaustive()
    }
}

/// Opaque, bounded compact JWE returned by an adapter.
pub struct CompactJwe {
    compact: Zeroizing<String>,
    key_id: Option<Zeroizing<String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DirectAgreementHeader {
    alg: String,
    enc: String,
    epk: EphemeralPublicJwk,
    #[serde(default)]
    kid: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EphemeralPublicJwk {
    kty: String,
    crv: String,
    x: String,
    y: String,
}

impl CompactJwe {
    /// Validate compact serialization grammar for direct ECDH-ES.
    pub fn new(value: String, max_compact_bytes: usize) -> Result<Self, JweEncryptionError> {
        if max_compact_bytes == 0 {
            return Err(JweEncryptionError::InvalidLimits);
        }
        if value.len() > max_compact_bytes {
            return Err(JweEncryptionError::CompactTooLarge);
        }
        let segments: Vec<_> = value.split('.').collect();
        if segments.len() != 5
            || segments[0].is_empty()
            || !segments[1].is_empty()
            || segments[2].is_empty()
            || segments[3].is_empty()
            || segments[4].is_empty()
            || segments.iter().any(|segment| !is_base64url(segment))
        {
            return Err(JweEncryptionError::InvalidCompactStructure);
        }
        let header_bytes = URL_SAFE_NO_PAD
            .decode(segments[0])
            .map_err(|_| JweEncryptionError::InvalidCompactStructure)?;
        if header_bytes.len() > 4_096 {
            return Err(JweEncryptionError::InvalidCompactStructure);
        }
        let header: DirectAgreementHeader = serde_json::from_slice(&header_bytes)
            .map_err(|_| JweEncryptionError::InvalidCompactStructure)?;
        if header.alg != JweKeyManagementAlgorithm::EcdhEs.as_str()
            || header.enc != JweContentEncryptionAlgorithm::A128Gcm.as_str()
        {
            return Err(JweEncryptionError::InvalidCompactStructure);
        }
        validate_ephemeral_key(&header.epk)?;
        if header.kid.as_ref().is_some_and(|kid| {
            kid.is_empty()
                || kid.len() > DEFAULT_MAX_JWE_KEY_ID_BYTES
                || !kid.bytes().all(|byte| matches!(byte, 0x21..=0x7e))
        }) {
            return Err(JweEncryptionError::InvalidCompactStructure);
        }
        let iv = URL_SAFE_NO_PAD
            .decode(segments[2])
            .map_err(|_| JweEncryptionError::InvalidCompactStructure)?;
        let ciphertext = URL_SAFE_NO_PAD
            .decode(segments[3])
            .map_err(|_| JweEncryptionError::InvalidCompactStructure)?;
        let tag = URL_SAFE_NO_PAD
            .decode(segments[4])
            .map_err(|_| JweEncryptionError::InvalidCompactStructure)?;
        if iv.len() != 12 || ciphertext.is_empty() || tag.len() != 16 {
            return Err(JweEncryptionError::InvalidCompactStructure);
        }
        Ok(Self {
            compact: Zeroizing::new(value),
            key_id: header.kid.map(Zeroizing::new),
        })
    }

    /// Explicitly reveal compact wire material for protocol composition.
    pub fn expose_sensitive_compact(&self) -> &str {
        &self.compact
    }

    /// Explicitly reveal the protected recipient key identifier, when present.
    pub fn expose_key_id(&self) -> Option<&str> {
        self.key_id.as_ref().map(|value| value.as_str())
    }

    /// Exact compact byte count.
    pub fn len(&self) -> usize {
        self.compact.len()
    }

    /// Compact JWE is never empty after validation.
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl fmt::Debug for CompactJwe {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CompactJwe")
            .field("compact_bytes", &self.compact.len())
            .field("has_key_id", &self.key_id.is_some())
            .finish_non_exhaustive()
    }
}

fn validate_ephemeral_key(key: &EphemeralPublicJwk) -> Result<(), JweEncryptionError> {
    if key.kty != "EC" || key.crv != "P-256" {
        return Err(JweEncryptionError::InvalidCompactStructure);
    }
    let x: [u8; 32] = URL_SAFE_NO_PAD
        .decode(&key.x)
        .map_err(|_| JweEncryptionError::InvalidCompactStructure)?
        .try_into()
        .map_err(|_| JweEncryptionError::InvalidCompactStructure)?;
    let y: [u8; 32] = URL_SAFE_NO_PAD
        .decode(&key.y)
        .map_err(|_| JweEncryptionError::InvalidCompactStructure)?
        .try_into()
        .map_err(|_| JweEncryptionError::InvalidCompactStructure)?;
    JweRecipientPublicKey::p256_coordinates(&x, &y, None, JweLimits::default())
        .map(|_| ())
        .map_err(|_| JweEncryptionError::InvalidCompactStructure)
}

fn is_base64url(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

/// Synchronous cryptographic capability supplied by an adapter.
pub trait JweEncrypter {
    /// Encrypt one validated request and return bounded compact material.
    fn encrypt(&self, request: JweEncryptionRequest) -> Result<CompactJwe, JweEncryptionError>;
}

/// Static, redaction-safe JWE boundary failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum JweEncryptionError {
    InvalidLimits,
    InvalidRecipientKey,
    InvalidKeyId,
    PlaintextTooLarge,
    CompactTooLarge,
    InvalidCompactStructure,
    EncryptionRejected,
    EncrypterUnavailable,
}

impl JweEncryptionError {
    /// Convert to the shared SDK error contract.
    pub const fn to_identus_error(self) -> IdentusError {
        let (code, kind, message) = match self {
            Self::InvalidLimits => (
                ErrorCode::new("jose.invalid_jwe_limits"),
                ErrorKind::InvalidInput,
                "JWE limits are invalid",
            ),
            Self::InvalidRecipientKey => (
                ErrorCode::new("jose.invalid_jwe_recipient_key"),
                ErrorKind::InvalidInput,
                "JWE recipient key is invalid",
            ),
            Self::InvalidKeyId => (
                ErrorCode::new("jose.invalid_jwe_key_id"),
                ErrorKind::InvalidInput,
                "JWE key identifier is invalid",
            ),
            Self::PlaintextTooLarge => (
                ErrorCode::new("jose.jwe_plaintext_too_large"),
                ErrorKind::InvalidInput,
                "JWE plaintext is outside its resource policy",
            ),
            Self::CompactTooLarge => (
                ErrorCode::new("jose.compact_jwe_too_large"),
                ErrorKind::InvalidInput,
                "JWE compact output is too large",
            ),
            Self::InvalidCompactStructure => (
                ErrorCode::new("jose.invalid_compact_jwe_structure"),
                ErrorKind::InvalidInput,
                "JWE compact structure is invalid",
            ),
            Self::EncryptionRejected => (
                ErrorCode::new("jose.jwe_encryption_rejected"),
                ErrorKind::Crypto,
                "JWE encryption was rejected",
            ),
            Self::EncrypterUnavailable => (
                ErrorCode::new("jose.jwe_encrypter_unavailable"),
                ErrorKind::Crypto,
                "JWE encrypter is unavailable",
            ),
        };
        IdentusError::public(code, kind, CAPABILITY, message)
    }
}

impl fmt::Display for JweEncryptionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.to_identus_error().public_message())
    }
}

impl std::error::Error for JweEncryptionError {}

impl From<JweEncryptionError> for IdentusError {
    fn from(value: JweEncryptionError) -> Self {
        value.to_identus_error()
    }
}

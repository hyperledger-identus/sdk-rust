//! Wallet and agent orchestration boundaries for legacy SDK parity.
//!
//! This crate will own wallet records, storage ports, backup/restore,
//! agent orchestration, credential lifecycle, mediator coordination,
//! and cross-protocol event handling.

use identus_core::{CapabilityId, ErrorCode, ErrorKind, IdentusError, IdentusResult};

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-wallet",
    summary: "Wallet storage ports, backup, and agent orchestration.",
};

/// Stable capability id for wallet storage errors.
pub const STORAGE_CAPABILITY: CapabilityId = CapabilityId::new("storage");

/// Secret classification applied before persistence.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SecretClass {
    /// Public data whose integrity still matters.
    PublicMetadata,
    /// Private wallet metadata that must not appear in public diagnostics.
    PrivateMetadata,
    /// Credential, presentation, or disclosure payload.
    CredentialPayload,
    /// Key material represented by a non-exporting handle.
    KeyMaterial,
    /// Backup seeds, shares, or deterministic recovery material.
    RecoveryMaterial,
}

impl SecretClass {
    /// Stable string used in fixtures and binding DTOs.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicMetadata => "public_metadata",
            Self::PrivateMetadata => "private_metadata",
            Self::CredentialPayload => "credential_payload",
            Self::KeyMaterial => "key_material",
            Self::RecoveryMaterial => "recovery_material",
        }
    }
}

/// Metadata carried by every secure storage record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageRecordMetadata {
    /// Storage namespace.
    pub namespace: String,
    /// Record id inside the namespace.
    pub id: String,
    /// Owning crate or capability.
    pub owner_crate: &'static str,
    /// Secret class used for storage policy.
    pub secret_class: SecretClass,
    /// Monotonic record version for compare-and-swap.
    pub version: u64,
    /// Encryption profile or key id used by the adapter.
    pub encryption_profile: Option<&'static str>,
    /// Migration marker used by backup/restore and wrapper upgrades.
    pub migration_label: Option<&'static str>,
}

/// Secure storage record whose payload is adapter-protected bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageRecord {
    metadata: StorageRecordMetadata,
    payload: Vec<u8>,
}

impl StorageRecord {
    /// Create a version-1 storage record with required metadata.
    #[must_use]
    pub fn new(
        namespace: impl Into<String>,
        id: impl Into<String>,
        owner_crate: &'static str,
        secret_class: SecretClass,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            metadata: StorageRecordMetadata {
                namespace: namespace.into(),
                id: id.into(),
                owner_crate,
                secret_class,
                version: 1,
                encryption_profile: None,
                migration_label: None,
            },
            payload,
        }
    }

    /// Return record metadata.
    #[must_use]
    pub const fn metadata(&self) -> &StorageRecordMetadata {
        &self.metadata
    }

    /// Return protected payload bytes.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Set the expected version for compare-and-swap writes.
    #[must_use]
    pub const fn with_version(mut self, version: u64) -> Self {
        self.metadata.version = version;
        self
    }

    /// Attach an encryption profile or key id.
    #[must_use]
    pub const fn with_encryption_profile(mut self, profile: &'static str) -> Self {
        self.metadata.encryption_profile = Some(profile);
        self
    }

    /// Attach a migration label.
    #[must_use]
    pub const fn with_migration_label(mut self, label: &'static str) -> Self {
        self.metadata.migration_label = Some(label);
        self
    }
}

/// Purpose attached to key handles.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum KeyPurpose {
    /// DID, credential, presentation, or `OpenID4VC` signing key.
    Signing,
    /// `DIDComm` or `OpenID4VC` key agreement key.
    KeyAgreement,
    /// Proof or disclosure binding key.
    Proof,
}

/// Non-exporting key handle used across wallet and binding boundaries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyHandle {
    /// Stable key id.
    pub id: String,
    /// Intended key purpose.
    pub purpose: KeyPurpose,
    /// Whether an adapter policy permits explicit export.
    pub exportable: bool,
}

/// Redaction-safe resolved secret descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedSecret {
    /// Stable key id.
    pub key_id: String,
    /// Intended key purpose.
    pub purpose: KeyPurpose,
}

/// Deterministic signature placeholder returned by storage test adapters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature {
    /// Signing key id.
    pub key_id: String,
    /// Signature bytes.
    pub bytes: Vec<u8>,
}

/// Versioned backup manifest for wallet snapshots.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupManifest {
    /// Manifest version.
    pub version: u16,
    /// Wallet namespace covered by the snapshot.
    pub namespace: String,
    /// Adapter policy that produced the snapshot.
    pub policy: &'static str,
    /// Number of records covered by the snapshot.
    pub record_count: usize,
}

/// Backup snapshot that does not expose raw key material.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupSnapshot {
    /// Snapshot manifest.
    pub manifest: BackupManifest,
    /// Protected records in the snapshot.
    pub records: Vec<StorageRecord>,
    /// Restorable key handles without raw key material.
    pub key_handles: Vec<KeyHandle>,
}

/// Namespaced secure record store.
pub trait SecureStore {
    /// Store or replace a record.
    ///
    /// # Errors
    ///
    /// Returns a typed storage error if the adapter rejects the record.
    fn put(&mut self, record: StorageRecord) -> IdentusResult<()>;

    /// Load a record by namespace and id.
    ///
    /// # Errors
    ///
    /// Returns `record_not_found` when the record is missing.
    fn get(&self, namespace: &str, id: &str) -> IdentusResult<StorageRecord>;

    /// Delete a record by namespace and id.
    ///
    /// # Errors
    ///
    /// Returns `record_not_found` when the record is missing.
    fn delete(&mut self, namespace: &str, id: &str) -> IdentusResult<()>;

    /// List record metadata by namespace.
    ///
    /// # Errors
    ///
    /// Returns a typed storage error if the adapter cannot enumerate records.
    fn list(&self, namespace: &str) -> IdentusResult<Vec<StorageRecordMetadata>>;

    /// Replace a record only when the stored version matches `expected_version`.
    ///
    /// # Errors
    ///
    /// Returns `record_not_found` when the record is missing and
    /// `storage_version_conflict` when the version does not match.
    fn compare_and_swap(
        &mut self,
        expected_version: u64,
        record: StorageRecord,
    ) -> IdentusResult<()>;
}

/// Key store that exposes handles instead of raw private keys.
pub trait KeyStore {
    /// Generate a non-exporting key using an injected entropy source.
    ///
    /// # Errors
    ///
    /// Returns a typed storage or crypto error if key generation fails.
    fn generate_key(
        &mut self,
        key_id: &str,
        purpose: KeyPurpose,
        entropy: &mut dyn EntropySource,
    ) -> IdentusResult<KeyHandle>;

    /// Import deterministic test key material.
    ///
    /// # Errors
    ///
    /// Returns a typed storage error if the key id already exists or the
    /// material is empty.
    fn import_test_key(
        &mut self,
        key_id: &str,
        purpose: KeyPurpose,
        key_material: Vec<u8>,
        exportable: bool,
    ) -> IdentusResult<KeyHandle>;

    /// Sign payload bytes through a key handle.
    ///
    /// # Errors
    ///
    /// Returns `secret_unavailable` when the key handle is not present.
    fn sign(&self, handle: &KeyHandle, payload: &[u8]) -> IdentusResult<Signature>;

    /// Delete a key handle.
    ///
    /// # Errors
    ///
    /// Returns `secret_unavailable` when the key handle is not present.
    fn delete_key(&mut self, key_id: &str) -> IdentusResult<()>;
}

/// Secret resolver that returns descriptors, not raw key bytes.
pub trait SecretResolver {
    /// Resolve a key handle for `DIDComm` or `OpenID4VC` use.
    ///
    /// # Errors
    ///
    /// Returns `secret_unavailable` when the key handle is not present.
    fn resolve_secret(&self, key_id: &str) -> IdentusResult<ResolvedSecret>;
}

/// Versioned backup and restore boundary.
pub trait BackupStore {
    /// Export a namespace snapshot.
    ///
    /// # Errors
    ///
    /// Returns a typed storage error when the snapshot cannot be produced.
    fn export_snapshot(&self, namespace: &str) -> IdentusResult<BackupSnapshot>;

    /// Import a namespace snapshot.
    ///
    /// # Errors
    ///
    /// Returns a typed storage error when the snapshot cannot be restored.
    fn import_snapshot(&mut self, snapshot: BackupSnapshot) -> IdentusResult<()>;
}

/// Injectable entropy source.
pub trait EntropySource {
    /// Fill output bytes with entropy.
    ///
    /// # Errors
    ///
    /// Returns a typed storage error when entropy is unavailable.
    fn fill(&mut self, output: &mut [u8]) -> IdentusResult<()>;
}

/// Create a redaction-safe storage error.
#[must_use]
pub const fn storage_error(
    code: ErrorCode,
    kind: ErrorKind,
    public_message: &'static str,
) -> IdentusError {
    IdentusError::public(code, kind, STORAGE_CAPABILITY, public_message)
}

/// Error returned when a record is missing.
#[must_use]
pub const fn record_not_found() -> IdentusError {
    storage_error(
        ErrorCode::new("record_not_found"),
        ErrorKind::NotFound,
        "storage record was not found",
    )
}

/// Error returned when a compare-and-swap version does not match.
#[must_use]
pub const fn storage_version_conflict() -> IdentusError {
    storage_error(
        ErrorCode::new("storage_version_conflict"),
        ErrorKind::Conflict,
        "storage record version conflict",
    )
}

/// Error returned when a secret handle cannot be resolved.
#[must_use]
pub const fn secret_unavailable() -> IdentusError {
    storage_error(
        ErrorCode::new("secret_unavailable"),
        ErrorKind::Storage,
        "secret handle is unavailable",
    )
}

/// Error returned when a storage input is invalid.
#[must_use]
pub const fn invalid_storage_input() -> IdentusError {
    storage_error(
        ErrorCode::new("invalid_storage_input"),
        ErrorKind::InvalidInput,
        "storage input is invalid",
    )
}

#[cfg(test)]
mod tests {
    use identus_core::CapabilityId;

    use super::{
        ErrorKind, KeyHandle, KeyPurpose, SecretClass, StorageRecord, invalid_storage_input,
        record_not_found, secret_unavailable, storage_version_conflict,
    };

    #[test]
    fn storage_record_metadata_is_policy_explicit() {
        let record = StorageRecord::new(
            "holder",
            "credential-1",
            "identus-wallet",
            SecretClass::CredentialPayload,
            b"ciphertext".to_vec(),
        )
        .with_encryption_profile("in-memory-test")
        .with_migration_label("legacy-sdk-import");

        assert_eq!(record.metadata().namespace, "holder");
        assert_eq!(record.metadata().id, "credential-1");
        assert_eq!(
            record.metadata().secret_class.as_str(),
            "credential_payload"
        );
        assert_eq!(record.metadata().version, 1);
        assert_eq!(record.payload(), b"ciphertext");
    }

    #[test]
    fn storage_errors_are_typed_and_redaction_safe() {
        for (error, code, kind) in [
            (record_not_found(), "record_not_found", ErrorKind::NotFound),
            (
                storage_version_conflict(),
                "storage_version_conflict",
                ErrorKind::Conflict,
            ),
            (
                secret_unavailable(),
                "secret_unavailable",
                ErrorKind::Storage,
            ),
            (
                invalid_storage_input(),
                "invalid_storage_input",
                ErrorKind::InvalidInput,
            ),
        ] {
            assert_eq!(error.code().as_str(), code);
            assert_eq!(error.kind(), kind);
            assert_eq!(
                error.capability().map(CapabilityId::as_str),
                Some("storage")
            );
            assert!(!error.to_string().contains("ciphertext"));
        }
    }

    #[test]
    fn key_handles_do_not_contain_private_material() {
        let handle = KeyHandle {
            id: "didcomm-key-1".to_owned(),
            purpose: KeyPurpose::KeyAgreement,
            exportable: false,
        };

        assert_eq!(handle.id, "didcomm-key-1");
        assert!(!handle.exportable);
    }
}

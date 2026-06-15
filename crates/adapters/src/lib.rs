//! Infrastructure adapter boundaries.
//!
//! This crate will collect optional adapters for `HTTP`, `SQLite`,
//! in-memory storage, mobile secure storage, browser storage,
//! neoprism `VDR` access, `KMS`, Secure Enclave, Android Keystore,
//! `DIDComm` transports, `BLE`, `NFC`, and `QR` handoff surfaces.

use std::collections::BTreeMap;

use identus_core::{ErrorCode, ErrorKind, IdentusError, IdentusResult};
use identus_wallet::{
    BackupManifest, BackupSnapshot, BackupStore, EntropySource, KeyHandle, KeyPurpose, KeyStore,
    ResolvedSecret, SecretResolver, SecureStore, Signature, StorageRecord, StorageRecordMetadata,
    invalid_storage_input, record_not_found, secret_unavailable, storage_error,
    storage_version_conflict,
};

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-adapters",
    summary: "Storage, HTTP, VDR, signer, and transport adapters.",
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct InMemoryKey {
    handle: KeyHandle,
    material: Vec<u8>,
}

/// Deterministic, Docker-free secure storage adapter for acceptance tests.
#[derive(Clone, Debug, Default)]
pub struct InMemorySecureStorageAdapter {
    records: BTreeMap<(String, String), StorageRecord>,
    keys: BTreeMap<String, InMemoryKey>,
}

impl InMemorySecureStorageAdapter {
    /// Create an empty in-memory adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            records: BTreeMap::new(),
            keys: BTreeMap::new(),
        }
    }

    fn record_key(namespace: &str, id: &str) -> (String, String) {
        (namespace.to_owned(), id.to_owned())
    }

    fn duplicate_key_error() -> IdentusError {
        storage_error(
            ErrorCode::new("key_already_exists"),
            ErrorKind::Conflict,
            "key handle already exists",
        )
    }
}

impl SecureStore for InMemorySecureStorageAdapter {
    fn put(&mut self, record: StorageRecord) -> IdentusResult<()> {
        let metadata = record.metadata();
        if metadata.namespace.is_empty() || metadata.id.is_empty() {
            return Err(invalid_storage_input());
        }
        let namespace = metadata.namespace.clone();
        let id = metadata.id.clone();

        self.records
            .insert(Self::record_key(&namespace, &id), record);
        Ok(())
    }

    fn get(&self, namespace: &str, id: &str) -> IdentusResult<StorageRecord> {
        self.records
            .get(&Self::record_key(namespace, id))
            .cloned()
            .ok_or_else(record_not_found)
    }

    fn delete(&mut self, namespace: &str, id: &str) -> IdentusResult<()> {
        self.records
            .remove(&Self::record_key(namespace, id))
            .map(|_| ())
            .ok_or_else(record_not_found)
    }

    fn list(&self, namespace: &str) -> IdentusResult<Vec<StorageRecordMetadata>> {
        let mut metadata = self
            .records
            .values()
            .filter(|record| record.metadata().namespace == namespace)
            .map(|record| record.metadata().clone())
            .collect::<Vec<_>>();
        metadata.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(metadata)
    }

    fn compare_and_swap(
        &mut self,
        expected_version: u64,
        record: StorageRecord,
    ) -> IdentusResult<()> {
        let metadata = record.metadata();
        let key = Self::record_key(&metadata.namespace, &metadata.id);
        let stored = self.records.get(&key).ok_or_else(record_not_found)?;
        if stored.metadata().version != expected_version {
            return Err(storage_version_conflict());
        }

        self.records
            .insert(key, record.with_version(expected_version.saturating_add(1)));
        Ok(())
    }
}

impl KeyStore for InMemorySecureStorageAdapter {
    fn generate_key(
        &mut self,
        key_id: &str,
        purpose: KeyPurpose,
        entropy: &mut dyn EntropySource,
    ) -> IdentusResult<KeyHandle> {
        if key_id.is_empty() || self.keys.contains_key(key_id) {
            return Err(Self::duplicate_key_error());
        }

        let mut material = vec![0_u8; 32];
        entropy.fill(&mut material)?;
        self.import_test_key(key_id, purpose, material, false)
    }

    fn import_test_key(
        &mut self,
        key_id: &str,
        purpose: KeyPurpose,
        key_material: Vec<u8>,
        exportable: bool,
    ) -> IdentusResult<KeyHandle> {
        if key_id.is_empty() || key_material.is_empty() {
            return Err(invalid_storage_input());
        }
        if self.keys.contains_key(key_id) {
            return Err(Self::duplicate_key_error());
        }

        let handle = KeyHandle {
            id: key_id.to_owned(),
            purpose,
            exportable,
        };
        self.keys.insert(
            key_id.to_owned(),
            InMemoryKey {
                handle: handle.clone(),
                material: key_material,
            },
        );
        Ok(handle)
    }

    fn sign(&self, handle: &KeyHandle, payload: &[u8]) -> IdentusResult<Signature> {
        let key = self.keys.get(&handle.id).ok_or_else(secret_unavailable)?;
        if key.material.is_empty() {
            return Err(secret_unavailable());
        }

        let mut accumulator = 0_u8;
        for byte in key.material.iter().chain(payload.iter()) {
            accumulator = accumulator.wrapping_add(*byte).rotate_left(1);
        }

        Ok(Signature {
            key_id: key.handle.id.clone(),
            bytes: vec![accumulator; 64],
        })
    }

    fn delete_key(&mut self, key_id: &str) -> IdentusResult<()> {
        self.keys
            .remove(key_id)
            .map(|_| ())
            .ok_or_else(secret_unavailable)
    }
}

impl SecretResolver for InMemorySecureStorageAdapter {
    fn resolve_secret(&self, key_id: &str) -> IdentusResult<ResolvedSecret> {
        self.keys
            .get(key_id)
            .map(|key| ResolvedSecret {
                key_id: key.handle.id.clone(),
                purpose: key.handle.purpose,
            })
            .ok_or_else(secret_unavailable)
    }
}

impl BackupStore for InMemorySecureStorageAdapter {
    fn export_snapshot(&self, namespace: &str) -> IdentusResult<BackupSnapshot> {
        let mut records = self
            .records
            .values()
            .filter(|record| record.metadata().namespace == namespace)
            .cloned()
            .collect::<Vec<_>>();
        records.sort_by(|left, right| left.metadata().id.cmp(&right.metadata().id));

        let mut key_handles = self
            .keys
            .values()
            .map(|key| key.handle.clone())
            .collect::<Vec<_>>();
        key_handles.sort_by(|left, right| left.id.cmp(&right.id));

        Ok(BackupSnapshot {
            manifest: BackupManifest {
                version: 1,
                namespace: namespace.to_owned(),
                policy: "deterministic-in-memory",
                record_count: records.len(),
            },
            records,
            key_handles,
        })
    }

    fn import_snapshot(&mut self, snapshot: BackupSnapshot) -> IdentusResult<()> {
        if snapshot.manifest.version == 0 || snapshot.manifest.namespace.is_empty() {
            return Err(invalid_storage_input());
        }

        for record in snapshot.records {
            let metadata = record.metadata();
            self.records.insert(
                Self::record_key(&metadata.namespace, &metadata.id),
                record.clone(),
            );
        }

        for handle in snapshot.key_handles {
            self.keys.entry(handle.id.clone()).or_insert(InMemoryKey {
                handle,
                material: Vec::new(),
            });
        }

        Ok(())
    }
}

/// Deterministic entropy source for Docker-free tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeterministicEntropy {
    next: u8,
}

impl DeterministicEntropy {
    /// Create deterministic entropy starting at `seed`.
    #[must_use]
    pub const fn new(seed: u8) -> Self {
        Self { next: seed }
    }
}

impl EntropySource for DeterministicEntropy {
    fn fill(&mut self, output: &mut [u8]) -> IdentusResult<()> {
        for byte in output {
            *byte = self.next;
            self.next = self.next.wrapping_add(1);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use identus_core::{CapabilityId, ErrorKind};
    use identus_wallet::{SecretClass, StorageRecord};

    use super::{
        BackupStore, DeterministicEntropy, InMemorySecureStorageAdapter, KeyPurpose, KeyStore,
        SecretResolver, SecureStore,
    };

    #[test]
    fn in_memory_secure_store_supports_cas_and_listing() {
        let mut store = InMemorySecureStorageAdapter::new();
        let record = StorageRecord::new(
            "holder",
            "credential-1",
            "identus-wallet",
            SecretClass::CredentialPayload,
            b"encrypted".to_vec(),
        );

        store.put(record).expect("record should be stored");
        let stored = store
            .get("holder", "credential-1")
            .expect("record should load");
        assert_eq!(stored.metadata().version, 1);
        assert_eq!(stored.payload(), b"encrypted");
        assert_eq!(store.list("holder").expect("list should work").len(), 1);

        let conflict = store
            .compare_and_swap(
                0,
                StorageRecord::new(
                    "holder",
                    "credential-1",
                    "identus-wallet",
                    SecretClass::CredentialPayload,
                    b"new".to_vec(),
                ),
            )
            .expect_err("stale version should conflict");
        assert_eq!(conflict.code().as_str(), "storage_version_conflict");
        assert_eq!(conflict.kind(), ErrorKind::Conflict);

        store
            .compare_and_swap(
                1,
                StorageRecord::new(
                    "holder",
                    "credential-1",
                    "identus-wallet",
                    SecretClass::CredentialPayload,
                    b"new".to_vec(),
                ),
            )
            .expect("matching version should update");
        assert_eq!(
            store
                .get("holder", "credential-1")
                .expect("updated record should load")
                .metadata()
                .version,
            2
        );
    }

    #[test]
    fn in_memory_key_store_returns_handles_not_key_material() {
        let mut store = InMemorySecureStorageAdapter::new();
        let mut entropy = DeterministicEntropy::new(7);
        let handle = store
            .generate_key("didcomm-key-1", KeyPurpose::KeyAgreement, &mut entropy)
            .expect("key should generate");

        assert_eq!(handle.id, "didcomm-key-1");
        assert!(!handle.exportable);

        let resolved = store
            .resolve_secret("didcomm-key-1")
            .expect("secret should resolve");
        assert_eq!(resolved.key_id, "didcomm-key-1");
        assert_eq!(resolved.purpose, KeyPurpose::KeyAgreement);

        let signature = store
            .sign(&handle, b"payload")
            .expect("test signature should be deterministic");
        assert_eq!(signature.key_id, "didcomm-key-1");
        assert_eq!(signature.bytes.len(), 64);
    }

    #[test]
    fn in_memory_backup_restore_keeps_records_and_redacts_missing_secrets() {
        let mut store = InMemorySecureStorageAdapter::new();
        store
            .put(StorageRecord::new(
                "holder",
                "credential-1",
                "identus-wallet",
                SecretClass::CredentialPayload,
                b"encrypted".to_vec(),
            ))
            .expect("record should store");
        store
            .import_test_key(
                "proof-key-1",
                KeyPurpose::Proof,
                b"test-key".to_vec(),
                false,
            )
            .expect("test key should import");

        let snapshot = store
            .export_snapshot("holder")
            .expect("snapshot should export");
        assert_eq!(snapshot.manifest.record_count, 1);
        assert_eq!(snapshot.key_handles.len(), 1);

        let mut restored = InMemorySecureStorageAdapter::new();
        restored
            .import_snapshot(snapshot)
            .expect("snapshot should restore");
        assert_eq!(
            restored
                .get("holder", "credential-1")
                .expect("restored record should load")
                .payload(),
            b"encrypted"
        );

        let missing = restored
            .sign(
                &restored
                    .resolve_secret("proof-key-1")
                    .map(|secret| identus_wallet::KeyHandle {
                        id: secret.key_id,
                        purpose: secret.purpose,
                        exportable: false,
                    })
                    .expect("handle metadata should restore"),
                b"payload",
            )
            .expect_err("raw key material should not be in backups");
        assert_eq!(missing.code().as_str(), "secret_unavailable");
        assert_eq!(
            missing.capability().map(CapabilityId::as_str),
            Some("storage")
        );
        assert!(!missing.to_string().contains("test-key"));
    }
}

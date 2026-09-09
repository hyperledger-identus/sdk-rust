#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicU64, Ordering};

use aries_askar::{Error as AskarError, ErrorKind as AskarErrorKind, Store, StoreKeyMethod};
use identus_wallet::{
    SecretStore, StorageDeleteCondition, StorageDeleteOutcome, StorageError, StorageFuture,
    StorageRevision, StorageWrite, StorageWriteCondition, StorageWriteOutcome, StorageWriteReceipt,
    Stored,
};

const CATEGORY: &str = "identus-spike.secret.v1";
const MAX_SCOPE_BYTES: usize = 512;
const MAX_KEY_BYTES: usize = 512;
const REVISION_BYTES: usize = size_of::<u64>();
const TEST_KEY_SEED: [u8; 32] = [0x53; 32];

/// Research-only exact-record adapter over an ephemeral encrypted Askar store.
#[derive(Debug)]
pub struct AskarSecretStore {
    store: Store,
    next_revision: AtomicU64,
}

impl AskarSecretStore {
    /// Provision a fresh encrypted in-memory SQLite store.
    pub async fn provision() -> Result<Self, StorageError> {
        let pass_key = Store::new_raw_key(Some(&TEST_KEY_SEED)).map_err(map_askar_error)?;
        let store = Store::provision(
            "sqlite://:memory:",
            StoreKeyMethod::RawKey,
            pass_key,
            None,
            true,
        )
        .await
        .map_err(map_askar_error)?;
        Ok(Self {
            store,
            next_revision: AtomicU64::new(1),
        })
    }

    /// Close the ephemeral candidate store.
    pub async fn close(self) -> Result<(), StorageError> {
        self.store.close().await.map_err(map_askar_error)
    }

    #[cfg(test)]
    async fn inject_malformed(&self, scope: &str, key: &str, value: &[u8]) {
        let name = record_name(scope, key).expect("fixed fixture name is bounded");
        let mut session = self.store.session(None).await.expect("test session opens");
        session
            .insert(CATEGORY, &name, value, None, None)
            .await
            .expect("malformed fixture row inserts");
    }

    fn fresh_revision(&self) -> Result<StorageRevision, StorageError> {
        let revision = self
            .next_revision
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .map_err(|_| StorageError::CapacityExceeded)?;
        StorageRevision::new(revision.to_be_bytes().to_vec())
    }
}

impl SecretStore for AskarSecretStore {
    type Scope = String;
    type Key = String;
    type Value = String;

    fn load<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
    ) -> StorageFuture<'a, Option<Stored<Self::Value>>> {
        Box::pin(async move {
            let name = record_name(scope, key)?;
            let mut session = self.store.session(None).await.map_err(map_askar_error)?;
            let entry = session
                .fetch(CATEGORY, &name, false)
                .await
                .map_err(map_askar_error)?;
            entry
                .map(|entry| decode_record(entry.value.as_ref()))
                .transpose()
        })
    }

    fn write<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        write: StorageWrite<Self::Value>,
    ) -> StorageFuture<'a, StorageWriteReceipt> {
        Box::pin(async move {
            let name = record_name(scope, key)?;
            let (value, condition) = write.into_parts();
            let mut transaction = self
                .store
                .transaction(None)
                .await
                .map_err(map_askar_error)?;
            let current = transaction
                .fetch(CATEGORY, &name, true)
                .await
                .map_err(map_askar_error)?;
            let current_revision = current
                .as_ref()
                .map(|entry| decode_revision(entry.value.as_ref()))
                .transpose()?;

            let allowed = match condition {
                StorageWriteCondition::Any => true,
                StorageWriteCondition::InsertOnly => current.is_none(),
                StorageWriteCondition::IfRevision(expected) => {
                    current_revision.as_ref() == Some(&expected)
                }
                _ => false,
            };
            if !allowed {
                transaction.rollback().await.map_err(map_askar_error)?;
                return Err(StorageError::Conflict);
            }

            let revision = self.fresh_revision()?;
            let encoded = encode_record(&revision, &value);
            let outcome = if current.is_some() {
                transaction
                    .replace(CATEGORY, &name, &encoded, None, None)
                    .await
                    .map_err(map_askar_error)?;
                StorageWriteOutcome::Replaced
            } else {
                transaction
                    .insert(CATEGORY, &name, &encoded, None, None)
                    .await
                    .map_err(map_askar_error)?;
                StorageWriteOutcome::Inserted
            };
            transaction.commit().await.map_err(map_askar_error)?;
            Ok(StorageWriteReceipt::new(outcome, revision))
        })
    }

    fn delete<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        condition: StorageDeleteCondition,
    ) -> StorageFuture<'a, StorageDeleteOutcome> {
        Box::pin(async move {
            let name = record_name(scope, key)?;
            let mut transaction = self
                .store
                .transaction(None)
                .await
                .map_err(map_askar_error)?;
            let current = transaction
                .fetch(CATEGORY, &name, true)
                .await
                .map_err(map_askar_error)?;

            match condition {
                StorageDeleteCondition::Any => {}
                StorageDeleteCondition::IfRevision(expected) => {
                    let actual = current
                        .as_ref()
                        .map(|entry| decode_revision(entry.value.as_ref()))
                        .transpose()?;
                    if actual.as_ref() != Some(&expected) {
                        transaction.rollback().await.map_err(map_askar_error)?;
                        return Err(StorageError::Conflict);
                    }
                }
                _ => {
                    transaction.rollback().await.map_err(map_askar_error)?;
                    return Err(StorageError::Internal);
                }
            }

            let outcome = if current.is_some() {
                transaction
                    .remove(CATEGORY, &name)
                    .await
                    .map_err(map_askar_error)?;
                StorageDeleteOutcome::Deleted
            } else {
                StorageDeleteOutcome::NotFound
            };
            transaction.commit().await.map_err(map_askar_error)?;
            Ok(outcome)
        })
    }
}

fn record_name(scope: &str, key: &str) -> Result<String, StorageError> {
    if scope.len() > MAX_SCOPE_BYTES || key.len() > MAX_KEY_BYTES {
        return Err(StorageError::CapacityExceeded);
    }
    Ok(format!("{}:{scope}{key}", scope.len()))
}

fn encode_record(revision: &StorageRevision, value: &str) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(REVISION_BYTES + value.len());
    encoded.extend_from_slice(revision.as_bytes());
    encoded.extend_from_slice(value.as_bytes());
    encoded
}

fn decode_revision(encoded: &[u8]) -> Result<StorageRevision, StorageError> {
    let bytes = encoded
        .get(..REVISION_BYTES)
        .ok_or(StorageError::Integrity)?;
    StorageRevision::new(bytes.to_vec()).map_err(|_| StorageError::Integrity)
}

fn decode_record(encoded: &[u8]) -> Result<Stored<String>, StorageError> {
    let revision = decode_revision(encoded)?;
    let value = encoded
        .get(REVISION_BYTES..)
        .ok_or(StorageError::Integrity)?;
    let value = String::from_utf8(value.to_vec()).map_err(|_| StorageError::Integrity)?;
    Ok(Stored::new(value, revision))
}

fn map_askar_error(error: AskarError) -> StorageError {
    match error.kind() {
        AskarErrorKind::Duplicate => StorageError::Conflict,
        AskarErrorKind::Busy => StorageError::Unavailable,
        AskarErrorKind::Encryption => StorageError::Integrity,
        AskarErrorKind::Backend
        | AskarErrorKind::Custom
        | AskarErrorKind::Input
        | AskarErrorKind::NotFound
        | AskarErrorKind::Unexpected
        | AskarErrorKind::Unsupported => StorageError::Internal,
    }
}

#[cfg(test)]
mod tests {
    use aries_askar::future::block_on;
    use identus_wallet::{SecretStore as _, StorageError};
    use identus_wallet_conformance::{ExactStoreFixture, check_secret_store};

    use super::AskarSecretStore;

    fn fixture() -> ExactStoreFixture<String, String, String> {
        ExactStoreFixture::new(
            "wallet-primary".to_owned(),
            "wallet-isolated".to_owned(),
            "secret-record".to_owned(),
            "missing-record".to_owned(),
            "initial-secret-canary".to_owned(),
            "replacement-secret-canary".to_owned(),
        )
    }

    #[test]
    fn encrypted_sqlite_adapter_passes_exact_store_conformance() {
        block_on(async {
            let store = AskarSecretStore::provision()
                .await
                .expect("store provisions");
            let report = check_secret_store(&store, &fixture())
                .await
                .expect("exact-store conformance passes");
            assert_eq!(report.completed_operations(), 16);
            assert_eq!(report.observed_entries(), 0);
            store.close().await.expect("store closes");
        });
    }

    #[test]
    fn collision_free_names_preserve_scope_and_key_boundaries() {
        block_on(async {
            let store = AskarSecretStore::provision()
                .await
                .expect("store provisions");
            let first_scope = "a".to_owned();
            let first_key = "bc".to_owned();
            let second_scope = "ab".to_owned();
            let second_key = "c".to_owned();
            store
                .write(
                    &first_scope,
                    &first_key,
                    identus_wallet::StorageWrite::new(
                        "first".to_owned(),
                        identus_wallet::StorageWriteCondition::InsertOnly,
                    ),
                )
                .await
                .expect("first insert");
            assert!(
                store
                    .load(&second_scope, &second_key)
                    .await
                    .expect("load")
                    .is_none()
            );
            store.close().await.expect("store closes");
        });
    }

    #[test]
    fn malformed_rows_and_oversized_names_fail_with_redacted_sdk_errors() {
        block_on(async {
            let store = AskarSecretStore::provision()
                .await
                .expect("store provisions");
            let scope = "integrity-scope".to_owned();
            let key = "integrity-key".to_owned();
            store.inject_malformed(&scope, &key, b"CANARY").await;
            let error = store
                .load(&scope, &key)
                .await
                .expect_err("row is malformed");
            assert_eq!(error, StorageError::Integrity);
            assert!(!error.to_string().contains("CANARY"));

            let oversized = "PRIVATE-CANARY".repeat(40);
            let error = store
                .load(&oversized, &"key".to_owned())
                .await
                .expect_err("oversized scope is rejected");
            assert_eq!(error, StorageError::CapacityExceeded);
            assert!(!error.to_string().contains("PRIVATE-CANARY"));
            store.close().await.expect("store closes");
        });
    }
}

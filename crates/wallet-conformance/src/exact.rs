use core::fmt;

use identus_wallet::{
    StorageDeleteCondition, StorageDeleteOutcome, StorageError, StorageFuture, StorageRevision,
    StorageWrite, StorageWriteCondition, StorageWriteOutcome, Stored,
};

use crate::{ConformanceFailure, ConformanceFailureKind, StorageConformanceReport, failure};

/// Exact-key values used to exercise one consumer storage capability.
pub struct ExactStoreFixture<Scope, Key, Value> {
    pub(super) scope: Scope,
    pub(super) isolated_scope: Scope,
    pub(super) key: Key,
    pub(super) missing_key: Key,
    pub(super) initial_value: Value,
    pub(super) replacement_value: Value,
}

impl<Scope, Key, Value> ExactStoreFixture<Scope, Key, Value> {
    /// Construct a fixture whose scopes and keys are known to be distinct.
    ///
    /// Use a disposable namespace: a failing adapter may leave the primary
    /// record behind because generic cleanup cannot safely override a failed
    /// compare-and-swap contract.
    #[must_use]
    pub const fn new(
        scope: Scope,
        isolated_scope: Scope,
        key: Key,
        missing_key: Key,
        initial_value: Value,
        replacement_value: Value,
    ) -> Self {
        Self {
            scope,
            isolated_scope,
            key,
            missing_key,
            initial_value,
            replacement_value,
        }
    }
}

impl<Scope, Key, Value> fmt::Debug for ExactStoreFixture<Scope, Key, Value> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExactStoreFixture")
            .finish_non_exhaustive()
    }
}

pub(super) trait ExactDriver: Sync {
    type Scope: Send + Sync;
    type Key: Send + Sync;
    type Value: Send + Sync;

    fn load<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
    ) -> StorageFuture<'a, Option<Stored<Self::Value>>>;

    fn write<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        write: StorageWrite<Self::Value>,
    ) -> StorageFuture<'a, identus_wallet::StorageWriteReceipt>;

    fn delete<'a>(
        &'a self,
        scope: &'a Self::Scope,
        key: &'a Self::Key,
        condition: StorageDeleteCondition,
    ) -> StorageFuture<'a, StorageDeleteOutcome>;
}

#[derive(Default)]
struct ExactEvidence {
    completed_operations: usize,
}

impl ExactEvidence {
    fn completed(&mut self) {
        self.completed_operations += 1;
    }

    const fn into_report(self) -> StorageConformanceReport {
        StorageConformanceReport::new(self.completed_operations, 0)
    }
}

pub(super) async fn run<D>(
    driver: &D,
    fixture: &ExactStoreFixture<D::Scope, D::Key, D::Value>,
) -> Result<StorageConformanceReport, ConformanceFailure>
where
    D: ExactDriver + ?Sized,
    D::Value: Clone + PartialEq,
{
    let mut evidence = ExactEvidence::default();
    prove_clean_state(driver, fixture, &mut evidence).await?;
    let first_revision = insert_and_prove_state(driver, fixture, &mut evidence).await?;
    reject_duplicate_insert(driver, fixture, &first_revision, &mut evidence).await?;
    let current_revision =
        replace_and_reject_stale(driver, fixture, first_revision, &mut evidence).await?;
    delete_and_prove_absence(driver, fixture, current_revision, &mut evidence).await?;
    Ok(evidence.into_report())
}

async fn prove_clean_state<D: ExactDriver + ?Sized>(
    driver: &D,
    fixture: &ExactStoreFixture<D::Scope, D::Key, D::Value>,
    evidence: &mut ExactEvidence,
) -> Result<(), ConformanceFailure> {
    let missing = load_optional(
        driver,
        &fixture.scope,
        &fixture.missing_key,
        "missing-load",
        evidence,
    )
    .await?;
    require_absent(
        missing,
        "missing-load",
        ConformanceFailureKind::UnexpectedRecordPresence,
    )?;

    let initial =
        load_optional(driver, &fixture.scope, &fixture.key, "clean-load", evidence).await?;
    require_absent(
        initial,
        "clean-load",
        ConformanceFailureKind::UnexpectedRecordPresence,
    )?;

    let isolated = load_optional(
        driver,
        &fixture.isolated_scope,
        &fixture.key,
        "isolated-clean-load",
        evidence,
    )
    .await?;
    require_absent(
        isolated,
        "isolated-clean-load",
        ConformanceFailureKind::ScopeIsolationViolation,
    )
}

async fn insert_and_prove_state<D>(
    driver: &D,
    fixture: &ExactStoreFixture<D::Scope, D::Key, D::Value>,
    evidence: &mut ExactEvidence,
) -> Result<StorageRevision, ConformanceFailure>
where
    D: ExactDriver + ?Sized,
    D::Value: Clone + PartialEq,
{
    let inserted = driver
        .write(
            &fixture.scope,
            &fixture.key,
            StorageWrite::new(
                fixture.initial_value.clone(),
                StorageWriteCondition::InsertOnly,
            ),
        )
        .await
        .map_err(|_| failure("insert-only", ConformanceFailureKind::OperationFailed))?;
    evidence.completed();
    if inserted.outcome() != StorageWriteOutcome::Inserted {
        return Err(failure(
            "insert-only",
            ConformanceFailureKind::WriteOutcomeMismatch,
        ));
    }
    let first_revision = inserted.revision().clone();

    let loaded = required_load(
        driver,
        &fixture.scope,
        &fixture.key,
        "read-after-insert",
        evidence,
    )
    .await?;
    require_value_and_revision(
        &loaded,
        &fixture.initial_value,
        &first_revision,
        "read-after-insert",
        ConformanceFailureKind::RevisionMismatch,
    )?;

    let isolated = load_optional(
        driver,
        &fixture.isolated_scope,
        &fixture.key,
        "scope-isolation",
        evidence,
    )
    .await?;
    require_absent(
        isolated,
        "scope-isolation",
        ConformanceFailureKind::ScopeIsolationViolation,
    )?;
    Ok(first_revision)
}

async fn reject_duplicate_insert<D>(
    driver: &D,
    fixture: &ExactStoreFixture<D::Scope, D::Key, D::Value>,
    first_revision: &StorageRevision,
    evidence: &mut ExactEvidence,
) -> Result<(), ConformanceFailure>
where
    D: ExactDriver + ?Sized,
    D::Value: Clone + PartialEq,
{
    expect_write_conflict(
        driver,
        &fixture.scope,
        &fixture.key,
        StorageWrite::new(
            fixture.replacement_value.clone(),
            StorageWriteCondition::InsertOnly,
        ),
        "duplicate-insert",
    )
    .await?;
    evidence.completed();

    let preserved = required_load(
        driver,
        &fixture.scope,
        &fixture.key,
        "insert-conflict-preserves",
        evidence,
    )
    .await?;
    require_value_and_revision(
        &preserved,
        &fixture.initial_value,
        first_revision,
        "insert-conflict-preserves",
        ConformanceFailureKind::ValueMismatch,
    )
}

async fn replace_and_reject_stale<D>(
    driver: &D,
    fixture: &ExactStoreFixture<D::Scope, D::Key, D::Value>,
    first_revision: StorageRevision,
    evidence: &mut ExactEvidence,
) -> Result<StorageRevision, ConformanceFailure>
where
    D: ExactDriver + ?Sized,
    D::Value: Clone + PartialEq,
{
    let replaced = driver
        .write(
            &fixture.scope,
            &fixture.key,
            StorageWrite::new(
                fixture.replacement_value.clone(),
                StorageWriteCondition::Any,
            ),
        )
        .await
        .map_err(|_| failure("replace", ConformanceFailureKind::OperationFailed))?;
    evidence.completed();
    if replaced.outcome() != StorageWriteOutcome::Replaced {
        return Err(failure(
            "replace",
            ConformanceFailureKind::WriteOutcomeMismatch,
        ));
    }
    let current_revision = replaced.revision().clone();
    if current_revision == first_revision {
        return Err(failure(
            "replace",
            ConformanceFailureKind::RevisionNotInvalidated,
        ));
    }

    expect_write_conflict(
        driver,
        &fixture.scope,
        &fixture.key,
        StorageWrite::new(
            fixture.initial_value.clone(),
            StorageWriteCondition::IfRevision(first_revision.clone()),
        ),
        "stale-write",
    )
    .await?;
    evidence.completed();
    prove_replacement_preserved(
        driver,
        fixture,
        &current_revision,
        "stale-write-preserves",
        evidence,
    )
    .await?;

    expect_delete_conflict(
        driver,
        &fixture.scope,
        &fixture.key,
        StorageDeleteCondition::IfRevision(first_revision),
        "stale-delete",
    )
    .await?;
    evidence.completed();
    prove_replacement_preserved(
        driver,
        fixture,
        &current_revision,
        "stale-delete-preserves",
        evidence,
    )
    .await?;
    Ok(current_revision)
}

async fn prove_replacement_preserved<D>(
    driver: &D,
    fixture: &ExactStoreFixture<D::Scope, D::Key, D::Value>,
    current_revision: &StorageRevision,
    step: &'static str,
    evidence: &mut ExactEvidence,
) -> Result<(), ConformanceFailure>
where
    D: ExactDriver + ?Sized,
    D::Value: PartialEq,
{
    let current = required_load(driver, &fixture.scope, &fixture.key, step, evidence).await?;
    require_value_and_revision(
        &current,
        &fixture.replacement_value,
        current_revision,
        step,
        ConformanceFailureKind::ValueMismatch,
    )
}

async fn delete_and_prove_absence<D: ExactDriver + ?Sized>(
    driver: &D,
    fixture: &ExactStoreFixture<D::Scope, D::Key, D::Value>,
    current_revision: StorageRevision,
    evidence: &mut ExactEvidence,
) -> Result<(), ConformanceFailure> {
    let deleted = driver
        .delete(
            &fixture.scope,
            &fixture.key,
            StorageDeleteCondition::IfRevision(current_revision),
        )
        .await
        .map_err(|_| failure("current-delete", ConformanceFailureKind::OperationFailed))?;
    evidence.completed();
    if deleted != StorageDeleteOutcome::Deleted {
        return Err(failure(
            "current-delete",
            ConformanceFailureKind::DeleteOutcomeMismatch,
        ));
    }

    let absent = load_optional(
        driver,
        &fixture.scope,
        &fixture.key,
        "read-after-delete",
        evidence,
    )
    .await?;
    require_absent(
        absent,
        "read-after-delete",
        ConformanceFailureKind::UnexpectedRecordPresence,
    )?;

    let missing_delete = driver
        .delete(&fixture.scope, &fixture.key, StorageDeleteCondition::Any)
        .await
        .map_err(|_| failure("missing-delete", ConformanceFailureKind::OperationFailed))?;
    evidence.completed();
    if missing_delete != StorageDeleteOutcome::NotFound {
        return Err(failure(
            "missing-delete",
            ConformanceFailureKind::DeleteOutcomeMismatch,
        ));
    }
    Ok(())
}

async fn load_optional<'a, D: ExactDriver + ?Sized>(
    driver: &'a D,
    scope: &'a D::Scope,
    key: &'a D::Key,
    step: &'static str,
    evidence: &mut ExactEvidence,
) -> Result<Option<Stored<D::Value>>, ConformanceFailure> {
    let loaded = driver
        .load(scope, key)
        .await
        .map_err(|_| failure(step, ConformanceFailureKind::OperationFailed))?;
    evidence.completed();
    Ok(loaded)
}

async fn required_load<'a, D: ExactDriver + ?Sized>(
    driver: &'a D,
    scope: &'a D::Scope,
    key: &'a D::Key,
    step: &'static str,
    evidence: &mut ExactEvidence,
) -> Result<Stored<D::Value>, ConformanceFailure> {
    load_optional(driver, scope, key, step, evidence)
        .await?
        .ok_or_else(|| failure(step, ConformanceFailureKind::UnexpectedRecordPresence))
}

fn require_absent<Value>(
    loaded: Option<Stored<Value>>,
    step: &'static str,
    kind: ConformanceFailureKind,
) -> Result<(), ConformanceFailure> {
    if loaded.is_some() {
        return Err(failure(step, kind));
    }
    Ok(())
}

fn require_value_and_revision<Value: PartialEq>(
    loaded: &Stored<Value>,
    expected_value: &Value,
    expected_revision: &StorageRevision,
    step: &'static str,
    revision_kind: ConformanceFailureKind,
) -> Result<(), ConformanceFailure> {
    if loaded.value() != expected_value {
        return Err(failure(step, ConformanceFailureKind::ValueMismatch));
    }
    if loaded.revision() != expected_revision {
        return Err(failure(step, revision_kind));
    }
    Ok(())
}

async fn expect_write_conflict<D: ExactDriver + ?Sized>(
    driver: &D,
    scope: &D::Scope,
    key: &D::Key,
    write: StorageWrite<D::Value>,
    step: &'static str,
) -> Result<(), ConformanceFailure> {
    match driver.write(scope, key, write).await {
        Err(StorageError::Conflict) => Ok(()),
        Err(_) => Err(failure(
            step,
            ConformanceFailureKind::UnexpectedStorageError,
        )),
        Ok(_) => Err(failure(step, ConformanceFailureKind::ExpectedConflict)),
    }
}

async fn expect_delete_conflict<D: ExactDriver + ?Sized>(
    driver: &D,
    scope: &D::Scope,
    key: &D::Key,
    condition: StorageDeleteCondition,
    step: &'static str,
) -> Result<(), ConformanceFailure> {
    match driver.delete(scope, key, condition).await {
        Err(StorageError::Conflict) => Ok(()),
        Err(_) => Err(failure(
            step,
            ConformanceFailureKind::UnexpectedStorageError,
        )),
        Ok(_) => Err(failure(step, ConformanceFailureKind::ExpectedConflict)),
    }
}

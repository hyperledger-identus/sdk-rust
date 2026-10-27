//! Pure complete-batch planning over caller-provided immutable snapshots.

use std::{collections::HashMap, fmt};

use identus_crypto::Sha256Digest;
use identus_prism_did::{
    CanonicalPrismDid, DeactivateDidOperation, OperationKind, PrismDidState, RawPrismOperation,
    UpdateDidOperation,
};
use identus_prism_vdr::{
    EntrySnapshot, LookupError, TraversalBudget, VdrLookup, VdrNode, VdrOperation, resolve_entry,
};

use crate::{Error, ErrorKind, TransactionBatch};

/// Read-only committed DID state port used during pure planning.
pub trait DidStateLookup {
    /// Load the committed materialized state for one canonical DID.
    fn did_state(&self, did: &CanonicalPrismDid) -> Result<Option<PrismDidState>, LookupError>;
}

/// One deterministic state mutation produced by complete-batch planning.
#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub enum Mutation {
    /// Replace the complete materialized DID state.
    Did(PrismDidState),
    /// Append a validated VDR node and replace its root snapshot.
    Vdr {
        /// Validated signed storage operation.
        operation: VdrOperation,
        /// Root create-operation hash.
        root: Sha256Digest,
        /// Resulting head/terminal state.
        entry: EntrySnapshot,
        /// Canonical owner used for authorization.
        owner: CanonicalPrismDid,
    },
}

impl fmt::Debug for Mutation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Did(state) => formatter
                .debug_struct("DidMutation")
                .field("deactivated", &state.is_deactivated())
                .finish_non_exhaustive(),
            Self::Vdr { entry, .. } => formatter
                .debug_struct("VdrMutation")
                .field("terminal", &entry.terminal)
                .finish_non_exhaustive(),
        }
    }
}

/// A fully validated transaction mutation plan.
#[derive(Clone)]
pub struct PlannedTransaction {
    mutations: Vec<Mutation>,
}

impl PlannedTransaction {
    /// Borrow mutations in source operation order.
    #[must_use]
    pub fn mutations(&self) -> &[Mutation] {
        &self.mutations
    }
}

impl fmt::Debug for PlannedTransaction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PlannedTransaction")
            .field("mutations", &self.mutations.len())
            .finish()
    }
}

/// Validate every operation and produce an in-memory mutation plan.
pub fn plan_transaction<L>(
    batch: &TransactionBatch,
    lookup: &L,
) -> Result<PlannedTransaction, Error>
where
    L: DidStateLookup + VdrLookup,
{
    let mut did_states = HashMap::<CanonicalPrismDid, PrismDidState>::new();
    let mut local_nodes = HashMap::<Sha256Digest, VdrNode>::new();
    let mut local_entries = HashMap::<Sha256Digest, EntrySnapshot>::new();
    let mut budget = TraversalBudget::default();
    let mut mutations = Vec::with_capacity(batch.records().len());

    for (position, record) in batch.records().iter().enumerate() {
        let position = u16::try_from(position).expect("batch maximum fits u16");
        let signed = &record.signed_operation;
        match signed.operation_kind() {
            OperationKind::CreateDid => {
                let state = PrismDidState::from_signed_create(signed)
                    .map_err(|_| Error::at(ErrorKind::InvalidOperation, position))?;
                if did_states.contains_key(state.did())
                    || lookup
                        .did_state(state.did())
                        .map_err(|_| Error::at(ErrorKind::LookupFailed, position))?
                        .is_some()
                {
                    return Err(Error::at(ErrorKind::Conflict, position));
                }
                did_states.insert(state.did().clone(), state.clone());
                mutations.push(Mutation::Did(state));
            }
            OperationKind::UpdateDid | OperationKind::DeactivateDid => {
                let raw = RawPrismOperation::parse(signed.operation_bytes())
                    .map_err(|_| Error::at(ErrorKind::InvalidOperation, position))?;
                let did = if signed.operation_kind() == OperationKind::UpdateDid {
                    UpdateDidOperation::parse(&raw)
                        .map_err(|_| Error::at(ErrorKind::InvalidOperation, position))?
                        .did()
                        .clone()
                } else {
                    DeactivateDidOperation::parse(&raw)
                        .map_err(|_| Error::at(ErrorKind::InvalidOperation, position))?
                        .did()
                        .clone()
                };
                let mut state = match did_states.get(&did).cloned() {
                    Some(state) => state,
                    None => lookup
                        .did_state(&did)
                        .map_err(|_| Error::at(ErrorKind::LookupFailed, position))?
                        .ok_or_else(|| Error::at(ErrorKind::MissingState, position))?,
                };
                state
                    .apply_signed(signed)
                    .map_err(|_| Error::at(ErrorKind::InvalidOperation, position))?;
                did_states.insert(did, state.clone());
                mutations.push(Mutation::Did(state));
            }
            OperationKind::CreateVdrEntry
            | OperationKind::UpdateVdrEntry
            | OperationKind::DeactivateVdrEntry => {
                let operation = VdrOperation::parse(signed)
                    .map_err(|_| Error::at(ErrorKind::InvalidOperation, position))?;
                let (root, owner, terminal) = if let VdrOperation::Create(create) = &operation {
                    let root = operation.operation_hash();
                    if local_nodes.contains_key(&root)
                        || lookup
                            .node(&root)
                            .map_err(|_| Error::at(ErrorKind::LookupFailed, position))?
                            .is_some()
                    {
                        return Err(Error::at(ErrorKind::Conflict, position));
                    }
                    (root, create.owner().clone(), false)
                } else {
                    let predecessor = operation
                        .predecessor()
                        .expect("non-create operation has a predecessor");
                    let resolved = resolve_entry(
                        predecessor,
                        &local_nodes,
                        &local_entries,
                        lookup,
                        &mut budget,
                    )
                    .map_err(|_| Error::at(ErrorKind::InvalidOperation, position))?;
                    (
                        resolved.root,
                        resolved.owner,
                        matches!(operation, VdrOperation::Deactivate(_)),
                    )
                };
                let state = match did_states.get(&owner).cloned() {
                    Some(state) => state,
                    None => lookup
                        .did_state(&owner)
                        .map_err(|_| Error::at(ErrorKind::LookupFailed, position))?
                        .ok_or_else(|| Error::at(ErrorKind::MissingState, position))?,
                };
                operation
                    .authorize(signed, &owner, &state)
                    .map_err(|_| Error::at(ErrorKind::InvalidOperation, position))?;
                let hash = operation.operation_hash();
                local_nodes.insert(
                    hash,
                    VdrNode {
                        operation_hash: hash,
                        predecessor: operation.predecessor(),
                        owner: matches!(operation, VdrOperation::Create(_)).then(|| owner.clone()),
                    },
                );
                let entry = EntrySnapshot {
                    head: hash,
                    terminal,
                };
                local_entries.insert(root, entry.clone());
                mutations.push(Mutation::Vdr {
                    operation,
                    root,
                    entry,
                    owner,
                });
            }
            _ => {
                return Err(Error::at(ErrorKind::InvalidOperation, position));
            }
        }
    }
    Ok(PlannedTransaction { mutations })
}

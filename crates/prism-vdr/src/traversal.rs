//! Finite local-first VDR ancestry traversal.

use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use identus_crypto::Sha256Digest;
use identus_prism_did::CanonicalPrismDid;

use crate::{Error, ErrorKind, LookupError, MAX_ANCESTRY_DEPTH, MAX_LOOKUP_NODES_PER_BATCH};

/// One immutable ancestry node.
#[derive(Clone, PartialEq, Eq)]
pub struct VdrNode {
    /// Exact operation hash identifying this node.
    pub operation_hash: Sha256Digest,
    /// Parent hash, absent only for a create root.
    pub predecessor: Option<Sha256Digest>,
    /// Root owner, present only for a create root.
    pub owner: Option<CanonicalPrismDid>,
}

/// Current committed state of a root entry.
#[derive(Clone, PartialEq, Eq)]
pub struct EntrySnapshot {
    /// Current operation head.
    pub head: Sha256Digest,
    /// Whether deactivation made the entry terminal.
    pub terminal: bool,
}

/// Root and current-state evidence returned by finite traversal.
#[derive(Clone, PartialEq, Eq)]
pub struct ResolvedEntry {
    /// Root create-operation hash.
    pub root: Sha256Digest,
    /// Canonical owner DID.
    pub owner: CanonicalPrismDid,
    /// Current committed-plus-local head.
    pub head: Sha256Digest,
    /// Whether the entry is terminal.
    pub terminal: bool,
}

impl fmt::Debug for VdrNode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VdrNode")
            .field("has_predecessor", &self.predecessor.is_some())
            .field("has_owner", &self.owner.is_some())
            .finish()
    }
}

impl fmt::Debug for EntrySnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EntrySnapshot")
            .field("terminal", &self.terminal)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for ResolvedEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResolvedEntry")
            .field("terminal", &self.terminal)
            .finish_non_exhaustive()
    }
}

/// Read-only committed VDR lookup port.
pub trait VdrLookup {
    /// Look up one committed ancestry node by exact operation hash.
    fn node(&self, operation_hash: &Sha256Digest) -> Result<Option<VdrNode>, LookupError>;

    /// Look up the committed head/terminal snapshot for a root.
    fn entry(&self, root: &Sha256Digest) -> Result<Option<EntrySnapshot>, LookupError>;
}

/// Shared lookup-work budget for one complete transaction batch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraversalBudget {
    remaining: usize,
}

impl Default for TraversalBudget {
    fn default() -> Self {
        Self::new(MAX_LOOKUP_NODES_PER_BATCH)
    }
}

impl TraversalBudget {
    /// Create a budget no larger than the absolute batch maximum.
    #[must_use]
    pub const fn new(limit: usize) -> Self {
        Self {
            remaining: if limit > MAX_LOOKUP_NODES_PER_BATCH {
                MAX_LOOKUP_NODES_PER_BATCH
            } else {
                limit
            },
        }
    }

    /// Return remaining lookup units.
    #[must_use]
    pub const fn remaining(self) -> usize {
        self.remaining
    }

    fn consume(&mut self, position: u16) -> Result<(), Error> {
        if self.remaining == 0 {
            return Err(Error::at(ErrorKind::ResourceLimitExceeded, position));
        }
        self.remaining -= 1;
        Ok(())
    }
}

/// Resolve a predecessor local-first and verify current-head/terminal state.
pub fn resolve_entry<L: VdrLookup>(
    predecessor: Sha256Digest,
    local_nodes: &HashMap<Sha256Digest, VdrNode>,
    local_entries: &HashMap<Sha256Digest, EntrySnapshot>,
    lookup: &L,
    budget: &mut TraversalBudget,
) -> Result<ResolvedEntry, Error> {
    let mut current = predecessor;
    let mut visited = HashSet::new();
    for position in 0..MAX_ANCESTRY_DEPTH {
        let position = u16::try_from(position).expect("ancestry bound fits u16");
        if !visited.insert(current) {
            return Err(Error::at(ErrorKind::Cycle, position));
        }
        budget.consume(position)?;
        let node = match local_nodes.get(&current).cloned() {
            Some(node) => node,
            None => lookup
                .node(&current)
                .map_err(|_| Error::at(ErrorKind::LookupFailed, position))?
                .ok_or_else(|| Error::at(ErrorKind::MissingParent, position))?,
        };
        if node.operation_hash != current {
            return Err(Error::at(ErrorKind::MissingParent, position));
        }
        if let Some(parent) = node.predecessor {
            current = parent;
            continue;
        }
        let owner = node
            .owner
            .ok_or_else(|| Error::at(ErrorKind::MissingParent, position))?;
        let root = node.operation_hash;
        budget.consume(position)?;
        let snapshot = match local_entries.get(&root).cloned() {
            Some(snapshot) => snapshot,
            None => lookup
                .entry(&root)
                .map_err(|_| Error::at(ErrorKind::LookupFailed, position))?
                .ok_or_else(|| Error::at(ErrorKind::MissingParent, position))?,
        };
        if snapshot.terminal {
            return Err(Error::at(ErrorKind::Deactivated, position));
        }
        if snapshot.head != predecessor {
            return Err(Error::at(ErrorKind::StalePredecessor, position));
        }
        return Ok(ResolvedEntry {
            root,
            owner,
            head: snapshot.head,
            terminal: false,
        });
    }
    Err(Error::at(
        ErrorKind::ResourceLimitExceeded,
        u16::try_from(MAX_ANCESTRY_DEPTH).expect("ancestry bound fits u16"),
    ))
}

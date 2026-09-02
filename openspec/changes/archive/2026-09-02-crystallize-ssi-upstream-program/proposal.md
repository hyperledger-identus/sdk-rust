## Why

The Oxid product roadmap contains thirty SDK-owned `IDR-*` dependency rows,
but the SDK repository currently has only a narrative blueprint and seven
early component issues. Without a repository-owned backlog, delivery state,
source provenance and repository boundaries can drift between Apollo,
NeoPRISM, midnight-identity, Lace ID Portal and Oxid.

GitHub issue #20 establishes the durable program record. This change makes the
program executable before any component port begins.

## What Changes

- Add a canonical SDK-only CSV for all thirty `IDR-*` rows.
- Separate portfolio commitment from actual delivery state and link every row
  to either its component issue or the program issue.
- Record immutable donor revisions, paths and extraction classifications.
- Define the generic SDK, Midnight family, NeoPRISM and product boundaries.
- Add a deterministic validation command for the CSV contract.
- Reconcile the SDK blueprint and roadmap with the canonical program.

## Capabilities

### New Capabilities

- `ssi-upstream-program`: governs the canonical upstream backlog, provenance,
  repository boundaries, issue linkage and completion evidence.

### Modified Capabilities

- None.

## Impact

This change affects planning data, architecture documentation and repository
checks. It does not port a cryptographic, DID, credential or protocol API; it
does not mutate a consumer repository; and it does not deprecate Apollo or
remove code from NeoPRISM. Those outcomes require independently testable
component and downstream-adoption issues.

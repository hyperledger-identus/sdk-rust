# Audit identus-core input boundaries

## Why

Issue #168 requires an evidence-backed inventory of inherited untrusted-input
boundaries. Issue #246 isolates `identus-core` as the first post-factory SDK
canary: its URL boundary is already explicit, while its numeric serde behavior
and the absence of dynamic input in its remaining public values have not been
recorded together as a reviewable crate-level receipt.

## What Changes

- Inventory every public externally constructible or deserializable
  `identus-core` value and classify byte, range, allocation and work bounds.
- Add focused serde-negative tests for the two public serialized `u64`
  time values without changing their wire format or public API.
- Record the crate-scoped evidence in the canonical core specification and
  `SDK-LIM-007` ledger while retaining the repository-wide limitation.
- Exercise the pinned Pi shell and record bounded canary evidence; any harness
  tuning remains a separate issue.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `core-error-conventions`: add a crate-level inherited-input inventory and
  deterministic numeric serde boundary evidence.

## Impact

- **Issue:** #246, child of #168 and factory canary of #243.
- **Public and wire API:** unchanged.
- **Dependencies, MSRV and targets:** unchanged.
- **Limitations:** `SDK-LIM-007` stays effective for unaudited crates and for
  allocation before SDK validation.
- **Rollback:** remove the inventory and added tests, and restore the prior
  ledger wording; do not remove the repository-wide limitation.

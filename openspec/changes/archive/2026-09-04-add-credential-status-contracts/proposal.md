## Why

The credential crate can preserve artifacts, metadata, schema shape, and
verification evidence, but every format adapter still has to invent its own
credential-status binding and freshness/query model. Midnight supplies the
strongest Rust skeleton, while W3C status methods demonstrate that the shared
vocabulary must remain open, purpose-aware, and capable of representing more
than one binding per credential.

## What Changes

- Add bounded role-specific status method and purpose identifiers.
- Add distinct bounded opaque text-or-byte reference, handle, revision, and
  observed-value types with redacted diagnostics.
- Add a bounded unique collection of complete status bindings.
- Add wire-free freshness and query-requirement values without selecting a
  clock, registry, transport, verifier, or trust decision.
- Add attributable status evidence with ordered optional observation/expiry
  timestamps.
- Extend stable credential construction errors and add consumer-shaped,
  privacy, boundary, and performance evidence.

## Capabilities

### New Capabilities

- `credential-status`: bounded format-neutral status binding, query, freshness,
  and evidence semantics.

### Modified Capabilities

- `sdk-governance-evidence`: inventory the new experimental status surface and
  its deliberate exclusions.

## Impact

- **Issue:** #77, child of #6 / `IDR-007` and program #20.
- **Code:** one focused module inside `identus-credentials`, plus errors and
  root exports.
- **Dependencies:** none; reuse `identus-core` time values already in the
  dependency cone.
- **Compatibility:** additive, experimental, unpublished, and wire-free.
- **Consumers:** donor and product repositories remain read-only; adapter
  adoption is separate issue-first work.

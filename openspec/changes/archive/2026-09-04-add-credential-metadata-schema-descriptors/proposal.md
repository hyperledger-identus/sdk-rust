## Why

The SDK now preserves opaque credential artifacts and staged verification
evidence, but adapters cannot project bounded issuer, subject, type, validity,
schema, or claim-disclosure metadata without inventing consumer-local models.
Midnight, Oxid, and Lace already carry overlapping shapes. Their intersection
belongs in the chain-neutral credential crate; their wire codecs, display
metadata, claim values, and product policy do not.

## What Changes

- Add validated descriptor strings for entity, credential-type, schema,
  schema-version, claim, value-type, and path-segment roles.
- Add claim disclosure modes and bounded claim paths/descriptors.
- Add bounded schema descriptors with unique type, claim-id, and claim-path
  invariants.
- Add normalized credential metadata with bounded subjects, types, schemas,
  and a valid optional time interval.
- Keep extraction, parsing, serialization, schema execution, verification,
  trust, localization, storage, and format-specific behavior outside the core.
- Extend stable credential errors and add consumer-shaped and performance
  evidence.

## Capabilities

### New Capabilities

- `credential-metadata`: bounded format-neutral descriptive metadata and
  schema/claim descriptor semantics.

### Modified Capabilities

- `sdk-governance-evidence`: inventory the expanded experimental credential
  surface and retain explicit exclusions.

## Impact

- **Issue:** #75, child of #6 / `IDR-007` and #20.
- **Code:** focused modules inside the existing `identus-credentials` crate.
- **Dependencies:** none; reuse `identus-core::UnixTimestampMillis`.
- **Compatibility:** additive, experimental, unpublished, and without a wire
  contract.
- **Consumers:** Oxid, midnight-identity, Lace ID Portal, NeoPRISM, and Apollo
  remain read-only evidence sources.

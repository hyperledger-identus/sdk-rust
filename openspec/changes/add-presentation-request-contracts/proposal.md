## Why

The SDK can describe credentials, verification evidence, and status, but its
presentation crate is still a quarantined marker. Oxid already has a useful
holder-side request/candidate model, while OpenID4VP Final defines the
protocol-independent need for format-aware credential queries, requested
claims, candidate selection, and replay binding. Without a shared semantic
boundary, each wallet or protocol adapter must invent incompatible request and
candidate types.

## What Changes

- Replace the `identus-presentations` marker with bounded request, credential
  query, claim-request, challenge, credential-handle, candidate, and validated
  candidate-set types.
- Reuse credential format and descriptor types rather than duplicate them.
- Enforce bounds, uniqueness, query membership, format agreement, requested
  claim scope, and required-claim coverage during construction.
- Add redaction-safe diagnostics, stable `presentation.*` errors, consumer-
  shaped tests, and a manual construction-throughput diagnostic.
- Inventory the crate as implemented, experimental, and deliberately free of
  protocol wire, selection policy, consent, proof, lifecycle, and storage.

## Capabilities

### New Capabilities

- `presentation-core`: bounded format-neutral presentation request/query and
  candidate semantics.

### Modified Capabilities

- `sdk-governance-evidence`: replace the presentation placeholder inventory
  with the accepted experimental IDR-008a surface and exclusions.

## Impact

- **Issue:** #79 under `IDR-008` and program #20.
- **Code:** replace the presentation marker with focused modules, errors,
  exports, tests, and component metadata.
- **Dependencies:** add only the inward workspace dependency on
  `identus-credentials`; retain `identus-core`; the lock records that local
  edge, with no external package or version change.
- **Compatibility:** additive experimental API in an unpublished placeholder;
  no wire or SemVer commitment.
- **Consumers:** all donor and product repositories remain read-only; adoption
  is separate issue-first work.

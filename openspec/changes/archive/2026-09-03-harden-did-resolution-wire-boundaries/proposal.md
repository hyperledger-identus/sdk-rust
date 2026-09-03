## Why

Issue #39 established useful W3C DID resolution and serialized dereferencing
result values, but their raw JSON entry points still permit duplicate object
names to collapse during deserialization. The datetime value also implements a
smaller four-digit civil-time subset than the Candidate Recommendation's
bounded XML Schema 1.1, UTC, whole-second intersection. These gaps make a
shared PRISM/Midnight producer and Lace/Oxid consumer boundary ambiguous.

Issue #41 (`IDR-005f`) is the bounded stabilization slice. It follows #38's
crate-private duplicate-aware scanner and raises result/data-boundary maturity
without importing resolution transports, DID methods, chain policy, or product
policy.

## What Changes

- Pin the 6 August 2026 W3C Candidate Recommendation Snapshot as the
  compatibility baseline and classify the 28 August draft/source delta.
- Run both untrusted result entry points through the duplicate-aware bounded
  scanner before typed deserialization.
- Add public result preflight ceilings and redaction-safe resolution reasons
  for duplicate names and raw resource exhaustion.
- Implement the bounded XML Schema 1.1 `dateTime` lexical intersection that is
  already adjusted to UTC and contains whole seconds only.
- Add deterministic generated/property matrices for scalar parsers, RFC
  9457-shaped errors, result states, open JSON budgets, and raw envelopes.
- Map portable assertions from the pinned official W3C implementation-report
  suite and record normalized performance/allocation-shape evidence.
- Keep arbitrary native dereferenced bytes in a future binding value paired
  with a media type; do not silently encode bytes in the at-risk JSON result.
- Document explicit legacy keyword-error migration.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `did-core`: strengthens the raw DID resolution and serialized dereferencing
  boundary, broadens datetime conformance, and pins generated standards
  evidence plus binding/migration decisions.

## Impact

- **Issue:** #41, child of #5 / `IDR-005`.
- **Affected code:** `crates/did` result parsing, datetime validation, errors,
  and conformance tests; canonical DID Core requirements and ADR 0017.
- **Public compatibility:** additive constants and non-exhaustive error reasons;
  broader standards-valid datetime acceptance; existing result shapes remain.
- **Wire compatibility:** unique-name JSON is unchanged; duplicate-name raw
  input is intentionally rejected before the first SDK release.
- **Dependencies:** no new normal or development dependency and no lock change.
- **Consumers:** all downstream repositories remain read-only; adoption is
  separately owned.
- **Rollback:** revert this issue's focused PR. No release, persisted SDK data,
  chain state, product repository, or `main` branch is changed.

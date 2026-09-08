# Make Apollo crypto parity evidence executable

## Why

Issue #211 requires the M2 Apollo comparison to stop depending on prose tables
that can silently drift. The current Discussion #178 contains a strong audited
snapshot, but capability completeness, disposition vocabulary, immutable
source links, vector mappings, test selectors and CI receipts are not checked
by the repository factory.

## What Changes

- Add one canonical TOML manifest for the pinned Apollo and sdk-rust crypto
  comparison.
- Give every audited Apollo capability one allowed disposition and explicit
  consumer impact.
- Map Apollo-owned and normative vector suites to immutable source revisions
  and executable sdk-rust test selectors.
- Add an offline validator and regression tests to the factory contract.
- Render a deterministic Markdown report from the manifest for Discussion
  #178 without making GitHub content the source of truth.

## Capabilities

### New Capabilities

- `apollo-crypto-parity`: provides a complete, machine-validated capability
  and vector disposition ledger for the pinned Apollo comparison.

### Modified Capabilities

None.

## Impact

- **Issue:** #211, child of M2 epic #9.
- **Runtime/public API:** none; no Rust crate or dependency changes.
- **Factory:** adds one required document, checker and regression suite.
- **Consumers:** no downstream repository is changed or claimed migrated.
- **Rollback:** remove the manifest/checker/factory wiring and restore
  Discussion #178 as a manually maintained report.

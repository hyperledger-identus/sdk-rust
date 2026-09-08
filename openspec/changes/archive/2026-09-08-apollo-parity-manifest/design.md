# Design

## Context

The parity report has three relationships: one pinned comparison baseline,
audited capabilities with dispositions, and source/vector suites mapped to SDK
test selectors. A flat prose table cannot reliably enforce those joins.

## Decisions

### One canonical TOML ledger

`docs/architecture/apollo-crypto-parity.toml` owns report metadata, immutable
baselines, computed summary counts, capability rows and vector rows. Every
table uses an exact key set so schema drift fails visibly.

### Fixed audited capability IDs

The checker contains the capability IDs audited for the pinned Apollo
revision. This intentional duplication turns removal or baseline expansion
into a checker review instead of letting a manifest author silently redefine
100% completeness. Every row uses exactly one of `parity`, `sdk-exceeds`,
`accepted-difference` or `gap`.

### Evidence appropriate to the disposition

`parity` and `sdk-exceeds` rows require at least one vector/evidence mapping,
an immutable SDK test link, a delivery PR and the exact green baseline CI
receipt. `accepted-difference` rows require a rationale, consumer impact,
reopen trigger and tracking issue. A `gap` is schema-valid for future refreshes
but must name its tracking issue; this closing manifest records zero gaps.

Vector rows record kind, origin repository/revision/path/license,
transformation, expected result, local SDK test path and selectors. The checker
proves each local path is safe and exists and each selector occurs in it. It
also proves GitHub blob links contain the declared source revision and path.

### Deterministic Discussion rendering

The checker validates by default and emits Markdown only with
`--render-markdown`. The rendered summary, capability table and vector table
are derived from parsed data in stable manifest order. Posting that output to
Discussion #178 is an external coordination step; the versioned manifest
remains authoritative.

### Offline and dependency-free

The checker does not call GitHub or clone Apollo. CI validates evidence shape,
revision binding and local selectors; hosted artifact availability is reviewed
when the Discussion/PR is updated. Python standard-library code keeps the
factory dependency cone unchanged.

## Failure behavior

All findings accumulate and print static context plus safe manifest metadata.
The checker exits nonzero on malformed TOML, unknown or missing keys,
baseline/link mismatch, capability coverage drift, invalid disposition,
summary mismatch, missing disposition evidence, unknown vector IDs, unsafe or
missing local paths, or absent selectors.

## Test design

Regression tests copy the manifest and referenced SDK files into an isolated
fixture, prove the canonical ledger passes, then mutate status, IDs, links,
summary, vector references and selectors to prove rejection. A renderer test
asserts deterministic headings and row counts without snapshotting mutable
GitHub prose.

## Rollback

Delete the manifest, checker, tests and factory wiring together. Never leave a
Discussion claim that says the removed offline contract is still enforced.

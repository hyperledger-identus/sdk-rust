# Pre-implementation semantic, security, and API review

- **Date:** 2026-09-03
- **Issue:** #41, child of #5 / `IDR-005`
- **Develop base:** `1e2e64265e048c5f7bfe06ffcba39de881611517`
- **Result:** contract is implementable with no unresolved blocker

## Standards and compatibility findings

The issue correctly names the 28 August Candidate Recommendation Draft. The
6 August Snapshot is the stronger compatibility pin; comparing editor source
from the snapshot publication through `71a5005` finds only non-normative
implementation-suite guidance and publication configuration. The existing
nine error URLs and result shapes therefore remain compatible.

The current four-digit datetime parser is a valid common subset but not the
complete intersection required by the draft's XML Schema 1.1 reference. A
bounded string validator can admit astronomical zero, negative/extended years,
and `24:00:00Z` without a datetime dependency or lossy conversion. Requiring
`Z` and rejecting fractions follows the additional DID Resolution constraints.

## Security and resource findings

Scanning after `serde_json::Value` or typed deserialization would be too late:
duplicates have already collapsed. The #38 streaming scanner is suitable when
the result byte cap is checked first and its complete-input traversal precedes
typed parsing. Result-specific static error reasons preserve diagnostic
clarity without carrying attacker strings.

The scanner's 64-depth, 16,384-node, 128-member, and 128-KiB-live-name limits
bound its object-local decoded-name sets. Existing typed semantic limits remain
tighter for open data. The double traversal is an acceptable availability
trade-off and receives fixed-work, no-threshold release evidence.

## Conformance, provenance, and boundary findings

The official implementation-report suite is BSD-3-Clause and currently tests
HTTP resolution rather than publishing a reusable result/dereferencing fixture
corpus. Its portable result assertions can be recreated as original Rust
vectors; copying JavaScript or claiming dereferencing-suite coverage would be
incorrect. Cargo-fuzz remains #35, while deterministic property matrices make
this result slice reproducible across the supported target matrix.

Arbitrary native bytes cannot be added losslessly to the current JSON value
without selecting an encoding. A future binding-specific media-type plus
bounded-byte value is the reversible boundary; implicit UTF-8/base64 and
network retrieval stay rejected. No chain/product dependency, unsafe code,
consumer mutation, release, publication, repository administration, or `main`
change is justified.

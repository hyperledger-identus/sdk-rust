# Constraint and limitation impact

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/159
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` remains effective because the DID lexical value stays generic
and chain-neutral. `SDK-COMPAT-004` and `SDK-COMPAT-005` remain effective under
the Rust 1.98.1 etalon and portable target checks. `SDK-SEC-001` is not relaxed:
the SDK adds no unsafe Rust, and dependency-owned unsafe code is an explicit
decision input. `SDK-SEC-003` preserves pre-allocation byte ceilings.
`SDK-DELIVERY-001` is satisfied by issue #159 and this specification-first
change.

## Introduced or changed constraints

No repository-wide value changes. The DID parser may not be replaced merely
to reduce local source ownership. A candidate must preserve the exact accepted
language, input ceilings before allocation, immutable validated values, owned
allocation reuse, borrowed component views, redacted facade and supported
targets without introducing unused query or relative-resolution semantics.

## Introduced or changed limitations

- The local DID/DID-URL parser remains SDK-owned if any stop condition holds.
- A retain decision does not claim the local parser implements normalization,
  equivalence, IRI syntax or DID method semantics.
- Candidate maintenance is a snapshot, not a prediction.
- Local portable-target compilation does not replace hosted Linux or weekly
  slow evidence.

## Consumer and product impact

No public type, behavior, serialization, error, dependency, persisted state or
consumer migration changes. The decision protects current NeoPRISM, Midnight,
Lace and Oxid-shaped identifiers while keeping method semantics downstream.

## Activation and rollback

The decision becomes repository policy only after the issue-linked PR passes
local and hosted gates and merges to `develop`. Reverting the PR restores the
prior portfolio wording; no production or data rollback is required.

## Evidence

The final research record must name the current implementation and consumer
evidence, primary source URLs, pinned revisions, exact candidate version and
features, license/provenance, MSRV, targets, direct/resolved dependency cone,
unsafe/native reach, supply-chain posture, public/wire and facade compatibility,
rollback, maintenance/release/security posture, protocol/draft currency,
reconsideration trigger, exact commands and intentionally unrun checks.

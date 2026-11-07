# Design

## Separate target relation from delivery state

The parity matrix is a new machine record keyed by the 24 normalized SDK-TS
capability IDs. It references rather than duplicates the source inventory.
Each record contains:

- one target relation;
- one current conformance state;
- exact SDK-Rust and SDK-TS evidence scope;
- required conformance dimensions;
- known deviations and exclusions;
- one owning issue and next action; and
- named consumer/rollback evidence where it exists.

The validator cross-checks complete one-to-one coverage against the SDK-TS
inventory. It rejects missing/extra/duplicate capability IDs, unknown
relations/states, a `platform-owned` relation on non-platform dispositions,
`not-applicable` without platform ownership, silent intentional differences,
and delivered claims without complete evidence dimensions.

## TS1 milestone

`TS1 — SDK-Rust/SDK-TS capability parity and conformance` is a cross-cutting
milestone after A1. It is not a new sequential protocol train and does not wait
for A2-A7 in bulk. Each capability record names its actual prerequisites and
can advance independently.

TS1 exits only when every record is either:

- delivered with its required conformance and consumer/rollback evidence;
- intentionally excluded or not applicable with accepted ownership rationale;
  or
- moved to a separately accepted later milestone with an explicit blocker and
  no misleading parity claim.

The first implementation remains #492's opt-in DID/DID URL facade. It proves
the mechanism, not the entire milestone.

## Local-first capability trains

TS1 implementation is delivered in cohesive integration trains rather than a
sequence of mechanical PRs. A normal train targets 5,000-10,000 meaningful
changed lines and carries the complete vertical evidence for one capability or
tightly coupled capability cluster:

- canonical implementation and public/binding adapters;
- normative and compatibility vectors plus negative cases;
- migration, observability, fallback, and rollback behavior;
- documentation and generated-surface updates; and
- focused, mutation, differential, target, and integration tests.

The target excludes generated/vendor/lockfile/evidence churn and is never a
reason to add unrelated changes. Signed internal commits form reviewable
checkpoints. Targeted local checks run during assembly, then the complete
applicable local factory gate qualifies one candidate push. Hosted CI remains
the protected final check. A train is split when any part needs independent
ownership, security or publication authority, rollback, or review.

## Report

A checked-in report explains the relation vocabulary, milestone policy, and
groups exact capability IDs by conformance state. It gives counts only as an
inventory check. It never computes a blended parity percentage.

## DID reconciliation

The readiness record binds the completed slow run and merged NeoPRISM adoption
to the exact SDK candidate. It records engineering qualification as complete
and publication as blocked by the separate administrator-owned gate. Backlog
IDR-005/006 and A0/A1/A2 states are aligned with the underlying issue/merge
evidence without changing package publication or API stability.

## Verification

Focused verification runs the backlog checker and mutation suite, platform
inventory/roadmap checker and mutation suite, the new parity checker and
mutations, strict OpenSpec validation, documentation links/build, and complete
factory structural gates as proportional evidence.

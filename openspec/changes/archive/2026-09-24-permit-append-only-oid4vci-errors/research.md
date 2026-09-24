# Append-only OID4VCI error research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

The wildcard-free router already makes every live error explicit, and the v1
golden independently freezes the original 171 contracts. The crate-local test
adds an unnecessary equality between the complete live inventory and the
historical fixture, preventing any additive variant. Inserting a new variant
would also shift stable baseline discriminants.

## Normative sources

ADR 0119, the canonical `oid4vci-error-contracts` specification, the immutable
fixture/checker and the live macro-generated inventory are sufficient local
authority. No external library or protocol source is relevant.

## Compatibility and dependency evidence

The v1 fixture and checker remain byte-for-byte unchanged. The implementation
changes only crate-local test assertions and adds no production, manifest,
feature, dependency, lockfile or target delta.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Exact immutable prefix plus unique live suffix | `adopt` | Preserves all historical evidence while allowing additive exhaustive evolution. |
| Rewrite the v1 fixture | `not-adopt` | Destroys independent pre-refactor evidence and its receipt binding. |
| Version a new full golden for every feature | `not-adopt` | Adds ceremony and self-blessing risk without improving protection of prior contracts. |
| Remove the live inventory check | `not-adopt` | Would lose ordered-prefix and uniqueness evidence. |
| Wildcard/default router | `not-adopt` | Hides unmapped variants and weakens compilation guarantees. |

## Security, privacy and maintenance evidence

Only test and governance logic changes. The old fixture remains byte-exact;
the complete router remains compile-exhaustive; the inventory remains unique.
Future feature tests must independently prove each appended code, kind,
message, redaction and conversion. No runtime path or input handling changes.

## Rejected or deferred candidates

A workspace-wide error-schema generator and automatic versioned live snapshots
are deferred because this focused issue needs neither new tooling nor another
self-authored oracle. Other crates retain their current governance unchanged.

## Open questions and blockers

None.

## Evidence commands

```text
scripts/factory research-ready permit-append-only-oid4vci-errors
scripts/factory constraints-ready permit-append-only-oid4vci-errors
cargo test -p identus-oid4vci --lib
scripts/check-error-golden.py .
```

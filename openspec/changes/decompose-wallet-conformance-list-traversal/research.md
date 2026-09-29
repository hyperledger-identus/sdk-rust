# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The verification-only `identus-wallet-conformance` crate exposes five async
checks over consumer-owned storage traits. Three checks compose a fixed
exact-record lifecycle with a paginated recovery-index traversal. The private
list scenario currently owns both external I/O and deterministic evidence:

1. reject another request after `expected_entries.len() + 1` completed pages;
2. call the consumer list port with the exact fixture page size and prior
   continuation cursor;
3. count the successful call before validating its returned page;
4. reject a page longer than the requested size;
5. reject an entry already observed on an earlier or current page;
6. reject observations beyond the expected fixture cardinality;
7. reject a repeated continuation cursor;
8. stop only when the adapter omits a continuation cursor;
9. require the final observed set to equal the expected set, independent of
   order; and
10. report completed calls plus observed entries.

For the existing three-entry, page-size-one fixture, list evidence contributes
three calls and three entries to the combined `19/3` report. The current
function is 82 SLOC, cognitive 11, and cyclomatic 17. The signal is marginal,
but the I/O/evidence boundary is semantic rather than line-count-driven.

## Normative sources

Issue #441, the #438 archived receipt, ADR 0115, and the canonical
`wallet-storage-conformance`, code-health, input-resource, crate-ring,
dependency-boundary, and spec-driven delivery contracts are authoritative.
Current tests cover success, repeated cursors, invalid fixtures, aggregate
counts, and diagnostic redaction. They do not independently bind overlong
pages, duplicate returned entries, or excess/wrong membership.

Characterization established that the `list-termination` branch cannot be
reached by a value created through the public `StoragePage` constructor. A
page with a continuation must contain at least one entry. Before a traversal
can exceed the fixture-derived page limit, each such entry therefore either
duplicates an observation or makes the observed cardinality exceed the
fixture. Those earlier checks deterministically return
`DuplicateObservedEntry` or `IndexMembershipMismatch`. The redundant private
branch may be removed; the public `PaginationDidNotTerminate` variant remains
for source compatibility.

No external protocol or library research is required. This is private SDK test
kit maintenance with no algorithm, dependency, serialization, or interop
decision.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private bounded evidence state plus small async coordinator | `adopt` | Separates external failure/timing from pure page, cursor, membership, and count invariants while retaining explicit control flow. | The split requires public types, callbacks, or more complex state than the original. |
| Keep the function unchanged and document an exception | `not-adopt` | The state machine is cohesive, but the uncharacterized deterministic invariants have a distinct change and test boundary from adapter I/O. | Characterization shows the proposed boundary cannot preserve steps or ordering cleanly. |
| Split every condition into a helper | `not-adopt` | Produces forwarding fragments and hides the state transition rather than creating ownership. | Never as metric-only decomposition. |
| Generic pagination framework shared with product code | `not-adopt` | Conformance evidence has different authority and failure vocabulary from product pagination. | A separately approved reusable invariant emerges in at least two consumers. |
| Adopt a testing or pagination dependency | `not-adopt` | No crate owns Identus storage ports or this exact closed evidence contract. | A maintained dependency proves exact semantics and target compatibility. |

## Compatibility and dependency evidence

`ListStoreFixture`, all five checker signatures, root exports, report and
failure types, page/cardinality limits, `PartialEq` bounds, successful counts,
and static error steps remain unchanged. The crate retains `identus-wallet` as
its only runtime dependency and no features. No manifest, lockfile, MSRV,
unsafe, native, FFI, target, serialization, or wire change is needed.

## Security, privacy and maintenance evidence

Scopes, entries, cursors, and adapter errors remain absent from diagnostics.
The evidence state is bounded by the validated fixture cardinality: observed
entries cannot grow past the expected count, every continued page must add an
entry, and duplicate/excess checks therefore bound cursor history without a
second counter-derived termination branch. The page is rejected before its
entries are retained when it exceeds the caller-selected size. The design uses
static dispatch and does not add callback tables, trait objects, executors, or
synchronization.

Test faults and transcripts use closed categories and counters only. They must
not add `Debug`, `Hash`, `Ord`, serde, formatting, or cloning requirements to
consumer entry types.

## Rejected or deferred candidates

Mechanical helpers, a generic pagination framework, and new dependencies are
rejected. Sorting, cursor decoding, stronger complexity guarantees, adapter
durability, encrypted receipts, and production performance thresholds remain
deferred.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate: if the
evidence owner cannot retain exact request order, bounds, static steps/kinds,
and consumer trait bounds without extra indirection, production code remains
unchanged and the signal is documented as a cohesive exception.

## Evidence commands

Planning inspected protected `develop@5031e7178bee1b031e9f7e84e677d03f95e936d3`,
issue #441, the #438 archive, the complete list source and memory-adapter tests,
the crate manifest, canonical report, and governing specifications. Before
production edits run the focused suite and add the closed failure matrix.
Afterward run focused/workspace tests, strict Clippy/format/docs, public/source,
code-health and factory checks, portable targets, relevant Nix gates, distinct
exact-diff review, and protected exact-head CI. Those implementation commands
are unrun at planning time.

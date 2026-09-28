# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The verification-only `identus-wallet-conformance` crate has one runtime
dependency, `identus-wallet`, and exposes five async entry points over consumer
storage traits. `run_exact` performs a fixed 16-call scenario:

1. missing-key load;
2. clean primary load;
3. clean isolated-scope load;
4. insert-only write;
5. read after insert;
6. scope-isolation load;
7. conflicting duplicate insert;
8. preservation load;
9. unconditional replacement;
10. conflicting stale write;
11. preservation load;
12. conflicting stale delete;
13. preservation load;
14. current-revision delete;
15. read after delete;
16. unconditional delete of the missing record.

The function also maps every failure to a static step and closed kind, retains
two revisions, counts completed calls, and produces the exact-only report.
Those responsibilities are coherent at crate level but not at one function
boundary. List traversal has different bounds and failure semantics and should
remain a sibling rather than be generalized with the exact scenario.

## Normative sources

Issue #438 and the `wallet-conformance-run-exact` disposition in the canonical
code-health report direct this maintenance slice. The archived #270 research,
ADR 0115, `wallet-storage-conformance`, code-health, input-resource, crate-ring,
dependency-boundary, and spec-driven delivery contracts are authoritative.
The current unit tests prove all five ports, aggregate counts, revision reuse,
cursor progress, fixture bounds, and non-Debug diagnostic redaction.

No external protocol or library research is needed: this changes private test
kit ownership only and introduces no algorithm, dependency, serialization, or
interop decision.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private exact/list modules plus named exact lifecycle phases | `adopt` | Makes invariants reviewable while preserving one crate facade and static dispatch. | A future public namespace is independently justified. |
| Only move `run_exact` unchanged to another file | `not-adopt` | Reduces file size but leaves the 250-line responsibility hotspot intact. | Never as a claimed code-health improvement. |
| Data-driven callbacks or dynamic scenario tables | `not-adopt` | Obscures mutation order, adds indirection, and weakens type/state review. | Many genuinely equivalent scenarios prove the shared model. |
| Share exact and pagination evidence machinery | `not-adopt` | Their bounds, states, and likely change cadence differ. | Both scenarios acquire a complete common invariant. |
| Adopt a conformance-testing dependency | `not-adopt` | No dependency owns Identus storage traits or their exact safety contract. | A maintained crate proves exact semantic and target compatibility. |

## Compatibility and dependency evidence

The five public checker signatures, two fixture types, failure types, report,
limit constant, trait authority, `Clone + PartialEq` bounds, 16/19 operation
counts, and root import paths remain unchanged. The crate still has
`identus-wallet` as its only runtime dependency and no production SDK crate
depends on it. No manifest, lockfile, feature, MSRV, target, unsafe, native,
FFI, allocation-budget, serialization, or wire change is required.

## Security, privacy and maintenance evidence

Scopes, keys, values, revisions, cursors, and index entries remain absent from
Debug/Display and failure values. Private phase helpers accept borrowed driver
and fixture state, use static dispatch, and allocate no scenario table.
Expected conflicts remain explicit success conditions; unknown operational
errors remain distinct from accepted conflicts. The report exposes only
aggregate counts.

Test-only transcript instrumentation records closed operation categories, not
fixture content. It must prove exact ordering and conditions without imposing
Debug, Hash, Ord, serde, or formatting bounds on consumer types.

## Rejected or deferred candidates

File-only movement, generic scenario frameworks, exact/list unification, and a
new dependency are rejected. Cancellation/single-flight (#50), adapter
implementations, encrypted downstream receipts, pagination changes, storage
performance guarantees, and new report fields remain deferred.

## Open questions and blockers

There are no planning blockers. Any public/source break, step/kind change,
operation reorder, broadened trait authority, diagnostic exposure, new
allocation/dynamic dispatch, manifest change, or runtime behavior blocks this
slice and must move to a separately authorized decision.

## Evidence commands

Planning inspected `develop@b454c0ec0e72e64c3208c37d43fa404465518610`,
issue #438, #270 evidence, the complete crate source/tests, Cargo manifest,
canonical code-health report, and governing specifications. Before production
edits run the focused crate suite and capture public/source identity. Afterward
run focused/workspace tests, strict Clippy/format/docs, code-health and factory
contracts, public/source comparison, portable targets, relevant Nix gates, and
protected exact-head CI. These implementation commands are unrun at planning
time.

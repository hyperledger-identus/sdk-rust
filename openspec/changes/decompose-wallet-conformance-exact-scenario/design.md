# Design

## Private ownership map

The crate root remains the public facade. Two private siblings own independent
conformance scenarios:

| Owner | Responsibility |
| --- | --- |
| `exact` | `ExactStoreFixture`, exact driver port, named lifecycle phases, revision/state assertions, and 16-call evidence count |
| `list` | `ListStoreFixture`, list driver port, bounded cursor traversal, membership evidence, and observed-entry count |
| crate facade | public failure/report vocabulary, five port adapters, public checker functions, and explicit result combination |

Neither child becomes a public namespace. The facade re-exports the two fixture
types at their existing crate-root paths.

## Exact lifecycle phases

`run_exact` becomes a small coordinator over five private async phases:

1. prove the missing key, primary key, and isolated scope are clean;
2. insert and prove value/revision plus scope isolation;
3. reject duplicate insert and prove preservation;
4. replace, prove revision invalidation, reject stale write/delete, and prove
   preservation after each conflict;
5. delete the current revision, prove absence, and prove missing delete.

A private evidence value increments only after the same successful or expected-
conflict calls as today and converts to the unchanged aggregate report. Phase
return values carry only the revisions needed by successors. No callback table,
trait object, heap scenario graph, executor, or synchronization is introduced.

## Characterization boundary

Before production movement, the test memory adapter records a closed test-only
operation enum and the success test asserts all 16 calls and conditions in
order. Existing count, revision-reuse, cursor, fixture-bound, and non-Debug
tests remain. Focused adversarial fixtures cover phase failure projection and
prove static step/kind diagnostics without retaining caller values.

## Compatibility and ratchet

The crate-root public inventory and generated documentation must be identical
apart from private source links. All five entry points call the same exact/list
owners with unchanged generic bounds. The successful exact report remains
`16/0`; list-capable reports remain `19/3` for the existing fixture.

Canonical code-health evidence must remove `run_exact` as a function hotspot,
must not replace it with an equivalent large helper, and must not weaken or
silently reclassify `run_list` or unrelated signals. Physical file movement is
supporting structure, not the claimed improvement; named invariant phases and
the transcript are the semantic ratchet.

## Risks and rollback

The main risk is accidental operation reordering or counter/failure drift while
moving code. The transcript, existing tests, public/source comparison, and
exact-diff review make this visible. The branch is independently revertible;
rollback restores the previous private layout without consumer or stored-data
migration.

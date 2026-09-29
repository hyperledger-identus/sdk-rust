# Design

## Private ownership boundary

`list::run` remains the only async scenario coordinator. A private
`ListEvidence<Entry>` owns deterministic bounded state:

- completed successful list calls;
- observed unique entries;
- previously accepted continuation cursors;

The coordinator creates the exact `StoragePageRequest`, awaits the consumer
port, maps operational failure to `list-page`, records the completed call, and
passes the returned page to the evidence owner. The owner checks page size,
entry uniqueness/cardinality, cursor progress, final membership, and
constructs the unchanged report.

No public namespace, trait, callback, heap scenario graph, runtime, executor,
or dependency is introduced. The fixture and driver remain in the existing
private `list` module.

## State transition order

The refactor preserves this order exactly:

1. call the adapter with the current cursor;
2. increment completed operations only after a successful port result;
3. reject page-size overflow before retaining entries;
4. inspect entries in returned order, rejecting duplicates before retention
   and excess membership immediately after retention;
5. accept absence of a next cursor as termination;
6. reject a repeated cursor before retaining it; otherwise retain it and make
   it the next request cursor;
7. after termination, prove exact unordered membership and emit the report.

The prior pre-request `max_pages` branch is unreachable for constructible
`StoragePage` values: every continued page contains an entry, so duplicate or
excess membership fails first. Removing that redundant private branch does not
change observable behavior for any consumer-returnable page.

The owner may expose small intention-revealing methods such as
`require_next_page`, `accept_page`, and `finish`; it must not scatter each
condition into forwarding-only helpers.

## Characterization boundary

Before production movement, the memory adapter gains closed test-only list
faults. Tests bind each current static projection:

| Fault | Step | Kind |
| --- | --- | --- |
| adapter error | `list-page` | `OperationFailed` |
| overlong page | `list-page-bound` | `PageBoundExceeded` |
| duplicate returned entry | `list-duplicate-entry` | `DuplicateObservedEntry` |
| excess or wrong final set | `list-membership` | `IndexMembershipMismatch` |
| repeated continuation | `list-cursor-progress` | `CursorDidNotProgress` |

`list-termination` is excluded from the executable fault matrix because the
validated page type makes that state unconstructible. The proof above and the
unchanged public failure variant document the compatibility decision.

A closed request transcript proves page size, first/continuation distinction,
call count, and termination ordering without retaining cursor bytes or entries.
Existing success, fixture, redaction, and exact-scenario tests remain.

## Compatibility and code-health ratchet

The crate-root public inventory and generated docs remain unchanged. The three
list-capable entry points call the same private owner with unchanged generic
bounds; the existing fixture still produces the `19/3` combined report.

The touched-scope comparison must remove the `list::run` function signal and
must not replace it with an equivalent large helper, generated code, moved
test, waiver, or weakened threshold. Canonical evidence is rebound to a
durable protected implementation commit after the production PR merges.

## Risks and rollback

The risks are condition reordering, premature retention, count drift, and
changed failure priority. The closed fault matrix and request transcript make
those observable. The branch is independently revertible; rollback recombines
the private evidence state into `run` without consumer or stored-data
migration.

## Squash-safe evidence delivery

Guarded delivery uses a protected squash commit. Therefore implementation and
canonical-evidence closeout remain two issue-linked PRs: first merge the
characterized private refactor, then regenerate/pin the report from that
durable protected commit, archive the change, and close #441. The second branch
retains planning ancestry through a signed synchronization merge.

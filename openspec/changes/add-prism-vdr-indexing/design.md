# Design

## Ownership and dependency rings

`identus-prism-vdr` owns method-specific storage payloads and root traversal.
`identus-prism-indexer` owns ordered batch planning and replay/cursor semantics.
Neither owns source polling, persistence transactions, scheduling or retries.

The dependency guard permits only:

```text
identus-prism-indexer
          |
          v
 identus-prism-vdr
          |
          v
 identus-prism-did -> identus-did / identus-crypto / identus-core
```

## VDR domain

The VDR crate decodes exact bounded payload bytes already accepted by
`identus-prism-did`. Public operations retain fixed-size operation hashes,
validated PRISM DID ownership, opaque bounded content and an optional exact
predecessor. Create establishes a root; update/deactivate require a predecessor.
Unknown or malformed variants fail closed. Content is not interpreted as a
generic credential or document format.

Root traversal receives a lookup trait for previously committed VDR nodes and
a bounded batch-local map. It walks the local map first, then the immutable
lookup snapshot, records visited hashes and terminates on root, missing parent,
cycle or depth exhaustion. The lookup also supplies the committed current head
for the resolved root. Update/deactivate predecessors must equal that head after
earlier batch-local operations are applied; a second child of an old head is a
stale fork and rejects the transaction. The lookup cannot mutate state.

Authorization uses the owner DID derived directly from create or from the
resolved root for update/deactivate. The caller supplies the materialized DID
state at the exact pre-operation ledger position. The VDR crate requires that
state to be active, locates the active `KeyUsage::Vdr` secp256k1 key named by
the envelope, and verifies the signature over exact nested operation bytes.
Malformed state, missing/revoked/wrong-purpose keys and bad signatures reject
the operation without a mutation. The storage proto's old “issuing key” comment
is not authoritative; the dedicated VDR key enforced by the donor processor is
the explicit SDK profile.

## Deterministic indexer core

A caller supplies one complete transaction batch with an expected cursor, next
cursor, opaque bounded transaction identity and declared operation count.
Records carry that same identity, a contiguous zero-based operation index and
exact signed operation bytes. Construction verifies identity, count, indices
and limits, so a caller cannot accidentally split or combine transactions.
The source adapter is responsible for extracting an authoritative transaction
boundary from its ledger; other chain metadata remains outside the plan.

The planner validates total cardinality and order, parses every operation,
reduces DID state from the caller's snapshot (including earlier operations in
the same transaction), resolves storage roots, authorizes every storage
signature against the state at that position, and builds all mutations in
memory. Any invalid record fails the batch before persistence. A successful plan is passed once to an
atomic commit trait together with expected/next cursors. The adapter must either
commit all mutations and advance the cursor or change nothing.

Immediately after structural batch validation, the indexer computes a
domain-separated SHA-256 transition digest over the exact ordered signed
records, their indices, transaction identity, expected cursor and next cursor.
It probes an immutable committed-transition receipt by digest before loading
current DID/VDR state. An exact receipt returns replay success without planning,
so duplicate creates or stale predecessors caused by the prior successful
commit cannot mask an ambiguous-response retry.

When the probe is absent, normal planning proceeds and the commit request
carries the same digest. Commit outcomes distinguish applied, exact replay and
cursor conflict. Atomically, the adapter checks exact digest equality first to
close the race between probe and commit, then applies expected-cursor
compare-and-set. Different content or a genuinely stale cursor fails
deterministically and is never automatically retried by the SDK.

## Backpressure, cancellation and retry

There is no internal channel. Public batch and byte limits are the backpressure
contract, so memory is finite and visible to adopters. Entry points are
synchronous and bounded; cancellation is caller-owned between calls. An async
adapter may cancel source/commit futures without the core knowing an executor.
Retry count, delay and finality are likewise adapter policy. The core provides
stable replay semantics so a retry is safe after an ambiguous commit response.

## Error and observability contract

Errors are non-exhaustive stable enums covering bounds, ordering, malformed
operation, missing parent, cycle, depth, cursor conflict and adapter failure.
Adapter errors are mapped to a redacted category. `Debug` reports categories,
counts and bounded positions but not payloads, identifiers or backend text.
Metrics are returned as deterministic counts/durations supplied by the caller;
the crate does not install tracing or a global metrics recorder.

## Conformance adapters

An in-memory lookup/commit adapter demonstrates atomic compare-and-set,
rollback on failure, exact replay and batch-local root resolution. A
NeoPRISM-shaped test adapter maps donor records into SDK inputs without pulling
Cardano, Tokio or SQL into production dependencies.

## Verification

Verification includes donor/spec vectors, malformed/orphan/cycle/depth cases,
order permutations, atomic rollback, replay conflicts, property traces, fuzz
targets for payload and batch boundaries, dependency graph tests, supported
target checks and directional parse/plan/traversal benchmarks. Performance data
is reported but is not a CI threshold without a later stability decision.

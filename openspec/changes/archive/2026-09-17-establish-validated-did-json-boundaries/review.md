# Exact-diff security, API, and architecture review

- **Review date:** 2026-09-17
- **Issues:** #315 and #297
- **Develop base:** `fee94946ca489f88dbc50282b5ee1eca095507f3`
- **Reviewed implementation head:** `4fda3dc53648034d327edf35b13416e5745c4649`
- **Result:** passed with no unresolved blocker

## Scope reviewed

The review re-read the complete 26-path exact diff, all public recursive-JSON
owners in `identus-did`, the public API rendering, the generated SBOM, hostile
depth and cardinality regressions, ADR 0131, the machine inventory, and the
narrowed `SDK-LIM-007` clause. It also searched for unsafe destruction, leaks,
public custom `Drop`, remaining `OneOrMany` consumers, and constructor paths
that validate owned JSON without the shared rejection guard.

## Findings

1. **Representation and domain policy — accepted.** `OneOrMany<T>` now owns
   only non-empty scalar-or-array shape. Every DID consumer retains its
   128-item ceiling in its validated owner: document contexts, controllers,
   service types, and existing document collections.
2. **Invalid-state closure — accepted.** `ContextEntry::Object` can contain
   only opaque `ContextObject`; its private field, fallible construction,
   validated deserialization, and compile-fail evidence prevent a raw map from
   directly entering the public domain enum. No mutable raw-map escape exists.
3. **Rejection ownership — accepted.** One private `RejectionGuard` arms before
   validation and one private iterative worklist dismantles every recursive
   array/object after failure. Success returns the original allocation. The
   complete audited document, resolution/dereferencing, query, and registration
   constructor family uses that mechanism.
4. **Stack and memory safety — accepted.** The implementation contains no
   unsafe code, intentional leak, stack enlargement, or public custom `Drop`.
   Regressions destroy 32,768-level alternating object/array trees through the
   affected families without recursive cleanup.
5. **Accepted-value invariant — accepted.** Successful public recursive values
   remain within the existing depth, node, property, collection, name, and
   string budgets, so their ordinary destruction is bounded. Breadth uses
   proportional worklist memory during rejected cleanup.
6. **Allocation boundary — accepted and disclosed.** Generic Serde or caller
   allocation can occur before typed SDK ownership. Cleanup cannot prevent that
   allocation; hostile byte inputs should continue to use bounded wire-slice
   parsers. The machine inventory and `SDK-LIM-007` retain this outer-owner
   limitation while removing only the native recursive-cleanup clause.
7. **Compatibility — accepted.** Successful JSON serialization and scalar/
   array representation are unchanged. Direct construction of the context
   enum and assumptions that `OneOrMany::try_many` applies DID policy are
   explicit pre-release source migrations for unpublished `0.0.0` crates.
8. **Diagnostics and privacy — accepted.** `ContextObject` debug output exposes
   only property count; the cleanup path never formats values. Existing static
   errors remain unchanged and hostile sentinel content is not reflected.
9. **Dependency and portability boundary — accepted.** There is no manifest,
   feature, lockfile, build-script, native-library, unsafe-policy, or dependency
   change. Rust 1.98.1, WASM, iOS ARM64, and Android ARM64 checks pass.
10. **Downstream convergence — accepted as evidence only.** Midnight Identity
    PR #78 at `2dcece66f17614968ce57d0aaa4786966763911a` independently uses depth
    32 and cardinality 128 for recursive extensions but exposes string-only
    document contexts. It neither constructs the changed inline context variant
    nor authorizes downstream mutation; adoption remains a separate slice.

## Decomposition decision

The exact diff exceeds both preferred thresholds at 26 paths and 1,625 changed
text lines. The implementation was already decomposed into four signed,
reviewable constructor-family commits after a planning-only commit and immutable
preflight receipt. The final PR must nevertheless carry the generic/domain
separation, shared cleanup primitive, complete audited owner family, regression
matrix, ADR, inventory, and canonical limitation transition atomically. Splitting
those parts across independently mergeable PRs would temporarily preserve the
known early-drop hole or claim a guarantee before the full family implements
it. No additional feature decomposition is warranted.

## Residual limitations

- Callers and generic deserializers may allocate input before typed SDK entry.
- `ServiceEndpoint` remains an input representation until `Service::new` or
  validated deserialization establishes the enclosing domain invariant.
- This change does not adopt Midnight Identity PR #78 or alter any downstream
  repository.
- Production promotion, publication, and release still require an unchanged
  candidate to pass the repository slow-line policy.

## Decision

The implementation applies the issue #315 recommendation without reproducing
DID policy inside the generic representation. It is cohesive, stack-safe,
redaction-safe, dependency-neutral, reversible, and ready for protected Linux
`fast` evidence. No unresolved security, API, architecture, portability,
dependency, or governance finding remains.

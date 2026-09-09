# Exact-diff architecture, correctness and security review

Review status: completed
Review date: 2026-09-09
Implementation head: 36f7f864a04d47a9a6505a36148543acc2d7f769
Specification commit: 5a9d4d76d1afe8ac28827d7bcf1202f36350d90f
Unresolved blockers: none

## Scope reviewed

The review inspected the complete
`develop@c04e495da719e1fa09beb77a5a05e744d75395fe...36f7f864` diff,
issue #162, ADR 0103, exact candidate lock, SDK-port adapter fixture,
isolation script, dependency ledgers and local verification receipts.

## Findings

1. **Architecture and cohesion — accepted.** The adapter is an unpublished
   nested research workspace. Askar, SQLite, Tokio/SQLx, candidate types and
   diagnostics do not enter generic crates, the root graph or public APIs.
2. **Exact-record semantics — accepted as spike evidence.** The adapter owns a
   collision-free scope/key mapping and revision envelope, and validates
   conditional writes/deletes under candidate transactions. Shared conformance
   passes all 16 exact-record operations.
3. **Forward-compatible delete matching — resolved.** Initial review found an
   `if let` that would have treated a future non-exhaustive delete condition as
   unconditional. The implementation now matches explicitly and fails closed
   on unknown variants.
4. **Error and input boundary — accepted.** Scope and key inputs are bounded,
   malformed rows become closed SDK integrity errors, and raw backend messages,
   pass keys and caller values are not returned.
5. **Dependency and supply chain — accepted as rejection evidence.** Exact
   0.4.6 with only `sqlite` resolves 192 host normal/build lines and 256 lock
   packages. The top-level crate still pulls every `askar-crypto` key family;
   SQLite also brings native C, SQLx and Tokio.
6. **Advisory nuance — accepted as rejection evidence.** `cargo deny` passes.
   Raw `cargo audit --deny warnings` reports RUSTSEC-2023-0071 for `rsa 0.9.10`
   in the lock, while target-complete inverse-tree inspection returns no path.
   The result is recorded rather than silently suppressed.
7. **Portability — accepted as negative evidence.** The exact graph compiles
   for host, iOS arm64 and Android arm64 with explicit native compilers. WASM
   fails at `getrandom 0.2.17`; the candidate is not portable core storage.
8. **Durability boundary — accepted as rejection evidence.** The process-local
   revision allocator, ephemeral fixed research key and `String` value prove
   interface adaptability only. Restart safety, cancellation/drop recovery,
   durable zeroization, file persistence and custody are deliberately unproved.
9. **Formatting gate — resolved.** The first full Nix run found the fixture
   manifest was not Taplo-formatted. The manifest was formatted and the
   complete local check was rerun.
10. **Delivery scope — accepted.** No production implementation, fast/slow CI,
    support policy, donor or downstream repository changes.

## Residual limitations

- Mobile observations are compile-only; no device/runtime behavior was run.
- Successful and conflict transaction paths are covered, but crash,
  cancellation, process concurrency, migration and recovery are not.
- Encryption-at-rest in an ephemeral fixture is not evidence for OS-backed
  key custody, backup, secure deletion, certification or production support.
- Performance and downstream payoff were not measured because the assessed
  package boundary already fails the production adoption threshold.

## Review decision

The fixture is deterministic, isolated and reversible, and it is sufficient to
classify published `aries-askar 0.4.6` as `not-adopt` for an SDK production
adapter. ADR 0103's reconsideration trigger preserves a path for a future
published, storage-sliced release. No unresolved correctness, architecture,
security, privacy, licensing or delivery finding remains for hosted review.

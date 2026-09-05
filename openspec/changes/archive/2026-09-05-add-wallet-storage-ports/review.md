# Pre-implementation architecture, API, security and performance review

- **Date:** 2026-09-05
- **Issue:** #89 under #20 / `IDR-010`
- **Develop base:** `5141044384e51cc26a73a19adb11d0f18f70f74a`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. Storage is cross-domain orchestration. Putting all ports in DID,
   credentials or presentations would create sideways/reverse dependencies;
   the existing orchestration-layer wallet package is the smallest owner.
2. Activating `identus-wallet` is explicitly authorized by the focused issue
   and remains reversible before release. It does not imply a wallet product.
3. A new `identus-ports` crate would require additional namespace and rulebook
   work without improving this slice. The current dependency cone can remain
   narrower with core plus the build-time port marker only.
4. Associated consumer types let Oxid, Midnight and Lace keep their identifiers
   and records. SDK-owned opaque wallet records would duplicate codecs and
   force product policy into the generic layer.
5. Five separate traits preserve interface segregation. A generic repository
   trait would either expose secret enumeration or make every capability pay
   for index and cache semantics it does not need.
6. Async object-safe methods are necessary for mobile keychains, IndexedDB,
   SQL, files and cloud implementations. Boxed borrowing futures follow the
   existing SDK convention without selecting an executor or macro.
7. Secret enumeration is unnecessary authority and must be absent by type.
   Exact-key secret reads/writes/deletes cover the observed Midnight need.
8. Exact-key status cache access is sufficient for this foundation. Sweeping,
   TTL clocks and eviction belong to adapters or the later status mechanism.
9. Credential, DID and protocol recovery need bounded listing. Consumer-owned
   index entries avoid reading every potentially large or secret-bearing value.
10. A 256-entry page and 1024-byte opaque cursor bound prevent unbounded result
    allocation while accommodating common backend continuation tokens.
11. An empty page with a next cursor is rejected so a generic caller cannot be
    trapped in a non-progress loop without product-specific heuristics.
12. Opaque revisions permit database integers, hashes and ETags. They must be
    non-empty, bounded and redacted because they may correlate wallet state.
13. `InsertOnly` and `IfRevision` make create and compare-and-swap intent
    explicit. A mismatch must fail closed as Conflict rather than overwrite.
14. Cross-record transaction, synchronization and merge policy lack two
    independent common implementations and would overextend this slice.
15. Missing exact reads are ordinary `None`; operational failures use a
    separate static error channel. This prevents absence from being confused
    with backend outage or integrity failure.
16. Access-denied is distinct from unavailable so keychain/OS adapters can
    represent locked or unauthorized access without leaking platform detail.
17. Debug and error implementations must not require value types to implement
    Debug. This structurally prevents generic wrappers from rendering secrets,
    credential bytes or protocol state.
18. Tests may implement memory-backed semantics inside the test target, but a
    production in-memory or encrypted adapter would falsely advance IDR-010
    acceptance without downstream conformance.
19. No donor code or fixtures are needed; only semantic behavior is adapted.
    Lace remains evidence-only because its repository license is unresolved.
20. Dispatch cost should be measured across trait objects, but no host-specific
    performance threshold belongs in correctness CI.

Verdict: READY to implement after ADR 0032 and strict OpenSpec validation pass.

# Post-implementation architecture, API, security and performance review

- **Date:** 2026-09-05
- **Reviewed production head:** `24c383549c48357e09ad6b8dde4235096bba6245`
- **Exact diff:** `develop@51410443...24c38354`
- **Result:** no unresolved finding

## Exact-diff findings

1. The wallet activation is cohesive: production behavior is isolated in one
   `storage` module and the crate root only documents and exports it. No wallet
   product, adapter, persistence format, encryption or custody code entered.
2. The complete runtime cone is `identus-core` plus the workspace's procedural
   port marker in `identus-derive`. No external, executor, async-trait, format,
   DID, credential, protocol, chain or product dependency was introduced.
3. Revisions and cursors reject empty and oversized inputs, preserve exact
   adapter bytes, expose no serialization or ordering contract and render only
   byte length. Page sizes and page construction enforce their 256-entry bound
   and the non-progress invariant.
4. `Stored<T>`, `StorageWrite<T>` and `StoragePage<T>` implement Debug without
   `T: Debug` and omit all values/entries/token bytes. Receipt Debug omits its
   revision bytes. A non-Debug canary proves these properties dynamically.
5. `StorageError` is data-free. Every variant maps to a fixed
   `wallet.storage_*` code, fixed public message and the `wallet.storage`
   capability; no backend cause or caller-owned value can cross the bridge.
6. Each port is a separate explicit top-level `#[identus::port]` trait. The
   source-based conformance guard discovers all declarations, while associated
   types keep consumer records out of the SDK and make fully specified trait
   objects object-safe.
7. `SecretStore` and `StatusCacheStore` expose only load, write and delete.
   Credential, DID and protocol stores alone expose bounded recovery indexes;
   there is no generic repository supertrait or accidental secret enumeration.
8. Conditional test-double behavior fails closed on stale/missing revisions,
   rotates successful write revisions and preserves the record after a failed
   mutation. Unconditional deletion of a missing key reports `NotFound`.
9. The recovery-index test is deterministic, scope-bound, resumable and never
   returns more than requested. Index entries remain independent consumer
   types and no stored value is needed to define the production list API.
10. String-shaped and struct-shaped consumers compile through independent
    dynamic ports. Shared trait objects dispatch concurrently using boxed Send
    futures without adding an SDK executor.
11. The release diagnostic exercised 1,000,000 ready loads across all five
    trait surfaces at approximately 14.3 million calls/s on this host. The
    accepted boxing cost is observable and no machine threshold was added.
12. Focused, native-workspace, factory and all 30 compatible local Nix checks
    passed, including Rust 1.85, WASM, Android, iOS, strict lints, supply-chain
    policy and the 356-test principal release suite. Consumer postflight state
    exactly matches preflight.

## Implementation-review corrections

1. An early local draft generated the five traits through declarative macros.
   That compiled but hid their names from the repository's source-based port
   discovery. It was replaced before the implementation commit with five
   explicit top-level declarations; naming conformance now observes them.
2. The first test executor waker would have unparked the thread performing the
   wake rather than the thread polling the future. It now captures and unparks
   the polling thread, preserving correct behavior if a future becomes pending.

Verdict: READY for specification synchronization and pull-request review.

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

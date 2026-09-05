# Pre-implementation architecture, API, security and performance review

- **Date:** 2026-09-05
- **Issue:** #91 under #20 / `IDR-010`
- **Develop base:** `db83fbc1d7dbe66f7c7a09bbc13a7f606559e67d`
- **Result:** ready to implement with no unresolved blocker

## Findings

1. Comparable downstream evidence requires a reusable executable contract;
   prose-only acceptance would drift between adapters.
2. A separate wallet conformance crate is more cohesive than adding consumer
   runtime dependencies to the repository architecture guard crate.
3. Verification may depend inward on wallet; production crates remain unable
   to depend outward on verification.
4. Direct entry points must compile against all five actual store traits so a
   consumer cannot test a substitute abstraction by accident.
5. Test fixtures may require clone/equality while production values remain free
   of those bounds.
6. Consumer types need no formatting, hashing, ordering or serialization.
7. Static failure kinds prevent secrets, credential bytes, identifiers,
   revisions and cursor tokens from entering test output.
8. Exact-record checks cover the common mutation contract; list checks remain
   separate because secret and status capabilities must not enumerate.
9. Replacement revision invalidation is necessary for compare-and-swap to have
   safety meaning and does not imply global revision uniqueness or order.
10. A preseeded list fixture avoids forcing generic write-to-index policy.
11. Cursor repetition, oversized pages, duplicate entries and nontermination
    must fail with bounded work.
12. No async runtime belongs in the library; public async checks can be awaited
    by Tokio, async-std, WASM or a consumer test executor.
13. A local memory implementation is valid only inside tests and cannot count
    as independent downstream adoption.
14. Encryption-at-rest needs separate threat-model evidence and cannot be
    inferred from generic behavior conformance.
15. The runtime cone can remain exactly `identus-wallet`; standard library
    futures and collections are sufficient.
16. Harness overhead is useful as a release diagnostic, but correctness CI
    must not include a host-dependent duration threshold.

Verdict: READY to implement after strict OpenSpec validation.

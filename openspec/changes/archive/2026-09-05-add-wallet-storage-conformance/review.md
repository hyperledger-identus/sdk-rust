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

# Post-implementation architecture, API, security and performance review

- **Date:** 2026-09-05
- **Reviewed production head:** `d6ff338061a81d394e0fe4af04ae47c9951d03f2`
- **Exact diff:** `develop@db83fbc1...d6ff3380`
- **Result:** no unresolved finding

## Exact-diff findings

1. The new crate is a verification leaf with one direct dependency:
   `identus-wallet`. No database, codec, encryption, async runtime, serde,
   donor, chain or product dependency entered.
2. The repository guard now derives a 15-package workspace and explicitly pins
   `identus-conformance` to core and `identus-wallet-conformance` to wallet.
   Production-to-verification edges remain rejected.
3. All five public checks dispatch the actual production traits. Secret and
   status checks have no list parameter or internal enumeration path.
4. Private driver wrappers share the exact lifecycle without adding a generic
   repository trait to the production storage API.
5. Exact checks cover 16 observable calls, including two post-conflict loads
   that prove rejected stale operations did not mutate current state.
6. Replacement revision equality is checked before stale operations, making
   compare-and-swap invalidation executable without implying revision order.
7. Scope isolation is tested after insert. Missing and clean-state reads make
   fixture preconditions explicit and fail closed on reused namespaces.
8. Public documentation now requires disposable exact-test namespaces because
   generic cleanup cannot safely overwrite state after a contract failure.
9. List fixtures accept only 2 through 4,096 unique expected entries and a page
   size smaller than the fixture, forcing multi-page traversal with bounded
   memory and work.
10. List checks enforce the requested size, cursor non-repetition, a maximum of
    expected-count plus one pages, duplicate rejection and exact set equality.
11. No scope, key, value or index type needs Debug, Display, Hash, Ord or serde.
    Only test values require Clone and equality; index values require equality.
12. Fixture Debug output shows no consumer values. Failure output consists only
    of a static step and closed enum; reports contain two aggregate counts.
13. The local memory implementation is compiled only in the crate test module.
    It is not exported and therefore cannot be mistaken for an SDK adapter.
14. Tests exercise all five entry points through string-shaped adapters, one
    secret trait object, a non-Debug canary, revision reuse and cursor replay.
15. The release diagnostic completed 160,000 port calls in 30.265542 ms,
    approximately 5.29 million calls per second on this host, with no threshold.
16. The first full Nix run caught only root TOML alignment. Taplo corrected it;
    the complete rerun then passed all compatible checks and 362/362 principal
    release tests.

Verdict: READY to synchronize canonical specs and archive the change.

# Pre-implementation semantic, API, privacy, and performance review

- **Date:** 2026-09-05
- **Issue:** #79 under `IDR-008` / #20
- **Develop base:** `9b0d10790b9257f94fc9eecd2af99a82c592ca70`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. Oxid's request/candidate domain is the strongest Rust seed, but its product
   IDs, labels, preview object, age threshold, and lifecycle enum mix reusable
   semantics with wallet policy. Only the bounded request/query/candidate
   intersection is adapted.
2. OpenID4VP Final makes format, unique query IDs, multiplicity, claims, and
   candidate selection real requirements. Its JSON/DCQL metadata, credential
   sets, claim sets, values, and protocol parameters belong to later protocol
   crates rather than this format-neutral core.
3. The credentials crate already owns format, issuer/type/schema descriptors,
   and segmented claim paths. Duplicating them would violate DRY and create
   inconsistent bounds. A one-way same-layer dependency is allowed by the
   crate-ring and preserves cohesion.
4. An unbounded raw request string, as used at the Oxid application port, is a
   transport artifact. The semantic request instead contains bounded validated
   queries and an opaque bounded challenge.
5. A challenge is meaningful across OID4VP nonce and Midnight proof contexts,
   but generation, freshness, replay storage, and comparison are protocol
   behavior. Exact opaque text/bytes preserve both without pretending they are
   interchangeable on the wire.
6. Predicate parameters do not have a shared safe type across DCQL, mdoc, and
   Midnight circuits. The generic core records reveal/predicate intent only;
   each profile retains its typed condition and proof semantics.
7. Optional issuer/type/schema filters are shared structural requirements, but
   matching and trust are not. `None` is the only unrestricted spelling;
   present empty/duplicate lists fail during construction.
8. Candidate construction alone cannot validate cross-query facts. A separate
   candidate-set constructor borrows the request and rejects unknown queries,
   wrong formats, unrequested paths, missing required paths, and duplicate
   query/handle pairs.
9. Candidate paths must be a subset of requested paths. Accepting extra paths
   would make accidental over-disclosure easier even though actual disclosure
   remains outside this slice.
10. Empty candidate sets are valid no-match evidence; empty request query sets
    are invalid. Distinguishing them removes sentinel objects and preserves
    normal search outcomes.
11. Verifiers, purposes, challenges, handles, filters, and paths can correlate
    a person or transaction. Custom aggregate Debug plus static data-free
    errors must avoid recursively formatting imported descriptor contents.
12. Bounds precede all pairwise scans. The largest scan is 64 by 64, small and
    predictable enough to avoid hash-map allocation while the API remains
    unpublished and measurable.
13. Midnight, Lace, Apollo, and NeoPRISM provide compatibility or negative
    evidence but no competing licensed generic Rust presentation core. Their
    repositories remain read-only and no dependency is justified.

Verdict: READY to implement after strict structural validation.

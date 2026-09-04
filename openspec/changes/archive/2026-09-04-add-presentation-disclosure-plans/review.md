# Pre-implementation semantic, API, privacy and performance review

- **Date:** 2026-09-05
- **Issue:** #81 under `IDR-008` / #20
- **Develop base:** `4ca4605a9b0ab9e378943fa5046d691e259efa26`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. A generic plan must describe an already-made choice. Candidate ranking,
   consent presentation and authorization policy are product responsibilities.
2. A selected claim needs both path and reveal-or-predicate intent to be useful
   to proof adapters. Exact validation against the request prevents semantic
   drift while keeping values and format parameters out.
3. Selection must reference candidate handles rather than credentials or
   private material. Existing opaque handles already provide the correct
   storage-neutral boundary.
4. A standalone credential selection cannot prove cross-object consistency.
   The enclosing plan must borrow the request and candidate set.
5. `PresentationCandidateSet` currently validates only during construction and
   does not encode request identity. Reusing a private `validate_against`
   routine in both constructors safely rejects accidental cross-request use.
6. Every current generic request query is required because alternative and
   optional credential sets are not yet modeled. Requiring plan coverage is
   accurate for this capability; DCQL alternatives belong to `IDR-024`.
7. `multiple == false` must mean exactly one selected credential, while true
   means one or more. Candidate sets may still contain many unselected options.
8. Query/handle pair uniqueness allows the same credential to satisfy distinct
   queries without permitting duplicate submissions for one query.
9. Candidate capability and request intent are separate checks. A selected
   path must be both requested with the same intent and listed as satisfiable
   by the chosen candidate.
10. Required claim coverage is conservative per selected credential. It keeps
    every selected proof input independently complete.
11. A query with no requested claims must have no explicit selected claims;
    mandatory format claims remain format-owned.
12. All identifiers and paths can correlate a holder. Aggregate Debug and
    static errors must expose only counts and safe enum values.
13. Bounds of 64 selections and 64 claims precede all scans. Repeated bounded
    slice scans keep the API simple and allocation-conscious; measurement will
    expose any material regression before optimizing.
14. Oxid and Midnight are conceptual and compatibility evidence only. Lace is
    conformance-only, and Apollo/NeoPRISM provide negative boundary evidence.
    No donor dependency or code copy is justified.

Verdict: READY to implement after strict structural validation.

# Post-implementation architecture, API, privacy and performance review

- **Date:** 2026-09-05
- **Reviewed head:** `d5017723735bf37ccb8ce940c96fbe8855545c63`
- **Develop base:** `4ca4605a9b0ab9e378943fa5046d691e259efa26`
- **Result:** no unresolved blocker

## Exact-diff findings

1. The implementation remains format-neutral and storage-neutral. It adds no
   manifest, lockfile, feature, serializer, runtime, chain or product-policy
   dependency.
2. The public surface is limited to bounded selected claims, bounded
   credential selections and a request-validated disclosure plan. Existing
   request and candidate types remain source compatible.
3. Validation binds each candidate set to a private clone of its exact request
   and reuses one private candidate/request consistency routine. A candidate
   set created for another request cannot be smuggled into a plan, and a
   selected query/handle pair must exist in the revalidated set.
4. Query coverage and multiplicity match the declared generic capability:
   every query is covered, single queries select exactly one credential, and
   multiple queries select one or more. The same opaque handle may satisfy
   different query IDs.
5. Claim validation is deliberately fail closed: each selected path is
   requested with the same intent, supported by the candidate and complete for
   required claims. Queries without explicit claim requests reject explicit
   claim selection.
6. Diagnostics reveal counts and stable static error identifiers only. They do
   not format query IDs, credential handles or claim paths.
7. Both collection bounds are enforced before nested scans. The maximum shape
   is approximately 1.3 million bounded equality comparisons, with no I/O or
   cryptography. Before request binding, the release diagnostic measured
   983,298 complete request/candidate/plan validations per second locally;
   after the hosted-review correction it measured 867,814 per second. These
   are evidence, not a portable threshold.
8. Positive tests cover DCQL-shaped, Midnight-shaped and unrelated format
   identifiers. Negative tests cover every new static error, both upper
   bounds, duplicates, cross-request use, unknown pairs, claim mismatches,
   incomplete required claims, query coverage, multiplicity and redaction.
9. Full native and Nix gates found one brittle pre-existing backlog negative
   fixture exposed by moving `IDR-008` from program issue #20 to delivery issue
   #81. The fixture now locates any program-owned `#20` row by contract instead
   of assuming row eight; its six tests and the complete Nix matrix pass.
10. Final donor and consumer receipts equal preflight. No donor repository was
    switched, staged, copied from or modified by this change.

Verdict: READY for specification synchronization and pull-request review.

## Hosted review correction

Hosted review found that structural revalidation alone did not observe changed
issuer, type or schema filters when another request reused query IDs, formats
and claim paths. This was a blocking cross-request integrity gap. Candidate
sets now retain a private exact request snapshot, plan construction rejects any
field mismatch with a new static redacted error, and a regression test covers
the formerly accepted restrictive-filter case. The reported signature/DCO
finding was disproved by `%G? = G` for every branch commit and the green hosted
DCO check.

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

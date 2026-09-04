# Pre-implementation semantic, API, security, and performance review

- **Date:** 2026-09-05
- **Issue:** #77, child of #6 / `IDR-007` and #20
- **Develop base:** `9463f1abfe7d0819c6a5944529fa3c1b8897c1d8`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. Midnight's closed status modes are adapter vocabulary. Copying them would
   exclude W3C and future methods and couple SDK releases to chain evolution;
   method and purpose therefore remain distinct open bounded strings.
2. `statusPurpose` describes why a status entry is processed; an observed
   value describes the method-specific fact. Combining them into a closed
   active/suspended/revoked/expired enum would lose W3C message semantics and
   improperly mix credential validity expiry into status.
3. Reference, handle, revision, and observed value have different roles and
   sensitivity even when represented as text or bytes. Public type aliases to
   one donor enum would permit accidental substitution, so the SDK uses
   role-specific wrappers over one private implementation.
4. W3C status entries may be multiple. A hard 1–16 collection bound supports
   revocation plus suspension and sharded registries while enabling simple
   allocation-free exact-duplicate checks.
5. A partial all-optional binding creates states no reader can execute. Queries
   own one complete binding; no-status is represented before query construction
   by an absent binding collection.
6. Freshness is transportable as a minimum opaque revision and/or maximum age,
   but revision ordering and age evaluation require method and clock adapters.
   The core stores requirements and does not claim to enforce them.
7. An empty present allow-list is ambiguous. Requiring 1–16 unique entries
   leaves `None` as the only spelling of unrestricted and makes configuration
   mistakes fail during construction.
8. A `required` flag and accepted-state list belong to credential-selection and
   usability policy, not an executable query. Omitting them prevents the domain
   shape from silently becoming a trust engine.
9. Arbitrary JSON proof/attestation payloads are unbounded and wire-specific.
   They require a later bounded artifact and verification-port design and are
   deliberately not copied from the donor.
10. References, handles, revisions, and values can correlate holders or reveal
    status. Custom Debug and static errors must protect them both directly and
    through aggregate types.
11. Pairwise scans are suitable only after bounds checks. With at most 16
    entries they avoid temporary set allocations and have deterministic small
    worst-case work.
12. Oxid, Lace, Apollo, and NeoPRISM supply no competing generic credential
    status model at the inspected revisions. Donors remain read-only evidence;
    no dependency or placeholder API is justified.

Verdict: READY to implement after strict structural validation.

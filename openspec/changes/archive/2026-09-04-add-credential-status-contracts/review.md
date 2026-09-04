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

# Post-implementation semantic, API, security, and performance review

- **Date:** 2026-09-05
- **Reviewed implementation:**
  `63ff1284f5cd5f0a7a7ae2bcf26c12359d83fa25`
- **Diff base:** `9463f1abfe7d0819c6a5944529fa3c1b8897c1d8`
- **Result:** accepted with no unresolved finding

## Exact-diff findings

1. The API uses six role-specific public value types over one private opaque
   implementation. No public alias permits a reference, handle, revision, or
   observed value to be substituted accidentally.
2. Method and purpose remain open and case-preserving. The implementation does
   not import the Midnight mode enum, create a universal lifecycle enum, or
   hard-code W3C terms.
3. Binding and query construction exclude incomplete and contradictory states.
   In particular, the reviewed implementation rejects a query whose binding is
   absent from its own method or purpose allow-list.
4. Every text constructor validates borrowed input before allocation; byte and
   collection constructors retain transferred vectors. All duplicate scans run
   only after 16-element bounds and allocate no temporary set.
5. Direct and aggregate Debug implementations were traced recursively.
   Reference, handle, revision, and value contents remain redacted; every new
   error holds no caller data and bridges to a static credential code.
6. Evidence preserves method-specific facts and rejects only a reversed known
   observation/expiry interval. It does not claim freshness, proof validity,
   trust, or usability.
7. The exact diff changes no manifest, dependency, feature, lockfile, unsafe
   code, wire codec, registry/network/ledger port, clock implementation, or
   downstream repository.
8. Boundary, deterministic corpus, W3C/Midnight consumer-shape, query,
   privacy, time, and complete error-contract tests exercise every constructor.
   The configured MSRV, etalon, cross-target, audit, deny, lint, docs, and
   workspace-test gates all passed.
9. The release diagnostic performs the full public query/evidence construction
   path and reported 2,818,146 pairs/second without turning host timing into a
   correctness threshold.

Verdict: no unresolved semantic, API, security, privacy, performance,
portability, or ownership-boundary finding remains.

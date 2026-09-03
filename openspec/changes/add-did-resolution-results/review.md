# Semantic and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #39 (child of #5 / `IDR-005`)
- **Develop base:** `569c0572a75c18c9b3a7fdff8b93e8ec94137d4f`
- **Reviewed contract:** OpenSpec `add-did-resolution-results` and ADR 0010
- **Pre-implementation result:** no unresolved blocker

## Pre-implementation findings

1. **Ownership:** transport-free DID resolution result values are generic SSI
   infrastructure and belong in `identus-did`. Method implementations, ports,
   I/O, source provenance, caching and wallet policy stay outside this slice.
2. **Standards:** the 28 August 2026 W3C Candidate Recommendation Draft uses
   URL-valued RFC 9457-style error objects. Older keyword errors need an
   explicit migration path, not permissive wire ambiguity.
3. **State model:** deactivation is neither success nor ordinary failure. Its
   null-document/no-error/`deactivated: true` state must survive as a deliberate
   constructor and serde invariant.
4. **Volatility:** DID URL dereferencing is at risk. Bounded open JSON content
   plus typed projections avoids prematurely freezing a closed representation.
5. **Trust boundary:** same-method and exact requested/document DID checks are
   generic syntax/integrity rules. Only method adapters can prove equivalence,
   authenticity, authorization or proof validity.
6. **Provenance:** donor repositories supplied compatibility evidence only; no
   source or fixture is copied. Lace remains evidence-only due to unresolved
   repository-level license evidence.
7. **Compatibility:** the API is additive in an unpublished `0.0.0` crate, adds
   no third-party dependency and leaves all downstream repositories unchanged.

## Post-implementation review

Pending implementation and exact-head review.

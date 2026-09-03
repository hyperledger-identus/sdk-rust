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

The complete `origin/develop...df5fb16` implementation diff was reviewed after
the focused suite passed and before final evidence was written.

1. **Construction integrity:** every public field is private. Native
   constructors and custom serde implementations converge on the same scalar,
   extension, state and aggregate-budget checks.
2. **State correctness:** success, ordinary failure and deactivation are
   mutually exclusive. Required nullable document/content members remain
   distinguishable from omitted wire members.
3. **Error compatibility:** all nine current W3C URLs classify without closing
   the extension space. Strict serde accepts only an RFC 9457-style object;
   legacy keywords require the explicit migration helper.
4. **Identifier correctness:** successful native and serde construction checks
   canonical/equivalent method names against the document subject. The
   request-aware entry point also rejects a returned document-id mismatch and
   validates metadata for document-less deactivation.
5. **Open-content safety:** known DID document, verification method, service
   and URI projections reuse their established validators. Unknown non-null
   JSON remains lossless and bounded; native bytes are not silently encoded.
6. **Resource behavior:** raw envelopes, purpose-specific strings, collections,
   maps, names, depth and aggregate nodes are bounded. Metadata maps that pass
   separately cannot exceed the shared budget after composition.
7. **Dependency and repository boundary:** `Cargo.lock` and crate manifests are
   unchanged. No chain, async, HTTP, datetime or donor dependency was added and
   no downstream repository was modified.

## Corrections made during review

- Made nullable `didDocument` and `content` wire fields required so omission
  cannot be confused with an explicit JSON null.
- Shared one aggregate JSON budget across composed result metadata/content
  after a regression demonstrated how per-map budgets could otherwise add up.
- Kept the current serialized `didUrlDereferencingMetadata` field spelling and
  rejected the older abstract-name spelling in conformance tests.
- Used Rust 1.85-compatible leap-year arithmetic before running the MSRV lane.

## Deliberate quality boundary

This slice stops at the requested 75% maturity point. Follow-up #41 owns CR/test
suite drift, resolution fuzz/property testing, duplicate raw JSON keys, the full
XML Schema datetime decision, native byte-stream binding and calibrated
performance baselines. Issues #35 and #38 own existing DID syntax/document
hardening; #10 owns resolver ports and HTTP behavior.

# ADR 0025: bound credential metadata and schema descriptors

- **Status:** Accepted
- **Date:** 2026-09-05
- **Issue:** #75
- **Decision scope:** second `IDR-007` credential-semantics slice

## Context

The SDK can preserve opaque credential artifacts and report verification
evidence, but its consumers still duplicate issuer/subject/type/validity and
claim/schema descriptor models. Midnight has the richest reusable claim-schema
shape, Oxid has holder-facing metadata, and Lace proves another family/schema
dialect. Copying any one model would also copy format, product, wire, or
localization choices.

## Decision

1. Add metadata and schema descriptors to the existing experimental
   `identus-credentials` crate without new dependencies or wire types.
2. Use role-specific bounded scalar newtypes sharing one private validation
   implementation. Accept exact trimmed control-free UTF-8; let adapters apply
   URL, DID, SemVer, and format-specific grammar.
3. Represent claim paths as 1–16 validated segments and disclosure as Public,
   Selective, Committed, or PredicateOnly. Descriptors contain no claim value.
4. Bound schemas to 16 unique types and 64 unique claim IDs/paths. Bound
   metadata to 16 subjects, types, and schemas, and reject reversed validity.
5. Check collection bounds before allocation-free pairwise duplicate scans.
   Retain caller-owned vectors rather than copying or building registries.
6. Keep metadata separate from opaque envelopes until a later parsed-state
   contract can bind the two without implying cryptographic verification.
7. Redact entity identifiers from Debug and every caller value from errors.
   Keep display/localization, extraction, codecs, schema execution, trust,
   holder/status binding, storage, and consumers outside this slice.

## Consequences

- Midnight, Lace, Oxid, W3C, and future format adapters can target one small
  structural API while preserving their stronger profile rules downstream.
- Generic construction proves only bounded internal consistency. It does not
  prove that metadata came from, describes, or is secured by any credential.
- The vector bounds make simple pairwise uniqueness checks predictable and
  allocation-conscious; a future larger profile would need a separate design.
- Sub-millisecond and pre-epoch wire timestamps remain adapter-owned because
  this normalized indexing view reuses unsigned Unix milliseconds.
- The additive API remains reversible before publication; downstream adoption
  and donor reduction require separate issues and immutable candidates.

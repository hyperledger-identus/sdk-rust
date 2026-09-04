# ADR 0026: use open bounded credential status contracts

- **Status:** Accepted
- **Date:** 2026-09-05
- **Issue:** #77
- **Decision scope:** third `IDR-007` credential-semantics slice

## Context

The generic credential crate needs reusable status bindings and query/evidence
shapes. Midnight has a useful plain-data model, but directly upstreaming its
closed modes, unbounded aliases, serde representation, arbitrary JSON evidence,
and partially known query would make the SDK chain- and wire-aware. W3C status
credentials allow extensible methods and purposes and more than one status
entry, so a fixed global lifecycle vocabulary is also insufficient.

## Decision

1. Add status semantics to the existing experimental `identus-credentials`
   crate without dependencies, serde, ports, wire forms, or infrastructure.
2. Model method and purpose as distinct open strings bounded to 256 bytes.
3. Model reference, handle, revision, and observed value as distinct bounded
   text-or-byte types. Preserve exact data through explicit accessors and redact
   all contents from Debug/errors.
4. Require complete method/purpose/reference/handle bindings. Represent absence
   outside the value and allow 1–16 exact-unique bindings per collection.
5. Represent freshness as one or both of an opaque minimum revision and maximum
   `DurationMillis` age. Keep revision comparison and clock selection in
   adapters.
6. Use optional non-empty, unique, bounded method/purpose allow-lists for query
   requirements. Query existence already requires a binding; required-status
   selection and accepted-value decisions remain verifier/product policy.
7. Attribute evidence to a complete binding, preserve an open value and
   optional revision/timestamps, and reject only a reversed known time range.
8. Check bounds before allocation-free pairwise duplicate scans, retain
   caller-owned vectors, and measure the release construction path without a
   timing assertion.

## Consequences

- Midnight, W3C Bitstring Status List, SD-JWT VC, and later status adapters can
  share one small domain API without expanding a central enum.
- The generic model preserves facts but cannot decide whether an arbitrary
  status value permits credential use. That decision remains purpose/method and
  product-policy specific.
- Minimum revisions are transportable but deliberately not generically
  comparable; a method adapter owns ordering semantics.
- Multiple entries and registries remain expressible, while incomplete and
  unbounded query shapes are excluded.
- Proof/attestation payloads, status retrieval/verification ports, concrete
  adapters, and downstream adoption require follow-up issue-first slices.

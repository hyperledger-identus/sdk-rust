# ADR 0029: correlate generated presentations with disclosure plans

- **Status:** Accepted
- **Date:** 2026-09-05
- **Issue:** #83
- **Decision scope:** third `IDR-008` presentation-semantics slice

## Context

The SDK validates presentation requests, candidates and disclosure plans, but
format adapters cannot return generated presentation bytes through a shared
bounded contract. OpenID4VP maps query IDs to one or more format-specific
presentations, and VCDM permits a presentation to aggregate several
credentials. A generic result also needs enough semantic information for a
wallet to create a disclosure receipt after its own completion policy succeeds.

## Decision

1. Make a disclosure plan privately retain its exact presentation request and
   reject later pairing with any unequal request.
2. Add an opaque presentation artifact with one format, 1–64 unique
   query-ID/credential-handle bindings and 1 byte–4 MiB of caller-owned data.
3. Permit one artifact to bind several plan selections of the same format so
   one-to-one and aggregated presentations share the same API.
4. Add a generated presentation containing 1–64 artifacts and at most 16 MiB
   total. Require every plan selection exactly once, reject unknown or repeated
   bindings, and require every artifact format to match its bound queries.
5. Derive a value-free receipt input in disclosure-plan order. Retain verifier,
   optional purpose, opaque handles, query IDs, formats and selected path/
   intent pairs; omit challenge and artifact bytes.
6. Treat receipt input as generation evidence only. Consent, transport success,
   verifier receipt/acceptance, timestamps, outcomes, storage and retention are
   consumer responsibilities.
7. Use checked byte totals, bounded slice scans and static redacted errors. Add
   no codec, serializer, map, set, hashing contract or dependency.

## Consequences

- OID4VP, Midnight, Oxid and future adapters gain one format-neutral output
  boundary without a chain, product, proof or wire dependency.
- Aggregated and per-credential presentations are both expressible, while each
  selected credential remains exactly accounted for.
- Receipt inputs remain useful owner-private data and must be protected by
  downstream access-control, minimization, retention and deletion policy.
- Exact request clones add bounded memory but prevent collision or lifetime
  coupling and preserve changed verifier/challenge/filter detection.
- Format codecs, VP Token grouping, proof verification, protocol lifecycle,
  delivery receipts, persistence, FFI, adoption and release remain follow-up
  work.

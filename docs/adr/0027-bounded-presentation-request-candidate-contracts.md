# ADR 0027: use bounded presentation request and candidate contracts

- **Status:** Accepted
- **Date:** 2026-09-05
- **Issue:** #79
- **Decision scope:** first `IDR-008` presentation-semantics slice

## Context

The SDK has reusable credential descriptors but no presentation domain API.
Oxid contains a useful holder-side model, while OpenID4VP Final requires
format-aware queries, requested claims, candidate selection, and replay
binding. Copying Oxid's labels, preview, threshold, state enum, application
ports, or raw request string would import product and protocol choices. Copying
DCQL wire objects would instead collapse the generic and protocol layers.

## Decision

1. Replace the unpublished `identus-presentations` marker with a first bounded
   request/query/candidate slice.
2. Add a one-way workspace dependency on `identus-credentials` and reuse its
   format, entity, type, schema, and claim-path values.
3. Use role-specific query ID, purpose, opaque challenge, and opaque local
   credential-handle values with fixed size/grammar checks and redacted Debug.
4. Represent a requested claim as one segmented path, reveal-or-predicate
   intent, and required flag. Keep values, labels, retention, predicate
   parameters, and proof syntax in their owning formats/profiles.
5. Represent a credential query as one ID/format, multiplicity and holder-
   binding requirements, optional non-empty unique issuer/type/schema filters,
   and up to 64 unique claim paths.
6. Represent a request as one verifier, optional purpose/challenge, and 1–16
   ordered queries with unique IDs.
7. Represent a candidate as query ID, opaque handle, format, and zero to 64
   unique satisfiable requested paths. Validate sets of zero to 64 candidates
   against the request for query membership, format agreement, requested-path
   scope, required-path coverage, and unique query/handle pairs.
8. Check bounds before allocation-free pairwise scans, retain transferred
   vectors, expose only static `presentation.*` errors, and measure the release
   construction path without a timing threshold.

## Consequences

- Midnight, Oxid, OpenID/DCQL, and future format adapters can share one small
  request and candidate boundary without introducing chain or product edges.
- The new same-layer dependency prevents duplicate credential descriptor
  vocabularies; dependency direction remains acyclic and inward.
- Candidate sets provide structural consistency, not credential existence,
  verification, trust, disclosure capability, or consent evidence.
- DCQL codecs/complete query semantics, selection/disclosure plans,
  presentation artifacts/receipts, protocol lifecycle, ports, persistence,
  FFI, downstream adoption, publication, and donor reduction require focused
  follow-up issues.

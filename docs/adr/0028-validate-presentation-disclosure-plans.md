# ADR 0028: validate presentation disclosure plans against requests and candidates

- **Status:** Accepted
- **Date:** 2026-09-05
- **Issue:** #81
- **Decision scope:** second `IDR-008` presentation-semantics slice

## Context

The SDK now models bounded presentation requests and structurally matching
credential candidates. Proof adapters still need a common input describing
which candidates and claims were selected. Importing Oxid consent state would
move product policy upstream; importing DCQL wire selection would couple the
generic model to one protocol; passing unrelated IDs and paths would leave
over-disclosure and mismatch errors to every adapter.

## Decision

1. Add a value-free selected claim containing a credential claim path and the
   existing reveal-or-predicate intent.
2. Add a bounded credential selection containing one query ID, one opaque local
   credential handle and zero to 64 unique-path selected claims.
3. Add a disclosure plan containing 1–64 credential selections and validate it
   against both a presentation request and candidate set.
4. Revalidate candidates against the supplied request rather than embedding a
   request fingerprint or trusting construction history.
5. Require every selection pair to be available, every selected path and
   intent to be requested, every path to be satisfiable, and every required
   claim to be selected.
6. Require at least one selection per request query and exactly one when the
   query disallows multiple credentials. Permit one handle under distinct query
   IDs.
7. Treat zero explicit request claims as zero explicit selections; formats own
   any mandatory disclosure surface.
8. Bound collections before allocation-free slice scans, retain transferred
   vectors, add only static redacted errors, and measure the release path
   without a machine-dependent threshold.

## Consequences

- OID4VP, Midnight, Oxid and future proof adapters receive one structurally
  validated, format-neutral input.
- The SDK does not decide consent, ranking, trust, optional alternatives or
  proof feasibility and does not carry private claim material.
- Candidate validation repeats bounded work at plan construction, avoiding a
  hidden identity or hashing contract.
- DCQL claim/credential sets, generated presentation/receipt values, lifecycle
  state, proof execution, storage, FFI, downstream adoption and release remain
  focused follow-up work.

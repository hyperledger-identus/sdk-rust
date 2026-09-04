## Context

IDR-007 requires reusable credential, claim, proof, holder-binding, schema,
status, and private-material foundations. Issues #71, #73, and #75 delivered
the artifact, verification-report, and metadata/schema slices. The next
cohesive missing seam is credential status.

The implementation base is
`develop@9463f1abfe7d0819c6a5944529fa3c1b8897c1d8`. Issue #77 records exact
donor revisions, dirty-state isolation, paths, digests, license evidence, and
negative searches. W3C VC Data Model 2.0 and Bitstring Status List v1.0
Recommendations dated 15 May 2025 are the normative generic-model references.
All donor and consumer repositories remain read-only.

## Goals / Non-Goals

**Goals:**

- represent Midnight, W3C, SD-JWT VC, and future status methods through one
  small open vocabulary;
- distinguish method, purpose, location, entry handle, revision, and observed
  value roles in the type system;
- support multiple complete purpose-specific bindings per credential;
- make size, uniqueness, freshness-shape, privacy, and evidence-time invariants
  facts of construction;
- retain caller-owned allocations and keep bounded duplicate checks simple.

**Non-goals:**

- serde or any JSON/CBOR/JWT wire model; status-list decoding; registry,
  network, ledger, reader/writer/verifier, proof, or attestation ports; clocks;
  trust or usability decisions; arbitrary JSON payloads; persistence; FFI;
  format adapters; publication; downstream migration; donor reduction.

## Decisions

### D1 — Open status method and purpose identifiers

`CredentialStatusMethod` and `CredentialStatusPurpose` are distinct owned
strings. Each accepts exact, trimmed, control-free UTF-8 up to 256 bytes and
validates before allocating. The SDK does not define closed enums for Midnight
modes, W3C entry types, or purpose values. Concrete adapters enforce their URL,
term, profile, or chain grammar and map names into these generic values.

This deliberately does not copy Midnight's `StatusMode` / `EnabledStatusMode`.
W3C permits purpose-specific status processing and extensible strings, while a
credential can carry multiple status entries. An SDK enum would turn each new
method or purpose into a core release.

### D2 — Role-specific bounded opaque values

Reference, handle, revision, and observed value are separate public types even
though each privately stores text or bytes. This prevents accidental role
substitution and avoids the weak aliases in the donor model. Text is non-empty,
trimmed, control-free UTF-8; bytes are non-empty. Reference is capped at 2,048
bytes, handle at 1,024, and revision/value at 256. Explicit `as_text` and
`as_bytes` accessors preserve exact accepted data. Debug reports variant and
length only.

Revision is intentionally opaque. Numeric ordering, ETags, timestamps, ledger
versions, and commitments have method-specific semantics. A query may request
a minimum revision, but only its adapter can interpret that request.

### D3 — Complete bindings and bounded multiplicity

A `CredentialStatusBinding` always contains method, purpose, reference, and
handle. There is no `None` variant and no partially known query shape: absence
is represented by absence of the binding collection. A
`CredentialStatusBindings` owns 1–16 complete bindings and rejects exact
duplicates after checking the bound. Distinct handles, references, methods, or
purposes remain valid, supporting multiple W3C entries and registry shards.

The collection retains its caller-owned vector. At a maximum of 16 elements,
an allocation-free pairwise duplicate scan is simpler and more predictable
than a temporary hash set.

### D4 — Freshness is a requirement value, not a policy engine

`CredentialStatusFreshness` carries an optional minimum opaque revision and
optional maximum age using `identus_core::DurationMillis`. At least one
criterion is required; the absent freshness option means no generic freshness
constraint. It does not read a clock or compare revisions.

`CredentialStatusRequirements` carries optional non-empty bounded unique
allow-lists for methods and purposes plus optional freshness. `None` means no
generic restriction; `Some(empty)` is rejected rather than overloaded as a
second spelling of unrestricted. It intentionally omits Midnight's `required`
flag and accepted lifecycle states: query existence already requires a binding,
while credential selection and usability decisions belong to verifier/product
policy. Query construction also rejects a complete binding excluded by its own
allow-lists, so contradictory executable-shaped state cannot escape.

### D5 — Queries and evidence remain attributable

A `CredentialStatusQuery` owns one complete binding and one requirements value.
`CredentialStatusEvidence` owns the complete binding it describes, one open
observed value, optional revision, and optional Unix-millisecond observation
and expiry timestamps. If both timestamps exist, observation must not follow
expiry. No constructor claims cryptographic verification, freshness, trust, or
credential usability.

Evidence deliberately excludes the donor's arbitrary JSON payload. Proof and
attestation artifacts need their own bounded, format-owned envelope when a
reader/verifier-port slice is designed.

### D6 — Redacted diagnostics and stable errors

References, handles, revisions, and values can reveal registry locations,
correlating indices, versions, or subject status. Their Debug output exposes
only kind/length; aggregate Debug output exposes method/purpose and structural
presence/counts but never opaque contents. Every failure maps to a static
`credential.*` code and text without caller data.

### D7 — Measure constructors without timing correctness

An ignored release diagnostic constructs representative W3C- and
Midnight-shaped values through the public API and reports throughput.
Correctness gates enforce bounds and invariants only; machine wall time is an
engineering receipt, not a flaky pass threshold.

## Risks / Trade-offs

- Generic method/purpose validation accepts spellings a concrete profile later
  rejects. This is deliberate separation between structural construction and
  profile conformance.
- Exact duplicate detection does not forbid two entries with the same method
  and purpose but different handles or references. Such entries are valid and
  potentially necessary; selection remains adapter/policy-owned.
- An opaque minimum revision cannot be compared generically. Encoding an
  ordering here would privilege the Midnight donor's `u64` shape and exclude
  other version schemes.
- Evidence does not decide active/revoked/suspended. Status values and their
  effect are method- and purpose-specific, and arbitrary message purposes make
  a universal closed lifecycle enum lossy.
- Pairwise allow-list/binding scans are quadratic but capped at 16 before the
  scan and allocate no temporary collections.

## Migration Plan

1. Land issue #77, this OpenSpec change, ADR 0026, and pre-implementation
   semantic/API/security/performance review.
2. Implement the module, errors, exports, tests, and release diagnostic.
3. Run focused/full gates and a fresh exact-diff review.
4. Synchronize canonical specs, archive the change, and deliver a signed/DCO
   issue-linked PR to `develop`.

Rollback is a focused revert while unpublished. Reader/verifier ports, wire
codecs, concrete adapters, downstream adoption, and donor reduction remain
separate follow-up issues.

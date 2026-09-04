## Context

IDR-007 requires a format-neutral credential, claim, proof, holder-binding,
schema, status, and private-material foundation. Issue #71 delivered only the
opaque envelope and issue #73 independently delivered verification evidence.
The next missing reusable seam is descriptive metadata and claim/schema shape.

The implementation base is
`develop@9aa33fcf8d2a6755270e46e435beae25f9ca75cf`. Exact donor revisions,
paths, digests, licenses, and conformance-only classifications are recorded in
#75. W3C VC Data Model 2.0 Recommendation (15 May 2025) is the normative model
reference. All donor and consumer repositories remain read-only.

## Goals / Non-Goals

**Goals:**

- give format adapters a small normalized metadata projection;
- preserve Midnight claim/schema semantics without importing its wire model;
- support W3C/SD-JWT and non-W3C identifiers without a central registry;
- make collection, duplicate, path, privacy, and validity invariants facts of
  construction;
- keep hot construction bounded and avoid duplicate-check allocations.

**Non-goals:**

- claim values, JSON-LD contexts, JSON/CBOR/JWT/mdoc codecs, serde, media types,
  schema resolution/execution, credential verification, trust, holder/status
  bindings, display/localization, storage, FFI, publication, or adoption.

## Decisions

### D1 — One credential-semantics crate, separate modules

Metadata and schema descriptors belong beside the existing envelope and report
inside `identus-credentials`. A new crate or dependency edge would separate
types that are always consumed together without isolating infrastructure or a
different lifecycle. The modules remain private and expose only deliberate
root re-exports.

### D2 — Role-specific scalar newtypes with one grammar

Entity identifiers, credential types, schema IDs/versions, claim IDs/value
types, and path segments remain distinct public types even though they share a
private validator. This prevents accidental role substitution while keeping
validation DRY.

The accepted grammar is deliberately not a URL, DID, JSON Pointer, or SemVer
parser: non-empty UTF-8, exact spelling, no surrounding whitespace/control,
and a role-specific byte bound. W3C adapters still enforce URL and
dateTimeStamp rules; Midnight can retain local family identifiers and Lace can
retain `digital-passport:v1` / `1.0`. A central enum or global schema registry
would incorrectly make the SDK own format namespaces.

### D3 — Segmented claim paths

Claim paths are 1–16 `CredentialClaimPathSegment` values rather than raw JSON
Pointer or dotted strings. An adapter can represent JSON object keys, CBOR map
labels, or circuit fields without escaping one format through another. Path
segments and IDs describe locations only; no claim value crosses this API.

### D4 — Bounded schema with invariant-preserving construction

A schema owns one ID, optional version, 1–16 credential types, and 0–64 claims.
The constructor first checks collection sizes and then uses bounded pairwise
comparisons for duplicate type, claim-ID, and complete-path detection. At these
small maxima, O(n²) comparison avoids a second allocation and hash behavior;
the worst case is fixed and measured. Vectors are retained rather than copied.

Disclosure has the semantic intersection of the Midnight and Oxid evidence:
Public, Selective, Committed, PredicateOnly. Stable spellings and reverse
parsing are required, but no serde/wire spelling is promised.

### D5 — Normalized metadata is descriptive, not verified

Metadata owns one issuer, up to 16 optional subjects, 1–16 credential types,
up to 16 schemas, and optional Unix-millisecond bounds. Subject IDs are
optional because W3C allows unidentified and multiple subjects. Schema lists
are optional because many credential formats do not carry a schema reference.
Duplicate subjects/types/schema IDs and reversed time intervals are rejected.

The model does not attach itself to `CredentialEnvelope` in this slice. That
avoids creating a false cryptographic association between arbitrary payload
bytes and caller-supplied metadata. Format adapters can return both values in
their own parsed/verified-state contract later.

### D6 — Privacy-safe diagnostics and static errors

Entity identifiers are correlating data. Their scalar Debug output reports
only length, and `CredentialMetadata` Debug reports counts and validity
presence without issuer or subjects. Schema/type descriptors may be public,
but all construction errors remain static and never retain caller input.

Errors distinguish invalid descriptor text, invalid collection size,
duplicates, invalid claim paths, invalid disclosure names, and invalid
validity ranges. Each bridges through the existing `credential` capability.

### D7 — Measure the real constructor without timing correctness

A manual ignored release test constructs representative metadata via public
parsers and constructors and reports operations per second. Correctness gates
assert bounds and invariants only; host-dependent wall time is evidence, not a
threshold.

## Risks / Trade-offs

- The common string grammar accepts identifiers that a concrete profile later
  rejects. That is intentional: parsed generic metadata is not wire/profile
  conformance.
- Pairwise duplicate scans are quadratic, but every collection is capped at 64
  or 16 before scanning and the path comparison is bounded. This is simpler
  and allocation-free compared with temporary sets.
- `UnixTimestampMillis` cannot preserve sub-millisecond or pre-epoch source
  dates. Adapters needing lossless wire values retain them outside this
  normalized indexing view; this API makes no codec promise.
- Metadata and envelope remain separate until a parsed-state contract can bind
  them without implying verification.

## Migration Plan

1. Land this contract, ADR, and pre-implementation review.
2. Implement scalar, claim/schema, metadata, error, and export surfaces.
3. Add consumer-shaped, negative, privacy, and performance evidence.
4. Run focused/full gates and a fresh exact-diff review.
5. Sync canonical specs, archive, and deliver a signed/DCO PR to `develop`.

Rollback is a focused revert while unpublished. Format adapters, extraction,
wire forms, downstream adoption, and donor-code deletion remain separate
issue-first work.

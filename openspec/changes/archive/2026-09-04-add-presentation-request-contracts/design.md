## Context

IDR-008 requires a generic presentation request, query, disclosure, candidate,
artifact/receipt, and protocol-state foundation. This first slice replaces the
placeholder only with request/query/candidate semantics. The implementation
base is `develop@9b0d10790b9257f94fc9eecd2af99a82c592ca70`, and issue #79
records immutable donor revisions, paths, hashes, license limits, and consumer
dirty-state receipts.

OpenID for Verifiable Presentations 1.0 Final (9 July 2025) is the normative
query/candidate reference. W3C VC Data Model 2.0 supplies the broader credential
and presentation semantics. Oxid's Rust presentation domain is the strongest
implementation seed. Midnight supplies claim-capability compatibility evidence;
Lace supplies OID4VP verifier-side shape only because its repository-level
license metadata is unresolved. No donor source is copied verbatim.

## Goals / Non-Goals

**Goals:**

- give protocol and chain adapters one bounded semantic request and candidate
  boundary;
- reuse accepted credential descriptors instead of introducing parallel
  format, issuer, type, schema, or claim-path models;
- make malformed, duplicate, cross-query, wrong-format, over-disclosing, and
  required-claim-incomplete candidate states fail at construction;
- preserve opaque challenges and local credential handles without exposing
  them through diagnostics;
- keep validation allocation-conscious and measurable.

**Non-goals:**

- DCQL or OID4VP wire models; complete DCQL metadata, values, claim-set, or
  credential-set semantics; verifier authentication; trust; candidate lookup;
  ranking; consent; optional-claim selection; claim values; predicate
  parameters; proof or holder-binding execution; generated presentations;
  receipts; lifecycle state; persistence; FFI; chain/product code;
  publication; downstream adoption.

## Decisions

### D1 — Presentation semantics reuse credential descriptors

`identus-presentations` depends on the sibling `identus-credentials` crate and
uses its `CredentialFormat`, `CredentialEntityId`, `CredentialType`,
`CredentialSchemaId`, and `CredentialClaimPath` values. The crate-ring permits
an inward same-layer edge, and one canonical descriptor vocabulary prevents
translation drift between credential inventory and presentation selection.

The dependency is intentionally one-way. Credential semantics never depend on
presentations, and protocol crates may depend on both.

### D2 — Role-specific bounded scalar values

`PresentationQueryId` is an exact, case-preserving ASCII token of 1–128 bytes:
an alphanumeric first byte followed by alphanumeric, `.`, `_`, `-`, or `:`.
`PresentationPurpose` is exact trimmed, control-free UTF-8 of 1–2,048 bytes.
`PresentationChallenge` and `PresentationCredentialHandle` each preserve exact
text or bytes of 1–1,024 bytes. Text is trimmed and control-free. Validation
precedes the one successful string allocation; transferred byte vectors are
retained.

Query IDs, purposes, challenges, handles, verifier IDs, and aggregate claim
paths can be correlating or sensitive. Their custom Debug representations show
only structural kind, length, counts, booleans, and safe enum/format values.

### D3 — Claim requests describe intent, not proof syntax

`PresentationClaimRequest` contains one credential claim path, a fixed generic
intent (`reveal` or `predicate`), and a required flag. A query may contain zero
to 64 requests with unique complete paths. Zero means the format's mandatory
presentation surface only; it does not mean disclose every selectively
disclosable claim.

Predicate kind, threshold, comparison value, circuit input, DCQL `values`, and
mdoc retention flags remain profile/format inputs. A later disclosure-plan
slice may add bounded selected options without changing the basic request.

### D4 — Credential queries carry only reusable filters

`PresentationCredentialQuery` owns one query ID and one credential format,
plus the `multiple` and `requires_holder_binding` flags. Optional issuer, type,
and schema allow-lists each mean unrestricted when absent and contain 1–16
unique values when present. Their exact matching and trust meaning belongs to
the candidate source; the core only preserves an unambiguous bounded shape.

This intentionally does not model arbitrary format metadata. DCQL codecs and
format-specific crates translate their typed metadata separately.

### D5 — A request is a bounded unique query envelope

`PresentationRequest` contains one verifier entity, optional purpose, optional
challenge, and 1–16 credential queries with unique IDs. It does not contain
transport, response URI/mode, client metadata, origin, protocol state, or
authorization policy. A challenge is opaque input; generation, entropy,
freshness, replay storage, and semantic comparison remain protocol concerns.

### D6 — Candidate sets validate against the request

A `PresentationCredentialCandidate` contains one query ID, one local opaque
credential handle, one format, and zero to 64 unique claim paths that it can
satisfy for this request. `PresentationCandidateSet::new` accepts zero to 64
candidates and validates each against the request:

- the referenced query exists;
- the format equals that query's format;
- every candidate path was requested by that query;
- every required request path is covered;
- no query-ID/credential-handle pair is duplicated.

The set does not prove that the credential exists, filters truly match, a
claim can be disclosed, or the holder consented. Candidate discovery and
selection remain ports/policy in later slices. Allowing an empty set represents
a valid no-match result without a sentinel candidate.

### D7 — Bounded pairwise scans favor predictable small work

All collection bounds are checked before duplicate, membership, and subset
scans. At maxima of 16 queries/filters and 64 claims/candidates, allocation-
free pairwise scans are simpler than temporary sets and retain caller-owned
vectors. An ignored release diagnostic measures representative request and
candidate validation without a machine-dependent pass threshold.

### D8 — Stable presentation errors remain static

Every failure is a data-free `PresentationError` variant mapped to capability
`presentation`, kind `InvalidInput`, and a stable `presentation.*` code/text.
No rejected input, verifier, query ID, challenge, handle, filter, or path is
stored in an error.

## Risks / Trade-offs

- Generic descriptor parsing accepts identifiers that a protocol profile later
  rejects. Format/protocol adapters retain stricter conformance checks.
- Issuer/type/schema allow-lists carry requirements but do not perform matching;
  this keeps storage, trust, and format interpretation outside the core.
- A candidate's claimed satisfaction is structural evidence from its producer,
  not cryptographic proof. The name and docs avoid validity language.
- Reveal/predicate is intentionally smaller than DCQL and Midnight predicate
  syntax. It preserves the shared intent while leaving non-shared parameters
  typed by their owning profile.
- Pairwise scans are quadratic but tightly bounded before execution and avoid
  extra allocations on the presentation hot path.

## Migration Plan

1. Land issue #79, this contract, ADR 0027, and a pre-implementation semantic,
   API, privacy, and performance review.
2. Replace the marker with implementation, tests, inventory, and diagnostic.
3. Run focused/full gates and a fresh exact-diff review.
4. Synchronize canonical specs, archive the change, and deliver a signed/DCO
   issue-linked PR to `develop`.

Rollback is a focused revert while unpublished. Disclosure selection,
presentation artifacts/receipts, lifecycle state, protocol codecs/engines,
storage ports, downstream adoption, and donor reduction remain follow-up work.

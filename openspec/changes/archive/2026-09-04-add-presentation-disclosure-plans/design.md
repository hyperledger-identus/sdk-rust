# Design: validated presentation disclosure plans

## Context

Issue #81 continues `IDR-008` from the request/candidate API merged at
`develop@4ca4605a9b0ab9e378943fa5046d691e259efa26`. OpenID4VP 1.0 Final makes
credential and claim selection an explicit holder-side step and forbids
unselected selectively disclosable claims. Oxid has useful request, preview and
disclosure vocabulary, while Midnight describes public, selective, committed
and predicate-only claim capabilities. Both also contain product, wire or
proof-specific concepts that cannot enter the generic SDK.

The new API represents the result of a choice. It does not make the choice.
This distinction lets product policy remain downstream while format and proof
adapters consume a validated common input.

## Provenance and isolation

No donor code or fixture is copied. The exact revisions, paths, SHA-256
digests, licenses and pre-existing dirty states are recorded in issue #81.
Oxid, midnight-identity, Lace ID Portal, NeoPRISM and Apollo remain read-only.

## Decisions

### D1 — Preserve selected claim intent explicitly

`PresentationSelectedClaim` owns a credential claim path and the existing
reveal-or-predicate intent. Repeating the intent makes a credential selection a
useful proof-adapter input without requiring the adapter to reconstruct it.
Plan validation requires an exact path-and-intent match with the request, so
the repetition cannot silently change semantics.

Claim values, commitment openings, predicate parameters and format-specific
proof inputs remain downstream. Paths are unique within a selection even if a
caller tries to attach two intents to the same path.

### D2 — A credential selection is a bounded reference, not evidence

`PresentationCredentialSelection` owns one query ID, one opaque credential
handle and zero to 64 selected claims. It is constructible before cross-object
validation so adapters can assemble input incrementally, but its documentation
does not call it authorized, available or valid. The enclosing disclosure plan
establishes only structural consistency.

### D3 — A disclosure plan covers the whole generic request

`PresentationDisclosurePlan::new` borrows a request and candidate set and
retains 1–64 supplied selections. It first revalidates the candidate set
against that request, then enforces:

- unique query-ID/handle pairs;
- each pair exists in the candidate set;
- every selected claim path exists in the query with the same intent;
- every selected path is satisfiable by that candidate;
- each required claim is selected for each chosen credential;
- every request query has at least one selected credential; and
- a query with `multiple == false` has exactly one selected credential.

Revalidating the candidate set prevents a valid set constructed for one
request from being paired with a different request that happens to reuse query
IDs. The bounded repeated scan is preferable to storing a request identity or
introducing a hidden fingerprint contract.

OID4VP credential and claim-set alternatives are intentionally absent. The
current request model means every query is required. Alternative/optional
query combinations belong to `IDR-024`, where their actual DCQL semantics can
be modeled without weakening this simpler invariant.

### D4 — No explicit claims means format-mandatory surface only

A query with no claim requests accepts only a selection with no explicit
selected claims. A format adapter may still emit claims that its format makes
mandatory. The generic plan neither enumerates nor interprets that surface.

### D5 — The same credential can serve separate queries

Uniqueness is scoped to query-ID/handle pairs. One local credential may satisfy
multiple distinct request queries, matching OpenID4VP semantics, while a pair
cannot be repeated. For a query permitting multiple credentials, at least one
and at most the global plan bound may be selected.

### D6 — Private diagnostics and stable errors

New custom Debug implementations reveal only safe intents and counts. New
errors contain no data and map through the existing `presentation` capability
and `InvalidInput` kind. Codes distinguish invalid collection bounds,
duplicates, unknown candidate pairs, query coverage, multiplicity, requested
claim mismatch, candidate capability mismatch and required-claim omission.

### D7 — Bounded scans before optimization

Both selections and per-selection claims are bounded at 64 before any scan.
The implementation reuses simple slice iteration and pairwise comparisons,
adds no map, set, external package or feature, and retains caller vectors. A
manual release diagnostic measures request/candidate/plan construction without
creating a host-specific performance promise.

## Risks and trade-offs

- The plan proves structural correspondence, not consent provenance,
  credential existence, cryptographic validity, proof feasibility or trust.
- Required claims are enforced per selected credential. This is conservative
  for multi-credential queries and prevents a proof adapter from receiving a
  partially satisfying selected credential.
- Candidate-set revalidation repeats bounded work. It avoids a request token,
  lifetime coupling or hash semantics while the API is experimental.
- The request model does not yet express alternative credential or claim sets.
  Adding them belongs to the OID4VP/DCQL component rather than approximating
  their semantics in the generic core.

## Migration and rollback

The API is additive and unreleased, with no serialized form. A focused revert
removes it without data migration. Downstream adoption, artifact/receipt types,
protocol state and proof execution require separate issues after merge.

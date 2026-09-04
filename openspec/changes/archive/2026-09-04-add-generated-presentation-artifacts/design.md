# Design: generated presentation artifacts and receipt inputs

## Context

Issue #83 continues `IDR-008` from the disclosure-plan API merged at
`develop@9074f7f7490759763a683a8fd879dba3272e4ebc`. OpenID4VP 1.0 Final
defines a VP Token as an artifact containing one or more presentations and
maps each query ID to an array of format-specific presentation values. W3C VC
Data Model 2.0 permits one verifiable presentation to aggregate multiple
credentials. Midnight exposes presentation codecs and family-owned proof
artifacts, while Oxid already treats generated proof bytes as opaque and
bounded.

The shared Rust boundary must correlate produced bytes with the disclosure
plan without owning their format, proof, transport or persistence semantics.

## Provenance and isolation

No donor code or fixture is copied. Exact revisions, paths, SHA-256 digests,
licenses and pre-existing consumer states are recorded in issue #83. Oxid,
midnight-identity, Lace ID Portal, NeoPRISM and Apollo remain read-only.

## Decisions

### D1 — Bind disclosure plans to their exact requests

`PresentationDisclosurePlan` becomes a private struct containing an exact
bounded request clone and its selections. A private `validate_against` equality
check rejects cross-request reuse before generated-artifact correlation. This
matches candidate-set binding, avoids a collision-prone fingerprint contract
and leaves the existing selection accessors intact.

### D2 — Artifacts carry opaque bytes and one or more selection bindings

`PresentationArtifactBinding` identifies a query-ID/credential-handle pair.
`PresentationArtifact` owns one credential format, 1–64 unique bindings and a
non-empty byte vector of at most 4 MiB. The bytes may encode a string, JSON,
CBOR or chain-specific proof; the generic core never parses them.

Allowing several bindings supports formats that aggregate multiple selected
credentials. Requiring one format per artifact keeps format dispatch explicit.
An adapter that produces one artifact per selection uses a one-element binding
vector. Bindings are internal correlation, not protocol wire metadata.

### D3 — Generated presentations require exact plan coverage

`GeneratedPresentation::new` borrows the exact request and disclosure plan,
then retains 1–64 artifacts. It checks the plan's request snapshot, a 16 MiB
aggregate payload ceiling using checked addition, and bounded pairwise
correlation:

- every binding names a plan selection;
- the artifact format matches the bound selection's request query;
- no binding appears in two artifacts; and
- every plan selection appears exactly once.

The constructor preserves artifact order and transferred payload buffers. It
does not verify, decode, hash or copy payloads and makes no statement about
proof validity, holder authorization, transport or verifier acceptance.

### D4 — Receipt input is a semantic handoff, not a success receipt

A receipt input is derived from a validated generated presentation and copies
one value-free entry per disclosure selection in plan order. It includes the
verifier, optional purpose, query ID, opaque local handle, format and selected
path/intent pairs. These are owner-private disclosure facts a product can use
after applying its own completion and retention policy.

Challenge and artifact bytes are deliberately absent. So are timestamps,
outcomes, transport identifiers, verification results and trust decisions.
Derivation proves only that the SDK accepted generated artifacts for the plan;
the type name and documentation must not imply external delivery.

### D5 — Private diagnostics and static errors

Custom Debug implementations expose only safe formats, intents, counts,
lengths and optional-field presence. Artifact bytes, request context and local
correlation values never render. New errors remain zero-data variants bridged
to static `presentation.*` invalid-input contracts.

### D6 — Bounded scans are simpler than indexes

Artifact count, binding count, per-artifact bytes and total bytes are bounded
before correlation scans. The plan already caps total unique selections at 64,
so pairwise scans remain small and avoid temporary maps, sets, hashing, new
dependencies and collision semantics. A release diagnostic measures the path
without defining a portable threshold.

## Risks and trade-offs

- Opaque bytes intentionally cannot be inspected for claim or proof accuracy;
  format adapters and verification layers own those guarantees.
- Receipt inputs contain correlating owner-private identifiers and disclosure
  paths. They are redacted from diagnostics but consumers must still apply
  minimization, access control, retention and deletion policy.
- A 4 MiB artifact and 16 MiB aggregate ceiling follows the existing Oxid
  safety precedent while preventing the 64-artifact worst case from reaching
  256 MiB. A future format requiring more must justify a focused bound change.
- Exact request clones add bounded memory but make cross-object binding
  collision-free and lifetime-independent.

## Migration and rollback

The public API is additive and unreleased. Adding private request state changes
only equality and memory behavior of an experimental type; accessors remain
source compatible. There is no wire or stored data migration. A focused revert
removes the slice before publication. Protocol state, format codecs, receipt
policy/storage, downstream adoption and release remain separate work.

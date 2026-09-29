# Design

## Private ownership boundary

`RawProtectedHeaderVisitor` remains the Serde adapter. A private closed member
enum classifies the seven accepted names and rejects every other name. One
private raw-field collector owns the optional values, decodes one classified
member, rejects duplicates, and finalizes required-algorithm plus exclusive
key-reference invariants into `RawProtectedHeader`.

The classifier owns the complete member vocabulary; the collector owns the
complete partial-state invariant. Neither becomes public, generic across
protocols, or split into one helper per field. Existing `read_string`, bounded
sequence visitors, `PublicKeyJwk`, marker mapping, and final validated
`ProtectedHeader` constructor retain their responsibilities.

## Exact parsing order

The design preserves this order:

1. Serde decodes the next property name;
2. classify it against the closed vocabulary, rejecting an unknown name;
3. decode its typed value while rejecting an already-populated field;
4. repeat in source order until the map ends;
5. require one `alg` value;
6. reject more than one of `kid`, `jwk`, and `x5c`; and
7. construct the same private raw header for existing validated conversion.

No field is normalized or reordered, and no error marker, string/chain limit,
public-JWK rule, evidence syntax rule, or trust boundary changes.

## Characterization boundary

Before production movement, a compact matrix binds the first error when a
duplicate or invalid known value appears before/after an unknown member, when a
missing algorithm coexists with ambiguous key references, and when ambiguity
coexists with a later duplicate/unknown/value failure. Existing tests continue
to bind every isolated rejection, exact successful header, serialization,
limits, redaction, RFC example, and round trip.

## Compatibility and code-health ratchet

The normalized public API and manifests remain identical. The touched signal
must disappear without an equivalent owner/helper or module signal,
forwarding-only chain, generated code, moved tests, waiver, new allocation, or
weaker threshold. Canonical evidence is rebound to the protected
implementation squash in a second issue-linked PR.

## Risks and rollback

Primary risks are input-order error drift, missing-alg/ambiguity inversion,
duplicate bypass, field normalization, wrong typed decoder, new partial state,
allocation growth, and diagnostic disclosure. The matrix, existing compact
corpus, allocation/redaction review, and exact diff make those visible. Rollback
inlines the private owners without public or stored-data migration.

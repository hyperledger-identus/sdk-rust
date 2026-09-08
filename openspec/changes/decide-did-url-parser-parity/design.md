## Context

The current `Did` and `DidUrl` values own exact strings, cache component byte
offsets, enforce 2 KiB/4 KiB ceilings before scanning or allocation, distinguish
bare DIDs from DID URLs, reuse owned input and expose redacted SDK errors. The
candidate exports one mutable owned `DID` type that also represents DID URLs.
Its private parser metadata cannot be reused without retaining the candidate's
owned object.

The comparison is therefore between substitutable implementation boundaries,
not simply two `parse(...).is_ok()` calls. A candidate can match most common
fixtures and still fail because its accepted language, representation,
resource work or facade behavior is incompatible.

## Decision method

1. Pin W3C DID Core 1.0 and RFC 3986 as normative sources.
2. Pin `did_url_parser 0.3.0` to its crates.io artifact and release commit.
3. Pin consumer repositories and select representative, attributable fixtures.
4. Compare ordinary cases plus exhaustive ASCII component insertion,
   percent-pair spellings, boundary whitespace/control bytes, terminal method
   identifier structure, exact SDK byte ceilings and owned construction.
5. Inspect source for allocation order, offset representation, public mutation,
   unsafe/native reach and query semantics.
6. Run candidate tests, Rust 1.98.1, `no_std + alloc`, portable targets and the
   SDK's strict library Clippy shape. Record failures rather than patching the
   candidate during the decision.
7. Apply every adoption stop condition independently. One unresolved semantic,
   resource, facade or safety mismatch is sufficient to retain the local parser.

## Corpus contract

The committed tab-separated corpus records a stable case id, source and pinned revision,
profile, escaped input, expected normative/SDK result, current result,
candidate result and mismatch class. Generated families record their generator
and aggregate count in the report; representative rows remain in the CSV so
the evidence is reviewable without executing code.

Bare-DID comparison treats a candidate parse as equivalent only when its path
is empty and query/fragment are absent, because the candidate has no distinct
bare type. Component equality includes exact stored serialization and slices,
not only success. Limits are SDK resource policy and intentionally produce
candidate mismatches even where W3C has no maximum.

## Decision boundaries

An adoption decision would require a separate production issue and OpenSpec
change. This slice may only retain the current implementation or demonstrate
that a later integration spike is warranted. It cannot silently wrap known
mismatches, because prechecking, reparsing and copying around the candidate
would preserve both local complexity and the dependency cone.

## Validation

- Factory research and constraint readiness.
- Corpus schema/count/hash and manual mismatch reconciliation.
- Candidate package/tag source equality, feature graph and normal cone.
- Rust 1.98.1 tests and `no_std + alloc` target checks for WASM, Android and
  iOS; strict Clippy result recorded exactly.
- Existing DID tests/fuzz receipts and full SDK Nix checks remain green because
  the final change is documentation/research only.

## Rollback

Revert the issue-linked decision PR. Production parsing, wire formats and
downstream repositories never change. A future candidate can be reconsidered
only when its pinned release addresses the named blockers and the same corpus
passes from scratch.

# ADR 0017: harden DID resolution wire boundaries

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** IDR-005 roadmap mandate and sdk-rust issue #41
- **Related work:** issues #5, #10, #35, #38, #39 and OpenSpec
  `harden-did-resolution-wire-boundaries`

## Context

The SDK has transport-free W3C DID resolution and serialized dereferencing
result values. Their raw entry points still allow duplicate JSON names to
collapse, and the datetime validator accepts only four-digit positive civil
years. The shared result boundary needs deterministic ambiguity/resource
handling and the complete bounded datetime profile before downstream adapters
converge on it.

The W3C DID Resolution v1 specification is a Candidate Recommendation. Its 6
August 2026 Snapshot is the compatibility baseline; the 28 August Draft and
editor source `71a5005` contain no later normative result change. Dereferencing
remains explicitly at risk.

## Decision

1. Both raw result entry points check the 512 KiB cap, stream the complete JSON
   through #38's duplicate-aware scanner, and only then deserialize typed data.
2. Result preflight caps are 64 containers, 16,384 values, 128 members/object,
   and 128 KiB of simultaneously live decoded names. Failures have static,
   redaction-safe resolution reasons.
3. `DidResolutionDateTime` accepts the bounded XML Schema 1.1 `dateTime`
   intersection required by DID Resolution: at most 128 ASCII bytes, UTC `Z`,
   whole seconds, proleptic Gregorian/astronomical years, and the `24:00:00`
   end-of-day form. Exact spelling is preserved; offsets and fractions fail.
4. Fixed deterministic property matrices cover every public resolution scalar,
   problem object, result state, open-data ceiling, and raw construction path.
   Sanitizer cargo-fuzz remains issue #35.
5. Portable assertions from the official test suite at `2649fdf` are recreated
   in Rust. Its HTTP behavior remains #10 and it provides no dereferencing
   fixture corpus to claim or copy.
6. Arbitrary native dereferenced content remains binding-owned as a future
   exact media type plus bounded opaque bytes. The core JSON envelope never
   guesses UTF-8/base64 or transport behavior while the feature is at risk.
7. Legacy keyword errors migrate only through the explicit adapter helper;
   strict result JSON remains current URL-object-only.
8. Performance evidence reports fixed work/bytes and normalized scanner
   overhead without a CI timing threshold. Allocation-shape evidence is the
   deterministic byte/node/member/live-name ceilings.

## Consequences

- Raw result ambiguity fails before one value can be selected, including inside
  documents, errors, content, metadata, and extension objects.
- Previously valid unique-name envelopes retain their wire behavior. Broader
  standards-valid datetimes become accepted; existing values remain exact.
- Valid raw parsing performs two bounded passes but avoids a second generic JSON
  tree and exposes no new normal dependency.
- Direct semantic serde is not advertised as an untrusted byte boundary.
- Native transport data, chain methods, trust, HTTP, FFI, release, and product
  policy remain outside this change.

## Provenance

The normative hashes and full W3C/test-suite revisions are recorded in the
OpenSpec design and issue #41. NeoPRISM `d6ad1ec`, midnight-identity `427f857`,
Lace `804de0a`, Oxid `bfe3b48`, and Apollo `ccee22b` are read-only
compatibility evidence. No donor source or fixture is copied.

## Rollback

Revert issue #41's focused PR. No released crate, persisted SDK data, consumer
repository, chain state, or reserved `main` branch is changed.

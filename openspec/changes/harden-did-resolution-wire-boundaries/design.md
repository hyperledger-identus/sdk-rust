## Context

The current result model follows the W3C DID Resolution v1 Candidate
Recommendation Draft dated 28 August 2026. The stable interoperability anchor
is the 6 August 2026 Candidate Recommendation Snapshot. Source inspection shows
that the later draft changes only non-normative implementation-suite guidance
and publication configuration; result shapes, nine error URLs, datetime rules,
and at-risk dereferencing behavior did not change.

Normative and conformance sources are pinned as follows:

| Source | Immutable pin | SHA-256 / revision | Use |
| --- | --- | --- | --- |
| W3C DID Resolution CR Snapshot | <https://www.w3.org/TR/2026/CR-did-resolution-1.0-20260806/> | `a4632a09600e0136022969114520dc3f8ee9af99ad80963e3ba1a368bea9af1a` | compatibility baseline |
| W3C DID Resolution CR Draft | <https://www.w3.org/TR/2026/CRD-did-resolution-1.0-20260828/> | `def1bf7cdcd38d712d52cb38d8bda538fff9f29fa5201e5d737d7efc073c79f1` | tracked drift |
| W3C editor source | `w3c/did-resolution` | `71a50058090417f9947b9f13985fc8b561a4ad59` | source/diff evidence |
| Official implementation-report suite | `w3c/did-resolution-test-suite` | `2649fdf719beadbd3c684d358eea23c3c2e514fe` | assertion mapping; BSD-3-Clause |
| VC Data Model 2.0 REC | <https://www.w3.org/TR/2025/REC-vc-data-model-2.0-20250515/> | `a9196a3d0b6601356c4e127bad24f8c7f2c17f6ed22e41b35755a910104156f8` | datetime reference chain |
| XML Schema 1.1 Part 2 REC | <https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/> | `cc2ae20cdeab02600c73fd13f1cc31dbbdd61f62aa455026fa84388b540443b8` | datetime grammar/calendar |
| RFC 9457 | <https://www.rfc-editor.org/rfc/rfc9457.txt> | `f2b3db92fb0bf3489cb3841a0da0c0d88dff40797b64d40b6123085183886c7b` | error object semantics |

The official suite currently exercises HTTP resolution implementations. It
asserts required result members, exact returned/document DID equality, empty
document metadata on failure, and `INVALID_DID`/`METHOD_NOT_SUPPORTED` error
URLs. It contains no reusable dereferencing fixture corpus. This slice maps
those portable assertions into original Rust vectors; HTTP behavior remains
issue #10 and no suite code is copied.

The read-only donor/consumer receipts are NeoPRISM `d6ad1ec`,
midnight-identity `427f857`, Lace `804de0a`, Oxid `bfe3b48`, and Apollo
`ccee22b`. Their exact paths, status, classifications, and license decisions
are recorded in issue #41. No donor implementation or fixture is imported.

## Goals / Non-Goals

**Goals:**

- Reject ambiguous JSON before any map can select a duplicate value.
- Bound the second-pass scanner independently of typed semantic validation.
- Accept the complete bounded datetime lexical space required by the pinned
  W3C sources while preserving exact spelling.
- Exercise every public resolution scalar parser and result-state boundary
  through deterministic generated evidence.
- Preserve chain/runtime neutrality and redaction-safe failures.

**Non-Goals:**

- Cargo-fuzz/sanitizer ownership already assigned to #35.
- HTTP, network resolution, caching, endpoint retrieval, method behavior,
  cryptography, trust, consent, custody, FFI, publication, or release.
- Changing downstream repositories or adding a native byte representation to
  the volatile core JSON envelope.

## Decisions

### Decision 1: pin the snapshot and track draft drift explicitly

The 6 August Candidate Recommendation Snapshot is the compatibility baseline.
The 28 August Candidate Recommendation Draft and editor source at `71a5005`
are tracked evidence. The source delta after the snapshot is non-normative, so
this change adopts no result-wire difference. A future normative draft change
requires a new compatibility decision rather than silently changing this API.

### Decision 2: preflight every raw result object recursively

`DidResolutionResult::from_json_slice` and
`DidUrlDereferencingResult::from_json_slice` first enforce the existing 512
KiB byte ceiling, then call #38's crate-private streaming scanner, then perform
typed deserialization. String entry points delegate to the byte entry points.

The result preflight permits at most 64 nested containers, 16,384 visited JSON
values, 128 members in one object, and 128 KiB of decoded property names held
by simultaneously open objects. It sees the entire envelope, including nested
errors, DID documents, content, content metadata, and extensions. Escaped
spellings compare after JSON decoding. These raw ceilings are wider than or
equal to typed semantic limits and do not weaken the existing 128-item,
64-extension-member, 256-byte-name, 64-KiB-string, 32-depth, and 4,096-node
open JSON policy.

Duplicate names receive a specific non-sensitive resolution reason. Depth,
node/live-key, member, and malformed-input failures map to static resolution
categories. No error contains a property name, value, offset, or input bytes.
Direct `serde_json::from_*::<T>` remains a semantic conversion for already
materialized/trusted representations; callers use the explicit SDK raw entry
points at untrusted byte boundaries.

### Decision 3: implement the bounded W3C datetime intersection

DID Resolution requires an ASCII XML Schema 1.1 `dateTime` adjusted to UTC and
without sub-second precision. `DidResolutionDateTime` therefore accepts:

- an optional leading minus and an XML Schema year with at least four digits,
  no unnecessary leading zeroes, and astronomical year zero;
- calendar-valid month/day values under proleptic Gregorian leap rules,
  calculated modulo 400 without converting an unbounded year to an integer;
- `00:00:00` through `23:59:59`, plus XML Schema's `24:00:00` end-of-day form;
- a required terminal `Z` and no fractional seconds or numeric offset.

The complete value is capped at 128 bytes, bounding the otherwise unbounded
XML Schema year. Exact valid spelling remains preserved. This is a compatible
acceptance expansion from four-digit years; offsets/fractions remain invalid
because they are not adjusted whole-second values.

### Decision 4: deterministic properties complement, not duplicate, fuzzing

Fixed generators cover valid and invalid media types, datetimes, version ids,
all standard/extension RFC 9457 errors, all resolution/dereferencing state
combinations, extension resource boundaries, raw duplicates, and
native/serde/raw-entry equivalence. Seeds and iteration counts are committed,
making failures reproducible on Rust 1.85 and every supported target. #35 owns
sanitizer-backed cargo-fuzz for DID lexical parsers; this issue neither claims
nor duplicates that campaign.

### Decision 5: native bytes belong to a binding value

The at-risk core `DidUrlDereferencingResult` remains serialized JSON. A future
HTTP or local binding that returns arbitrary content defines a separate bounded
representation containing an exact `MediaType` and opaque bytes, then maps to
the core JSON envelope only when the media representation itself defines a
lossless JSON form. The SDK never guesses UTF-8 or base64 and never fetches a
resource merely to classify it. This keeps native payload ownership in the
binding layer without freezing an at-risk core model.

### Decision 6: performance evidence uses portable work units

An ignored release diagnostic reports fixed iterations, input bytes processed,
and hardened-versus-typed-only parse ratio. It has no timing threshold.
Deterministic allocation-shape evidence is supplied by the raw byte, node,
per-object member, and simultaneously live decoded-name ceilings; allocator-
specific counts are not portable CI criteria.

## Threat Contract

**Assets:** state integrity, extension fidelity, standards interoperability,
resolver availability, caller privacy, and chain/transport separation.

**Threats addressed:** duplicate key smuggling, escaped-name aliasing,
unbounded recursive/object/key allocation, malformed or extreme year parsing,
contradictory result states, legacy/current error ambiguity, and diagnostic
reflection.

**Residual boundaries:** validation does not authenticate returned data, prove
DID equivalence, authorize keys, interpret proof extensions, define endpoint
safety, or establish product trust. Native byte size and transport behavior are
future binding-specific contracts.

## Migration and Rollback

Legacy producers translate recognized keyword errors with
`DidResolutionError::from_legacy_keyword`; strict raw JSON continues to require
the current error object and absolute type URL. Duplicate raw JSON must be
regenerated with one unambiguous value. No data migration is required.

Rollback is a focused PR revert. No released package, persisted SDK record,
consumer repository, chain state, or reserved `main` branch is affected.

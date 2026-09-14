# ADR 0117: use lean crate-local presentation error contracts

- **Status:** Accepted for the presentations slice
- **Date:** 2026-09-15
- **Decision authority:** umbrella issue
  [#271](https://github.com/hyperledger-identus/sdk-rust/issues/271) and
  delivery issue
  [#279](https://github.com/hyperledger-identus/sdk-rust/issues/279)
- **Applies to:** `identus-presentations` only
- **Related decisions:** ADR 0110 and ADR 0116
- **Assessed revision:** sdk-rust
  `105308771dbebceb473b99d9eb82b0fa0178ab09`

## Context

`PresentationError` has 48 fieldless variants and 48 public stable error-code
constants. Its current private `contract()` is one wildcard-free 190-line
match. Both `Display` and the public `const to_identus_error()` bridge already
consume that match, so there is no duplicated local/public message decision.
Every variant has the same `InvalidInput` kind and `presentation` capability,
and every local message equals its public safe message.

The maintenance problem is therefore catalogue navigation and review locality,
not behavioral deduplication. ADR 0116 is informative but explicitly applies
only to the credentials pilot, whose heterogeneous errors required a five-field
record. Copying that record here would repeat invariant data 48 times.

## Decision

1. `identus-presentations` owns a private two-field contract containing only
   `ErrorCode` and the single static message used by both current surfaces.
   `ErrorKind::InvalidInput` and `CAPABILITY` remain centralized crate
   invariants rather than repeated row fields.
2. Private catalogue modules group records into request/query, candidate
   matching, disclosure selection, artifact assembly, and lifecycle ownership
   boundaries with 13, 9, 13, 9, and 4 variants respectively.
3. One private macro invocation adjacent to `PresentationError` generates only
   the exhaustive wildcard-free router and a test-only variant inventory. It
   does not generate the public enum, constants, documentation, serialization,
   or bindings.
4. The public enum, variant order, derives, `#[non_exhaustive]`, constants,
   `From`, `Display`, `Error`, and `pub const fn to_identus_error` remain
   explicit and unchanged.
5. A planning-only 48-row golden captured from the assessed revision pins
   constant identity/visibility, code, kind, capability, local/public/full
   displays, and empty source behavior. The stable test copy is byte-identical
   and immutably hash-bound to the active or unique archived planning copy.
6. The factory golden checker is extended through a second bounded
   configuration, not a copied validator. Active/archive ambiguity, missing
   inputs, coordinated drift, and symlinked components remain fail-closed.
7. No public/shared error framework, dependency, feature, retryability field,
   wire schema, FFI, unsafe/native code, protocol behavior, consumer change,
   or other error-crate migration is introduced.

## Consequences

- The largest presentation error review unit should fall from 190 lines to at
  most 60 while all 48 decisions stay explicit and crate-owned.
- Behavioral decision count remains 48 before and after; claiming duplication
  reduction would be false. Total production lines may rise and must be
  reported honestly.
- Uniform kind/capability cannot drift per row because they are centralized.
- Later JOSE and OID4VCI slices still require independent designs; this record
  does not standardize the two-field shape.

## Alternatives

Keeping the single 190-line match preserves behavior but leaves unrelated
domains coupled in every review. Reusing the credentials five-field record
duplicates invariants. A shared crate, public trait, derive/procedural macro,
or build-time schema adds an unjustified dependency or public toolchain. All
are rejected for this slice.

## Verification and rollback

Verification requires exact 48-row compatibility, empty public API and
manifest/lock diffs, const-use and source-free tests, direct supported-target
compile checks, strict Clippy/docs/factory/Nix gates, and measured function and
line changes. Rollback restores the single match and removes only private
catalogue/test infrastructure; no data or wire migration exists.

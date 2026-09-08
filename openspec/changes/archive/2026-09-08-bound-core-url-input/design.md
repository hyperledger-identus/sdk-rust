# Design

## Context

All public validated constructors for `Url(String)` delegate to
`validate_url(&str)` through `identus-derive`. One early check therefore covers
borrowed parsing, owned move construction and validated deserialization without
changing the macro or duplicating policy at call sites.

## Decisions

### One public byte budget

`pub const MAX_URL_BYTES: usize = 8_192` lives beside `Url` and is re-exported
from `identus-core`. The value is a semantic compatibility/security contract,
not a tunable runtime option. UTF-8 bytes are used because memory, wire and
input-work budgets operate on bytes and this matches existing SDK types.

### Reject length before syntax work

`validate_url` checks `s.len() > MAX_URL_BYTES` before searching or traversing
the input. The resulting `UrlError::TooLong` is stable, contains no dynamic
input or measured length, and bridges to the existing `core.invalid_url` /
`InvalidInput` surface. Existing error variants and syntax order remain
unchanged for in-budget values.

### Preserve one validation funnel

No generated constructor is hand-overridden. Tests exercise `parse`,
`try_new`, `TryFrom<String>`, `FromStr` and serde so a future derive change
cannot accidentally bypass the shared validator. The crate remains free of new
runtime dependencies.

### Narrow one limitation, not the audit

The canonical spec gains the `Url` size contract. `SDK-LIM-007` stays effective
for all other inherited boundaries and explicitly retains the outer-allocation
caveat. No repository-wide completion claim is made.

## Compatibility

Adding `UrlError::TooLong` can break exhaustive external matches and rejecting
previously accepted oversized values is a behavioral break. That is accepted
under issue #193 for the unpublished 0.1 active-development surface. Making the
enum non-exhaustive is not bundled into this slice because it creates another
match-site compatibility event and does not improve the limit itself.

## Security and privacy

The static error variant and public message disclose neither the input nor its
length. Validation syntax work starts only after the budget passes. The caller
or deserializer may already own an oversized allocation, so adapters must apply
outer body, field, decompression and nesting limits before constructing `Url`.

## Test design

- Build a valid ASCII URL whose total length is exactly `MAX_URL_BYTES` and
  prove acceptance.
- Append one byte and prove `TooLong` through every generated constructor.
- Build valid URLs with a repeated two-byte Unicode scalar in the path and
  prove the boundary uses bytes rather than character count.
- Deserialize exact-boundary and oversized JSON strings, and prove no input
  fragment appears in bridged error display.
- Preserve all existing syntax and round-trip tests.

## Rollback

Revert the constant, variant, tests and canonical requirement together and
restore the named `Url` limitation. Never retain the narrowed limitation text
if runtime enforcement is removed.

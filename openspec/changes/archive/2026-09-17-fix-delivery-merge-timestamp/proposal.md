# Fix delivery merge timestamp validation

## Why

The first live recovery attempt for PR #321 failed closed after a valid merge.
GitHub returned second-precision UTC timestamp `2026-09-17T09:48:40Z`, while
the validator required an exact round trip through JavaScript
`Date.toISOString()`, which emits `2026-09-17T09:48:40.000Z`.

## What changes

- Accept canonical UTC RFC 3339 timestamps at second precision or with one to
  nine fractional-second digits.
- Continue rejecting offsets, invalid calendar/time values, lowercase or
  missing `Z`, whitespace, and trailing content.
- Cover the live GitHub shape plus negative variants in deterministic tests.
- Recover PR #321's verified immutable receipt without performing another
  merge.

## Capability

### Modified capability

- `factory-operations`: make hosted merge-time validation interoperable with
  GitHub while retaining a closed UTC grammar.

## Non-goals

No product crate, Rust API, dependency, toolchain, workflow, release,
publication, repository setting, consumer, branch deletion, or `main` change.

## Delivery

Issue #322 owns this focused repair against protected `develop`. Required
exact-head `fast` CI and one discovery review remain mandatory.

# Decompose presentation error contracts

## Why

Issue #279 is the second independently revertible slice of #271. The
credentials pilot is merged, but presentations has a materially different
starting point: one correct 190-line private contract already feeds both public
surfaces. Reviewers still must scan all 48 unrelated domain mappings together.

## What changes

- Capture an immutable exact-base 48-row presentation error golden before
  production edits.
- Add the presentations-scoped ADR 0117.
- Replace the single private tuple catalogue with a lean two-field record and
  five crate-private domain catalogues.
- Preserve one exhaustive router and generate its test-only inventory from the
  same private list.
- Extend the existing immutable golden/factory validation without copying its
  security-sensitive path logic.
- Record exact compatibility, target, quality, and maintainability evidence.

## Capabilities

### New capability

- `presentation-error-contracts`: exact presentation error compatibility and
  crate-local catalogue ownership.

## Impact

Only `identus-presentations`, test/factory evidence, Nix source filtering, ADR
0117, and this OpenSpec change may be modified. No feature, dependency, public
API, protocol behavior, wire/FFI surface, consumer, #7, or #168 work is in
scope.

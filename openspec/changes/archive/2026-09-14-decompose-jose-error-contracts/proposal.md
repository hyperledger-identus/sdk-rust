# Decompose JOSE error contracts

## Why

Issue #280 is the third independently revertible slice of #271. JOSE currently
maps 51 stable errors through a 214-line function with separate code/message
and kind decisions. The kind match uses a wildcard, so a future variant could
silently inherit `InvalidInput`.

## What changes

- Capture an immutable exact-base 51-row JOSE error golden before production
  edits.
- Add JOSE-scoped ADR 0118.
- Replace two mapping sites with one private three-field record per variant in
  four responsibility-owned catalogues.
- Preserve a single exhaustive, wildcard-free router and test-only inventory.
- Extend the existing immutable golden/factory/Nix mechanism by configuration.
- Record exact compatibility, target, quality, and maintainability evidence.

## Capabilities

### New capability

- `jose-error-contracts`: exact JOSE error compatibility and crate-local
  catalogue ownership.

## Impact

Only `identus-jose`, test/factory evidence, Nix source filtering, ADR 0118, and
this OpenSpec change may be modified. No dependency, feature, public API,
protocol/algorithm behavior, wire/FFI surface, consumer, OID4VCI, #7, or #168
work is in scope.

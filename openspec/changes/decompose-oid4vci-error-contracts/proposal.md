# Decompose OID4VCI error contracts

## Why

Issue #277 is the fourth independently revertible slice of #271. OID4VCI
currently maps 171 stable fieldless errors through one correct but 860-line
public const function. It has no duplicate mapping or wildcard default, yet a
reviewer must traverse six protocol responsibility areas to assess one change.

## What changes

- Capture an immutable exact-base 171-row OID4VCI error golden before
  production edits.
- Add OID4VCI-scoped ADR 0119.
- Move code/kind/message records into six crate-private protocol-cohesive
  catalogues while retaining one explicit exhaustive router.
- Preserve the exact public API and all typed OID4VCI wire/Serde behavior.
- Extend the existing immutable golden/factory/Nix mechanism by configuration.
- Record exact compatibility, target, quality, and maintainability evidence.

## Capabilities

### New capability

- `oid4vci-error-contracts`: exact OID4VCI error compatibility and crate-local
  protocol-cohesive catalogue ownership.

## Impact

Only `identus-oid4vci` private error implementation, test/factory evidence,
Nix source filtering, ADR 0119, and this OpenSpec change may be modified. No
dependency, feature, public API, OID4VCI wire/protocol behavior, JOSE behavior,
FFI/binding surface, consumer, issue #7 behavior, or issue #168 behavior is in
scope.

# Bound hierarchical-derivation inputs and work

## Why

`DerivationPath::from_path` currently splits an arbitrarily long string into
an eagerly allocated `Vec`, and the reusable Cardano path consumers accept an
unbounded programmatically constructed axis list. `EdHDKey::init_from_seed`
also hashes an arbitrary seed even though SLIP-0010 defines a 16–64-byte range,
and `EdHDKey` depth arithmetic can wrap in release builds. Issue #199 isolates
these inherited resource and fail-closed gaps under #168 and the Apollo parity
program in #9.

## What Changes

- Export derivation budgets for 4,096 path bytes, 255 axes and 16–64-byte
  BIP-32/SLIP-0010 seeds.
- Reject oversized textual paths before splitting or parsing and parse them in
  one pass without an eager intermediate collection.
- Reject over-depth path work before any child-key operation in every
  hierarchical key consumer.
- Apply the normative seed range to `EdHDKey` as well as `HDKey`.
- Make `HDKey` and `EdHDKey` child-depth transitions reject depth 255 rather
  than overflow or create an unserializable BIP-32 depth.
- Preserve current vectors, public key shapes and the stable redacted
  `Error::DerivationFailed` boundary.
- Update the canonical crypto specification and narrow the derivation clause
  in `SDK-LIM-007` without closing the parent audit.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crypto`: define resource and depth boundaries for reusable hierarchical
  derivation primitives.

## Impact

- **Issues:** #199, child of #168 and #9.
- **Public API:** additive constants and `DerivationPath::len`/`is_empty`;
  existing key/path types and error variants remain SDK-owned.
- **Accepted behavior:** paths over 4,096 UTF-8 bytes or 255 axes, Ed25519 HD
  seeds outside 16–64 bytes, and children below a depth-255 parent are newly
  rejected.
- **Dependencies, features and wire formats:** unchanged.
- **Rollback:** remove the checks, tests and specification together and
  restore the derivation wording in `SDK-LIM-007`; changing a limit later
  requires a new material compatibility/resource decision.

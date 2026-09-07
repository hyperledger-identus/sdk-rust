# Correct BIP-32 scalar validation behind `HDKey`

## Why

`HDKey` currently reduces an invalid BIP-32 child tweak modulo the secp256k1
group order instead of rejecting it, accepts invalid master scalars and
unbounded seed lengths, and increments depth without checking overflow. These
are standards and fail-closed correctness defects in security-sensitive code.

The planned `bip32 0.5.3` adoption does not survive focused research. Its
`k256` backend represents the child tweak as `NonZeroScalar`, so it rejects
`IL = 0` even though BIP-32 permits that value when the resulting child key is
nonzero. Its mandatory `bs58` and `ripemd` dependencies support extended-key
serialization that the hardened-only SDK facade neither exposes nor needs.

## What Changes

- Retain the existing private HMAC construction and use the already-adopted
  `k256` scalar parser without modular reduction.
- Enforce the complete BIP-32 seed range of 16 through 64 bytes, validate the
  master scalar, reject `IL >= n`, permit a zero child tweak, reject a zero
  resulting key, and fail on depth overflow.
- Preserve `HDKey`, hardened-only paths, Apollo compatibility, redacted errors,
  and zeroizing ownership.
- Record `bip32 0.5.3` in the non-adoption ledger and supersede only the BIP-32
  disposition in ADR 0061.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crypto`: make the existing BIP-32 derivation boundary standards-correct and
  fail closed without adding a public or private BIP-32 framework dependency.

## Impact

- **Issue:** #153, child of dependency-first cleanup #151 and crypto epic #9.
- **API:** signatures and public types remain unchanged; invalid seed lengths,
  invalid scalars, and depth overflow now return `Error::DerivationFailed`.
- **Dependencies:** no package is added; the existing `k256` implementation is
  reused for exact scalar validation and arithmetic.
- **Security:** secret intermediates stay zeroizing and no dependency formatter,
  serializer, or extended-key surface enters the SDK.
- **Rollback:** revert one focused correctness PR; no data migration is needed.

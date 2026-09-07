# Design

## Context

The current hardened-only `HDKey` facade already contains the BIP-32 HMAC
construction and depends on `k256 0.13`. It incorrectly uses `Reduce<U256>` for
the left HMAC half (`IL`) and parent key, turning out-of-range values into valid
scalars. Master construction copies the left half without scalar validation,
and depth uses unchecked addition.

The candidate `bip32 0.5.3` crate implements a much broader extended-key model.
Its selected secp256k1 feature still resolves Base58Check and RIPEMD packages.
More importantly, its low-level k256 child operation parses `IL` as a
`NonZeroScalar`. BIP-32 invalidates `IL >= n` and a zero resulting child, but
does not invalidate `IL = 0` by itself. Adopting the crate would therefore
require a local standards workaround while adding unused serialization
coupling.

## Decisions

### 1. Reuse `k256`, not the `bip32` framework

Parse master and parent private-key bytes with `Scalar::from_repr` plus an
explicit nonzero check. Parse child `IL` with the same function, which rejects
values at or above the group order without modular reduction and permits zero.
Add it to the validated parent's scalar and reject a zero result before
serializing the child.

This delegates curve scalar representation, validation, arithmetic and
serialization to the established RustCrypto primitive already required by the
crate. The SDK retains only the small BIP-32 message construction and HMAC
orchestration that is specific to its deliberately narrow facade.

### 2. Preserve the hardened-only public boundary

`HDKey`, its public fields, `derive`, `derive_child`, error mapping and redacted
`Debug` remain unchanged. Non-hardened derivation, public derivation,
xprv/xpub, Base58Check, fingerprints and network/version policy remain outside
this change.

### 3. Enforce normative seed and scalar boundaries

`init_from_seed` accepts every byte length from 16 through 64 inclusive, not
only the common 16/32/64 lengths. The master HMAC left half must be a valid
nonzero secp256k1 secret scalar. Child derivation rejects `IL >= n`, accepts
`IL = 0` when the parent remains nonzero, and rejects a zero sum. Depth uses
`checked_add` and maps overflow to `Error::DerivationFailed`.

### 4. Keep failure diagnostics and secret ownership stable

All new failures use `Error::DerivationFailed`, whose public bridge is already
redacted. HMAC output, serialized scalar bytes, and child material are bounded
by `Zeroizing`; `HDKey` remains `Zeroize + ZeroizeOnDrop`. Test seams inject
synthetic HMAC halves without making them public or compiling them in normal
builds.

## Alternatives rejected

- **Adopt `bip32 0.5.3`:** rejected for this boundary because its k256 backend
  rejects a standards-valid zero tweak and its mandatory graph includes unused
  extended-key serialization packages. A workaround would leave the risky
  edge local while increasing coupling.
- **Use the high-level extended-key API:** rejected because it narrows seed
  lengths to 16/32/64 and exposes behavior the SDK intentionally does not own.
- **Continue modular reduction:** rejected because BIP-32 explicitly requires
  invalid `IL >= n` values to be skipped/rejected, not reduced.
- **Implement secp256k1 arithmetic manually:** rejected because `k256` already
  provides constant-time scalar parsing and arithmetic behind the SDK facade.

## Risks and mitigations

- Synthetic invalid HMAC values are computationally infeasible to obtain from
  ordinary vectors. Private test helpers exercise exact boundary bytes.
- Tightening seed length changes behavior for callers relying on inputs outside
  BIP-32's 128–512-bit range. The existing API is fallible and returns its
  established derivation error; the correction is intentional before 1.0.
- BIP-32 says an invalid child should proceed to the next index. The SDK API
  derives an explicitly requested index, so it returns an error and lets the
  caller choose policy rather than silently deriving a different child.

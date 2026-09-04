## Why

The canonical entropy port currently allocates an attacker-sized `Vec`, cannot
report backend failure, and forces production adapters and key generation to
panic. That is unsuitable for a reusable crypto foundation and propagates an
unsafe contract into every future binding and wallet consumer.

## What Changes

- Replace `SecureRandom::generate_seed(num_bytes) -> Vec<u8>` with the
  fallible caller-owned `fill_bytes(&mut [u8]) -> Result<(), Error>` contract.
- Make random Ed25519, X25519, secp256k1 and P-256 key generation fallible.
- Make random BIP-39 mnemonic and seed creation fallible.
- Return `Error::SecureRandomFailure` for system-RNG failure and exhausted
  bounded EC scalar retries; never expose backend detail at the stable error
  surface.
- Update the system and deterministic entropy adapters and all callers/tests.

## Capabilities

### Modified Capabilities

- `crypto`: the entropy port and every random-construction API become
  fallible without changing algorithms, wire formats, custody boundaries, or
  the stable error catalogue.
- `adapters-entropy`: adapters fill caller-owned buffers and report failure.

## Impact

- **Public API:** breaking source change in unpublished `0.0.0` crates;
  callers must handle `Result` and adapter implementations must fill a supplied
  slice.
- **Allocation:** entropy implementations no longer allocate from a caller
  supplied length. Current crypto callers use fixed 32-byte buffers.
- **Security:** RNG failure and invalid-scalar exhaustion are errors instead of
  panics. No key, seed, backend detail, or entropy bytes enter errors.
- **Dependencies/targets:** unchanged; no new dependency, feature, or target.
- **Scope:** issue #67, child of #9 / `IDR-004` and #20.

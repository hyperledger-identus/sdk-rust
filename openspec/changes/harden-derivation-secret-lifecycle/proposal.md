## Why

The existing HD key types derive `Debug` over raw private-key and chain-code
arrays and do not erase those owned arrays on drop. Key generation, mnemonic
creation, HMAC derivation, and PBKDF2 seed creation also leave avoidable secret
intermediates in ordinary stack or heap buffers. This conflicts with the SDK
rule that raw secrets must not cross formatting surfaces and with the IDR-004
exit condition for redacted, zeroizing secret material.

## What Changes

- Give `HDKey` and `EdHDKey` redacted `Debug` implementations that expose only
  type and derivation metadata.
- Make both HD types implement `Zeroize` and `ZeroizeOnDrop` for their owned
  private key, chain code, and metadata.
- Hold curve-generation entropy, mnemonic entropy, HMAC input/output, and
  PBKDF2 output intermediates in zeroizing storage.
- Add compile-time trait, explicit-erasure, debug-redaction, and unchanged
  vector tests.
- Add `zeroize` 1.9 as a direct, workspace-governed dependency without
  changing the existing feature surface.

## Capabilities

### Modified Capabilities

- `crypto`: harden the lifecycle and formatting of existing secret-bearing
  derivation, mnemonic, and random-construction buffers without changing
  algorithms, outputs, custody boundaries, or public error behavior.

## Impact

- **Public API:** no signature, field-access, feature, or wire change; the
  observable `Debug` representation of HD key types is intentionally redacted.
- **Security:** owned secret buffers are erased on normal drop where the Rust
  ownership model permits. The contract is best effort and does not cover
  caller copies, allocator copies, swap, crash dumps, or hostile hardware.
- **Dependencies:** `zeroize` 1.9.0 becomes a direct dependency of
  `identus-crypto`; it is already present in the locked dependency graph.
- **Scope:** issue #69, child of #9 / `IDR-004` and #20.

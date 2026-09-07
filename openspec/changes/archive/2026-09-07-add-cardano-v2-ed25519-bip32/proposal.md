## Why

Apollo consumers require Cardano/IOG Ed25519-BIP32 V2 derivation, including
soft public derivation, while `identus-crypto` currently exposes only the
incompatible hardened-only SLIP-0010 `EdHDKey`. GitHub issue #177 isolates the
missing capability so downstream Rust projects can retire Apollo crypto code
without confusing two distinct Ed25519 derivation schemes.

## What Changes

- Add SDK-owned Cardano V2 extended-private and extended-public key types.
- Support hardened and soft private derivation, soft public derivation, and
  explicit rejection of hardened public derivation.
- Adopt `ed25519-bip32 0.4.3` only as a private, feature-gated implementation
  dependency after recording provenance, dependency-cone, unsafe, platform,
  maintenance, compatibility and rollback evidence.
- Prevent dependency types and secret-revealing formatting from crossing the
  Identus facade; expose private bytes only through an explicit method.
- Add self-contained Apollo/upstream vectors, public/private soft-derivation
  equivalence tests, redaction tests and public-API boundary evidence.
- Keep the existing `EdHDKey` behavior and name unchanged.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crypto`: add feature-gated Cardano/IOG Ed25519-BIP32 V2 derivation behind
  Identus-owned redacted key and error types.

## Impact

- **Issue:** #177, child of crypto parity epic #9.
- **API:** additive feature-gated types with names distinct from `EdHDKey`.
- **Dependencies:** feature-gated `ed25519-bip32 0.4.3`, enabled by the
  all-capabilities default; neither it nor `cryptoxide` appears in the public
  API or a feature-disabled build.
- **Consumers:** Apollo, NeoPRISM and future Cardano consumers gain one reusable
  byte-compatible implementation; no downstream repository changes here.
- **Security:** the dependency's unsafe zeroing and secret-revealing formatting
  remain encapsulated and are explicitly recorded as residual risk.
- **Rollback:** revert this focused feature and dependency without changing the
  pre-existing derivation contract.

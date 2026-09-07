# ADR 0078: conditionally adopt Cardano V2 Ed25519-BIP32

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#177](https://github.com/hyperledger-identus/sdk-rust/issues/177)
- **Research:** [OpenSpec research](../../openspec/changes/add-cardano-v2-ed25519-bip32/research.md)
- **Hardening follow-up:** [#179](https://github.com/hyperledger-identus/sdk-rust/issues/179)
- **Supersedes:** the unresolved Cardano/KMP derivation part of ADR 0022; the
  SLIP-0010 `EdHDKey` decision remains unchanged

## Context

Apollo uses Cardano/IOG Ed25519-BIP32 V2, while the SDK currently implements
only hardened SLIP-0010 Ed25519 derivation. Treating the algorithms as one type
would silently produce incompatible private keys and remove Apollo's soft
public derivation capability.

The maintained published implementation, `ed25519-bip32 0.4.3`, continues the
exact Apollo donor algorithm and has a narrow dependency cone. Its public
security ergonomics do not meet SDK policy: `XPrv` formatting reveals private
bytes, and its drop path uses manual unsafe zeroing. Available alternatives are
semantically incomplete, wrap the same dependency, are broader and younger, or
retain the same leakage concern.

## Decision

Conditionally adopt exactly `ed25519-bip32 0.4.3` behind a dedicated
`cardano-bip32` feature and SDK-owned facade:

1. Expose `CardanoV2ExtendedPrivateKey` and
   `CardanoV2ExtendedPublicKey`, never `XPrv`, `XPub`, `cryptoxide` or
   dependency errors.
2. Keep the existing `EdHDKey` SLIP-0010 contract unchanged and name both
   schemes explicitly.
3. Store the SDK private representation independently in zeroizing owned
   memory. Dependency values are short-lived implementation details.
4. Provide redacted `Debug`, no `Display` or serde for private material, and
   one explicitly named caller-owned raw export.
5. Reject hardened public derivation through the stable redacted
   `Error::DerivationFailed` surface.
6. Include `cardano-bip32` in the all-capabilities default while allowing
   feature-disabled/minimal consumers to exclude its complete dependency cone.
7. Pin Apollo and independent V2 vectors, supported-target builds,
   dependency/public-API inspection, unsafe review and supply-chain gates.
8. Record that the SDK lock resolves `cryptoxide 0.6.5` and that the candidate
   activates all of its default features; package count is narrow but compiled
   algorithm surface is not.
9. Track upstream redacted formatting, maintained zeroization, and a minimal
   cryptoxide feature declaration. If that path fails, a minimal temporary
   fork of the exact revision is permitted only with the same vectors and an
   upstream-return sunset trigger.

This is a bounded implementation-dependency decision, not approval of
`cryptoxide` as a general SDK backend or a claim that the dependency itself is
free of unsafe code.

## Consequences

- Apollo and NeoPRISM can reuse one Rust implementation without carrying
  Apollo's platform wrappers.
- Consumers cannot accidentally confuse SLIP-0010 and Cardano V2 types.
- Default crypto builds add `ed25519-bip32` and `cryptoxide`; minimal builds
  retain their current narrow graphs. The current upstream manifest compiles
  all cryptoxide default features until the hardening follow-up lands.
- Secret diagnostics and lifetime behavior are governed by SDK-owned code,
  while transient dependency zeroing remains a residual reviewed risk.
- Replacement remains local because no dependency type or error crosses the
  public boundary.

## Alternatives rejected

### Reimplement V2 using RustCrypto primitives

This would remove `cryptoxide` but create a fresh implementation of subtle
scalar, point, HMAC and child-derivation mechanics. Current alternatives do not
provide enough independent assurance to justify that cryptographic rewrite.

### Adopt `ed25519-bip32-core`

It uses `zeroize` for its drop helper but retains secret-revealing formatting,
an older `cryptoxide`, and weaker maintenance evidence.

### Adopt a Cardano wallet or serialization suite

Pallas wallet and Cardano serialization libraries either wrap an older
`ed25519-bip32` or add ledger/serialization coupling. They do not eliminate the
underlying risk and violate the narrow-engine rule.

### Use standard `bip32` or hardened-only Ed25519 crates

RustCrypto `bip32` implements standard BIP-32 with secp256k1 backends;
`ed25519-dalek-bip32` implements hardened-only SLIP-0010 behavior. Neither can
produce Apollo/Cardano V2 soft private and public children.

## Verification and rollback

Acceptance requires byte-exact Apollo/upstream vectors, soft private/public
equivalence, hardened-public rejection, invalid-key handling, redaction and
explicit-zeroization tests, absence of forbidden dependency names from public
API, minimal/default feature-graph evidence, Rust 1.85 and primary 1.98.1
checks, supported portable-target compilation, RustSec/dependency policy and a
distinct security review.

Rollback reverts the issue #177 PR. Reconsider adoption on an advisory,
maintenance loss, target regression, vector drift, public-boundary leak, or a
maintained exact implementation that removes the residual risks with no larger
coupling cost.

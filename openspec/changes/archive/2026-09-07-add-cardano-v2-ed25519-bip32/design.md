## Context

`identus-crypto::EdHDKey` implements hardened-only SLIP-0010 with a 32-byte
secret. Apollo instead delegates its Ed25519 hierarchy to
`input-output-hk/rust-ed25519-bip32@e9d995b1fe29c428d5f569ba96700872f68fab88`:
a 96-byte Cardano/IOG extended private key, V2 little-endian child indices,
soft and hardened private derivation, and watch-only soft public derivation.
The algorithms and key shapes are not interchangeable.

The current published continuation is `ed25519-bip32 0.4.3` at signed tag
`ed25519-bip32-v0.4.3` (`6539dc9f792174fa5c2290c9e0a23710a1e1ecef`).
It is narrow, `no_std`, Rust 1.81, MIT OR Apache-2.0, and resolves only
`cryptoxide 0.6.5` in the SDK lockfile. The candidate does not disable
`cryptoxide`'s default features, so the full cryptoxide algorithm surface is
compiled even though the resolved package cone remains two crates. It also
formats `XPrv` as secret hex and uses an unsafe manual memory write on drop.
Those dependency behaviors require an owned Identus boundary and an upstream
feature-minimization follow-up; they are not suitable as the SDK contract.

## Goals / Non-Goals

**Goals:**

- Reproduce Apollo's Cardano V2 private and public derivation byte-for-byte.
- Give Rust consumers stable, scheme-specific Identus types and errors.
- Make owned SDK private material redacted and zeroizing independently of the
  dependency's public formatting surface.
- Preserve minimal-feature, MSRV, primary Rust, WASM, Android and iOS builds.
- Keep replacement or removal of the dependency local to one module.

**Non-Goals:**

- Change `EdHDKey`, implement CIP-1852 path policy, derive Icarus roots,
  provide custody/storage, expose FFI, or migrate downstream repositories.
- Re-export dependency types, reproduce the dependency's signature API, or
  claim that dependency zeroing alone is sufficient for SDK secret ownership.
- Rewrite elliptic-curve or HMAC primitives locally.

## Decisions

### Decision 1: add explicitly Cardano-named key types

The public types are `CardanoV2ExtendedPrivateKey` and
`CardanoV2ExtendedPublicKey`. Child operations consume `DerivationAxis`; path
operations consume the existing `DerivationPath`. The name deliberately does
not reuse `EdHDKey`, `HDKey`, `XPrv` or `XPub` and therefore cannot imply
SLIP-0010 or standard secp256k1 BIP-32 semantics.

The private type accepts a verified 96-byte representation and Apollo's
non-extended 32-byte secret plus 32-byte chain-code construction. The public
type is produced from a private key or a 32-byte public key plus chain code.
Private derivation supports either axis. Public derivation rejects hardened
axes before invoking the dependency.

### Decision 2: conditionally adopt `ed25519-bip32 0.4.3`

The dependency is declared once at workspace level and enabled by the
`cardano-bip32` feature. The all-capabilities default includes that feature;
`--no-default-features` does not resolve the dependency. The module imports it
privately and maps every result to Identus-owned values.

The published crate is preferred because it is the maintained continuation of
Apollo's donor, implements the complete V2 behavior, has a two-package
resolved cone and passed Rust 1.85 plus Rust 1.98.1 host/WASM/iOS/Android
compile probes. `ed25519-bip32-core` retains secret-revealing formatting and
an older `cryptoxide`; `pallas-wallet` and Cardano serialization crates wrap
older `ed25519-bip32`; `ed25519-dalek-bip32` is hardened-only SLIP-0010;
RustCrypto `bip32` is standard secp256k1; and `outscript` is a broad young
multi-chain crate without zeroizing Cardano private-key storage.

### Decision 3: store secrets in an SDK-owned zeroizing representation

`CardanoV2ExtendedPrivateKey` stores a private `[u8; 96]` and derives
`Zeroize` plus `ZeroizeOnDrop`. Each operation constructs a short-lived
dependency `XPrv`, copies the result into the SDK-owned representation, and
drops the dependency value. The SDK type has a hand-written redacted `Debug`,
no `Display`, no serde implementation and no public dependency accessor.

Raw export is an explicitly named `expose_secret_bytes` operation returning a
caller-owned array. This mirrors the existing primitive-key convention while
making the transfer of secret ownership visible. Public-key bytes and chain
codes have explicit accessors; the public key's `Debug` also omits bytes so a
chain code cannot enter routine logs.

The dependency's unsafe zeroing remains reachable for transient `XPrv`
values. Encapsulation reduces exposure but does not eliminate that residual
risk. An upstream hardening follow-up will request redacted formatting and a
maintained zeroization primitive; a minimal pinned fork is the fallback if
upstream does not accept it.

### Decision 4: collapse dependency errors into the stable crypto error

Invalid private representations and failed public derivations return
`Error::DerivationFailed`. Hardened public derivation returns the same stable,
redacted error without invoking the dependency. Dependency error types never
appear as variants, sources, signatures or re-exports, so replacement does not
change consumer code or error text.

### Decision 5: preserve algorithm evidence independently of Apollo

Tests copy fixed byte vectors, not source code or a runtime dependency on
Apollo. They cover the Apollo donor hardened vector, independent upstream soft
private derivation, public/private soft equivalence, hardened public rejection,
round trips, redacted formatting and explicit zeroization. A dependency-boundary
test searches generated public API evidence for forbidden names.

## Risks / Trade-offs

- **Reachable unsafe and broad `cryptoxide` defaults remain** → keep the
  dependency private, record the exact package and feature cones, run
  repository supply-chain gates, and pursue upstream formatting, zeroization,
  and feature-minimization hardening; rollback is one feature/module revert.
- **A caller can explicitly export secret bytes** → make export naming explicit
  and document that the returned copy becomes caller-owned secret material.
- **An all-on default adds two compiled packages** → retain a dedicated Cargo
  feature so minimal consumers can exclude them; the resolved cone is bounded
  to the implementation and its one dependency.
- **Two Ed25519 derivation schemes may confuse consumers** → preserve `EdHDKey`
  and use Cardano/V2 names in types, module docs, feature and examples.
- **The dependency's future semver release may change behavior** → pin golden
  vectors and dependency-policy evidence; upgrades require a focused review.

## Migration Plan

1. Land the issue-linked specification, research, constraint record and ADR.
2. Add the private dependency, feature and SDK-owned types with focused tests.
3. Run MSRV/primary and supported-target compile evidence plus the full factory.
4. Archive and sync the canonical crypto specification after verification.
5. Open a PR to `develop`; downstream Apollo/NeoPRISM adoption remains separate.

Rollback reverts the focused PR. Existing serialized values, default types and
`EdHDKey` behavior are unchanged.

## Open Questions

None block implementation. Upstream formatting and zeroization hardening is a
separate follow-up because this change remains safe at the SDK boundary without
claiming the dependency itself has been remediated.

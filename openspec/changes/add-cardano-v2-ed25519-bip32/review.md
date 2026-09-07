# Exact-diff correctness and security review

- **Reviewer:** Codex fresh contract and implementation pass
- **Date:** 2026-09-08
- **Base:** `2201424628f89f7106c7c53077bb53fdffa3022d`
  (`origin/develop` at branch creation)
- **Scope:** issue #177, ADR 0078, complete OpenSpec contract, manifests,
  dependency lock, Cardano V2 facade, conformance tests, and generated public
  API
- **Result:** no unresolved blocker; one accepted upstream residual is tracked
  by #179

## Findings

1. **Verified — Cardano V2 and SLIP-0010 remain distinct.** The new public
   names, module documentation, and independent feature prevent callers from
   confusing the 96-byte Cardano/IOG hierarchy with the existing 32-byte,
   hardened-only `EdHDKey`. No existing derivation implementation changed.
2. **Verified — private and public derivation match Apollo.** The suite covers
   Apollo's m/1' private and m/1852/1815/0 public vectors, the donor's m/0'
   vector, an independently computed soft-private vector, and direct soft
   private/public equivalence.
3. **Verified — invalid and impossible operations fail safely.** Serialized
   private scalar bits are validated before retention and invalid input is
   zeroized before error return. Hardened public derivation is rejected before
   entering the dependency. Both paths use stable, redacted SDK errors.
4. **Verified — the SDK owns secret lifecycle and diagnostics.** The private
   key owns a private `[u8; 96]`, derives `Zeroize` and `ZeroizeOnDrop`, has a
   fieldless `Debug`, no `Display` or serde contract, and names raw export
   `expose_secret_bytes`. Short-lived dependency values do not persist in an
   SDK object.
5. **Verified — third-party types remain private.** `cargo-public-api 0.52.0`
   found no `ed25519-bip32`, `cryptoxide`, `XPrv`, `XPub`, or dependency error
   in the isolated feature's public API. Only Identus types and the intentional
   existing `zeroize::Zeroize` security trait appear.
6. **Accepted residual — the package cone is narrow but the feature cone is
   broad.** The SDK lock adds only `ed25519-bip32 0.4.3` and `cryptoxide 0.6.5`,
   but upstream activates every cryptoxide default feature. The full source
   scan and exact checksum are recorded; #179 requires minimal upstream
   features, redacted private formatting, and maintained zeroization, with a
   time-bounded fork decision as fallback.
7. **Verified — portability and compatibility remain bounded.** The default
   and isolated feature compile under primary Rust 1.98.1, the workspace builds
   under effective MSRV 1.85, and default crypto compiles for WASM, Android,
   and iOS. No runtime or certification claim is inferred from compile checks.
8. **Verified — scope and rollback are focused.** No Cardano ledger, address,
   CIP-1852 policy, storage, binding, downstream repository, publication, or
   release behavior changed. Reverting this PR removes the module, feature,
   two locked packages, tests, and contract together.

## Non-blocking repository observation

An additional `cargo clippy --workspace --all-targets --all-features` sweep on
Rust 1.98.1 reaches a pre-existing `manual_noop_waker` lint in
`crates/credentials/tests/verifier.rs`. The official workspace Clippy gate and
the stricter scoped crypto all-target feature gate pass. The unrelated
credentials test is intentionally not changed in this PR.

## Decision

The implementation satisfies issue #177 and ADR 0078. Its residual upstream
risk is explicit, bounded, reversible, and separately actionable in #179. The
change is approved for exact-head validation, guarded OpenSpec archive, and an
issue-linked PR to `develop`.

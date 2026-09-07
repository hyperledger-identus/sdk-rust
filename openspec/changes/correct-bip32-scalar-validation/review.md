# Exact-diff correctness and security review

- **Reviewer:** Codex fresh contract and implementation pass
- **Date:** 2026-09-08
- **Base:** `f94028b1a09c80c55e64647c20ed2bac95f72819`
  (`origin/develop` after issue #152 merged)
- **Scope:** issue #153, ADR 0080, complete OpenSpec contract, `HDKey`, tests,
  dependency decision, generated public API and gate evidence
- **Result:** no unresolved blocker; explicit-index invalid-child behavior and
  caller-owned raw-field copies remain recorded residuals

## Findings

1. **Verified — invalid scalars are no longer reduced.** `Scalar::from_repr`
   rejects values at or above the secp256k1 order. Explicit nonzero checks
   reject invalid master/parent keys, and the child sum is checked before
   serialization. The former `Reduce<U256>` path is gone.
2. **Verified — zero tweak follows BIP-32.** The child parser uses `Scalar`, not
   `NonZeroScalar`; an injected zero `IL` succeeds when the parent is valid,
   while parent 1 plus `n - 1` is rejected as a zero result. This is the exact
   edge on which `bip32 0.5.3` fails semantic fitness.
3. **Verified — master and metadata boundaries fail closed.** The public
   constructor admits every byte length in 16..=64 and rejects adjacent
   boundaries. Invalid synthetic master halves and checked depth overflow use
   the stable derivation error without values.
4. **Verified — secret ownership is not broadened.** HMAC output, copied `IL`,
   parsed parent/tweak/child scalars and serialized child bytes use zeroizing
   ownership. Existing public raw arrays remain the only caller-visible copy;
   `Debug` omits them and no serde, error, log or FFI surface was added.
5. **Verified — compatibility remains exact.** Official BIP-32 vectors 1–4,
   Apollo master and hardened `m/0'/0'/0'`, non-hardened rejection, path
   parsing, BIP-39, SLIP-0010 and Cardano V2 suites all pass.
6. **Verified — the dependency gate prevented negative-value adoption.** The
   candidate is maintained, licensed, portable and advisory-clean, but would
   require a zero-tweak workaround and import unused serialization packages.
   Reusing existing `k256` has higher cohesion and no new dependency surface.
7. **Verified — public and feature surfaces are unchanged.** The all-feature
   public API diff is empty; manifests and `Cargo.lock` are byte-unchanged.
   Minimal/default/KMP, Rust 1.85/1.98, nightly, portable target and complete
   workspace gates pass.
8. **Verified — scope and rollback are focused.** No xprv/xpub, public or
   non-hardened derivation, Bitcoin network, account, Cardano, custody, binding,
   downstream, release or publication behavior changed. One issue-linked PR
   can revert the implementation and decision records without migration.

## Decision

The implementation satisfies issue #153 and ADR 0080. The retained local code
is narrow protocol orchestration over existing RustCrypto primitives, not
fresh curve cryptography. The change is approved for exact-head validation,
guarded OpenSpec archive and an issue-linked PR to `develop`.

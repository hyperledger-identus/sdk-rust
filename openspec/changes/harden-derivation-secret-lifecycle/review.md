# Pre-implementation semantic, security, and API review

- **Date:** 2026-09-05
- **Issue:** #69, child of #9 / `IDR-004` and #20
- **Develop base:** `688f29f399b5d9d46faa0e934182a1925c7d9120`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. Derived `Debug` on both HD types currently emits the complete private key
   and chain code. This is a direct violation of the repository secret-format
   boundary and needs manual redaction.
2. RustCrypto and dalek types protect the secrets they own, but SDK-created
   entropy, HMAC, derivation-data, mnemonic, and PBKDF2 buffers are independent
   copies. They need their own zeroizing guard.
3. `ZeroizeOnDrop` is the enforceable type-level contract for long-lived HD
   values. Safe tests can prove trait implementation and explicit erasure, but
   cannot soundly read memory after drop; the specification correctly avoids
   that false claim.
4. Keeping raw HD fields public preserves current source compatibility. The
   documentation must make caller responsibility for copied bytes explicit;
   making secrets opaque belongs with a later key-handle/custody contract.
5. `zeroize` should be an optional direct dependency activated by each
   secret-bearing feature. This keeps the representation-only
   `--no-default-features` dependency cone unchanged while avoiding reliance on
   transitive dependency visibility.
6. The scope must not alter derivation arithmetic, BIP-39 salt behavior,
   random retry limits, signatures, errors, wire values, or feature names.
   Existing published and KMP vectors are the compatibility evidence.
7. The change is generic sdk-rust hardening and needs no consumer mutation,
   product policy, key storage, release, repository setting, or `main` action.

Verdict: READY to implement after strict OpenSpec validation.

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

# Post-implementation semantic, security, and API review

- **Reviewed head:** `631a40332b32295a7a9a52b2a8f2b840c97b26bf`
- **Result:** passed with no unresolved finding

The exact `develop...631a403` production diff adds an optional direct
`zeroize` dependency to secret-bearing features, replaces derived HD debug
formatting with metadata-only output, gives both HD types explicit and
drop-time erasure, and guards SDK-owned curve, DH, mnemonic, HMAC, salt, and
PBKDF2 intermediates. There is no algorithm, retry, signature, derivation,
salt, wire, error, target, custody, or secret-return API change.

The dependency is absent from the representation-only
`--no-default-features` graph and present when a secret-bearing feature is
selected. The locked graph gains no new package. The public raw HD fields
remain compatible and explicitly documented as caller-owned when copied.

Safe tests prove `Zeroize + ZeroizeOnDrop` trait availability, metadata reset
under explicit erasure, exact redacted debug text, and all inherited vector
outputs. The review intentionally does not inspect memory after drop: such a
test would require invalid or unsafe access and would overstate best-effort
erasure. The specification accurately excludes compiler copies, allocator
state, swap, dumps, hardware, and caller copies.

Host verification passed focused all-feature crypto tests, five supported
single-feature compile checks, the representation-only minimal build, factory
validation, workspace all-feature tests, strict Clippy, and warning-denied
docs. The full pinned Nix gate passed all 31 checks, including Rust 1.85 MSRV,
native, Android ARM64, iOS ARM64, browser WASM, minimal/KMP feature lanes,
supply-chain policy, and nextest (260/260 workspace tests plus focused feature
lanes).

Attempts to run all integration tests with only one selected feature exposed
the pre-existing fact that those integration-test files are not individually
feature-gated. They failed at imports of intentionally disabled modules, not
in changed production code; the repository-supported single-feature `cargo
check` lanes and the declarative Nix feature gates passed. No gate was waived
or made less strict.

The Nix run emitted the already-recorded Darwin archive-fixup segmentation
diagnostics, Rust 1.85 future-target cfg warnings, and offline audit-index yank
lookup diagnostics; derivations continued and ended with `all checks passed!`.
No consumer repository, `main`, release, publication, setting, or secret was
changed.

Verdict: READY to synchronize, archive, receipt, and publish as an issue-linked
signed/DCO pull request for exact-head hosted review.

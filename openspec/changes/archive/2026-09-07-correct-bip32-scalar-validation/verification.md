# Verification receipt

## Identity and provenance

- Delivery issue: `#153`; parent dependency decision: `#151`.
- Exact base: `f94028b1a09c80c55e64647c20ed2bac95f72819`
  (`origin/develop` after issue #152 merged).
- Reviewed implementation head: `b4b64b3c520e17020bdbdefe2db4694886774432`.
- Rejected `bip32 0.5.3` release source:
  `240679a2454945783acc4f9e7d3bae839359b0b7`; crates.io artifact checksum:
  `db40d3dfbeab4e031d78c844642fa0caa0b0db11ce1607ac9d2986dff1405c69`.
- Apollo reference: `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c`.
- Primary compiler: Rust `1.98.1`; effective workspace MSRV: Rust `1.85.0`.

## Local gates

- Focused tests passed 7/7 new private edge cases and 28/28 derivation tests.
  The latter retain official BIP-32 vectors 1–4, Apollo master and
  `m/0'/0'/0'`, BIP-39 and SLIP-0010 behavior.
- `nix flake check --print-build-logs` passed all 28 aarch64-Darwin-compatible
  checks for content-identical implementation head `b4b64b3`. The workspace
  suite passed 607/607 with 22 skipped diagnostics; the KMP crypto lane passed
  113/113.
  Rust 1.85, Rust 1.98, nightly etalon, WASM, Android, iOS, docs, formatting,
  Clippy, audit, deny, factory and all feature-specific lanes passed.
- A current online `cargo audit` through devshell auditor 0.22.2 loaded 1,242
  RustSec advisories and found no vulnerability in 135 dependencies.
- Strict OpenSpec, research, constraint, archive-preservation and
  `git diff --check` gates passed.

## Conformance, API and dependency receipt

- Every seed length from 16 through 64 bytes succeeds with deterministic test
  inputs; lengths 15 and 65 fail. Synthetic master zero/order, child order,
  zero result and depth overflow fail through `Error::DerivationFailed`.
- A synthetic zero child tweak succeeds, retains the parent private scalar,
  adopts the new chain code and advances metadata exactly as BIP-32 requires.
- `cargo-public-api 0.52.0` reports no removed, changed or added item between
  `origin/develop` and the implementation head with all crypto features.
- No manifest or lockfile changed. `bip32`, `bs58` and `ripemd` do not enter the
  dependency graph; minimal/default/KMP graphs remain the already-gated base.
- Artifact SHA-256: implementation
  `178c84d66d07d0d43b27eeba5c379006138be19958fa8b89312a69731285f9b9`;
  research `3dc67a6de39d58da6a130f6a5acae0a74e13114b44fda18dae802c9fa0cf6335`;
  unchanged lockfile
  `5eca413ab59f652dc5fd2c3ec041247e828bce7cb30e2ef8c26b9cc0de079dd7`.

## Residuals and review result

- The raw `HDKey` fields remain public caller-copyable secret material under
  the existing compatibility contract. The owned key itself remains
  `Zeroize + ZeroizeOnDrop` with redacted formatting.
- BIP-32's “proceed to the next index” instruction is represented as a failure
  for this explicit-index API; the caller retains path-selection policy.
- Compile gates prove target compatibility, not runtime device certification or
  guaranteed physical-memory erasure.
- The fresh correctness/security review found no unresolved blocker. The
  change is ready for guarded archive and hosted CI; merge remains prohibited
  until every required PR check is green.

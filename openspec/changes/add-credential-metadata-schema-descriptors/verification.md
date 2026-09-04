# Verification evidence

- **Date:** 2026-09-05
- **Issue:** #75, child of #6 / `IDR-007` and #20
- **Develop base:** `9aa33fcf8d2a6755270e46e435beae25f9ca75cf`
- **Reviewed implementation:**
  `b605adb7f8d626607f6ef46c3c66dc48030802bb`
- **Local platform:** `aarch64-apple-darwin`
- **Result:** every applicable local gate passed

## Focused and workspace gates

- `cargo test -p identus-credentials --all-features`: 34 passed and two manual
  diagnostics ignored, including 13 new metadata tests.
- Focused no-default build, strict Clippy, warning-denied rustdoc, and root
  formatting passed.
- `cargo test --workspace --all-features` and
  `cargo test --workspace --no-default-features` passed.
- Workspace strict Clippy, warning-denied rustdoc, formatting,
  `git diff --check`, and `./scripts/factory check` passed.
- Factory validation accepted 20 active capability/change items; the bootstrap
  inventory remained valid at 14 packages and the upstream backlog at 30 rows.

`nix flake check --print-build-logs` passed all 26 applicable locked
`aarch64-darwin` checks. This includes Rust 1.85 MSRV and minimal-feature
builds, release Nextest, KMP and entropy matrices, strict Clippy, rustdoc,
formatting, WASM, Android AArch64, iOS AArch64, architecture/factory contracts,
cargo-deny, pinned RustSec audit, and Nix/TOML/text linting. The default matrix
ran 294 tests: 294 passed and 11 were skipped; the KMP profile ran 90 tests: 90
passed and one was skipped. Nix reported `x86_64-linux` as incompatible with
the local host; hosted CI owns Linux verification.

The known hermetic cargo-audit derivation cannot resolve yanked-package state
from its offline index but returns success after scanning the pinned advisory
database. The known non-fatal Darwin `audit-tmpdir.sh` fixup warning also
appeared. Both are baseline toolchain diagnostics, not branch regressions.

## Performance observation

The ignored release diagnostic constructed 100,000 representative metadata
values in `86.718625ms`, approximately `1,153,155 operations/s`, using
`rustc 1.95.0` on an Apple arm64 host. This is observational evidence without
a portable pass threshold.

## Compatibility and provenance

- No manifest, dependency, feature, lockfile, wire format, serializer, parser,
  adapter, storage surface, FFI, or consumer repository changed.
- Oxid remained at `bfe3b481568dc738f0732c2b27548fab8721fd95` on
  `integration`; its inspected source digest remained
  `dfc3317d3789c63a1953d0ad39ceb2f07f2329b1238aa93edb9133e35ee833a8`.
- Midnight Identity remained at
  `427f8571950c42967a18726cbcbefecc19ef8d79` on `develop`; its two inspected
  digests remained `ae6c4da7dbef1f46474cd1d6979b59e58315ade0b528cc41ca37ce9c2ffd58d1`
  and `92e77200bc3eb7405266f89525bca59a55c1db0256696d134998f66b2d67c9d9`.
- Lace ID Portal remained at
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` on `main`; its two inspected
  digests remained `a271d3403556340e24aa632b714945813e6399a1c1091388b3cc813a4b1bc5af`
  and `e7e5667fcc307d165928c01f9da290650caf0c65a60f6757378f3ea47d5d98cf`.
- NeoPRISM remained clean at
  `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` on `main`. Apollo remained at
  `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` on `main` with its pre-existing
  modified nested `secp256k1` checkout. All other pre-existing untracked or
  nested-worktree statuses also matched preflight.

## Local iteration effort

From issue creation at `2026-09-04T18:58:43Z` through complete local review and
the reproducible matrix at `2026-09-04T19:16:27Z`, elapsed delivery time was
17 minutes 44 seconds. Hosted CI, review, and merge timing is recorded on the
pull request and issue.

# Verification

- **Date:** 2026-09-17
- **Planning head:** `ff7046727f751263916289043f7459c8f743582d`
- **Preimplementation receipt head:**
  `2530acb29d721ae4f651d966c98cda0478c86073`
- **Implementation scope:** release compiler/support policy, Nix evidence,
  candidate metadata, validators, tests, and public documentation

## Passed locally

- `python3 scripts/check-support-policy.py`: compiler, host, target, profile,
  limitation, and Nix-provider contract passed.
- `python3 scripts/tests/support-policy.py`: 212 mutation tests passed.
- `./scripts/factory check`: factory/OpenSpec structural contract passed.
- Canonical Nix primary/MSRV host suite: build, formatting, Clippy, docs,
  policy, candidate, and 764 workspace tests passed on macOS ARM64.
- Nix primary profile gates passed for hash-only and no-default
  `kmp-compat`; MSRV 1.89.0 passed workspace, all-features, minimal,
  hash-only, and no-default `kmp-compat` gates.
- Nix primary and MSRV compile gates passed for `wasm32-unknown-unknown`,
  `aarch64-linux-android`, and `aarch64-apple-ios`.
- `scripts/prepare-crypto-candidate.py --package-only` constructed all three
  `0.1.0-rc.1` archives in dependency order under Rust 1.98.1 and recorded
  declared MSRV 1.89.0 plus all five candidate profiles.
- `git diff --check`: passed.

## Receipt boundary

The local package-only receipt was generated from the preimplementation head
with `sourceDirty=true`, took 41.525 seconds, and marked publication
`prohibited`. Its checksums are development evidence only. After merge, #325
and #326 receive the exact protected `develop` revision and hosted fast result;
the unchanged revision must then receive complete slow/frozen-candidate
evidence under #326 before release approval.

## Excluded claims

No tag, registry resolution, `cargo publish`, GitHub release, `main`
promotion, downstream mutation, runtime target test, FFI package,
certification, or production-support claim was performed.

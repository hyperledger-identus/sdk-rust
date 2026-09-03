# Verification evidence

- **Date:** 2026-09-03
- **Issue:** #35, child of #5 / `IDR-005`
- **Develop base:** `267b435ece23d935fc4a4946df0b496c91466e66`
- **Reviewed implementation head:** `f959fb289eb9f6c78bbc22eadf0e0fc274ba897c`
- **Local platform:** `aarch64-darwin`
- **Result:** every applicable local gate passed

## Fuzz campaign evidence

| Command | Result |
| --- | --- |
| `./scripts/fuzz-did.sh replay all` | passed; 10 executions per target across 18 committed raw seeds |
| `./scripts/fuzz-did.sh smoke all` | passed; fixed seed `424242`, 4,096 executions per target, no finding |
| fuzz `cargo fmt --check` | passed |
| fuzz strict Clippy | passed with warnings denied |
| fuzz cargo-deny | passed bans, licenses, sources, and advisory policy |
| fuzz RustSec audit | passed; 74 locked dependency packages scanned |

The exact-head smoke discovered 104 new in-memory `Did` corpus units and 214
`DidUrl` units without a sanitizer or invariant failure. Peak RSS was 42 MiB
and 45 MiB respectively under the 1 GiB cap. The `Did` campaign completed below
libFuzzer's one-second reporting resolution; the sequential wrapper reported
one second for `DidUrl`. These observations prove the PR budget is small but do
not create a machine-specific performance threshold.

Both targets also ran exact/over-limit startup probes. The generator ceiling
was 8 KiB, individual timeout five seconds, mutation reload disabled, one
process used, and generated corpus growth stayed in a temporary directory.
There were no artifacts to minimize or promote.

## Focused and workspace gates

The following passed at the implementation head:

- `cargo test -p identus-did` (114 passed, 8 manual diagnostics ignored);
- focused strict Clippy, rustdoc with warnings denied, and root formatting;
- `cargo test --workspace --all-features`;
- `cargo test --workspace --no-default-features`;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
- workspace rustdoc with warnings denied and root formatting;
- shellcheck, actionlint, yamllint, Taplo, and editorconfig-checker;
- strict OpenSpec/factory validation (18 capability/change items);
- `git diff --check` against the exact develop merge base.

`nix flake check --print-build-logs` passed all 26 applicable locked
`aarch64-darwin` checks. This includes Rust 1.85 MSRV and minimal-feature
builds, workspace release tests, entropy/KMP matrices, strict Clippy, rustdoc,
formatting, WASM, Android AArch64, iOS AArch64, architecture/factory contracts,
cargo-deny, pinned RustSec audit, and Nix/TOML/text linting. Nix correctly
reported `x86_64-linux` as an incompatible local system; hosted CI owns Linux
and the sanitizer workflow itself.

## Compatibility and dependency evidence

- No existing root workspace manifest, root lock, production Rust source,
  public type, serialization, feature, or support-policy record changed.
- The independent fuzz manifest pins cargo-fuzz's runtime and serde helpers;
  `identus-did` and all published dependency cones remain unchanged.
- The NCSA exception is restricted to exact `libfuzzer-sys@0.4.13`; the general
  license allow-list is unchanged.
- All five downstream receipts and the clean reserved `main` worktree match
  preflight. No release, publication, administration, consumer, or chain state
  was touched.

## Effort evidence

- **Estimate:** 2–4 active agent-hours, excluding hosted CI and scheduled soak.
- **Actual through implementation, correction, review, and complete local
  gates:** 1 hour 24 minutes 28 seconds wall-clock
  (`09:44:39Z`–`11:09:07Z`).
- **Hosted CI/review/merge:** recorded separately on the pull request and issue.

# Verification evidence

- **Date:** 2026-09-03
- **Issue:** #58, child of #9 / `IDR-004`
- **Develop base:** `b300240b2bf2b1ccd9c5085c6595a6c295cec6ce`
- **Reviewed implementation head:** `cf2b664ebf21781d88a5e8d7ca12400d5bb59212`
- **Local platform:** `aarch64-darwin`
- **Result:** every applicable local gate passed

## Fuzz campaign evidence

| Command | Result |
| --- | --- |
| `./scripts/fuzz-crypto.sh replay all` | passed; seven executions per target across twelve committed text seeds |
| `./scripts/fuzz-crypto.sh smoke all` | passed; seed `424242`, 4,096 executions per target, no finding |
| `./scripts/fuzz-did.sh replay all` | passed unchanged; ten executions per target |
| `./scripts/fuzz-did.sh smoke all` | passed unchanged; seed `424242`, 4,096 executions per target, no finding |
| fuzz formatting and strict Clippy | passed for all four binaries |
| fuzz cargo-deny | passed advisories, bans, licenses, and sources |
| fuzz RustSec audit | passed; 74 locked dependency packages scanned |

The final fixed crypto smoke discovered 149 in-memory JWK units and 159 COSE
units without sanitizer or invariant failure. Peak RSS was 50 MiB and 53 MiB
respectively, below the 1 GiB ceiling; the wrapper reported one second per
target. The repeated DID smoke discovered 60 and 168 units at 42 MiB and 45
MiB. These values are diagnostics rather than machine-specific thresholds.

All campaigns used an 8 KiB generated-input ceiling, five-second per-input
timeout, one worker, disabled reload, and temporary writable corpus copies.
The COSE target executed its one-time 4,096/4,097-byte boundary probes. No
artifact required minimization, promotion, or a production correction.

## Focused and workspace gates

The following passed at the reviewed implementation head:

- crypto all-feature tests (85 passed, one manual diagnostic ignored);
- crypto `cose,jwk-thumbprint` minimal library/JWK/COSE tests (32 passed, one
  manual diagnostic ignored);
- crypto and workspace rustdoc;
- root and fuzz formatting;
- fuzz and workspace strict Clippy;
- workspace all-feature and no-default-feature tests;
- shell syntax and `git diff --check`;
- EditorConfig Checker, Yamllint, and ShellCheck after hosted-CI correction;
- strict factory/OpenSpec validation (18 capability/change items);
- independent-lock cargo-deny and RustSec audit; and
- `nix flake check` with all 30 applicable `aarch64-darwin` checks.

The Nix matrix includes Rust 1.85 MSRV and crypto minimal-feature builds,
WASM32, Android AArch64, iOS AArch64, workspace nextest matrices, strict
Clippy/rustdoc/formatting, dependency policy, RustSec, architecture/factory
contracts, and Nix/TOML/text linting. Nix reported `x86_64-linux` as locally
incompatible; hosted CI owns Linux and the AddressSanitizer workflow.

Two exploratory commands were corrected without changing code. Direct
`openspec` was absent from the ambient shell, so `scripts/factory check`
executed the pinned Nix copy and passed. An initial minimal-feature command
selected every integration test, including unrelated feature-dependent tests;
the focused `--lib --test jwk --test cose` command is the applicable gate and
passed. No required gate remains failing or unrun.

The first hosted file-hygiene run failed before inspecting repository files
because the organization workflow's old setup action searched for the removed
`ec-linux-amd64` asset after upstream v4 became `latest`. Correction head
`513f576` pins the upstream verified v4-aware action commit and exact checker
`v4.0.0`, preserves the other immutable lint actions locally, and explicitly
runs the checker. The initial PR-policy run also required the exact receipt
line `Local review: passed`; the PR body now contains it. Both corrections are
branch-owned delivery repairs and receive fresh hosted checks on push.

## Compatibility and downstream evidence

- No production source, root manifest/lock, public API, wire shape, feature,
  target policy, persisted data, consumer, release, or `main` branch changed.
- The independent package/lock contains exact fuzz-only `hex@0.4.3` and
  `libfuzzer-sys@0.4.13`; published dependency cones remain unchanged.
- End-of-review remote receipts remain NeoPRISM `180f1eb`, Apollo `ccee22b`,
  and midnight-identity `5cb0590`. Lace and Oxid independently advanced to
  `68c9be9` and `644e99d`; neither was touched by this branch.
- Local Apollo, NeoPRISM, midnight-identity, Lace, and Oxid worktrees retain
  their previously recorded user-owned status. The clean reserved sdk-rust
  `main` worktree remains `2c267d65af5c`.

## Effort evidence

- **Estimate:** 2–4 active agent-hours, excluding hosted CI and scheduled soak.
- **Elapsed through issue creation, implementation, repeated campaigns,
  review, and complete local gates:** 6 hours 15 minutes
  (`12:44:36Z`–`18:59:24Z`), including automation pauses and Nix builds.
- **Active agent effort:** not separately metered; no fabricated estimate is
  reported as actual.
- **Hosted CI/review/merge:** recorded on the pull request and issue.

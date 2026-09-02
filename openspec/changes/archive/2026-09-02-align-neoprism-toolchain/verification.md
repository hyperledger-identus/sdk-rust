# Verification evidence

Verified on 2026-09-02 from `codex/align-neoprism-toolchain`, based on
`origin/develop` at `6399b880d5304e8d139ebbcb43d744b827731311`.

## Reference receipt

| Reference | Value |
| --- | --- |
| NeoPRISM repository | `hyperledger-identus/neoprism` |
| Inspected revision | `8becb225132efb1d9302b2c5f6ed4d87b84e8685` |
| Rust channel | nightly `2026-03-18` |
| rust-overlay | `f17186f52e82ec5cf40920b58eac63b78692ac7c` |
| nixpkgs | `c27cdad491a991b11ed731760aa2ef8db0cb0410` |
| Resolved `rustc` | `1.96.0-nightly (91021ccc7 2026-03-17)` |
| Resolved `cargo` | `1.96.0-nightly (cbb9bb8bd 2026-03-13)` |
| Resolved Nix package | `2.34.8` |

## Passing gates

| Gate | Result |
| --- | --- |
| Strict OpenSpec validation | Passed for `align-neoprism-toolchain` |
| `scripts/factory check` | Passed all 12 active specs/changes |
| `cargo fmt --all -- --check` | Passed in the pinned devshell |
| Strict workspace clippy | Passed with all targets and features |
| Workspace tests, all features | Passed including all UI and doc tests |
| Workspace tests, no default features | Passed including the KMP feature gate |
| Workspace docs | Passed with `--no-deps` |
| Workspace WASM build | Passed for `wasm32-unknown-unknown` |
| `nix flake check --print-build-logs` | Passed all 13 `aarch64-darwin` checks |

Both hermetic Nextest variants ran 104 tests and passed all 104. The flake gate
also passed factory, Nix/TOML/text lint, format, both clippy variants, docs,
WASM, dependency policy and the advisory derivation.

The pinned compiler required three text-only `trybuild` fixture updates from
`associated function or constant` to `function or associated item`. The
negative cases and tested public behavior did not change.

## Environment diagnostics

- The macOS flake run omitted incompatible `x86_64-linux` outputs; GitHub CI
  remains the Linux confirmation.
- Nix's macOS `audit-tmpdir.sh` helper emitted non-fatal segmentation warnings
  during some fixup phases. All affected derivations and the final flake gate
  completed successfully.
- The existing hermetic `cargo audit` invocation reported that yanked-package
  lookups were unavailable in its offline index, then the derivation completed
  successfully. This pre-existing advisory-gate behavior should be hardened in
  a separate change; it is not represented here as a clean vulnerability scan.

## Boundary evidence

- `main` remains at `2c267d65af5c6b6dc9c8fd6826266c8ad0c3256a`.
- The feature branch is isolated in a dedicated SDK worktree.
- NeoPRISM was inspected at its remote revision; its working tree was not
  switched, staged or edited.
- No downstream consumer source was changed.

Independent review and repository-hosted Linux CI remain required before merge.

## Pre-archive readiness receipt

```text
Change: align-neoprism-toolchain
Branch: codex/align-neoprism-toolchain
Head SHA: 6399b880d5304e8d139ebbcb43d744b827731311
Develop merge base: 6399b880d5304e8d139ebbcb43d744b827731311
Factory contract: passed
Product-specific gates: recorded above
```

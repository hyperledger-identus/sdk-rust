## Context

The existing gate generator has two compiler classes: `etalon` and `msrv`.
Ordinary build, lint, test, docs and target gates use the etalon nightly. The
new policy needs three non-interchangeable providers without weakening the
manifest-derived gate contract.

## Decisions

### Keep three explicit compiler providers

`craneLib` and `cargoArtifacts` become Rust 1.98.1 primary-stable providers.
`etalonCraneLib` and `etalonCargoArtifacts` retain NeoPRISM's nightly. The
existing `msrvCraneLib` remains Rust 1.85. The gate manifest selects exactly
`primary`, `etalon` or `msrv`.

### Add one bounded etalon gate

All existing quality, feature and compile-target gates move to primary stable.
A separate locked workspace build proves the etalon remains compatible. It
does not duplicate all primary checks or substitute for MSRV.

### Preserve the NeoPRISM Nix baseline

The original rust-overlay and nixpkgs pins remain unchanged. A separately
named current-stable overlay is imported only to obtain Rust 1.98.1. Its exact
revision is recorded in the support policy and flake lock.

### Keep MSRV activation separate

This change records Rust 1.89 as the next candidate but intentionally leaves
Cargo `rust-version`, the MSRV toolchain and every MSRV gate at 1.85. A later
issue must prove supported targets and named consumers before activation.

## Risks and mitigations

- Two overlays can be cross-wired: the offline validator binds every provider,
  revision, toolchain and artifact class and negative tests exercise drift.
- Current-stable churn can become automatic: the exact 1.98.1 patch and overlay
  revision are pinned; future updates require a focused issue.
- A single etalon build may miss nightly-only diagnostics: the SDK prohibits
  nightly-only code; etalon is forward/integration evidence, not the primary
  quality compiler.
- Host success can overstate mobile/WASM support: existing compile-only tiers
  and limitations remain unchanged, and CI supplies its declared target gates.

## Rollout

Update the contract and tests, update the flake input/lock and three provider
graph, migrate gates, run focused and full checks, then archive the OpenSpec
change and open an issue-linked PR to `develop`.

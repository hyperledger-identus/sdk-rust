## Context

NeoPRISM revision `8becb225132efb1d9302b2c5f6ed4d87b84e8685`
pins Rust nightly `2026-03-18`, rust-overlay
`f17186f52e82ec5cf40920b58eac63b78692ac7c` and nixpkgs
`c27cdad491a991b11ed731760aa2ef8db0cb0410`. The latter supplies Nix
`2.34.8`. The SDK instead uses `stable.latest` and a different nixpkgs input.

## Goals / Non-Goals

**Goals:**

- reproduce NeoPRISM's Rust and Nix baseline exactly;
- keep the alignment visible and reviewable;
- retain the SDK's existing checks, WASM target and development tools;
- avoid coupling SDK source code to NeoPRISM.

**Non-Goals:**

- changing the Rust 1.85 MSRV;
- introducing nightly-only language or library features;
- copying NeoPRISM's full flake, dependencies or CI topology;
- automatically tracking future NeoPRISM updates;
- resolving all remaining B01 stabilization work.

## Decisions

### 1. Copy immutable pins, not a moving branch reference

The Rust date and locked input revisions are recorded directly in SDK-owned
files. This preserves reproducibility if NeoPRISM advances and permits each
future synchronization to receive focused review.

### 2. Align only the shared baseline

The SDK retains Crane, devshell, OpenSpec and its stricter action pinning. Those
are repository-specific factory choices and do not affect whether Rust and the
Nix package come from the NeoPRISM baseline.

### 3. Keep MSRV policy separate

The pinned nightly is the development and full-gate compiler. The Cargo MSRV
continues to describe consumer compatibility, so this change neither raises it
nor claims that an MSRV CI lane already exists.

## Risks / Trade-offs

- A nightly compiler can change semantics relative to stable releases. The
  exact date prevents unreviewed drift, and nightly-only features remain banned.
- NeoPRISM may update its pins frequently. SDK synchronization remains an
  intentional dependency change rather than an automatic lockfile update.
- Compiler-sensitive snapshots may need adjustment. The complete test gate
  determines whether any fixture update belongs in this change.

## Migration Plan

Update the toolchain expression and locked inputs, run the complete SDK flake
gate, synchronize the `nix-tooling` specification and merge through a reviewed
pull request to `develop`. Rollback is a revert to the prior lockfile and
`stable.latest` expression.

## Why

The SDK currently follows an unversioned stable Rust channel and a different
nixpkgs channel from NeoPRISM. The project sponsor has selected NeoPRISM as the
Identus Rust repository etalon, so the SDK needs an immutable, shared
toolchain baseline before further monorepo growth.

## What Changes

- Pin the SDK Rust toolchain to NeoPRISM's Rust nightly `2026-03-18`.
- Pin the root nixpkgs and rust-overlay inputs to the revisions used by the
  inspected NeoPRISM baseline.
- Record Nix `2.34.8` as the devshell package version supplied by that nixpkgs.
- Document that the declared Rust 1.85 MSRV remains unchanged and nightly-only
  language features remain prohibited.
- Keep SDK-specific flake modules, checks, targets and CI hardening unchanged.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `nix-tooling`: replace the moving stable channel with the immutable
  NeoPRISM-etalon Rust and Nix pin set.

## Impact

The change affects Nix inputs, the Rust toolchain expression, toolchain-facing
documentation and compiler-sensitive test execution. It does not change Rust
public APIs, workspace membership, the declared MSRV or downstream consumers.

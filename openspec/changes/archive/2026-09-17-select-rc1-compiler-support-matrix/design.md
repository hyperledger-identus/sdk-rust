# Design

## Compiler separation

Keep one primary/etalon provider on exact Rust 1.98.1 and restore a separate
minimal Rust 1.89.0 MSRV provider. Fast CI continues to select only primary
gates. Existing manifest-derived MSRV gates move to the lower provider in the
weekly/manual slow graph, so compatibility cost stays off the PR critical
path.

## Release matrix

Add a machine-readable release-candidate section naming exactly
`identus-derive`, `identus-core`, and `identus-crypto`, their version, compiler
pair, host tiers, compile-only targets, feature profiles, and review date.
Keep workspace-wide experimental support entries separate so the three-crate
publication cannot imply release of unrelated packages.

The release profiles are default, all-features, no-default-features,
`kmp-compat`, and hash-only. Hash-only gets independent primary Clippy/test and
MSRV build gates. The compile-only targets get both primary and MSRV builds of
the release train. Proc-macro dependencies build for the host as Cargo
requires; no target runtime is claimed for `identus-derive` itself.

## Candidate metadata

Published manifests carry `rust-version = "1.89.0"`. Candidate preparation
continues on Rust 1.98.1 and records both the preparation compiler and declared
MSRV. The release receipt lists the exact profiles and target/compiler
evidence; packaging does not silently redefine support.

## Validation

- Structural support-policy and mutation suites bind Cargo, Nix providers,
  gate operations, package/profile selections, and documentation.
- Rust 1.89.0 checks the workspace and isolated release profiles locally and
  in slow CI.
- Rust 1.98.1 runs normal build/test/Clippy/docs and candidate generation.
- Primary/MSRV cross-target gates compile the three release crates for browser
  WASM, Android ARM64, and iOS ARM64.
- The dependency boundary continues to reject chain/product crates and
  repositories.

## Rollback

Revert the policy, Cargo floor, MSRV provider, gates, candidate metadata, ADR,
and canonical specs together. The reverted state is explicitly
release-ineligible, so no published consumer migration exists before release.


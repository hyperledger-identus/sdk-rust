# Verification

Verification date: 2026-10-28

## Exact focused evidence

- `cargo test -p identus-prism-indexer --test indexing
  portable_stack_e2e_creates_resolves_updates_and_replays`: passed.
- `cargo test -p identus-prism-did -p identus-prism-vdr
  -p identus-prism-indexer --all-features --all-targets`: passed, including 52
  PRISM DID tests, 12 indexer integration tests, 9 VDR tests, and the registered
  benchmark targets.
- `cargo clippy -p identus-prism-did -p identus-prism-vdr
  -p identus-prism-indexer --all-features --all-targets -- -D warnings`:
  passed.
- `scripts/check-ssi-upstream-backlog.py`: 30 rows passed.
- `./bootstrap.sh --check`, `./scripts/factory check`, formatting, and
  `git diff --check`: passed.
- `nix build .#docs-site --no-link`: passed after every documentation
  correction, including mdBook, Graphviz rendering, and offline link checks.

## Reproducible full matrix

`nix flake check` passed all 44 compatible checks on aarch64-darwin with Nix
2.34.6 and repository Rust 1.98.1. The matrix included native and MSRV builds,
minimal/default/all-feature profiles, WASM, iOS, Android, strict Clippy,
rustdoc, formatting, factory and OpenSpec contracts, dependency policy,
`cargo-audit`, `cargo-deny`, and release Nextest suites. The incompatible
x86_64-linux system remains the hosted CI responsibility.

## E2E contract covered

The deterministic in-memory scenario constructs and signs a PRISM DID create
operation, creates a VDR entry in the same ordered atomic transaction, commits
through caller-owned state, projects the DID document, resolves the VDR root
and head, verifies caller-retained payload bytes, replays the exact transaction
without a second commit, then applies and resolves a bounded VDR update.

The test does not simulate Cardano, networking, SQL, HTTP, a service runtime,
or product policy. Those remain explicit downstream ports and stop boundaries.

## Review and provenance

Two iterative local Codex CLI reviews found and drove six documentation and
roadmap corrections. The final exact-head review of
`c763c3bd0020f9c726eb08f390629a1b353e5e01..7dc3722464bb1432fc972a45f85d1274661406ce`
reported no actionable defect. NeoPRISM was read-only evidence; no downstream
repository or protection was changed.


# Local review

- **Review date:** 2026-09-17
- **Review angle:** compiler independence, support-claim truthfulness, feature
  isolation, target scope, chain neutrality, release authority, and rollback
- **Planning head:** `ff7046727f751263916289043f7459c8f743582d`
- **Preimplementation receipt head:**
  `2530acb29d721ae4f651d966c98cda0478c86073`
- **Result:** passed after the KMP isolation finding below was resolved

## Resolved finding

The first implementation described `kmp-compat` as a no-default-features
candidate profile while its three Nix gates still inherited default features.
That would have overstated isolation. The primary Clippy/test gates, MSRV build
gate, and machine-readable feature record now all require
`--no-default-features --features kmp-compat`; the corrected gates pass.

## Contract review

- Rust 1.89.0 is the single published MSRV for the complete `0.1.x` line;
  Rust 1.98.1 is the independently pinned primary/etalon compiler.
- The normal pull-request fast lane remains one Linux Rust 1.98.1 lane. MSRV,
  all-feature, target, security, and candidate evidence remains slow/manual or
  release-scoped, so the support promise does not inflate iteration latency.
- The candidate contains only `identus-derive`, `identus-core`, and
  `identus-crypto`, with exact internal requirements and the five documented
  profiles: default, all-features, no-default-features, hash-only, and
  KMP-compatible.
- Linux x86_64 and macOS ARM64 are host-tested. WASM, Android ARM64, and iOS
  ARM64 are compile-checked; no runtime, FFI, app-store, browser, WASI, Windows,
  certification, or performance guarantee is implied.
- Future MSRV increases are prohibited during `0.1.x`. A later pre-1.0 minor
  requires a new ADR, dependency audit, consumer evidence, and migration note.
- The SDK owns only generic primitives and ports. Consumer repositories,
  chains, DID methods, ledger/runtime types, domain tags, and product policy
  remain downstream and are not introduced by this change.

## Security and compatibility review

This change adds no dependency, unsafe block, build script, network operation,
runtime capability, wire shape, chain primitive, or publication authority. It
changes only compiler declarations, independently pinned Nix toolchains,
evidence gates, release metadata, validators, and public support statements.

The preparatory package-only receipt is deliberately dirty and unpublished;
it proves archive construction but is not the immutable release receipt. The
post-merge exact revision must be rebuilt by the complete slow/release line
before any human release decision.

No unresolved blocking finding remains.

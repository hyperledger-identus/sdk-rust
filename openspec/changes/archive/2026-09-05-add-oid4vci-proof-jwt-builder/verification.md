# Verification receipt

## Scope

- Issue: #99, first independently reversible delivery
- Backlog: IDR-004 under #8 and #20
- Base: `0b2d5139f4ec626305dc3cfb7113b555506f1624`
- Branch: `codex/oid4vci-proof-builder`
- Specification commit: `41a29b0`
- Implementation commit: `febde98`
- Owner: `identus-jose`

This slice provides holder-side OID4VCI proof JWT construction. Issuer-side
signature, DID authorization, freshness, replay and trust policy remain the
second #99 delivery and are not claimed here.

## Focused evidence

- `cargo test -p identus-jose`: passed; compact conformance ran 11 cases with
  one manual diagnostic ignored, and proof-builder conformance ran six cases
  with one manual diagnostic ignored.
- `cargo test -p identus-jose --no-default-features`: passed.
- `cargo clippy -p identus-jose --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-jose --no-deps`: passed.
- `scripts/check-bootstrap-inventory.py`: all package records passed.
- `scripts/factory check`: all OpenSpec and factory records passed.
- `cargo fmt --all -- --check` and `git diff --check`: passed.

## Workspace and reproducible matrix

- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.
- `nix flake check`: all 26 compatible `aarch64-darwin` checks passed. This
  includes Rust 1.85 MSRV, native and minimal features, Android AArch64, iOS
  AArch64, browser-WASM, strict Clippy, documentation, formatting, release
  Nextest profiles, architecture/factory, dependency, license, advisory, Nix,
  TOML and text gates. Nix omitted incompatible local `x86_64-linux`; hosted
  Ubuntu CI supplies that independent confirmation.

## Performance diagnostic

`cargo test -p identus-jose --release --test oid4vci_proof_builder
proof_preparation_throughput_diagnostic -- --ignored --nocapture` prepared
100,000 proof signing inputs in 79.423416 ms, approximately 1,259,075
operations/second. This is informational evidence, not a machine-independent
threshold.

## Review and boundaries

The distinct post-implementation review is recorded in `review.md`. Its one
allocation-bound finding was resolved before the implementation commit and the
full gate run; no blocker remains.

Oxid remains at `bfe3b481568dc738f0732c2b27548fab8721fd95` with its pre-existing
`.claude/` and `.pi/taskflows/` entries. Lace ID Portal remains at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with its pre-existing
`.pi-subagents/`, `.pi/` and `tmp/` entries. No consumer repository, SDK
`main`, live GitHub setting, chain state or release state was modified.

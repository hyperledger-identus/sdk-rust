# Verification evidence

## Focused codec gates

- `cargo test -p identus-jose --all-features`: 8 passed, 1 release diagnostic
  ignored by default.
- `cargo check -p identus-jose --no-default-features`: passed.
- strict all-target/all-feature Clippy and warning-denied docs: passed.
- direct browser WASM, Android ARM64 and iOS ARM64 checks: passed.
- release diagnostic: 100,000 parses in 110.21925 ms, approximately 907,283
  operations/second; measurement only.

## Architecture and factory gates

- `cargo test -p identus-conformance`: 24 passed.
- bootstrap inventory: 16 packages passed.
- SSI upstream backlog: 30 rows passed.
- support-policy mutation suite: 146 passed.
- strict OpenSpec/factory validation: 25 items passed before archive.

## Authoritative pinned Nix gate

`nix flake check --print-build-logs --keep-going` passed all 26 compatible
`aarch64-darwin` checks on the reviewed implementation. Evidence includes:

- principal release suite: 373 passed, 18 diagnostics skipped;
- Rust 1.85 workspace build including `identus-jose`;
- pinned-nightly WASM, Android ARM64 and iOS ARM64 builds explicitly including
  `identus-jose`;
- formatting, strict Clippy, warning-clean documentation, Nix/TOML/text lint;
- factory, license/source/ban and offline advisory gates.

The macOS Nix fixup hook emitted intermittent audit subprocess segmentation
diagnostics after successful derivations, but every derivation completed and
the final flake result was `all checks passed`.

## Donor isolation

- Oxid remained at `bfe3b481568dc738f0732c2b27548fab8721fd95`; both observed
  source digests and the Apache-2.0 license digest matched preflight, and only
  the pre-existing `.claude/` and `.pi/taskflows/` paths were untracked.
- Lace ID Portal remained at
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`; both observed source digests
  matched preflight, no repository license file appeared, and only the
  pre-existing `.pi-subagents/`, `.pi/` and `tmp/` paths were untracked.

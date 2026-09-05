# Verification evidence

## Focused guard gates

- `cargo test -p identus-conformance`: 22 passed.
- strict all-target/all-feature Clippy: passed.
- Rust formatting and diff hygiene: passed.

## Factory gates

- Strict active-change OpenSpec validation: passed.
- Bootstrap inventory: 15 packages passed.
- SSI upstream backlog: 30 rows passed.
- Support policy and factory structure: passed.

## Scope

Only verification source and specification evidence changed. The fix adds no
runtime dependency or public SDK behavior.

## Pinned Nix gate

The repository-pinned `nix flake check --print-build-logs` passed all 26
compatible `aarch64-darwin` checks. Evidence includes Rust 1.85 MSRV,
WASM/Android/iOS builds, strict Clippy, documentation, source/license/advisory
checks, factory contracts and the principal release suite: 363 passed, 17
diagnostics skipped.

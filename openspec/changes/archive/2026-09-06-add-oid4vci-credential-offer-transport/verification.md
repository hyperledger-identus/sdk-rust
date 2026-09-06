# Verification receipt

## Candidate

- Issue: #111, child of #7 and #20 / `IDR-023`
- Develop base: `ddf64219ada97f9cb2287cfcb6da808d98f8b1a9`
- Specification commit: `f73fa2fb3ccff9874cd0df40ae79e9986036b11f`
- Implementation commit: `e52d22c53ee4dab57d0c9541fe418c187173054b`
- Branch: `codex/oid4vci-credential-offer`
- Owner: unpublished `identus-oid4vci` protocol-semantics crate

## Focused and workspace evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test -p identus-oid4vci --all-features --locked`: 11 passed and one
  release diagnostic skipped.
- `cargo test -p identus-oid4vci --no-default-features --locked`: 11 passed
  and one release diagnostic skipped.
- `cargo clippy -p identus-oid4vci --all-targets --all-features --locked --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-oid4vci --all-features
  --locked --no-deps`: passed.
- `cargo test -p identus-conformance --locked`: 25 passed.
- `cargo test --workspace --all-features --locked`: passed.
- `cargo clippy --workspace --all-targets --all-features --locked --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --locked --no-deps`:
  passed.
- `scripts/tests/factory-contract.sh`: passed its 6/146/20/11 grouped suites
  and PR-policy contract.
- `./scripts/factory check`: 28 OpenSpec/factory items passed.
- `git diff --check`: passed.

## Full reproducible matrix

`nix flake check --print-build-logs` passed all 27 compatible
`aarch64-darwin` checks. The matrix included Rust 1.85 MSRV, native workspace,
strict Clippy, warning-denied docs, browser WASM, Android ARM64, iOS ARM64,
factory/policy/text/TOML/Nix lint, cargo-deny, cargo-audit, and release Nextest
lanes. The principal release suite ran 431 tests: 431 passed and 22 were
skipped. Dedicated getrandom and no-default crypto profiles also passed.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Ubuntu CI supplies that independent gate. Existing nonfatal offline crates.io
yanked-lookup and macOS fixup-hook diagnostics did not fail a derivation.

## Diagnostic and review evidence

The ignored release diagnostic parsed and consumed 40,000 embedded/reference
invocations in 21.097 ms, approximately 1,896,004 operations per second on the
local machine. This is observational evidence, not a portable threshold.

The distinct exact-diff review is recorded in `review.md`. It corrected the
unsupported error taxonomy, an authority-normalization edge, and standards
fixture provenance. Focused tests and strict lint/docs passed after correction;
no finding remains unresolved.

## Repository isolation and deferred scope

Oxid remains at `bfe3b481568dc738f0732c2b27548fab8721fd95` with its pre-existing
untracked `.claude/` and `.pi/taskflows/` paths. Lace ID Portal remains at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with its pre-existing untracked
`.pi-subagents/`, `.pi/`, and `tmp/` paths. Neither consumer was edited.

`IDR-023` remains `in_progress`: offer semantics, metadata, authorization,
token, nonce, credential, deferred, notification, state-machine, networking,
consumer adoption, publishing, release, and `main` remain outside this slice.

# Verification receipt

## Scope

- Issue: #104
- Backlog: IDR-004 follow-up
- Base: `25e388b4ed62a7dac98b140855f31d974c9cff11`
- Branch: `codex/oid4vci-attestation-capabilities`
- Owner: `identus-jose`
- Specification commit: `426434d36035565dc4ad9dcc1f8b70d89aa0be37`

## Provenance and repository isolation

No donor code or fixture was copied. The contract is derived from OpenID4VCI
1.0 Final Appendix D.1/F.1 and OpenID Federation 1.0 Final section 4. Consumer
repositories were inspected read-only and retained their exact preflight state
at postflight:

| Repository | Revision | Existing state preserved |
| --- | --- | --- |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | `?? .claude/`, `?? .pi/taskflows/` |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | `?? .pi-subagents/`, `?? .pi/`, `?? tmp/` |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | dirty nested `third_party/midnight-did` |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` | clean |

No consumer file, branch, index or worktree was modified.

## Focused and workspace evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test -p identus-jose --test oid4vci_trust_evidence`: 8 passed.
- All focused JOSE test binaries: 53 passed and 4 manual diagnostics ignored.
- `cargo clippy -p identus-jose --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-jose --all-features
  --no-deps`: passed.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.
- `./scripts/factory check` and `git diff --check`: passed.

## Full reproducible matrix

The final pre-archive `nix flake check --print-build-logs` passed all 25
compatible aarch64-darwin checks. Rust 1.85 MSRV, native, Android AArch64, iOS AArch64,
browser WASM, feature variants, strict Clippy, rustdoc, formatting, factory,
text/Nix lint, cargo-deny and all release Nextest suites passed. The principal
release suite ran 419 tests: 419 passed and 21 manual diagnostics were skipped.
An earlier run identified only archive-intent TOML alignment after all Rust and
target lanes had passed. The file was formatted, the exact `lint-toml`
derivation passed, and the complete matrix above was then rerun successfully.

The local invocation reports `x86_64-linux` as incompatible with the host;
hosted Ubuntu CI supplies that independent gate. Existing nonfatal offline
crates.io yanked-lookups and macOS fixup-hook diagnostics did not fail a
derivation.

After `scripts/factory archive` safely updated both canonical specifications
and validated all 27 current OpenSpec/factory items, the complete 24-check
post-archive Nix matrix was rerun against the archived tree and passed.

## Review

The distinct exact-diff review is recorded in `review.md`. Its two test-only
coverage findings were resolved and the affected focused tests, strict Clippy
and warning-denied docs passed afterward. No unresolved finding remains.

The first hosted Codex review on `3838d7fc264a6bacba9618ce5611551edfc4a6f5`
found that alphabet-only compact-token checking admitted undecodable Base64url
segments. Canonical decode/re-encode validation and constructor/raw-parser
regressions were added. The replacement head must repeat focused, full CI and
hosted-review gates before merge.

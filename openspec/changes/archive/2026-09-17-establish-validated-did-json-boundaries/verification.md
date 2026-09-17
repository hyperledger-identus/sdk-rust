# Verification

- **Date:** 2026-09-17
- **Issues:** #315 and #297
- **Develop base:** `fee94946ca489f88dbc50282b5ee1eca095507f3`
- **Verified implementation head:** `4fda3dc53648034d327edf35b13416e5745c4649`
- **Environment:** aarch64-darwin, repository-pinned Rust 1.98.1 and Nix inputs

## Focused and workspace gates

- `cargo test -p identus-did --all-features`: passed, including context-object
  wire/accessor/redaction tests, 129-entry domain rejection, and hostile-depth
  cleanup across every native constructor family.
- `cargo clippy -p identus-did --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed,
  including the compile-fail raw-context-map example.
- `cargo fmt --all -- --check` and `git diff --check`: passed.
- `scripts/check-input-resource-boundaries.py .`: passed with 37 governed
  boundaries.
- `scripts/factory check`: passed all OpenSpec and repository contracts.

## Public API and supply-chain evidence

`cargo-public-api 0.52.0` rendered `identus-did` from Rustdoc JSON. The report
shows opaque `ContextObject`, fallible construction, borrowed/consuming
accessors, redacted `Debug`, validated `Deserialize`, and
`ContextEntry::Object(ContextObject)`. Its local SHA-256 was
`3412a7dce4b5699afda75ad5c9a0bb4f6f5fbb5cb3256e187a2d107fb26763dd`.

`cargo-cyclonedx 0.5.9` generated a CycloneDX 1.5 SBOM for `identus-did` with
22 components and SHA-256
`a654b5f3a711088250e8197d7ed58e810020424badb9a37e3298111119782d61`.
The artifact remained local and no dependency delta exists. `cargo deny check`
passed advisories, bans, licenses, and sources; its existing unmatched-allowance
and duplicate-`syn` diagnostics remained warnings.

## Portable and reproducible target closure

The following direct checks passed:

- `cargo check -p identus-did --all-features --target wasm32-unknown-unknown`;
- `cargo check -p identus-did --all-features --target aarch64-apple-ios`;
- `cargo check -p identus-did --all-features --target aarch64-linux-android`.

`nix flake check --no-write-lock-file` passed all 31 compatible
`aarch64-darwin` checks. The closure included repository contracts, formatting,
text/TOML/Nix lint, dependency audit and deny policy, Rust 1.98.1 etalon and
MSRV profiles, default/minimal/KMP/entropy builds and tests, strict Clippy,
Rustdoc, and WASM/iOS/Android compilation. Nix reported `x86_64-linux` as
incompatible with the local host; the required hosted `fast` lane supplies that
independent evidence.

An unscoped `taplo format --check` also inspected generated
`target/tests/trybuild` manifests and correctly reported them as unformatted.
Those build outputs are not governed source. The Nix `lint-toml` derivation,
which applies the repository's tracked/generated exclusions, passed.

## Exact-diff routing

`scripts/factory plan --base fee94946ca489f88dbc50282b5ee1eca095507f3
--head 4fda3dc53648034d327edf35b13416e5745c4649 --profile
production-ready` classified 26 paths and 1,625 changed text lines across Rust,
specification, and documentation. Required PR status is the Linux `fast` lane;
portable targets, security, and fuzz conformance remain slow-line
recommendations. The threshold decomposition note is recorded in `review.md`.

## Repository boundary

Midnight Identity PR #78 was inspected read-only at exact head
`2dcece66f17614968ce57d0aaa4786966763911a`; its checks were green at inspection
time. No downstream repository was mutated. The related integration remains an
explicit future adoption slice rather than part of this SDK invariant change.

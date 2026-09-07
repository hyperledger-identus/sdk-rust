# Verification evidence

- **Issue:** #139
- **Program:** IDR-023o under IDR-023 / issue #7
- **Develop base:** `74d264867a1ceb5f464ef452c99991863d3a1795`
- **Branch:** `codex/oid4vci-jwt-credential-request`
- **Specification commits:** `fb920aab07f2fa4bbd303fc46f4ad292b1aa99bb`,
  `599cafea3b08c31ef34388e7bf1b1220f9c565ad`
- **Implementation commits:** `45ff47ebe24d584ca25d1ad2391e31ea99457a37`,
  `1ffc217ef976a9d1613bb99367d092250c695a2d`
- **Environment:** aarch64-darwin with repository-pinned Nix and Rust
  toolchains

## Provenance and consumer evidence

OpenID4VCI 1.0 Final section 8.2 was inspected at SHA-256
`f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
RFC 6750 was inspected at SHA-256
`c458bb43ff32efb811466120efdec411010cdde537e59948342240a32dcd5a1c`.
Issue #139 records the exact source locations, consumer paths and Apache-2.0
license evidence. No donor or consumer source or fixture was copied.

Oxid remained clean at
`5ba38b9bbc9326c294b353daaf2a074eca18c22f`. Lace ID Portal remained at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with its pre-existing
`.pi-subagents/`, `.pi/` and `tmp/` untracked trees. Neither consumer was
edited, switched, staged or built.

## Focused capability gates

- `cargo test -p identus-oid4vci --test jwt_credential_request
  --all-features`: 8 passed.
- `cargo test -p identus-oid4vci --test jwt_credential_request
  --no-default-features`: 8 passed.
- Complete `identus-oid4vci` all-feature and no-default-feature suites:
  passed.
- Strict package Clippy and rustdoc: passed.
- Focused dependency-graph conformance: 11 passed.
- `cargo tree -p identus-oid4vci --depth 1`: the direct internal cone is
  exactly `identus-core` plus `identus-jose`; no feature or external
  dependency was added.

## Workspace and factory gates

- `cargo fmt --all -- --check`: passed.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed.
- `./scripts/factory check`: 39 active/canonical items passed with archive
  preservation.
- Bootstrap inventory, upstream backlog and support-policy contracts: passed.
- `git diff --check origin/develop...HEAD`: passed.

## Full reproducible matrix

`nix flake check --print-build-logs` passed all 31 compatible
aarch64-darwin checks. This included Rust 1.85 MSRV, native, Android AArch64,
iOS AArch64 and `wasm32-unknown-unknown` builds; default, minimal, KMP,
deterministic and getrandom feature lanes; formatting, Clippy, rustdoc,
factory, text/Nix/TOML lint, cargo-deny, cargo-audit and release Nextest.

The principal release workspace suite ran 545 tests: 545 passed and 22 were
skipped. The KMP profile ran 91 tests: 91 passed and one was skipped. Focused
minimal and entropy profiles also passed.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Ubuntu CI supplies that independent gate. Existing nonfatal offline crates.io
yanked-lookups and macOS fixup-hook diagnostics did not fail a derivation.

## Exact artifact receipts

- `crates/oid4vci/src/jwt_credential_request.rs`:
  `2c625591cf3141a6551c5f0883ff1296267e72e323d422ebd50ecfbb2e37468b`
- `crates/oid4vci/src/limits.rs`:
  `e7757810afbd0198ba9b783c693339d3d2da4a43df1ee85484011c15309910e7`
- `crates/oid4vci/tests/jwt_credential_request.rs`:
  `ab4ee359ed0395ffc7792d1735d3c9c461936edfd44579f8dd2adf88d626f1fd`
- `crates/conformance/src/guard/dep_graph.rs`:
  `d1e053593ebecc34459012149d43d2d0d7198cb924ffb3887b2e365766316897`

The distinct exact-diff review is recorded in `review.md`. Its single
documentation finding was corrected, and no unresolved finding remains.

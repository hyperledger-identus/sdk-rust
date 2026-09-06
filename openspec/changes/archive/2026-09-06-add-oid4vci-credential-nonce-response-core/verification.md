# Verification receipt

## Scope

- Issue: #131
- Backlog: IDR-023k under IDR-023
- Base: `bbffe94c8731b2cadc33949761e93bfffc7fbf39`
- Branch: `codex/oid4vci-credential-nonce-response-core`
- Owner: `identus-oid4vci`

## Normative provenance

- OpenID for Verifiable Credential Issuance 1.0 Final, published 2025-09-16,
  sections 7, 7.1, 7.2, 8.2, and 12.2.4; SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.

No normative or consumer source is copied. The implementation is an
independent expression of the reviewed body, extension, resource, and
correlation-sensitive lifecycle behavior.

## Consumer isolation

| Repository | Revision | Evidence and preserved state |
| --- | --- | --- |
| Oxid | `5ba38b9bbc9326c294b353daaf2a074eca18c22f` | clean; positive nonce fixture SHA-256 `20c1d1252e39dc09ada8398584d194f38f55b7f5f4b144099ae7daa1ccb3fbdc`; response parser SHA-256 `80ae940e1f74783f406922d90d0239ae1ddb86e01655ca56b2af8a2cd1545b05` |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | pre-existing `?? .pi-subagents/`, `?? .pi/`, `?? tmp/`; token port SHA-256 `23d356308e7c432f573d92dccc3d408c1218b66b53155aa48c58add5c7545174` |

Postflight revisions, status entries, and path digests match preflight exactly.
No consumer file was modified. Oxid is Apache-2.0 evidence; the private Lace
checkout has no detected SPDX license and is used only as behavior evidence.

## Focused evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test --locked -p identus-oid4vci --all-features
  --test credential_nonce_response_core`: 8 passed.
- `cargo test --locked -p identus-oid4vci --no-default-features
  --test credential_nonce_response_core`: 8 passed.
- `cargo clippy --locked -p identus-oid4vci --all-targets --all-features --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p identus-oid4vci --no-deps
  --all-features`: passed.
- `cargo tree --locked -p identus-oid4vci`: runtime cone unchanged;
  `identus-core`, `serde_json`, `uriparse`, and `zeroize` only.
- `./scripts/factory check` passed 35 items with one active change;
  backlog/archive-preservation and `git diff --check` passed.

## Workspace and reproducible matrix

- `cargo test --locked --workspace --all-features`: passed.
- `cargo test --locked --workspace --no-default-features`: passed.
- `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features
  --no-deps`: passed.
- `/nix/var/nix/profiles/default/bin/nix flake check --print-build-logs`:
  all 27 compatible aarch64-darwin checks passed, including Rust 1.85 MSRV,
  native, Android AArch64, iOS AArch64, browser-WASM, feature, lint,
  documentation, factory, dependency, license, advisory, and release Nextest
  lanes. The principal suite ran 520 tests: 520 passed and 22 skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Linux CI remains mandatory. Existing nonfatal offline yank-index and platform
evaluation warnings did not fail a derivation.

## Exact implementation evidence

- Specification commit:
  `1f733afad08b41c98564f457807496887a2253df`.
- Reviewed production implementation commit:
  `cd2027767a5626fe8c44f656b820d70c3d42da2e`.
- Production response-state SHA-256:
  `2ed191e8b6739ef8d868a871f4b05931a41902d86bcaec222b1a25eb4b41b8ba`.
- Strict scanner SHA-256:
  `10ec1180ff43da6cde6230a9d018ae122bd99bfbe2e18a42077ff2d8721d8470`.
- Focused test SHA-256:
  `4c0b7513711929320d471ace5611d39b4f829891fd6e08c24403e3b75a084dd6`.
- No manifest, lockfile, Cargo feature, dependency, consumer, chain, or product
  repository changed.
- Both existing commits have valid local signatures and DCO sign-offs.
- The distinct exact-diff review is recorded in `review.md`; no unresolved
  finding remains.

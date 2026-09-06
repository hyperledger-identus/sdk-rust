# Verification receipt

## Scope

- Issue: #127
- Backlog: IDR-023i under IDR-023
- Base: `39d8446aef19130e32a235c9f6be8734c40711ed`
- Branch: `codex/oid4vci-token-response-core`
- Owner: `identus-oid4vci`

## Normative provenance

- OpenID for Verifiable Credential Issuance 1.0 Final, published 2025-09-16,
  sections 6.2 and 13.10; SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- OAuth 2.0 RFC 6749 section 5.1 and Appendix A.4/A.12-A.17; RFC Editor HTML
  SHA-256
  `535362fa3b4ca668d4734c244c6ed8c811f2ccd7ea1612a4201e5d8922a74b4e`.

No normative or consumer source is copied. The implementation is an
independent expression of the reviewed response, extension, resource, and
secret-lifecycle behavior.

## Consumer isolation

| Repository | Revision | Evidence and preserved state |
| --- | --- | --- |
| Oxid | `5ba38b9bbc9326c294b353daaf2a074eca18c22f` | clean; `portal_response.rs` SHA-256 `80ae940e1f74783f406922d90d0239ae1ddb86e01655ca56b2af8a2cd1545b05`; `portal.rs` SHA-256 `d1a1d975ca8a740da1d51b6d93627a18811b96c9cccf0832ed92c86c9e7d48cb` |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | pre-existing `?? .pi-subagents/`, `?? .pi/`, `?? tmp/`; `token.rs` SHA-256 `0d36a7ab0b0902cae87ef1eb2673dc736e205f4a606b8590331775b9118f28b0` |

Postflight revisions, status entries, and path digests match preflight exactly.
No consumer file was modified.

## Focused evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test --locked -p identus-oid4vci --all-features`: 83 passed and one
  manual diagnostic skipped, including all ten new response tests.
- `cargo test --locked -p identus-oid4vci --no-default-features`: 83 passed
  and one manual diagnostic skipped.
- `cargo clippy --locked -p identus-oid4vci --all-targets --all-features --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p identus-oid4vci --no-deps
  --all-features`: passed.
- `cargo tree -p identus-oid4vci --edges normal`: runtime cone unchanged;
  `identus-core`, `serde_json`, `uriparse`, and `zeroize` only.
- `./scripts/factory check`, backlog/archive-preservation checks, and
  `git diff --check`: passed.

## Workspace and reproducible matrix

- `cargo test --locked --workspace --all-features`: passed.
- `cargo test --locked --workspace --no-default-features`: passed.
- `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps
  --all-features`: passed.
- `PATH=/nix/var/nix/profiles/default/bin:$PATH nix flake check
  --print-build-logs`: all 27 compatible aarch64-darwin checks passed,
  including Rust 1.85, native, Android AArch64, iOS AArch64, browser-WASM,
  feature, lint, documentation, factory, dependency, license, advisory, and
  release Nextest lanes.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Linux CI remains mandatory. Existing nonfatal platform evaluation warnings did
not fail a derivation.

## Exact implementation evidence

- Specification commit:
  `6cdf8f3c07b20d73d005cdb828f9a7d69c35a13b`.
- Reviewed production implementation commit:
  `7bedf9b8de1f47531e5c0af26b6d3930036f1784`.
- Production response-state SHA-256:
  `164a02152672f635100ab739ca8e383cb8d3820082105fba3f568d920ea1497f`.
- Strict scanner SHA-256:
  `31d2e6039ca4b098772621892d16d28ff2b88667a1f399da08d29f0aa030f4a1`.
- Focused test SHA-256:
  `b693c43daacfbb1c2bab56c7ddc73d55b665ec54853578a181620e0157905c9d`.
- No manifest, lockfile, Cargo feature, dependency, consumer, chain, or product
  repository changed.
- Both implementation commits have valid local signatures and DCO sign-offs.
- The distinct exact-diff review is recorded in `review.md`; no unresolved
  finding remains.

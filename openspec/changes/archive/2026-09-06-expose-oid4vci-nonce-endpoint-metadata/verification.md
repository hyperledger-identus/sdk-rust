# Verification receipt

## Scope

- Issue: #133
- Backlog: IDR-023l under IDR-023
- Base: `f9771a16707889bf516d1a23a25603ed95ed207b`
- Branch: `codex/oid4vci-nonce-endpoint-metadata`
- Owner: `identus-oid4vci`

## Normative provenance

- OpenID for Verifiable Credential Issuance 1.0 Final, published 2025-09-16,
  sections 7 and 12.2.4; SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.

No normative or consumer source is copied. The implementation is an
independent expression of the reviewed optional metadata, URL, resource and
redaction behavior.

## Consumer isolation

| Repository | Revision | Evidence and preserved state |
| --- | --- | --- |
| Oxid | `5ba38b9bbc9326c294b353daaf2a074eca18c22f` | clean; Final metadata fixture SHA-256 `7c8562a13310722ba2b554daaa1fd5f9e44e7757c6c2689ada0f85425e39aa71`; adapter SHA-256 `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af` |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | pre-existing `?? .pi-subagents/`, `?? .pi/`, `?? tmp/`; well-known handler SHA-256 `4dff93d21e02b221598c325a8afdc9ff7311b3c5041727652cea9ecac81419b1`; issuer routes SHA-256 `573004e23f015a7592dbab71675bef4ee0bf9c5eb70f0f31a44d600082ca4690` |

Postflight revisions, status entries and path digests match preflight exactly.
No consumer file was modified. Oxid is Apache-2.0 evidence; the private Lace
checkout has no detected SPDX license and is used only as behavior evidence.

## Focused evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test --locked -p identus-oid4vci --all-features`: 103 passed and one
  release-only diagnostic skipped.
- `cargo test --locked -p identus-oid4vci --no-default-features`: 103 passed
  and one release-only diagnostic skipped.
- `cargo clippy --locked -p identus-oid4vci --all-targets --all-features --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p identus-oid4vci --no-deps
  --all-features`: passed.
- `cargo tree --locked -p identus-oid4vci`: runtime cone unchanged;
  `identus-core`, `serde_json`, `uriparse` and `zeroize` only.
- `./scripts/factory check` passed 36 items with one active change;
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
  documentation, factory, dependency, license, advisory and release Nextest
  lanes. The principal suite ran 523 tests: 523 passed and 22 skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Linux CI remains mandatory. Existing nonfatal offline yank-index, platform
evaluation and Darwin fixup warnings did not fail a derivation.

## Exact implementation evidence

- Specification commit:
  `9eb7b503477c0ae3dab10443600ffa40b1f0590e`.
- Reviewed production implementation commit:
  `147d437e3da63e233b991d867a20f3a41c4b8634`.
- Metadata state SHA-256:
  `19830423e8ff33b96cc40dff4c83eeb0a8e9eae08068cf2dde47e18fbd4dd1a7`.
- Strict scanner SHA-256:
  `6645cf9ce509b307e7934f5c31fe6f9cefd7e4ba911747e43d7fe61cbfa7e6b0`.
- Focused test SHA-256:
  `961b94735a148e3fc3c1c62f2bb241679ad7fd2a60e08032e2ec9ae88a6828bc`.
- No manifest, lockfile, Cargo feature, dependency, consumer, chain or product
  repository changed.
- The four branch commits through archive commit
  `7562dd53b693257cbe77e8bacde6167e745157f5` have valid local signatures and
  DCO sign-offs. GitHub's ephemeral pull-request merge commit is CI input, not
  a repository-facing commit; the protected squash merge remains subject to
  the repository's verified-signature and DCO controls.
- The distinct exact-diff review is recorded in `review.md`; no unresolved
  finding remains.

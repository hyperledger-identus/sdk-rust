# Verification receipt

## Scope

- Issue: #121
- Backlog: IDR-023f under IDR-023
- Base: `04f5dc8bfe7df4f8089751af19e18fcf2f9410ae`
- Branch: `codex/oid4vci-pre-authorized-server`
- Owner: `identus-oid4vci`

## Normative provenance

- OpenID for Verifiable Credential Issuance 1.0 Final, published 2025-09-16,
  sections 11.2, 12.2.4, and 12.3; SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- RFC 8414 sections 2 and 3; SHA-256
  `16c816e4e0fdbffb7e910ff3017867bf39debe9cb7f52f5cbc508a052ed660e8`.

No normative or donor source is copied. The implementation is an independent
expression of the reviewed cross-document behavior.

## Consumer isolation

| Repository | Revision | Evidence and preserved state |
| --- | --- | --- |
| Oxid | `5ba38b9bbc9326c294b353daaf2a074eca18c22f` | clean; adapter SHA-256 `d1a1d975ca8a740da1d51b6d93627a18811b96c9cccf0832ed92c86c9e7d48cb`; tests SHA-256 `a1160157356ef3d030993ac50b0c0ba7c6c45da1ed7ee66f634a855526c8354f`; immutable metadata fixture SHA-256 `3514a5d3acb75eceb79923960e3463af451741fc770b9cec60fa0f9c466ab7a1` |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | pre-existing `?? .pi-subagents/`, `?? .pi/`, `?? tmp/`; immutable token route SHA-256 `c284250def5b6ee1dff407015d888f7ae7e7c18bb9d9ba6a4c468ab2324fa93c`; metadata producer SHA-256 `0d5908f54fe1d56960c20ffca6680848e0ba638c52d8958dd9b373a08376a4eb` |

Postflight revisions, status entries, and path digests match preflight exactly.
No consumer file was modified.

## Focused evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test --locked -p identus-oid4vci --all-features`: 59 passed and one
  manual diagnostic skipped, including all nine new binding tests.
- `cargo test --locked -p identus-oid4vci --no-default-features`: 59 passed and
  one manual diagnostic skipped.
- `cargo clippy --locked -p identus-oid4vci --all-targets --all-features --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p identus-oid4vci --no-deps
  --all-features`: passed.
- `cargo tree -p identus-oid4vci --edges normal`: runtime cone unchanged;
  `identus-core`, `serde_json`, `uriparse`, and `zeroize` only.
- `./scripts/factory check`, backlog/archive checks, and `git diff --check`:
  passed.

## Workspace and reproducible matrix

- `cargo test --locked --workspace --all-features`: passed.
- `cargo test --locked --workspace --no-default-features`: passed.
- `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps
  --all-features`: passed.
- `PATH=/nix/var/nix/profiles/default/bin:$PATH nix flake check
  --print-build-logs`: all 27 compatible aarch64-darwin checks passed,
  including Rust 1.85 MSRV, native, Android AArch64, iOS AArch64, browser-WASM,
  feature, lint, documentation, factory, dependency, license, advisory, and
  release Nextest lanes. The principal suite ran 479 tests: 479 passed and 22
  skipped.

The direct `nix` command was unavailable on the interactive PATH and was
rerun successfully with the configured Nix profile. Nix reported
`x86_64-linux` as incompatible with the local system; hosted Linux CI remains
mandatory. Existing nonfatal offline yank-index and macOS fixup-hook
diagnostics did not fail a derivation.

## Exact implementation evidence

- Specification commit:
  `bcedb54a4e45a06612a0d6c98b3ce75900e85515`.
- Reviewed production implementation commit:
  `03bae391505cf5aa7a76316212d15ee7fee88c9b`.
- Production transition SHA-256:
  `c4dbf1172c5cf1383f794a764a1d2c7ac2808e493e47dece6c51b2a19c2ce204`.
- Focused test SHA-256:
  `249bb9fd3148776a9d8ab1dc60200ad377f81ca1b82812cb751d1828af099e54`.
- No manifest, lockfile, Cargo feature, parser, limit, or dependency changed.
- The distinct exact-diff review is recorded in `review.md`; no unresolved
  finding remains.

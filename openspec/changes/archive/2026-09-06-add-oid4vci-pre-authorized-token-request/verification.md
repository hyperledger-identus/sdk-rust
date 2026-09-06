# Verification receipt

## Scope

- Issue: #125
- Backlog: IDR-023h under IDR-023
- Base: `f8941b16c1474aca64a18c7b538286a570323410`
- Branch: `codex/oid4vci-pre-authorized-token-request`
- Owner: `identus-oid4vci`

## Normative provenance

- OpenID for Verifiable Credential Issuance 1.0 Final, published 2025-09-16,
  section 6.1; SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- OAuth 2.0 RFC 6749 sections 3.2, 4.1.3, 4.5, and Appendix B; RFC Editor HTML
  SHA-256
  `535362fa3b4ca668d4734c244c6ed8c811f2ccd7ea1612a4201e5d8922a74b4e`.

No normative or donor source is copied. The implementation is an independent
expression of the reviewed mandatory-field, form-encoding, resource, and
secret-lifecycle behavior.

## Consumer isolation

| Repository | Revision | Evidence and preserved state |
| --- | --- | --- |
| Oxid | `5ba38b9bbc9326c294b353daaf2a074eca18c22f` | clean; Portal adapter SHA-256 `d1a1d975ca8a740da1d51b6d93627a18811b96c9cccf0832ed92c86c9e7d48cb`; generic adapter SHA-256 `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af` |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | pre-existing `?? .pi-subagents/`, `?? .pi/`, `?? tmp/`; behavior-only historical route evidence unchanged |

Postflight revisions, status entries, and path digests match preflight exactly.
No consumer file was modified.

## Focused evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test --locked -p identus-oid4vci --all-features`: 73 passed and one
  manual diagnostic skipped, including all six new request tests.
- `cargo test --locked -p identus-oid4vci --no-default-features`: 73 passed
  and one manual diagnostic skipped.
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
  --print-build-logs`: all 27 compatible aarch64-darwin checks passed after
  the first run identified and the formatter corrected one archive-intent
  alignment-only issue. The passing run includes Rust 1.85, native, Android
  AArch64, iOS AArch64, browser-WASM, feature, lint, documentation, factory,
  dependency, license, advisory, and release Nextest lanes. The principal
  suite ran 493 tests: 493 passed and 22 skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Linux CI remains mandatory. Existing nonfatal offline yank-index and macOS
fixup-hook diagnostics did not fail a derivation.

## Exact implementation evidence

- Specification commit:
  `c32297f9695b9b6dffcec32d1b839aabb221d3cb`.
- Reviewed production implementation commit:
  `c8a92b5c91a54ffa2b08bfc87bdc0f05e06973a8`.
- Production request construction SHA-256:
  `9628502aef6368f9bf3da3148d1e7631fe3410ffeac23af91430cf0c165691c1`.
- Focused test SHA-256:
  `8466dd3d54e5073915d3da6a4a569329240bd79f2b4046245248813125508a39`.
- No manifest, lockfile, Cargo feature, parser, existing limit, or dependency
  changed.
- Both commits have valid local signatures and DCO sign-offs.
- The distinct exact-diff review is recorded in `review.md`; no unresolved
  finding remains.
- The guarded archive completed, and its generated trailing blank lines were
  removed before the archive commit without semantic change.

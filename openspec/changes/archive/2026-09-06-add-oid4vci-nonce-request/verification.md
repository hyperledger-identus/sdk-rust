# Verification receipt

## Scope

- Issue: #135
- Backlog: IDR-023m under IDR-023
- Base: `82d0e507c4c2c34d909f433b2392740f0e3470d6`
- Branch: `codex/oid4vci-nonce-request`
- Owner: `identus-oid4vci`

## Normative provenance

- OpenID for Verifiable Credential Issuance 1.0 Final, section 7.1; SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.

No normative or consumer source is copied. The implementation is an
independent expression of the reviewed POST, endpoint, empty-body,
authorization and redaction behavior.

## Consumer isolation

| Repository | Revision | Evidence and preserved state |
| --- | --- | --- |
| Oxid | `5ba38b9bbc9326c294b353daaf2a074eca18c22f` | clean; `portal.rs` SHA-256 `d1a1d975ca8a740da1d51b6d93627a18811b96c9cccf0832ed92c86c9e7d48cb`; `portal_internal_tests.rs` SHA-256 `a1160157356ef3d030993ac50b0c0ba7c6c45da1ed7ee66f634a855526c8354f` |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | pre-existing `?? .pi-subagents/`, `?? .pi/`, `?? tmp/`; no Final Nonce Endpoint route found in the inspected Rust server surface |

Postflight revisions, status entries and Oxid path digests match preflight
exactly. No consumer file was modified. Oxid is Apache-2.0 evidence; the
private Lace checkout has no detected SPDX license and is used only as
behavior evidence.

## Focused evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test --locked -p identus-oid4vci --all-features`: 107 passed and one
  release-only diagnostic skipped.
- `cargo test --locked -p identus-oid4vci --no-default-features`: 107 passed
  and one release-only diagnostic skipped.
- `cargo clippy --locked -p identus-oid4vci --all-targets --all-features --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p identus-oid4vci --no-deps
  --all-features`: passed.
- `cargo tree --locked -p identus-oid4vci`: runtime cone unchanged;
  `identus-core`, `serde_json`, `uriparse` and `zeroize` only.
- `./scripts/factory check` passed 37 items with one active change;
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
  lanes. The principal suite ran 527 tests: 527 passed and 22 skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Linux CI remains mandatory. Existing nonfatal offline yank-index, platform
evaluation and Darwin fixup warnings did not fail a derivation.

## Exact implementation evidence

- Specification commit:
  `c05f7cd4a91248e8dd55aa69c9418728770b0c7e`.
- Reviewed production implementation commit:
  `2865b7e27245f62fcb580d6f598b90615d2f91b6`.
- Review and verification-evidence commit:
  `23ae2eb11c85e00a577201f55f06287e6929a44c`.
- Guarded OpenSpec archive commit:
  `77ead3be52e38a79d380868a74954e65c475e6bd`.
- Request implementation SHA-256:
  `369cf7ecedc7c0d91b6d490231717d88b6be1d7a8d9b9f17ef9be3fc5880ca0c`.
- Metadata state SHA-256:
  `5a183986a3229209eeb017188c8990a8845972d3f82cad78406fb3c146f0d24d`.
- Focused test SHA-256:
  `962d7c5da095496da964405bfef870319f90d328332af399453a934b0087ba82`.
- No manifest, lockfile, Cargo feature, dependency, consumer, chain or product
  repository changed.
- The four branch commits through the guarded archive have valid local
  signatures and DCO sign-offs. The final receipt-only commit is checked with
  the same local signature and DCO policy before delivery.
- The distinct exact-diff review is recorded in `review.md`; no unresolved
  finding remains.

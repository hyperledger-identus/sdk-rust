# Verification receipt

## Scope

- Issue: #119
- Backlog: IDR-023e under IDR-023
- Base: `1d94a634994ab4da7d5e902216e8f47106e4bbce`
- Branch: `codex/oid4vci-authorization-metadata`
- Owner: `identus-oid4vci`

## Normative provenance

- OpenID for Verifiable Credential Issuance 1.0 Final, published 2025-09-16,
  sections 12.2.4 and 12.3; SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- RFC 8414 text; SHA-256
  `16c816e4e0fdbffb7e910ff3017867bf39debe9cb7f52f5cbc508a052ed660e8`.

No normative or donor source is copied. The implementation is an independent
expression of the reviewed behavior and preserves the partial-conformance
boundary recorded in issue #119 and ADR 0045.

## Consumer isolation

| Repository | Revision | Evidence and preserved state |
| --- | --- | --- |
| Oxid | `5ba38b9bbc9326c294b353daaf2a074eca18c22f` | clean; adapter SHA-256 `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af`; immutable fixture SHA-256 `3514a5d3acb75eceb79923960e3463af451741fc770b9cec60fa0f9c466ab7a1` |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | pre-existing `?? .pi-subagents/`, `?? .pi/`, `?? tmp/`; producer SHA-256 `4dff93d21e02b221598c325a8afdc9ff7311b3c5041727652cea9ecac81419b1` |

Postflight revisions, state entries and path digests match preflight exactly.
No consumer file was modified.

## Focused evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test --locked -p identus-oid4vci --all-features`: 50 passed and one
  manual diagnostic skipped, including all nine new metadata tests.
- `cargo test --locked -p identus-oid4vci --no-default-features`: 50 passed and
  one manual diagnostic skipped.
- `cargo clippy --locked -p identus-oid4vci --all-targets --all-features --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p identus-oid4vci --no-deps
  --all-features`: passed.
- `cargo tree -p identus-oid4vci --edges normal`: runtime cone unchanged;
  `identus-core`, `serde_json`, `uriparse` and `zeroize` only.
- `scripts/check-factory.sh`: factory structure, 17-package inventory,
  30-row SSI backlog, support policy and active archive preservation passed.
- `git diff --check`: passed.

## Workspace and reproducible matrix

- `cargo test --locked --workspace --all-features`: passed.
- `cargo test --locked --workspace --no-default-features`: passed.
- `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps
  --all-features`: passed.
- `nix flake check --print-build-logs`: all 27 compatible aarch64-darwin checks
  passed, including Rust 1.85 MSRV, native, Android AArch64, iOS AArch64, WASM,
  feature, lint, documentation, factory, dependency, license, advisory and
  release Nextest lanes. The principal suite ran 470 tests: 470 passed and 22
  skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted Linux
CI supplies that independent gate. Existing nonfatal macOS fixup-hook
segmentation diagnostics did not fail a derivation.

## Exact implementation evidence

- Specification commit:
  `f94b9cfe1c8c0ddbbf563f645a947a46da6a9510`.
- Implementation commit:
  `30f12009b14c97b89b15943eb377ea6fa2a4bb30`.
- Documentation-review correction:
  `08c6acc06ed26e4e0daf35667abc703c0860cb62`.
- Nix hygiene correction and reviewed production head:
  `3591e03d663838c5b6c2236ea28f786c0e1188d5`.
- Production module SHA-256:
  `ff931e39eb0c67a056ed8003e6246966449678acf5992c9e08a840bc684addff`.
- Focused test SHA-256:
  `5eb59116247d58132296157d2e1409f4e88c6dbab889c575a222f03e97c3d452`.
- No manifest, lockfile, Cargo feature or dependency changed.
- The distinct exact-diff review is recorded in `review.md`; both local findings
  were corrected and no unresolved finding remains.

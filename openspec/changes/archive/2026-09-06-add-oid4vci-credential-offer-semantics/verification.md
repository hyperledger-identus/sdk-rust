# Verification: bounded OID4VCI Credential Offer semantics

## Receipt

- Issue: #113, child of #7 and #20 / `IDR-023`.
- Base: `develop@ff227f68927d0958230600c4f0d448d585c08ad0`.
- Implementation candidate: `b62bd5bb21f7ea8de21a778ed09a52f26f80944d`.
- Normative source: OpenID4VCI 1.0 Final sections 4.1.1, 12.2.1,
  and 13.5; HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- Compatibility: additive API in an unpublished crate; no wire output,
  dependency change, consumer adoption, publication, release, or `main` work.

## Local commands passed

```text
./scripts/factory doctor
./scripts/factory check
cargo fmt --all -- --check
cargo clippy -p identus-oid4vci --all-targets --all-features -- -D warnings
cargo test -p identus-oid4vci --all-features
cargo test -p identus-oid4vci --no-default-features
RUSTDOCFLAGS='-D warnings' cargo doc -p identus-oid4vci --no-deps
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps
PATH=/nix/var/nix/profiles/default/bin:$PATH nix flake check --print-build-logs
git diff --check ff227f68927d0958230600c4f0d448d585c08ad0..HEAD
cargo tree -p identus-oid4vci --edges normal
```

The local `aarch64-darwin` flake ran 27 checks. Evidence includes Rust 1.85
MSRV, browser-WASM, Android ARM64, iOS ARM64, strict Clippy/docs/format,
factory/archive preservation, 441/441 principal nextest cases, feature
matrices, dependency policy, deny, and audit derivations. The configured audit
gate reported unavailable yanked-index lookups in its offline index while the
advisory scan and overall derivation passed; no dependency changed in this
slice. Nix reported that the incompatible `x86_64-linux` system was omitted
locally; hosted Linux remains mandatory before merge.

## Focused evidence

- 9/9 semantic integration tests passed with all features and without default
  features; the 12 existing transport tests also remained green.
- Official Final and independently reconstructed Oxid/Lace shapes pass.
- Missing/type-confused fields, unsafe issuers, empty/duplicate/oversized ID
  collections, non-object grants, exact field limits, arbitrary-magnitude
  ignored extensions, transition equivalence, and diagnostic canaries pass.
- The normal dependency tree remains `identus-core`, `serde_json`, `uriparse`,
  and `zeroize` only.
- All four branch commits before archive have good local GPG signatures and
  DCO trailers.

## Consumer isolation

- Oxid final: `bfe3b481568dc738f0732c2b27548fab8721fd95`; only pre-existing
  untracked `.claude/` and `.pi/taskflows/`.
- Lace ID Portal final: `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`;
  only pre-existing untracked `.pi-subagents/`, `.pi/`, and `tmp/`.
- Path digests match the proposal/design preflight. Consumer changed: no.

## Review and remaining gates

The distinct post-implementation review in `review.md` found no blocker or
advisory issue. Hosted Linux/macOS CI, DCO/policy/hygiene, and exact-head hosted
review remain required before protected merge. Grant semantics, metadata,
protocol state, adoption, publication, release, and `main` remain follow-ups.

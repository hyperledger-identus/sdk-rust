# Verification receipt

## Identity and provenance

- Issue: `#137` (`IDR-023n`).
- Exact base: `38c0fc4313ef735fe564a1cb5080af38f4d2bf14`
  (`origin/develop`).
- Reviewed implementation head:
  `3f9cca16173339f33c1ae5eca70d6c1ebdafcd34`.
- OpenID4VCI 1.0 Final HTML SHA-256:
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- RFC 9110 HTML SHA-256:
  `d431760660ea44e130f6e919dab216df2d0b3a490567a98089267523368fe1e5`.
- RFC 9111 HTML SHA-256:
  `ce91ee9848d2b9ac46386b0f0cbd4bfd9c0cd1948ebc363834cadc7e0997f3d7`.

## Artifact hashes

- `credential_nonce_http_response.rs`:
  `d26b204323f54c69768f11a1b8a058139faa3f5bf87c66ae28f6cab604116c1c`.
- `limits.rs`:
  `de31f5dc8833f3b22507f869f718c6146b2d0478c3c7e235e0a7c56cd1cc7853`.
- `credential_nonce_http_response.rs` integration test:
  `4898d2545d99589740f32170ed81056f6f75f884ca9c5cfdce0f1081e5b641da`.

## Local gates

- `cargo test -p identus-oid4vci --all-features --test
  credential_nonce_http_response`: passed, 10/10.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed.
- `cargo fmt --all --check`, `git diff --check`, and
  `./scripts/factory check`: passed; 38/38 OpenSpec items.
- `nix flake check --print-build-logs`: passed all 27 compatible native
  checks after the branch-owned Taplo formatting correction. The release
  nextest suite passed 537/537 with 22 explicitly skipped diagnostics; MSRV,
  WASM, Android, iOS, Clippy, docs, deny, audit, text, TOML, Nix, format and
  factory checks passed.

## Boundary receipts

- `Cargo.toml`, `Cargo.lock`, and `crates/oid4vci/Cargo.toml` have no diff from
  the exact base.
- Runtime cone remains `identus-core`, `serde_json`, `uriparse`, and `zeroize`
  plus their existing transitive dependencies.
- Oxid remained untouched and clean at
  `5ba38b9bbc9326c294b353daaf2a074eca18c22f`.
- Lace ID Portal remained untouched at
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`; its pre-existing untracked
  `.pi-subagents/`, `.pi/`, and `tmp/` paths are unchanged.
- No Midnight Identity, NeoPRISM, Apollo, publication, release, branch-policy,
  `main`, or downstream mutation occurred.

## Review result

The semantic review and the distinct exact-diff implementation review have no
unresolved finding. All four implementation commits are signed and carry the
developer certificate of origin. The slice is ready for guarded archive and a
ready, issue-linked PR to `develop`.

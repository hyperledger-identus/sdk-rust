# Verification receipt

## Scope

- Issue: #123
- Backlog: IDR-023g under IDR-023
- Base: `b22ba18cde2b6e420499ee8922ab1bd2f3e2ad1e`
- Branch: `codex/oid4vci-transaction-code-input`
- Owner: `identus-oid4vci`

## Normative provenance

- OpenID for Verifiable Credential Issuance 1.0 Final, published 2025-09-16,
  sections 3.5, 4.1.1, and 6.1; SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.

No normative or donor source is copied. The implementation is an independent
expression of the reviewed input-presence and resource-bound behavior.

## Consumer isolation

| Repository | Revision | Evidence and preserved state |
| --- | --- | --- |
| Oxid | `5ba38b9bbc9326c294b353daaf2a074eca18c22f` | clean; generic adapter SHA-256 `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af`; Portal adapter SHA-256 `d1a1d975ca8a740da1d51b6d93627a18811b96c9cccf0832ed92c86c9e7d48cb`; tests SHA-256 `a1160157356ef3d030993ac50b0c0ba7c6c45da1ed7ee66f634a855526c8354f`; immutable positive/negative fixture SHA-256 values `78d551fd52e55777f01480a16607972ea46eebfce19b1a9709df8592b6ff6154` and `31f2d0925d553f759923f2d845ca441be92d549bbaeb450ecbb72cce3b7793d4` |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | pre-existing `?? .pi-subagents/`, `?? .pi/`, `?? tmp/`; immutable token route SHA-256 `c284250def5b6ee1dff407015d888f7ae7e7c18bb9d9ba6a4c468ab2324fa93c`; offer producer SHA-256 `c643b96aa4b27d62e39925a6c4a5cd7aa9d1e2dc3fbe543f72e0e7955c7404e5` |

Postflight revisions, status entries, and path digests match preflight exactly.
No consumer file was modified.

## Focused evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test --locked -p identus-oid4vci --all-features`: 67 passed and one
  manual diagnostic skipped, including all eight new input tests.
- `cargo test --locked -p identus-oid4vci --no-default-features`: 67 passed
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
  --print-build-logs`: all 27 compatible aarch64-darwin checks passed,
  including Rust 1.85 MSRV, native, Android AArch64, iOS AArch64, browser-WASM,
  feature, lint, documentation, factory, dependency, license, advisory, and
  release Nextest lanes. The principal suite ran 487 tests: 487 passed and 22
  skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Linux CI remains mandatory. Existing nonfatal offline yank-index and macOS
fixup-hook diagnostics did not fail a derivation.

## Exact implementation evidence

- Specification commit:
  `9f49bc0d779f25733cf4132c5cab30e26f15dbc7`.
- Reviewed production implementation commit:
  `3fd92d97ca3cc3f8c9c518fb8dc491d3960fd18f`.
- Production transition SHA-256:
  `2f0ba724554243eedab2ccc53d3a46b9328e59a9f1d8ffd884018b6f748b4715`.
- Focused test SHA-256:
  `90c11eefe134000e0d8a2ca9e2ec5695151f76b66ec9e7b37b0ee23740f82c49`.
- No manifest, lockfile, Cargo feature, parser, existing limit, or dependency
  changed.
- The distinct exact-diff review is recorded in `review.md`; no unresolved
  finding remains.
- The guarded archive completed and its generated trailing blank-line warnings
  were corrected before the archive commit; canonical semantic validation is
  unchanged.

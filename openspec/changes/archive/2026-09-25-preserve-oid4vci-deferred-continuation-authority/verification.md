# Verification

Verification date: 2026-09-25
Reviewed implementation head: `da0c7ed9e64c6e3de5333dbb355ecb2449872d84`

## Exact focused evidence

- `cargo test -p identus-oid4vci --all-features`: passed, including eight
  consuming Credential Endpoint tests and ten legacy deferred-request tests.
- `cargo clippy -p identus-oid4vci --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-oid4vci --all-features
  --no-deps`: passed.
- `cargo fmt --all -- --check`: passed.
- `scripts/factory check`: 84 OpenSpec/factory items passed.
- `scripts/factory backlog-live`: 30 rows and nine live issues passed.
- The immutable preimplementation receipt validates exact planning commit
  `2b70661b1dad631cc6a3b279ea4b307be03eb045` against
  `develop@342d936107624aee95788f1788b8cee6d62641c8` for issue #368.

## Workspace and documentation evidence

- `cargo build --locked --workspace --all-targets --all-features`: passed.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D
  warnings`: passed.
- `cargo test --locked --workspace --all-features` and `cargo test --locked
  --workspace --no-default-features`: passed; only declared diagnostic tests
  were ignored.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features
  --no-deps`: passed for every workspace package.
- `cargo tree -p identus-oid4vci --edges normal --locked` and an exact manifest/
  lockfile diff confirm the runtime cone is unchanged.
- Every branch commit verifies with a good OpenPGP signature and DCO sign-off.
- `nix flake check` could not run because this host has no `nix` executable;
  the hosted required `fast` lane must supply Nix evidence before merge.

## Portable compile evidence

The changed `identus-oid4vci` package and its portable internal dependency set
passed locked `cargo build` for each compile-only target:

- `wasm32-unknown-unknown`;
- `aarch64-apple-ios`; and
- `aarch64-linux-android`.

No target-specific code, native dependency, build script or unsafe code was
introduced.

## Compatibility result

The public change is additive and unpublished. The legacy structural request
constructor uses the same extracted private constructor and its complete
regression suite passes. The immutable OID4VCI error golden remains byte-exact.
No dependency, feature, manifest, lockfile, stored-data, consumer, release or
publication behavior changed.

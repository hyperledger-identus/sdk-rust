# Verification

Verification date: 2026-09-25
Reviewed implementation head: `5aec47807b15dbf90f46f632d7f260d7e3d07bf1`

## Exact focused evidence

- `cargo test -p identus-oid4vci --test deferred_credential_endpoint_response`:
  passed seven branch, authority, correlation, bounds and redaction tests.
- `cargo test -p identus-oid4vci --all-features`: passed, including all legacy
  borrowed deferred success/error and initial Credential Endpoint tests.
- `cargo clippy -p identus-oid4vci --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-oid4vci --all-features
  --no-deps`: passed.
- `cargo fmt --all -- --check`: passed.
- `scripts/factory check`: 84 OpenSpec/factory items passed.
- `scripts/factory backlog-live`: 30 rows and nine live issues passed, including
  open successor #372.
- The immutable preimplementation receipt validates exact planning commit
  `957f3b17fc563dd368df9777d591cbcd3733a387` against
  `develop@f42b2a5862f541ff68d6a5c8b9b270fbc600924e` for issue #370.

## Workspace and documentation evidence

- `cargo build --locked --workspace --all-targets --all-features`: passed.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D
  warnings`: passed.
- `cargo test --locked --workspace --all-features` and `cargo test --locked
  --workspace --no-default-features`: passed; only declared diagnostic tests
  were ignored.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features
  --no-deps`: passed for every workspace package.
- `cargo tree -p identus-oid4vci --edges normal --locked` and the exact manifest/
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

The public change is additive and unpublished. Both borrowed Deferred
Credential validators use the extracted shared parsing helpers and retain their
complete regression suites. The immutable OID4VCI error golden remains
byte-exact. No dependency, feature, manifest, lockfile, stored-data, consumer,
release or publication behavior changed.

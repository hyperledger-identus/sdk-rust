# Verification

Verification date: 2026-09-25
Reviewed implementation head: `a96d2c7bfcca229911ecf10e7aa02ba5783c9070`

## Exact focused evidence

- `cargo test -p identus-oid4vci --test authorization_code_credential_request`:
  4 passed.
- `cargo test -p identus-oid4vci --test jwt_credential_request`: 11 passed.
- `cargo clippy -p identus-oid4vci --all-targets -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed.
- `scripts/factory check`: 83 OpenSpec/factory items passed.
- `scripts/factory backlog-live`: 30 rows and 9 live issues passed.
- The immutable preimplementation receipt validates exact specification commit
  `722a9e0beb348600dae4c6e43ad09b54a2b942eb` against
  `develop@0630dc38f50119ac2ba4ca532dcd542935d079c7` for issue #364.

## Workspace and documentation evidence

- `cargo test --workspace --all-targets`, `cargo test --workspace
  --all-features`, and `cargo test --workspace --no-default-features`: passed;
  only declared diagnostic tests were ignored.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed for all workspace packages.
- Every branch commit is signed and carries a DCO sign-off.
- `nix flake check` could not run because this host has no `nix` executable;
  the hosted required `fast` lane must supply the Nix evidence before merge.

## Portable compile evidence

The changed `identus-oid4vci` package passed `cargo check` for each supported
compile-only portable target:

- `wasm32-unknown-unknown`;
- `aarch64-apple-ios`; and
- `aarch64-linux-android`.

No target-specific code, native dependency, build script or unsafe code was
introduced.

## Compatibility result

The public change is additive and unpublished. Both existing constructors use
the same extracted private serializer and their exact regression suite passes.
The slice adds no error, dependency, feature, lockfile, stored-data, consumer,
release or publication behavior.

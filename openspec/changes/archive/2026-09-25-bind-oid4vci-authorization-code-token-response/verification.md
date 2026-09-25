# Verification

Verification date: 2026-09-25

## Exact focused evidence

- `cargo test -p identus-oid4vci --test
  authorization_code_token_http_response`: eight passed at reviewed
  implementation head `fb944e7973d4328d1c9b78d934f2f4c5b44ef5d6`.
- `cargo check -p identus-oid4vci --all-features`: passed.
- `cargo test -p identus-oid4vci`: passed.
- `cargo test -p identus-oid4vci --no-default-features`: passed.
- `cargo doc -p identus-oid4vci --no-deps`: passed.
- `scripts/factory backlog-live`: 30 rows and nine live issues passed; IDR-023
  points to open successor #362.
- The preimplementation receipt validates exact contract commit
  `41314c8b2227451cf8c951fdfcd025d6ae93f666` against
  `develop@e5cdbc3ccdac110acb3e36384045e288ce0d440b`.

## Workspace and portable evidence

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test --workspace --all-features`: passed; only declared diagnostics
  were ignored.
- `cargo test --workspace --no-default-features`: passed; only declared
  diagnostics were ignored.
- `cargo doc --workspace --no-deps`: passed for the workspace.
- `scripts/factory check`: 81 OpenSpec items and all repository contracts
  passed.
- `cargo check -p identus-oid4vci --target wasm32-unknown-unknown
  --no-default-features`: passed.
- `cargo check -p identus-oid4vci --target aarch64-linux-android
  --no-default-features`: passed.
- `cargo check -p identus-oid4vci --target aarch64-apple-ios
  --no-default-features`: passed.
- `nix flake check`: unrun because `nix` is unavailable on this host; the
  required hosted `fast` lane remains the integration gate.
- An initial `scripts/factory backlog-live --offline` invocation exited with
  usage status because that command has no `--offline` option. The corrected
  live command above passed; no repository state was changed by the rejected
  invocation.
- Every branch commit is signed and carries a DCO sign-off.

## Compatibility result

The change is additive. Nine static fieldless error contracts append after the
immutable prior inventory. Cargo manifests and the lockfile are unchanged; no
dependency, feature, unsafe/native, existing wire, storage, product, chain,
target or release contract changes.

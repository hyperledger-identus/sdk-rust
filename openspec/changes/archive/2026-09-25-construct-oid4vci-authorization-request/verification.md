# Verification

Verification date: 2026-09-25

## Exact focused evidence

- `cargo test -p identus-oid4vci --test authorization_request`: 7 passed.
- `cargo test -p identus-oid4vci --lib authorization_request::tests`: 1
  focused private-codec test passed.
- `cargo test -p identus-oid4vci --all-targets`: passed; only the declared
  release-mode throughput diagnostic was ignored.
- `cargo test -p identus-oid4vci --test pre_authorized_token_request`: 6
  passed, preserving exact predecessor form bytes.
- `cargo test -p identus-oid4vci --test credential_offer_transport`: 12
  passed and 1 declared diagnostic ignored, preserving strict decoding.
- `cargo clippy -p identus-oid4vci --all-targets -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed.
- `scripts/factory check`: 78 OpenSpec/factory items passed.
- `scripts/factory backlog-live`: 30 rows and 9 live issues passed, including
  focused successor #356.
- The preimplementation receipt validates exact contract commit
  `63396de4f84fdc9795629dc9c01a3aeb1b42ac95` against
  `develop@1b211e9246808e3883fa5fa3ac01a40b36e89076`.

## Workspace evidence

At reviewed implementation head `c876ae7f6be728877b987554cc8a25efb03ff1a2`:

- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test --workspace --all-targets --all-features`: passed; only declared
  diagnostic tests were ignored.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed for 21 packages.
- `nix flake check` could not run locally because the current host does not
  expose a `nix` executable; the protected hosted `fast` lane remains the
  required Nix evidence before merge.
- Every branch commit is signed and carries a DCO sign-off.

## Compatibility result

The public change is additive. Seven static error contracts append after the
immutable baseline/live prefix. Existing form behaviors are regression-tested
after private factoring. The slice adds no external dependency, feature,
lockfile, unsafe/native, storage, downstream, target or release behavior.

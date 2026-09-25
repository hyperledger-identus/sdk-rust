# Verification

Verification date: 2026-09-25
Reviewed implementation head: `f55e7067d4ce6d621f129023a934e37dde1b8c4b`

## Focused behavior

- `cargo test -p identus-oid4vci --test pre_authorized_token_http_response
  --test authorization_code_token_http_response`: 15 tests passed.
- `cargo test -p identus-oid4vci`: the complete crate suite passed; the one
  declared manual throughput diagnostic remained ignored.
- The focused tests cover exact success/error lineage, Final pre-authorized
  error classes, unsupported/401 status, status-selected shape mismatch,
  header grammar/order, exact and one-over limits, request queries, static
  error identity and redaction canaries.

## Workspace and factory evidence

- `cargo test --workspace --all-features`: passed with only declared manual
  diagnostics ignored and no failure.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features
  --no-deps`: passed for all workspace packages.
- `cargo fmt --all -- --check` and base-to-head `git diff --check`: passed.
- `scripts/factory check`: passed; the matrix reports 24 rows: 14 implemented,
  5 partial, 4 unsupported and 1 missing.
- `scripts/factory backlog-live`: passed for 30 backlog rows, 24 conformance
  rows and nine distinct live issue owners; IDR-023 and the remaining missing
  matrix row both point to open #376.
- `./bootstrap.sh --check`: passed the structural, pinned Pi/runtime-policy and
  55 operational factory tests.
- The preimplementation receipt validates planning commit
  `930519add7c889ba8faa6a6ee060e6909736402c` against
  `develop@46a94e882159492a9125f1468bf362bee9374d57` for issue #375.

## Portable compile evidence

`identus-oid4vci` passed `cargo check` for:

- `wasm32-unknown-unknown`;
- `aarch64-apple-ios`; and
- `aarch64-linux-android`.

## Compatibility and provenance evidence

- Base-to-head manifest and lockfile diff is empty.
- `cargo tree --locked -p identus-oid4vci --edges normal --depth 2` confirms
  the existing exact dependency cone.
- All four issue-branch commits through the reviewed head carry valid OpenPGP
  signatures and DCO sign-offs.

## Unrun and unclaimed evidence

The hosted Linux `fast` gate, weekly slow security/fuzz/conformance lanes,
official OpenID conformance, live issuer/HTTP behavior, consumer builds,
publication and certification are not local claims. The hosted gate remains
mandatory before merge, and production promotion continues to rely on the
independent slow line.

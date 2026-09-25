# Verification

Verification date: 2026-09-25
Reviewed implementation head: `70a1a31fb41aafb66cee38384a37929b0cf98e79`

## Exact conformance and factory evidence

- `scripts/check-oid4vci-conformance.py .`: 24 rows passed; 13 implemented,
  5 partial, 4 unsupported and 2 missing.
- `scripts/tests/oid4vci-conformance.py`: seven mutation tests passed.
- `scripts/tests/factory-contract.sh`: passed, including the integrated matrix
  checker in the synthetic repository fixture.
- `scripts/factory check`: passed; all 84 OpenSpec/factory items and structural
  contracts were green.
- `scripts/factory backlog-live`: 30 rows and nine distinct live issue owners
  passed; IDR-023 points to open successor #375.
- The immutable preimplementation receipt validates exact specification commit
  `03ee4c2c2e94e2c3066a2959c2f9f5c10218c49e` against
  `develop@7f23129a245072e34ea5ddf1e601e14a4b493ed5` for issue #372.

## Rust and documentation evidence

- `cargo test --locked -p identus-oid4vci --all-features` and
  `--no-default-features`: passed; only the declared throughput diagnostic was
  ignored.
- `cargo test --locked --workspace --all-features`: passed; declared manual
  diagnostics remained ignored and no test failed.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D
  warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features
  --no-deps`: passed for all workspace packages.
- `cargo fmt --all -- --check`, `git diff --check`, Python byte-compilation,
  ShellCheck and markdownlint passed.
- `cargo tree --locked -p identus-oid4vci --edges normal --depth 2` and the
  exact manifest/lockfile diff confirm that the dependency cone is unchanged.

## Portable compile evidence

The unchanged `identus-oid4vci` package passed locked builds for:

- `wasm32-unknown-unknown`;
- `aarch64-apple-ios`; and
- `aarch64-linux-android`.

## Read-only consumer evidence

- Oxid remained at `183664aeca500c25d6d27a22fa402b4d40c649d3` on `develop`
  with the same four pre-existing status entries.
- Lace ID Portal remained at
  `d284b85bfcbb4a7e5d2200837703419c145c60f5` on `develop` with the same one
  pre-existing status entry.
- No consumer file, branch, index, worktree or remote was changed.

## Unrun and unclaimed evidence

No official OpenID conformance harness, live issuer, network transport,
consumer build, fixture execution, publication, performance benchmark or
certification ran in this slice. The matrix records these boundaries and M4
remains open pending #375 and #376.

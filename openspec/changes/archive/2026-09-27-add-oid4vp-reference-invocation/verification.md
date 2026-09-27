# Verification receipt

Verification date: 2026-09-28
Issue: #394
Develop base: `2ace51f35b0c24201616e74048dd180d9e6bf0af`
Planning commit: `9c84354ca75804b334206556b2bda6b98d5d5882`
Preimplementation receipt commit: `f80f39946f7496d008e53a2832a1f6fb77ed8025`
Reviewed implementation commit: `e66c8b2eede13a012e665d4de86a8384d286a17b`

## Focused evidence

- `cargo test -p identus-oid4vp`: passed 10 integration tests plus doc tests.
- `cargo clippy -p identus-oid4vp --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p identus-oid4vp --no-deps`:
  passed.
- The v1 error fixture freezes 15 unique static code/kind/message contracts.
- Positive GET/default and POST vectors, decoded-name collision, malformed
  form, ambiguous transport, unsupported parameter, unsafe URI, every
  configurable limit and redaction canary classes pass.
- A dedicated regression rejects the non-normative Oxid
  `openid4vp://authorize` route at the Final static boundary.

## Workspace and factory evidence

- `cargo fmt --all -- --check`: passed.
- `cargo build --locked --workspace --all-targets`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test --locked --workspace --all-targets`: passed; only declared
  manual diagnostics were ignored.
- `scripts/factory check`: 89 OpenSpec/factory items passed.
- Bootstrap inventory, resource boundaries, support policy, IDR backlog and
  exact core-only internal dependency guard passed.

## Compiler, target and dependency evidence

- Rust 1.89.0 host compile passed with the locked graph.
- Rust 1.98.1 locked compile checks passed for `wasm32-unknown-unknown`,
  `aarch64-linux-android` and `aarch64-apple-ios`.
- The direct graph is `identus-core`, `fluent-uri =0.4.1` and `zeroize 1.9`;
  there is no HTTP, JSON, DCQL, SIROS, chain or product dependency.

These are source and compile claims, not mobile/browser runtime, linker,
packaging, performance, certification or publication claims.

## Read-only consumer receipt

Oxid remained at `183664aeca500c25d6d27a22fa402b4d40c649d3` and its
status-porcelain SHA-256 remained
`7787c4d6f646be87dca93e62aab770e2dac866d60d85cbbecc7b7c2a85277aa1`.
No consumer file, index, branch, stash or remote was changed.

## Review and remaining hosted evidence

The exact-diff review found and corrected one blocking standards mismatch,
then reported no unresolved architecture/security finding at the reviewed
implementation commit. The protected PR must still supply the independent
Linux `fast` gate for its exact final head. No network interoperability,
Request Object retrieval/JAR validation, DCQL, credential selection, consent,
response processing, downstream adoption, release or certification is claimed.

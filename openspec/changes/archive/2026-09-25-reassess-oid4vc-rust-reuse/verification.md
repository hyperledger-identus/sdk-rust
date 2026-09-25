# Verification receipt

Verification status: passed locally
Verification date: 2026-09-26
Base: develop@c5543efc807c5bbcd8a9bd1d45f4282c10b58057
Implementation head: 9daede220561e40b7a6e66b69fba0555b8bc6bb6
Pull request: pending

## Candidate fixture receipt

- Exact `siros-dcql 0.3.0` and `serde_json 1.0.151` are resolved through the
  separately tracked lock under Rust 1.98.1.
- `scripts/check-siros-dcql-spike.sh` passed four deterministic tests and
  strict Clippy. The tests cover candidate-free summary mapping, pre-parse
  byte rejection, redacted errors, complete holder-bound credential matching,
  and the missing-`meta`/identifier-grammar mismatches.
- Root-configured `cargo deny 0.20.2` passed advisories, bans, licenses, and
  sources with expected unmatched-root-policy warnings. `cargo audit 0.22.2
  --deny warnings` passed 13 lock entries.
- The checker observed 12 normalized host normal/build cone lines and proved
  `siros-dcql` is absent from the root manifest and lock.

## Compiler and target receipt

- Rust 1.98.1 host tests and strict Clippy passed.
- Rust 1.89.0 host compile passed with `--ignore-rust-version` only for the
  research package's repository-wide 1.98.1 declaration; candidate and adapter
  source compiled successfully.
- Exact locked Rust 1.98.1 compile checks passed for
  `wasm32-unknown-unknown`, `aarch64-apple-ios`, and
  `aarch64-linux-android`.

These are compile observations, not runtime, linker, packaging, FFI, or
support claims.

## Repository receipt

- Research and constraint readiness passed before implementation, and the
  immutable preimplementation receipt binds issue #391 to specification commit
  `52e8e9b3245f17fb6287d718bbb2f11c757c3d74`.
- `scripts/factory check` passed every governance, research, constraint,
  traceability, source-distribution, release-train, and OpenSpec gate.
- `cargo test --workspace --all-targets --all-features --locked` passed the
  complete local workspace suite; only explicitly ignored diagnostics remained.
- Exact-diff whitespace checks passed. All branch commits are GPG-signed and
  carry DCO trailers.

## Exclusions

No browser/mobile runtime, linker/package, downstream consumer, deployed
interoperability, performance, publication, release, certification,
presentation construction, cryptographic verification, trust, consent, or
transport test was run or inferred.

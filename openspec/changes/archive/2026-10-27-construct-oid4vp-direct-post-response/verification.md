# Verification receipt

Verification date: 2026-10-27
Issue: #487
Develop base: `fe3a14b767a6b13f8aa2fb2acf92bc6e37d93c03`
Reviewed implementation: `18187dfae0cfc7195628f53c6c026598f6b0e2e4`

## Protocol and E2E evidence

- The immutable `oid4vp.direct-post-response.v1` packet executes invocation
  parsing, Request URI binding, Ed25519 JAR verification, Authorization Request
  validation, DCQL response-rule retention, result binding, deterministic
  `vp_token` construction and independent form decoding with exact state.
- `cargo test -p identus-oid4vp --test direct_post_response` passed all 10
  positive, negative, property, cardinality, limit, redaction and packet tests.
- The suite preserves arbitrary-precision numeric lexemes, rejects duplicate
  members, enforces parser work before retention and accepts 4,000 distinct
  members under an explicit bounded policy.
- This is runtime-neutral protocol E2E evidence. Network delivery, verifier
  trust, consent, presentation authenticity and downstream product E2E remain
  explicit non-claims.

## Workspace and factory evidence

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed.
- `scripts/factory check`: 111 OpenSpec/factory items passed.
- `scripts/tests/factory-contract.sh`: the complete mutation suite passed after
  the synthetic fixture was extended with the new vector and fuzz harness.
- `nix flake check --no-build --show-trace`: all host derivations evaluated.

## Portable, fuzz and performance evidence

- `identus-oid4vp` compiled with all features for
  `wasm32-unknown-unknown`, `aarch64-apple-ios` and
  `aarch64-linux-android`.
- The AddressSanitizer fuzz campaign completed 5,814,256 executions over 301
  seconds with zero crashes or timeouts and 566 MiB peak RSS.
- The release diagnostic recorded a 76,562 ns median for the ordinary E2E
  pipeline and 368,833 ns for the 60 KiB near-limit pipeline, each across 21
  samples of 10 iterations. These values establish no cross-host threshold and
  do not claim allocation measurement.

## Review and remaining hosted evidence

Codex CLI review corrected streaming work enforcement, exact JSON retention,
container-depth semantics, duplicate-member complexity and evidence-claim
accuracy. No unresolved blocker/high/medium finding remains. Protected Linux
CI must still validate the final PR head before guarded merge; no publication,
release or downstream adoption is part of this slice.

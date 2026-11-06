# Verification receipt

Verification date: 2026-10-29
Issue: #543
Develop base: `b9c2be2ed7877172d8461266a6522ff4a9b72187`
Planning commit: `013a5d5cf6a04fdaf00975ac8476645015e63452`
Preimplementation receipt commit: `5bd1e15eb0478412cdf0dc67d73468df6af3b8b7`
Reviewed implementation commit: `fe9bc263033a6db9d18aa33d51d024344576a2d6`

## Contract and focused evidence

- Research readiness, constraints readiness, strict OpenSpec validation, and
  issue-bound preimplementation validation passed before implementation.
- `cargo test -p identus-siopv2 --all-features`: 2 unit, 1 conformance, 13 E2E,
  and 3 property tests passed.
- `cargo test -p identus-siopv2 --no-default-features`: the same complete suite
  passed without default features.
- `cargo clippy -p identus-siopv2 --all-targets --all-features -- -D warnings`:
  passed.
- `cargo doc -p identus-siopv2 --no-deps --all-features`: passed.
- `scripts/factory quality-plan siopv2.did-authentication`: passed.

## Security, performance, and portability evidence

- The AddressSanitizer/libFuzzer soak executed 723,593 units in 301 seconds,
  averaged 2,403 executions/second, reached 527 MiB peak RSS, and produced no
  crash, timeout, or OOM artifact.
- The request-verification benchmark reported a 86,947 ns median across 21
  samples with 20 iterations per sample.
- `wasm32-unknown-unknown`, `aarch64-apple-ios`, and
  `aarch64-linux-android` checks passed with all SIOPv2 features.

## Factory and repository evidence

- `scripts/factory check`: passed after the implementation and documentation
  updates.
- `git diff --check`: passed.
- `/nix/var/nix/profiles/default/bin/nix flake check --print-build-logs`:
  passed all 38 host checks. This included Rust 1.98.1 and MSRV 1.89.0 builds,
  workspace/all-feature clippy, rustdoc, formatting, dependency policy,
  Android/iOS/WASM compile targets, text/TOML linting, the complete factory
  mutation suite, and 1,195 workspace nextest tests (1,195 passed, 23 declared
  skipped).

## Independent review and remaining hosted evidence

Codex CLI 0.163.0 with `gpt-6-sol` at high reasoning reviewed the exact diff
through `fe9bc263033a6db9d18aa33d51d024344576a2d6` and reported no actionable
defect. Its local Nix-cache sandbox limitation was superseded by the green host
flake gate above. Protected exact-head Linux CI and delivery-policy evidence
remain required before guarded merge.

# Verification evidence

- **Issue:** #44
- **Develop base:** `e0e276d4aeae097363db4b705ef571676322f9e5`
- **Specification commit:** `c74192743752a96cea28e2f3f4edd99b76527a71`
- **Implementation commit:** `afd4695c0932cbea79d196a3b66e26c6a5076d48`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust toolchains

## Product and workspace gates

- `cargo test -p identus-did --test did_method_registry`: 6 passed; one
  release diagnostic intentionally ignored.
- `cargo test -p identus-conformance`: 20 passed.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.
- `cargo fmt --all -- --check`, `git diff --check`, and
  `./scripts/factory check`: passed.

## Full reproducible matrix

`nix flake check --print-build-logs` passed all 26 compatible native checks.
This included Rust 1.85 MSRV, default/minimal/compat feature builds, Android
aarch64, iOS aarch64, `wasm32-unknown-unknown`, strict Clippy, rustdoc,
formatting, factory, text/Nix/TOML lint, cargo-deny, cargo-audit and release
Nextest suites. The main workspace profile ran 193 tests: 193 passed and 6 were
skipped. The KMP compatibility profile ran 85 tests: 85 passed and one was
skipped. The all-feature, deterministic-only and getrandom-only entropy
profiles also passed.

The local aarch64-darwin invocation reports x86_64-linux as an incompatible
system; hosted Ubuntu CI provides the independent Linux Nix gate before merge.

## Performance diagnostic

In release mode, 100,000 registered DID queries completed in `53.951125ms`,
approximately `1,853,529 queries/s` on the local machine. This is observational
evidence, not a portable pass threshold.

## Review and iteration effort

The implementation diff was independently re-read after all local gates. No
unresolved semantic, API, security or portability finding remains. From issue
#44 creation at `2026-09-03T04:01:35Z`, the specification commit took about 25
minutes, the implementation commit about 29 minutes, and the complete local
gate plus review about 35 minutes. These are operational elapsed times, not
person-hours.

## Deferred contracts

- #10: HTTP binding and network policy.
- #45: injectable cache and clock policy.
- #46: bounded DID URL dereferencing algorithm.
- #47: mature DID Registration lifecycle and secret modes.
- #41: deeper result/wire integrity hardening.

## Repository boundary

Apollo, NeoPRISM, midnight-identity, Lace ID Portal and Oxid were not edited,
switched, staged, copied from or built. Their immutable revisions were used as
read-only architecture evidence. SDK `main` was not changed.

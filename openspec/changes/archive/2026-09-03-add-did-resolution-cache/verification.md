# Verification evidence

- **Issue:** #45
- **Develop base:** `2cef96ca6c986440d6a340fe1b1f999a5c113633`
- **Specification commit:** `d92e95afdc6485ea6cf186ca0a2f8e497956eb1b`
- **Implementation commit:** `b4ef090a3ffdd2b0832bc788fe825cad5c16cf22`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust toolchains

## Product and workspace gates

- `cargo test -p identus-core`: 19 passed.
- `cargo test -p identus-did --test did_query_ports`: 7 passed; one release
  diagnostic intentionally ignored.
- `cargo test -p identus-did --test did_resolution_cache`: 10 passed; one
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
Nextest suites. The main workspace profile ran 207 tests: 207 passed and 7 were
skipped. The KMP compatibility profile ran 85 tests: 85 passed and one was
skipped. The all-feature, deterministic-only and getrandom-only entropy
profiles also passed.

The local aarch64-darwin invocation reports x86_64-linux as an incompatible
system; hosted Ubuntu CI provides the independent Linux Nix gate before merge.

## Performance diagnostic

In release mode, 100,000 cached DID resolutions completed in `95.498958ms`,
approximately `1,047,132 hits/s` on the local machine. This is observational
evidence, not a portable pass threshold.

## Review and iteration effort

The implementation diff was independently re-read after all local gates. No
unresolved semantic, API, security or portability finding remains. From issue
#45 creation at `2026-09-03T04:01:36Z`, the specification commit took about 66
minutes, the implementation commit about 80 minutes, and the complete local
gate plus review about 87 minutes. These are operational elapsed times, not
person-hours.

## Deferred contracts

- #50: cancellation-safe DID resolution single-flight, only after two
  consumers prove matching semantics.
- #10: HTTP binding, network and cache-header policy.
- #46: bounded DID URL dereferencing algorithm.
- #47: mature DID Registration lifecycle and secret modes.
- #41: deeper result/wire integrity hardening.

## Repository boundary

Apollo, NeoPRISM, midnight-identity, Lace ID Portal and Oxid were not edited,
switched, staged, copied from or built. Their immutable revisions were used as
read-only architecture evidence. SDK `main` was not changed.

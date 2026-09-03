# Verification evidence

- **Issue:** #46
- **Develop base:** `c002fc52d3ee1864c3499a3ca548232d774fbf9c`
- **Specification commit:** `e37ddd1b63e7e15ee951272d31cbdb2f0ec2897f`
- **Implementation commit:** `f991be6a24d57b94517586ae4b3c6e2656280fcb`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust toolchains

## Product and workspace gates

- `cargo test -p identus-did --test did_url_dereferencing`: 12 passed; one
  release diagnostic intentionally ignored.
- `cargo test -p identus-did`: passed the full DID suite.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.
- `cargo fmt --all -- --check`, `git diff --check`, and
  `./scripts/factory check`: passed.

## Focused coverage

Repository-independent `cargo-llvm-cov 0.9.0` measurement of the new focused
suite recorded 94.31% line coverage (630/668 lines), 91.92% region coverage
and 98.36% function coverage for `crates/did/src/dereference.rs`. This exceeds
the slice's 70–80% acceptance boundary without turning a machine/tool version
into a permanent repository dependency.

## Full reproducible matrix

`nix flake check --print-build-logs` passed all 26 compatible native checks.
This included Rust 1.85 MSRV, default/minimal/compat feature builds, Android
aarch64, iOS aarch64, `wasm32-unknown-unknown`, strict Clippy, rustdoc,
formatting, factory, text/Nix/TOML lint, cargo-deny, cargo-audit and release
Nextest suites. The main workspace profile ran 219 tests: 219 passed and 8 were
skipped. The KMP compatibility profile ran 85 tests: 85 passed and one was
skipped. The all-feature, deterministic-only and getrandom-only entropy
profiles also passed.

The local aarch64-darwin invocation reports x86_64-linux as an incompatible
system; hosted Ubuntu CI supplies the independent Linux Nix gate before merge.

## Performance diagnostic

On the exact reviewed implementation in release mode, 100,000 complete bare
DID-document dereferences through an object-safe recording mock completed in
`556.012583ms`, approximately `179,852 operations/s`. This is observational
evidence, not a portable pass threshold.

## Review and repository boundary

The implementation diff was independently re-read after all local gates. No
unresolved semantic, API, security or portability finding remains. The issue
was created at 2026-09-03T04:01:38Z; the complete reviewed/gated slice was ready
at 2026-09-03T06:38:11Z. These are operational elapsed times, not person-hours.

Apollo, NeoPRISM, midnight-identity, Lace ID Portal and Oxid were not edited,
switched, staged, copied from or built. Their immutable revisions were used as
read-only architecture evidence. SDK `main` was not changed.

## Deferred contracts

- #10: HTTP binding, endpoint retrieval, SSRF and redirect policy.
- #50: cancellation-safe DID resolution single-flight after consumer proof.
- #47: mature DID Registration lifecycle and secret modes.
- #41: deeper result/wire integrity hardening.

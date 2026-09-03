# Verification evidence

- **Issue:** #47
- **Develop base:** `558677ef0359f9af241232ed13422a005d5cc5b0`
- **Specification commit:** `1e990b71d565e55127632212e0db2280cdb49f77`
- **Implementation commit:** `5d178dba7047c145a4e0f0dda6bd1a86b5a40c31`
- **Environment:** aarch64-darwin, repository-pinned Nix and Rust toolchains

## Product and workspace gates

- `cargo test -p identus-did --test did_registration`: 16 passed; one release
  diagnostic intentionally ignored.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed.
- `cargo fmt --all -- --check`, `git diff --check`, and
  `./scripts/factory check`: passed.

## Focused coverage

Matching LLVM tools from the active Rust toolchain measured 87.36% region
coverage (712/815), 89.96% line coverage (600/667) and 89.91% function coverage
(98/109) for `crates/did/src/registration.rs`. This exceeds the slice's 70–80%
acceptance boundary without adding a coverage package to the repository.

## Full reproducible matrix

`nix flake check --print-build-logs` passed all 26 compatible native checks.
This included Rust 1.85 MSRV, default/minimal/compat feature builds, Android
aarch64, iOS aarch64, `wasm32-unknown-unknown`, strict Clippy, rustdoc,
formatting, factory, text/Nix/TOML lint, cargo-deny, cargo-audit and release
Nextest suites. The main workspace profile ran 235 tests: 235 passed and nine
were skipped. The KMP compatibility profile ran 85 tests: 85 passed and one was
skipped. The all-feature, deterministic-only and getrandom-only entropy
profiles also passed.

The first matrix run correctly rejected an unstable `if let` chain under Rust
1.85. The code was rewritten with equivalent nested conditions; the full
matrix then passed. The local aarch64-darwin invocation reports x86_64-linux as
an incompatible system, so hosted Ubuntu CI supplies the independent Linux Nix
gate before merge.

## Performance diagnostic

On the exact reviewed implementation in release mode, 100,000 complete exact
registry dispatches through an object-safe registrar completed in
`32.033458ms`, approximately `3,121,736 requests/s`. This is observational
evidence, not a portable pass threshold.

## Review and repository boundary

The implementation diff was independently re-read after all local gates. No
unresolved semantic, API, security or portability finding remains. The issue
was created at 2026-09-03T04:01:40Z; the complete reviewed/gated slice was ready
at 2026-09-03T07:36:05Z. These are operational elapsed times, not person-hours.

Apollo, NeoPRISM, midnight-identity, Lace ID Portal and Oxid were not edited,
switched, staged, copied from or built. Their immutable revisions were used as
read-only architecture evidence. SDK `main` was not changed.

## Deferred contracts

- DIF JSON/HTTP bindings, redirect/callback and decryption-action profiles;
- execute and DID URL resource mutation operations;
- custody/signing providers, persistence, VDR execution and finality policy;
- wallet confirmation, compensation and automatic resolver-cache coordination;
- production downstream adapter adoption.

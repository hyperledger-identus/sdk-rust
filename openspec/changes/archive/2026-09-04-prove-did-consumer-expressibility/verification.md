# Verification evidence

- **Issue:** #65
- **Develop base:** `d3380abc1b2238cc9ccfe4071104c2243ec02ba7`
- **Specification commit:** `1d72341ab06e57fc539b51c3851ec13d37814e21`
- **Implementation commit:** `f1b7603ff319341180efbf692187f0349fd5be9d`
- **Environment:** aarch64-darwin, repository-pinned Nix/Rust plus plain Cargo

## Focused evidence

- `cargo test -p identus-did --test did_consumer_compatibility`: 4 passed.
- `cargo test -p identus-did --all-features`: passed.
- `cargo test -p identus-did --no-default-features`: passed.
- Four named cases cover NeoPRISM, midnight-identity, Lace ID Portal and Oxid.
- Strict parsing, semantic JSON preservation, explicit error migration,
  private-JWK rejection, redacted malformed input and object-safe PRISM/
  Midnight dispatch are exercised.

## Workspace and factory gates

- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.
- `cargo fmt --all -- --check`, `git diff --check` and
  `./scripts/factory check`: passed.

## Reproducible matrix

`nix flake check --print-build-logs` passed all 26 compatible
aarch64-darwin checks. This included Rust 1.85 MSRV surfaces, nightly host
build/lint/test/docs, WASM, Android ARM64, iOS ARM64, minimal and KMP feature
surfaces, dependency/source/license policy, RustSec audit and factory/text/Nix/
TOML checks. The main release Nextest profile ran 255 tests: 255 passed and 9
were skipped; all four new consumer cases passed.

The local flake invocation omits incompatible `x86_64-linux`; hosted Ubuntu CI
must supply that independent Nix gate before merge. The audit derivation emitted
non-fatal offline sparse-index yanked-status diagnostics, and the Darwin Nix
fixup hook emitted intermittent non-fatal scan-process diagnostics; the full
flake command exited successfully. Neither was introduced by this test-only
change.

## Repository boundary

NeoPRISM, midnight-identity, Lace ID Portal and Oxid were inspected only at the
recorded immutable revisions. They were not edited, staged or built. No donor
source or fixture was copied, no package was published, and SDK `main` was not
changed.

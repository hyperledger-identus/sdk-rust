# Verification receipt

## Scope

- Issue: #101
- Base: `ed6cbed292e27de4096adb2cb81622591ce1c7c5`
- Branch: `codex/narrow-did-crypto-features`
- Owners: `identus-did`, `identus-crypto` test configuration and SDK support
  policy

## Dependency evidence

- Before the change, `identus-did` activated `identus-crypto` defaults and 14
  direct crypto dependencies, including Ed25519, X25519, secp256k1, P-256,
  hashing, derivation and COSE support.
- After the change, `cargo tree -p identus-did --no-default-features --depth
  2` contains only `identus-core`, `identus-derive`, Serde and JSON production
  dependencies; the test graph adds only `uriparse`.
- `cargo tree -p identus-did --no-default-features -i identus-crypto` reports
  that `identus-crypto` is not present.
- No donor code or fixture was copied, and the Oxid, midnight-identity and Lace
  repositories remained read-only.

## Focused verification

- `cargo check -p identus-did`: passed.
- `cargo check -p identus-did --no-default-features`: passed.
- `cargo test -p identus-did`: passed.
- `cargo test -p identus-did --no-default-features`: passed.
- `cargo clippy -p identus-did --all-targets --no-default-features -- -D
  warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-did --no-deps`: passed.
- `cargo test -p identus-conformance`: 24 tests passed.
- `cargo test -p identus-crypto --no-default-features`: 8 always-available
  tests passed; optional integration targets were skipped by their declared
  prerequisites.
- Each minimum crypto integration-target surface passed independently: `cose`;
  all four curves; `derivation,x25519`; `jwk-thumbprint`; and
  `hex,secp256k1`.

## Full reproducible matrix

- `cargo test --workspace --no-default-features`: passed.
- `cargo test --workspace --all-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.
- `scripts/check-support-policy.py`: passed.
- `scripts/factory check`: passed.
- Strict OpenSpec validation: passed.
- `nix flake check --print-build-logs`: all 29 compatible aarch64-darwin
  checks passed, including Rust 1.85 MSRV, native, Android AArch64, iOS
  AArch64, WASM, lint, documentation, factory, dependency, license, advisory,
  feature and release Nextest lanes. The principal lane ran 411 tests (411
  passed, 21 intentionally skipped); the new crypto-minimal lane ran 8 tests
  (8 passed).

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Ubuntu CI supplies that independent gate. Existing nonfatal offline crates.io
yanked-lookups and macOS fixup-hook diagnostics did not fail a derivation.

## Review evidence

- Specification commit:
  `14e4c7461d03068e2b76a4dd21ee1ff5115c8d7c`.
- Dependency-boundary commit:
  `4dedffa796431a476dc46720fd2fe65e3c00bfa9`.
- Test-isolation and machine-gate commit:
  `f035e36ad773c26447babd083c973621b1b5f96d`.
- All three commits carry valid cryptographic signatures and DCO trailers.
- The distinct exact-diff review is recorded in `review.md`; it resolved the
  hidden Serde feature coupling and crypto integration-test feature masking
  before delivery and has no unresolved finding.

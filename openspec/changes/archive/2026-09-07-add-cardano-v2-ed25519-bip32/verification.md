# Verification receipt

## Identity and provenance

- Delivery issue: `#177`; dependency-hardening follow-up: `#179`.
- Exact base: `2201424628f89f7106c7c53077bb53fdffa3022d`
  (`origin/develop` at branch creation).
- Reviewed implementation head: `699c3538b37c9ed37e0273d7f3ccb11fa3ee47d5`.
- Upstream `ed25519-bip32` tag commit: `6539dc9`; Apollo reference:
  `ccee22b`; independent vector donor: `e9d995b`.
- Primary compiler: Rust `1.98.1`; effective workspace MSRV: Rust `1.85.0`.

## Local gates

- Isolated `cardano-bip32` and default-feature suites: 9/9 passed in each lane.
- Workspace all-feature and no-default-feature tests: passed.
- `nix flake check --print-build-logs`: all 30 aarch64-Darwin-compatible checks
  passed. The primary nextest suite passed 596/596 with 22 skipped diagnostics;
  the Cardano/KMP-compatible crypto lane passed 100/100. Primary and MSRV
  builds, Clippy, rustdoc, formatting, audit, deny, WASM, Android, iOS, text,
  TOML, and repository-policy checks passed.
- Strict all-feature docs, official workspace Clippy with warnings denied,
  stricter isolated/default crypto Clippy, `cargo fmt --all --check`, and
  `git diff --check`: passed.
- Current online `cargo audit` and `cargo deny check`: passed. Nix's offline
  audit index emitted non-fatal missing-yank metadata while completing its
  derivation; the online run had no advisory finding.
- Factory validation, research-readiness, constraint-readiness, and complete
  factory gates: passed.

## Conformance, API and dependency receipt

- Apollo and upstream hardened vectors, an independently generated soft vector,
  Apollo public-path bytes, and private/public soft equivalence pass byte for
  byte. Hardened public derivation fails before dependency dispatch with a
  stable redacted SDK error.
- Private SDK state is a non-public `[u8; 96]` with `Zeroize` and
  `ZeroizeOnDrop`; `Debug` is fieldless and no private-key `Display` or serde
  contract exists. Invalid imported material is zeroized before error return.
- `cargo-public-api 0.52.0` found no `ed25519-bip32`, `cryptoxide`, `XPrv`,
  `XPub`, or dependency error in the isolated feature's public API.
- The no-default feature graph contains neither new package. Enabling
  `cardano-bip32` adds only `ed25519-bip32 0.4.3`, `cryptoxide 0.6.5`, and the
  already-governed `zeroize` dependency.
- Artifact SHA-256: implementation
  `5602acb1f55338cad196266bcefd5b85dfaae0283e6cdb5a1d68221342c421d3`;
  tests `8d7ad6f3778c84290beb4a36318119c23d1f55e40198868462f7b6172d8c11fb`;
  research `aab587cfe9aa56cf73fe166a1a9e7716b098c3a46721327d24348cd8c8ba984f`;
  lockfile `97f3d73b21d27c807f2cd01784a13332942654d43963799e7ea7dcb3e134baab`.

## Residuals and review result

- The exact package cone is narrow, but `ed25519-bip32` activates the broad
  default feature set of `cryptoxide`. The dependency's secret formatting and
  manual unsafe wipe also remain upstream risks. Issue `#179` owns the bounded
  upstream-or-fork hardening decision; none of these surfaces cross the SDK API.
- An extra, non-required workspace `--all-targets --all-features` Clippy sweep
  reaches the pre-existing Rust 1.98 `manual_noop_waker` lint in
  `crates/credentials/tests/verifier.rs`. The official workspace gate and the
  stricter crypto-scoped gate pass; unrelated credentials code is unchanged.
- The distinct correctness/security review found no unresolved blocker. The
  implementation is ready for guarded archive and exact-head hosted CI; merge
  remains prohibited until every required PR check is green.

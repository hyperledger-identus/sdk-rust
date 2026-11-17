# Verification

## Rust and target matrix

- `nix flake check --print-build-logs` passed on `aarch64-darwin` at source
  commit `ed57537a554719ee9fcbee3c5b74469d48cc58d3` with Rust/Cargo 1.99.1.
- The workspace Nextest partition passed 1,195 of 1,195 tests with 23 skipped.
- The isolated MSRV, primary, etalon, all-feature, minimal-feature, hash-only,
  KMP, deterministic/system entropy, WASM, Android ARM64, iOS ARM64, UniFFI,
  strict Clippy, rustdoc, deny, audit, and text-hygiene partitions passed.
- `x86_64-linux` was host-incompatible locally and remains hosted exact-head CI
  evidence. No Linux result is inferred from the Darwin run.

## Policy, compatibility, and supply-chain evidence

- `./scripts/factory check` passed after the consumer-visible compatibility
  correction with 126 OpenSpec items, 22 ledger records, 57 vectors, and 84
  quality obligations.
- `python3 scripts/tests/support-policy.py` passed 212 tests.
- `python3 scripts/tests/constraints.py` passed 13 tests.
- `python3 scripts/tests/release-candidates.py` passed its mutation suite.
- Cross-language vector, quality-evidence, consumer-ledger, archive-intent,
  source-distribution, candidate, and UniFFI generator supply-chain checks
  passed. Both active locks resolve patched `rustix 1.1.6`; no rustix advisory
  waiver was added.
- The Rust 1.99.1 requirement is classified as a consumer-visible
  `runtime-requirement-break` with migration, compatibility window, fallback,
  rollback, vector, quality declaration, and stable ledger ID
  `factory.operations.rust-1-99-compiler-floor.v1`.
- Published 0.1 artifacts, receipts, accepted historical ADRs, and candidate
  baselines remain unchanged.

## Review and integration state

Independent review of PR #597 at `3387f824a709ed9c665e34c5c89e45eefddd9f1d`
found three blockers: incomplete current-spec deltas, incorrect
behavior-neutral classification, and premature review wording. The first two
are corrected in the following commit; the third remains open until a fresh
exact-head review accepts the fixes. Archival and task closeout occur only
after that re-review.

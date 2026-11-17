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
were corrected in `af8682fa9f628109251c55e3ae2c3fe62d5b7b26`; the third
remains open until a fresh exact-head review accepts the fixes. Re-review of
that commit found two additional factory-integrity defects: the compiler-floor
packet was absent from the mutation-suite fixture tree, and the constraint
activation text omitted ADR 0200. Both are corrected in the next commit.
`python3 scripts/tests/cross-language-vectors.py`,
`python3 scripts/tests/constraints.py`, `git diff --check`, and
`./scripts/factory check` pass after those corrections. Archival and task
closeout occur only after a fresh exact-head review.

Independent exact-head re-review of
`8ed9c66141a3de8e6132ed59520120dd48948e8d` found no actionable issues and
confirmed both corrections. The reviewed commit is signed, DCO passes, and
the repository policy, compatibility, security, and evidence contracts are
consistent with issue #596. The archival delta receives a separate final
review after `factory archive` moves this packet into immutable history.

Exact-head hosted run
[`44145436160`](https://github.com/hyperledger-identus/sdk-rust/actions/runs/44145436160)
passed at that reviewed commit. The immutable Rust core job passed before the
binding-impact planner released the isolated browser, Node.js, React web,
package-types, and inapplicable React Native lanes; every selected lane passed.

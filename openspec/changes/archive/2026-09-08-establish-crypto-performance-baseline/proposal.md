# Proposal: establish measurement-only crypto performance baselines

## Why

M2 requires reproducible performance evidence, but Apollo at the pinned
revision has no benchmark suite. SDK-Rust therefore needs an honest,
measurement-only baseline that cannot be mistaken for cross-language parity or
a release threshold.

## What changes

- Add a stable-Rust, dependency-free benchmark example for representative
  `identus-crypto` operations using deterministic inputs and setup outside the
  timed loops.
- Emit a machine-readable JSON artifact with exact revision, toolchain,
  platform, feature, sampling and percentile metadata.
- Add a slow-lane job that runs the benchmark weekly/manually and uploads the
  artifact; the fast PR lane remains unchanged.
- Extend the executable Apollo parity ledger and Discussion #178 with the
  explicit “Apollo unavailable / SDK measurement-only” disposition.
- Record the harness choice in ADR 0095.

## Non-goals

No Apollo-vs-Rust performance claim, optimization, pass/fail latency threshold,
entropy timing, donor change, public API, runtime dependency, or fast-lane job
is introduced.

## Issue

Implements <https://github.com/hyperledger-identus/sdk-rust/issues/214> under
M2 issue #9.

# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

Apollo `main@ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` has no benchmark,
JMH, or performance suite, so a numeric cross-language parity claim has no
source. SDK-Rust has isolated manual throughput diagnostics but no representative
crypto baseline or uploaded artifact. Issue #214 requires at least 20 samples,
median and p95, exact environment/revision metadata and no M2 threshold.

## Normative sources

- Rust [`std::hint::black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html)
  documents a best-effort optimization barrier for
  benchmarks and explicitly forbids relying on it for correctness.
- Cargo documents [stable custom benchmark targets](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)
  with `harness = false`.
- [Criterion 0.8.2](https://crates.io/crates/criterion/0.8.2) is Apache-2.0/MIT,
  MSRV 1.86, and has at least 15 normal
  direct dependencies before optional plotting/rayon/async dependencies.
- [Divan 0.1.21](https://crates.io/crates/divan/0.1.21) is Apache-2.0/MIT,
  MSRV 1.80, with six normal direct
  dependencies and statistical runner behavior.
- [Gungraun 0.19.4](https://crates.io/crates/gungraun/0.19.4) is
  Apache-2.0/MIT, MSRV 1.85.1, and requires its runner plus
  Valgrind-family host tooling; it is unsuitable for the macOS half of the
  current slow matrix.

Version and dependency evidence was retrieved from `cargo info --verbose` and
the projects' official repositories/documentation on 2026-09-09.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Stable custom harness using `Instant` and `black_box` | `adopt` | Meets the measurement-only need with zero new crates, explicit sampling, portable JSON, and full control over setup/timing boundaries. | Stable runner history requires statistical inference or comparison tooling. |
| Criterion 0.8.2 | `not-adopt-now` | Mature statistics and reports, but a large development cone and optional report stack are unnecessary before a regression policy exists. | Two or more benchmark suites need confidence intervals or automated baseline comparison. |
| Divan 0.1.21 | `not-adopt-now` | Smaller than Criterion, but still adds macros/CLI dependencies while M2 needs one fixed matrix and explicit JSON. | Ergonomic registration becomes more valuable than the current fixed contract. |
| Gungraun 0.19.4 | `not-adopt-now` | Deterministic instruction evidence is attractive, but Valgrind/runner coupling is Linux-specific and expands CI provisioning. | The slow lane adds a dedicated Linux profiling tier. |
| Nightly `#[bench]` | `reject` | Conflicts with the Rust 1.98 stable etalon and creates a third toolchain requirement. | Never while stable custom harnesses remain sufficient. |

## Compatibility and dependency evidence

No dependency or lockfile changes are required. The benchmark is an example
target compiled with the existing default crypto features plus `kmp-compat`.
All key construction occurs before timing; deterministic fixed inputs separate
operation cost from entropy. The JSON schema is diagnostic evidence, not a
public library API.

## Security, privacy and maintenance evidence

The artifact records operation names and timing aggregates only. It never
prints messages, mnemonic words, seeds, private/extended keys, signatures, or
shared secrets. Private values remain inside SDK-owned redacted/zeroizing types
and are consumed through `black_box`. The source forbids unsafe code.

Twenty samples after warm-up are the publication minimum. Per-operation batch
sizes keep clock granularity useful; each sample reports nanoseconds per
operation. Median uses nearest-rank p50 and p95 uses nearest-rank p95. Results
are comparable only within matching artifact metadata.

## Rejected or deferred candidates

No optimization is justified by this evidence slice. Thresholds, trend
storage, dedicated hardware, instruction counts and Apollo comparison remain
deferred until multiple stable hosted runs exist or Apollo publishes a harness.

## Open questions and blockers

None for a measurement-only baseline. Future threshold governance must use a
separate ADR and cannot reinterpret these samples as a product promise.

## Evidence commands

- `cargo search` and `cargo info --verbose` for Criterion 0.8.2, Divan 0.1.21
  and Gungraun 0.19.4.
- Read-only Apollo tree/search at the pinned revision.
- `scripts/factory doctor` on `origin/develop@1e6df5c` before edits.

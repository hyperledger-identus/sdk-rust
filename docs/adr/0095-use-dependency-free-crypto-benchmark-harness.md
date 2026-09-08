# ADR 0095: use a dependency-free crypto benchmark harness

- **Status:** Accepted for implementation
- **Date:** 2026-09-09
- **Issue:** [#214](https://github.com/hyperledger-identus/sdk-rust/issues/214)
- **Parent:** [#9](https://github.com/hyperledger-identus/sdk-rust/issues/9)
- **Decision authority:** M2 measurement-only performance evidence

## Context

Apollo's pinned source has no comparable benchmark. SDK-Rust needs a portable
baseline artifact, but M2 has neither stable runner history nor a regression
threshold that justifies adopting a statistical framework.

Criterion 0.8.2 is mature but brings a broad reporting/statistics dependency
cone. Divan 0.1.21 is smaller but still adds runner and macro coupling.
Gungraun 0.19.4 adds a runner and Valgrind-family host requirement that does not
fit the current Linux/macOS slow matrix. All fit Rust 1.98.1 and have compatible
licenses; MSRV is not the rejection reason.

## Decision

Use a custom stable harness built from `std::time::Instant` and
`std::hint::black_box`. It SHALL use deterministic pre-built inputs, explicit
batch sizes, one warm-up batch, at least 20 measured samples, nearest-rank p50
and p95, and machine-readable metadata. It SHALL run only in the weekly/manual
slow workflow and SHALL introduce no threshold.

## Consequences

- No dependency or lockfile change is required.
- The JSON format and sampling code are owned and tested locally.
- Results expose trends but do not offer Criterion-style statistical inference
  or Gungraun instruction counts.
- Timing is valid only with matching environment metadata.
- The fast PR lane, runtime SDK and public API remain unchanged.

## Reconsideration triggers

Adopt a maintained harness only when at least two benchmark suites need common
registration/reporting, stable hosted history supports a threshold, or a Linux
profiling tier justifies deterministic instruction counts. Any adoption gets a
fresh dependency-cone and MSRV review.

## Verification and rollback

Schema/percentile/CLI tests, a release 20-sample run, artifact inspection,
slow-lane upload and distinct security/performance review are required. Remove
the benchmark, runner, workflow job and parity metadata together to roll back.

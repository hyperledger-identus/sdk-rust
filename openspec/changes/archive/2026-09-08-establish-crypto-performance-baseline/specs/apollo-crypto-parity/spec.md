## ADDED Requirements

### Requirement: SDK crypto performance remains observable without a parity claim

The repository SHALL provide a stable-Rust, dependency-free diagnostic harness
for representative `identus-crypto` operations. A publishable run SHALL use at
least 20 samples after warm-up and record exact SDK revision, Rust compiler,
platform, CPU description, feature profile, per-operation batch sizes, median
and p95. Inputs SHALL be deterministic and setup SHALL remain outside timed
regions so entropy is not included accidentally.

#### Scenario: Weekly or manual baseline is published

- **WHEN** the slow benchmark job completes
- **THEN** it SHALL upload a machine-readable artifact tied to the exact commit
- **AND** the artifact SHALL identify itself as measurement-only
- **AND** it SHALL state that Apollo comparison is unavailable

#### Scenario: Publication metadata is incomplete

- **WHEN** revision, compiler, platform, CPU, features or the minimum sample
  count is absent
- **THEN** the runner SHALL fail without publishing a final artifact

### Requirement: Performance evidence does not become an implicit threshold

M2 performance evidence SHALL NOT impose a pass/fail latency threshold or make
a cross-machine, cross-language, constant-time, certification or product
latency claim. Any future threshold SHALL require a separate ADR based on
stable runner history.

#### Scenario: Timing changes between valid runs

- **WHEN** a later diagnostic produces different timing values
- **THEN** correctness and CI SHALL remain green unless a separately approved
  threshold policy is active

## ADDED Requirements

### Requirement: Sanitizer-backed DID lexical fuzzing

The DID Core capability SHALL provide separate sanitizer-backed fuzz targets
for the public `Did` and `DidUrl` lexical boundaries. Targets SHALL accept
arbitrary bytes, exercise every UTF-8 value within a bounded generator profile,
and treat rejection as valid. Every accepted value SHALL preserve exact
spelling across borrowed access, Display, owned construction, `FromStr`, and
serde round trips. Accepted DID URL component views SHALL select valid in-bounds
UTF-8 subslices and reconstruct the complete owned value exactly.

Fuzz-only dependencies SHALL remain in an independent workspace outside every
published crate dependency cone. Method-specific validation, resolution, chain
state, trust, custody and product policy SHALL NOT enter these targets.

#### Scenario: hostile input cannot violate a public invariant

- **WHEN** arbitrary bounded bytes contain valid, invalid, non-ASCII, malformed,
  delimiter-heavy, percent-encoded, or over-limit identifier text
- **THEN** parsing SHALL either reject it without panic or return a value whose
  complete public construction, serialization and component invariants hold

#### Scenario: fuzz tooling does not become an SDK dependency

- **WHEN** production, minimal-feature, mobile, WASM, or downstream dependency
  cones are built
- **THEN** cargo-fuzz, libFuzzer and sanitizer support SHALL NOT be required by
  or exposed from an SDK crate

### Requirement: Reproducible bounded fuzz campaigns

The repository SHALL pin its sanitizer-capable compiler, fuzz runner and runtime
binding and SHALL expose one documented command interface for committed-corpus
replay, deterministic fixed-run smoke, and time-boxed soak modes. Pull-request
and integration smoke SHALL use a fixed seed, run count, input ceiling, timeout,
memory ceiling and one worker. Scheduled/manual soak SHALL be bounded separately
so ordinary delivery latency does not depend on a long random campaign.

Original seed corpora and dictionaries SHALL cover generic W3C and consumer-
shaped lexical boundaries without asserting method semantics. Failing artifacts
SHALL be retained for triage; each accepted defect SHALL be minimized and
promoted to committed corpus and deterministic regression evidence. Execution
rate and wall time MAY be recorded but SHALL NOT have a machine-specific pass
threshold.

#### Scenario: ordinary CI is repeatable and fast

- **WHEN** the same revision runs the pull-request fuzz gate
- **THEN** each target SHALL receive the same seed, run count and resource
  limits through the pinned Nix environment and SHALL terminate deterministically

#### Scenario: longer search remains bounded and diagnosable

- **WHEN** a scheduled or manually dispatched soak finds a sanitizer or
  invariant failure
- **THEN** the target SHALL stop within its documented resource/time envelope
  and preserve the failing artifact for minimization without printing its bytes
  as trusted data

## MODIFIED Requirements

### Requirement: Reproducible DID document hardening evidence

The DID Core test suite SHALL deterministically generate bounded URI/document
structures and raw duplicate mutations with a fixed reproducible algorithm.
It SHALL cover native and unique-name semantic-wire equivalence, every scanner
limit, malformed/trailing input, and retained minimized regressions. A release
diagnostic SHALL report hardened scan-and-parse throughput without a
machine-specific pass threshold. Sanitizer-backed DID/DID URL lexical fuzzing
SHALL use the separately bounded campaign contract, and resolution-envelope
scanning SHALL retain its own deterministic evidence.

#### Scenario: generated evidence is stable in ordinary CI

- **WHEN** the focused conformance suite runs on the same revision
- **THEN** it SHALL exercise the same bounded cases without randomness,
  network, filesystem, clock, nightly compiler, or consumer repository

#### Scenario: performance remains observable but portable

- **WHEN** the representative hardened document parser runs in release mode
- **THEN** throughput and the scanner's explicit resource shape SHALL be
  recorded without a hardware-specific acceptance threshold

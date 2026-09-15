## MODIFIED Requirements

### Requirement: Reproducible bounded fuzz campaigns

The repository SHALL pin its sanitizer-capable compiler, fuzz runner and runtime
binding and SHALL expose one documented command interface for committed-corpus
replay, deterministic fixed-run smoke, and time-boxed soak modes. Pull-request
and integration smoke SHALL use a fixed seed, run count, input ceiling, timeout,
memory ceiling and one worker. Native hosted weekly/manual soak and exact local
reproduction SHALL be bounded separately so ordinary delivery latency does not
depend on a long random campaign.

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

- **WHEN** a hosted weekly/manual or locally reproduced soak finds a sanitizer
  or invariant failure
- **THEN** the target SHALL stop within its documented resource/time envelope
  and preserve the failing artifact for minimization without printing its bytes
  as trusted data

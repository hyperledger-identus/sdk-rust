## MODIFIED Requirements

### Requirement: Reproducible bounded crypto fuzz campaigns

The repository SHALL expose one documented crypto command interface for
committed-corpus replay, deterministic fixed-run smoke, and time-boxed soak
modes through its pinned sanitizer compiler, runner, and runtime binding.
Pull-request and integration smoke SHALL fix seed, run count, input ceiling,
execution timeout, memory ceiling, mutation reload, and worker count. Native
hosted weekly/manual soak and exact local reproduction SHALL remain separately
bounded and outside required pull-request evidence.

Original corpora and dictionaries SHALL cover standards-shaped public-key and
consumer-shaped representation boundaries without containing production key
material or asserting trust. Exact binary COSE seeds MAY use a documented
text-only transport decoded solely by the harness. Failure artifacts SHALL be
retained for triage; an accepted defect SHALL be minimized and promoted to
committed corpus and deterministic regression evidence. Performance SHALL be
recorded without a hardware-specific pass threshold.

#### Scenario: ordinary crypto fuzz CI is repeatable

- **WHEN** the same revision runs the pull-request crypto fuzz gate
- **THEN** both targets SHALL receive the same seed, run count, and resource
  limits through the pinned Nix environment and terminate deterministically

#### Scenario: longer search remains bounded and diagnosable

- **WHEN** a hosted weekly/manual or locally reproduced crypto soak finds a
  sanitizer or invariant failure
- **THEN** the target SHALL stop inside the documented envelope and preserve
  its untrusted artifact for minimization without custom logging of its bytes

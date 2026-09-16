# factory-operations delta

## MODIFIED Requirements

### Requirement: CI routing is immutable and keeps the fast and slow model

The factory SHALL derive one target plan from an exact base/head diff and
delivery profile. The active-development PR requirement SHALL remain the
single Linux `fast` gate; active hosted weekly/manual slow evidence or its exact
local reproduction SHALL be recommended for toolchain, Nix, security, FFI and
release-sensitive paths. Unknown diff state SHALL fail closed to the complete
public evidence recommendation.

The target plan SHALL distinguish integration readiness from production-
promotion readiness. Fast integration SHALL retain policy, OpenSpec,
formatting, normal workspace build, strict Clippy, tests, and bounded
first-party analysis in the pinned Linux environment. Its initial execution
SLO SHALL be p50 at most six minutes and p95 at most eight minutes; a rolling
p95 above ten minutes SHALL trigger focused optimization without silently
removing evidence. Slow promotion SHALL retain complete exact-revision
platform, target, binding, security, conformance, coverage, performance,
fuzz/sanitizer, packaging, and receipt evidence as applicable. A slow failure
SHALL block production promotion, publication, and release preparation but
SHALL NOT add another required ordinary PR build or retroactively invalidate
an unrelated green integration.

#### Scenario: Factory-only pull request is planned

- **WHEN** an exact diff changes factory scripts, contracts or CI
- **THEN** the plan requires `fast`, records the factory area and recommends
  the complete slow backstop without creating three per-PR Rust builds

#### Scenario: Ordinary active-development slice is planned

- **WHEN** a bounded exact diff targets protected `develop`
- **THEN** integration readiness requires the single `fast` status while
  production-promotion readiness remains a separate exact-SHA decision

#### Scenario: Fast latency drifts beyond its ceiling

- **WHEN** a rolling comparable sample reports p95 execution above ten minutes
- **THEN** the factory creates or selects a focused optimization issue and does
  not remove correctness evidence without an accepted replacement

#### Scenario: Candidate is promoted toward production

- **WHEN** publication, release preparation, or a production-support claim is
  requested
- **THEN** the exact unchanged candidate SHA requires a green complete slow
  receipt and no unresolved release-blocking finding

## ADDED Requirements

### Requirement: Delivery iteration is locally batched and review-bounded

The factory SHALL treat one automatic discovery review and one remediation
round as the normal review budget. It SHALL perform focused local checks and a
distinct local review before the first candidate push, batch related repairs,
and preserve final exact-head evidence. A round budget SHALL NOT waive a P0/P1,
security regression, introduced defect, or failed acceptance criterion. New
independent non-blocking findings after the remediation cutoff SHALL become
linked follow-up issues instead of silently expanding the current slice.

One slice SHALL own one coherent behavior. Crossing 12 changed files or 1,000
non-generated changed lines SHALL require a decomposition note, but SHALL NOT
automatically waive or reject a cohesive migration, fixture, or security
change.

#### Scenario: First candidate is ready for hosted evidence

- **WHEN** focused local checks and local review pass for the bounded slice
- **THEN** the supervisor pushes one candidate head and requests one automatic
  discovery review after the first green fast result

#### Scenario: Review finds an introduced defect

- **WHEN** any round identifies a security regression, P0/P1, failed acceptance
  criterion, or defect introduced by the slice
- **THEN** the finding remains blocking until fixed or the slice is withdrawn

#### Scenario: Later review discovers independent hardening

- **WHEN** the remediation round is complete and a new P2/P3 finding is
  independent of the slice's acceptance and introduced behavior
- **THEN** the supervisor links a follow-up issue with evidence and does not
  expand an otherwise eligible pull request

#### Scenario: Slice exceeds decomposition guidance

- **WHEN** the exact diff crosses the documented file or non-generated line
  threshold
- **THEN** the plan records why the slice remains cohesive or names the split;
  the number alone neither approves nor rejects integration

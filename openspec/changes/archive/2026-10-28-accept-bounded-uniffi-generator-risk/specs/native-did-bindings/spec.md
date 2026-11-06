## MODIFIED Requirements

### Requirement: Host consumers execute the versioned value slice

The facade SHALL expose binding API version `1`. Swift and Kotlin/JVM host tests
SHALL compile, link and execute valid, invalid, oversized, redacted-error and
version checks against the built dynamic library. The host execution and
separately locked generator supply-chain checks SHALL run in the active hosted
weekly/manual slow macOS job or its exact local reproduction, not the
Linux-only fast pull-request lane.

A temporary generator advisory acceptance SHALL be limited to one exact
build-only dependency path and advisory, SHALL expire automatically, and SHALL
NOT weaken runtime, root or fuzz audit policy.

#### Scenario: host language consumes API version one

- **WHEN** the Swift and Kotlin/JVM smoke programs load the generated binding
- **THEN** each SHALL observe API version `1` and pass the same success and
  failure behavior families

#### Scenario: fast and slow CI remain deliberately separated

- **WHEN** ordinary pull-request CI runs during the active-development phase
- **THEN** the Linux fast line SHALL retain factory, build, lint and test gates,
  while macOS host execution and generator deny/audit evidence remain in the
  native weekly/manual slow workflow or exact local reproduction

#### Scenario: accepted generator graph remains exact and current

- **WHEN** the weekly lane evaluates the temporary RUSTSEC-2026-0374 acceptance
- **THEN** it SHALL fail before host execution if the date is expired, the exact
  reviewed inverse dependency path has changed, or any other audit finding exists

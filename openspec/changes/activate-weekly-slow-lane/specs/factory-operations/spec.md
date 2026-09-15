## MODIFIED Requirements

### Requirement: CI routing is immutable and keeps the fast and slow model

The factory SHALL derive one target plan from an exact base/head diff and
delivery profile. The active-development PR requirement SHALL remain the
single Linux `fast` gate; active hosted weekly/manual slow evidence or its exact
local reproduction SHALL be recommended for toolchain, Nix, security, FFI and
release-sensitive paths. Unknown diff state SHALL fail closed to the complete
public evidence recommendation.

#### Scenario: Factory-only pull request is planned

- **WHEN** an exact diff changes factory scripts, contracts or CI
- **THEN** the plan requires `fast`, records the factory area and recommends
  the complete slow backstop without creating three per-PR Rust builds

## MODIFIED Requirements

### Requirement: Coverage stays outside the fast pull-request lane

Coverage instrumentation and reporting SHALL run only in the active hosted
weekly/manual slow workflow or its exact local reproduction during the ADR 0081
temporary phase. The pull-request and `develop` fast job SHALL remain unchanged.

#### Scenario: Coverage is added to fast CI

- **WHEN** workflow validation detects the coverage runner in the fast job
- **THEN** policy validation SHALL fail the change

#### Scenario: Weekly coverage evidence runs

- **WHEN** the native slow schedule runs from protected default `develop`
- **THEN** coverage SHALL publish SHA-bound reviewable evidence without
  becoming a required pull-request or release threshold

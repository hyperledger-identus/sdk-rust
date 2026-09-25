# ssi-upstream-program Specification

## ADDED Requirements

### Requirement: IDR-023 advances beyond request-bound Credential Request construction

Issue #364 SHALL consume #362 correlated authority and construct one bounded
authorized-dataset Credential Request without transport, trust, storage or
product effects. IDR-023 SHALL remain `delivery_status=in_progress` and point
to focused open successor #366 before integration.

#### Scenario: correlation hands off to request construction

- **WHEN** #362 is delivered and #364 is open
- **THEN** the canonical backlog references #364 without marking IDR-023 done

#### Scenario: request construction hands off to response binding

- **WHEN** #364 is ready to integrate
- **THEN** the canonical backlog references #366 and keeps IDR-023 in progress

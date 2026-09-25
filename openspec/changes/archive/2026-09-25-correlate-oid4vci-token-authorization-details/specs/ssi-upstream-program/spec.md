# ssi-upstream-program Specification

## ADDED Requirements

### Requirement: IDR-023 advances beyond Token Authorization Details correlation

Issue #362 SHALL consume the #360 request-bound success and correlate its
bounded credential Authorization Details with the exact selected Credential
Configuration without adding request construction, transport, trust, storage
or product effects. IDR-023 SHALL remain `delivery_status=in_progress`.

#### Scenario: response binding hands off to correlation

- **WHEN** #360 is delivered and #362 is open
- **THEN** the canonical backlog references #362 without marking IDR-023 done

Before #362 integrates, the row SHALL reference focused successor #364 for
request-bound authorized-dataset Credential Request construction.

#### Scenario: correlation remains independently reversible

- **WHEN** #362 is ready to integrate
- **THEN** IDR-023 references #364 and remains in progress

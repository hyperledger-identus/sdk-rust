# ssi-upstream-program Specification

## ADDED Requirements

### Requirement: IDR-023 advances beyond Pre-Authorized Token response binding

Issue #375 SHALL consume the pre-authorized request and bind a bounded Token
Endpoint success/error response to exact non-secret offer/server lineage. It
SHALL NOT add transport, trust, storage, request construction or product
effects. After delivery, IDR-023 SHALL remain `in_progress` and reference open
focused interoperability-vector issue #376.

#### Scenario: required binder hands off to conformance vectors

- **WHEN** #375 is delivered and #376 remains open
- **THEN** the canonical backlog references #376 without marking IDR-023 or M4 complete

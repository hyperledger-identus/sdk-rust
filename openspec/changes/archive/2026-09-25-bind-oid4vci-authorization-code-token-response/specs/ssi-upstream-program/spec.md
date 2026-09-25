# ssi-upstream-program Specification

## ADDED Requirements

### Requirement: IDR-023 advances beyond Authorization Code Token response binding

Issue #360 SHALL consume the #358 request and bind a bounded success/error
Token Endpoint HTTP response to its exact lineage without adding transport,
trust, token-storage or product effects. IDR-023 SHALL remain
`delivery_status=in_progress`.

#### Scenario: request construction hands off to response binding

- **WHEN** #358 is delivered and #360 is open
- **THEN** the canonical backlog references #360 without marking IDR-023 done

Before #360 integrates, the row SHALL reference one open focused successor for
the next independently reversible OID4VCI wallet-engine transition.

#### Scenario: response binding remains independently reversible

- **WHEN** #360 is ready to integrate
- **THEN** IDR-023 references its open successor and remains in progress

## ADDED Requirements

### Requirement: IDR-023 advances through bound deferred continuation construction

Issue #368 SHALL preserve the minimum authority from a request-bound initial
HTTP 202 response and consume it into one authorized Deferred Credential
Request without marking IDR-023 complete. Before integration, the canonical
backlog SHALL reference one focused open successor for deferred response
binding without transport or polling effects.

#### Scenario: #368 integrates

- **WHEN** authority-preserving deferred request construction is reviewed and green
- **THEN** the canonical backlog owner advances from #368 to its open successor
- **AND** #7 and #20 remain open component and program parents

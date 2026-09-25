## ADDED Requirements

### Requirement: advance OID4VCI response binding to deferred continuation

Issue #366 SHALL deliver consuming initial Credential Endpoint response
classification without marking IDR-023 complete and SHALL reference focused
successor #368 for authority-preserving deferred continuation construction.

#### Scenario: #366 integrates

- **WHEN** the request-bound response classifier is reviewed and green
- **THEN** the canonical backlog owner advances from #366 to open issue #368

## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and reference open
focused child #345 while request-bound validation of unencrypted Deferred
Credential Endpoint success responses is being delivered. Issue #250 delivered
only bounded construction of the unencrypted Final Deferred Credential Request;
issue #345 retains its transaction identifier and validates status, media type,
bounded success bodies and pending-response correlation.

The active slice SHALL NOT claim the full OID4VCI engine, HTTP execution, token
ownership/validation, TLS, interval scheduling, retry/invalidation policy,
request/response encryption, extension parameters, error response handling,
credential verification/storage, proof-count correlation, format/chain
extensions, consumer adoption, publication or release.

#### Scenario: Focused response-correlation child is active

- **WHEN** issue #345 is open while its bounded request-bound success response
  transition is being delivered
- **THEN** `IDR-023` references #345 with `delivery_status=in_progress`
- **AND** #7 and #20 remain open component and program parents

#### Scenario: request-bound response validation is not engine completion

- **WHEN** a Deferred Credential Request validates bounded `200` and correlated
  `202` responses without HTTP, token, timing, lifecycle, error or encryption
  behavior
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered

# ssi-upstream-program Specification

## MODIFIED Requirements

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and reference open
focused child #348 while bounded unencrypted Deferred Credential payload-error
handling is delivered. Issue #250 delivered bounded request construction and
issue #345 delivered bounded issued/pending success responses with exact
pending transaction correlation. Issue #348 composes the existing Credential
Error parser and adds only deferred-specific classification and lifecycle
guidance.

The active slice SHALL NOT claim the full OID4VCI engine, HTTP execution, RFC
6750 challenge parsing, token ownership/validation, TLS, interval scheduling,
retry/invalidation effects, request/response encryption, credential
verification/storage, proof-count correlation, format/chain extensions,
consumer adoption, publication or release.

#### Scenario: Focused deferred payload-error child is active

- **WHEN** issue #348 is open while its bounded payload-error transition is
  being delivered
- **THEN** `IDR-023` references #348 with `delivery_status=in_progress`
- **AND** #7 and #20 remain open component and program parents

#### Scenario: payload-error validation is not engine completion

- **WHEN** a Deferred Credential Request validates and classifies bounded
  payload errors without transport, authorization, timing or lifecycle effects
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered

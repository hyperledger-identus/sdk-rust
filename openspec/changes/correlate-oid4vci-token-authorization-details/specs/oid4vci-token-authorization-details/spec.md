# oid4vci-token-authorization-details Specification

## ADDED Requirements

### Requirement: Response-local validation supports private consuming reuse

The existing public `TokenResponseWithAuthorizationDetails` behavior SHALL
remain exact. The crate MAY privately decompose that consumed state into its
Token Response core, recognized details and unknown-type count so a stronger
request-bound capability can move validated zeroizing owners without cloning
or reparsing them.

#### Scenario: public response-local compatibility remains exact

- **WHEN** existing callers validate and inspect Authorization Details
- **THEN** every existing public method, value order, limit, error and redacted
  Debug behavior remains unchanged

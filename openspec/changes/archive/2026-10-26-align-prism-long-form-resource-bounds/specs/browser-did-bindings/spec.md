# browser-did-bindings Specification

## MODIFIED Requirements

### Requirement: Browser failures remain bounded and redacted

The facade SHALL preserve the domain parser's 131,072-byte DID and
139,264-byte DID URL ceilings. Invalid and oversized input SHALL throw a closed
SDK-owned error with a stable constant code and SHALL NOT expose caller text,
parser details or implementation diagnostics.

#### Scenario: invalid browser input is rejected

- **WHEN** a caller supplies invalid or oversized input containing a canary
- **THEN** Chromium and Firefox SHALL observe the matching constant error code
  and every exposed error property SHALL omit the canary

# oid4vci-authorization-code-token-request Specification

## ADDED Requirements

### Requirement: Response binding is the consuming typed continuation

`AuthorizationCodeTokenRequest` SHALL expose a consuming response-binding
transition that removes the request's zeroizing form body before remote
response validation. No parallel Clone, retry-token, raw-parts or
non-consuming response-binding path SHALL be introduced.

#### Scenario: malformed remote input still consumes the request

- **WHEN** response status, headers or body are rejected
- **THEN** the caller cannot recover a reusable typed request, authorization
  code or PKCE verifier from the failed transition

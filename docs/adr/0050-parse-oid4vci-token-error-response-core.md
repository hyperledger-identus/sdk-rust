# ADR 0050: parse a bounded OID4VCI Token Error Response core

- **Status:** Accepted for implementation
- **Date:** 2026-09-07
- **Decision authority:** standing product mandate and IDR-023 issue #129
- **Related work:** issues #7, #20, #127, and #129

## Context

The SDK parses the successful Token Response but has no reusable error
boundary. OID4VCI Final delegates token errors to RFC 6749 and adds
Pre-Authorized Code clarifications. OAuth defines six standard codes while
keeping the error registry extensible. Optional descriptions and URI references
are attacker-controlled remote metadata, not trusted wallet guidance.

## Decision

1. Add a partial `TokenErrorResponseCore` rather than an HTTP or recovery state.
2. Preserve the exact bounded code in `TokenEndpointErrorCode` and classify
   the six RFC values through `TokenEndpointErrorKind`, with all other valid
   codes represented as `Extension`.
3. Validate optional description and URI-reference fields under independent
   limits. Mark description access as explicitly untrusted and provide no URI
   dereference operation.
4. Structurally validate and discard unknown fields; retain no raw response.
5. Zeroize retained strings and keep Debug/errors free of remote content.
6. Preserve the existing dependency, feature, target, and unsafe-code cone.

## Consequences

- Headless consumers can distinguish standard token failures without losing
  extension interoperability.
- Remote developer text and URIs cannot be mistaken for trusted UI or network
  policy through the SDK surface.
- Parsing cannot claim HTTP validity, request correlation, retryability,
  issuer trust, or protocol-engine completion.
- Later transport/state slices can consume the core under their own explicit
  status, header, correlation, and recovery contracts.

# ADR 0049: parse a bounded OID4VCI Token Response core

- **Status:** Accepted for implementation
- **Date:** 2026-09-07
- **Decision authority:** standing product mandate and IDR-023 issue #127
- **Related work:** issues #7, #20, #125, and #127

## Context

The SDK can construct the mandatory Pre-Authorized Code Token Request but has
no reusable response boundary. OID4VCI Final delegates the successful response
core to RFC 6749 and optionally adds Authorization Details. OAuth requires
unknown response parameters to be ignored while leaving token sizes undefined.
Access and refresh tokens are secrets, and treating an unvalidated
Authorization Details value as issuance authority would be unsafe.

## Decision

1. Add a partial `TokenResponseCore` rather than a full successful issuance
   state.
2. Validate required and optional RFC 6749 core fields under positive
   aggregate and independent limits while retaining exact token-type spelling.
3. Keep the exact input and sensitive extracted strings in zeroizing storage;
   expose tokens and scope only through explicitly sensitive accessors.
4. Syntax-check and ignore unknown members. Record only the presence of
   `authorization_details`; a later consuming transition must validate it
   before Credential Request construction.
5. Do not couple parsing to HTTP, a request, trust, time, token-type policy,
   replay, storage, or a consumer implementation.
6. Preserve the existing dependency, feature, target, and unsafe-code cone.

## Consequences

- Headless consumers gain a bounded, redaction-safe OAuth response core.
- Extension members, including historical nonce fields, do not cause an
  otherwise valid Final core to fail.
- The partial state cannot accidentally represent Authorization Details or
  token trust as validated.
- Later slices can consume the privately retained exact JSON into stronger
  OID4VCI states without broadening this API.

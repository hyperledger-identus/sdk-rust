# ADR 0059: validate the OID4VCI Credential payload-error HTTP response

- **Status:** Accepted for implementation
- **Date:** 2026-09-07
- **Decision authority:** standing product mandate and IDR-023 issue #147
- **Related work:** issues #7, #20, #143, #145, and #147

## Context

The SDK parses a bounded Credential Error Response body but does not validate
the Final HTTP envelope that distinguishes Credential Request payload errors
from success, deferred issuance, and RFC 6750 Authorization Error Responses.
The Final text requires status 400 and `application/json`, displaces generic
`invalid_request` for payload failures, and shows `Cache-Control: no-store`
only in a non-normative example.

## Decision

1. Add composed `CredentialErrorHttpResponseLimits` with existing body limits
   and an independent positive Content-Type byte bound.
2. Add `CredentialErrorResponseCore::parse_http_response`, checking exact
   status 400 before media and checking bounded RFC-shaped `application/json`
   before the existing body parser.
3. Reject exact generic `invalid_request` after bounded body parsing while
   preserving every other syntactically valid extension code.
4. Return only the existing body core and retain neither status nor media, so
   the API does not imply HTTP provenance or request correlation.
5. Do not accept or require Cache-Control and keep RFC 6750 Authorization Error
   Responses outside this capability.
6. Preserve the existing dependency, feature, target, unsafe-code, zeroizing,
   and redaction boundaries.

## Consequences

- Headless consumers can validate the Final payload-error envelope without an
  HTTP framework or product recovery policy.
- Non-conforming status/media and the explicitly displaced generic code fail
  closed with static diagnostics.
- Unknown valid extension codes remain interoperable but gain no retry,
  remediation, blame, denial, or UI meaning.
- Parsing cannot prove HTTP execution, origin, request correlation, issuer
  truth, authorization state, trust, credential validity, or replay safety.
- Authorization errors, transport execution, deferred/encrypted responses,
  recovery policy, and downstream adoption remain later issue-first slices.

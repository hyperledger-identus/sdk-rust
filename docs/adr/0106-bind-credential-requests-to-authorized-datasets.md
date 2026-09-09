# ADR 0106: bind Credential Requests to authorized datasets

- **Status:** Accepted for implementation
- **Date:** 2026-09-09
- **Decision authority:** standing product mandate and IDR-023 issue #239
- **Related work:** issues #7, #139, #237, and #239
- **Normative baseline:** OpenID4VCI 1.0 Final sections 6.2 and 8.2

## Context

ADR 0105 introduced a typed Token Response state containing validated
Credential Dataset identifiers. The existing Credential Request constructor
can emit only `credential_configuration_id` and correctly rejects Token
Responses that contain Authorization Details. A separate transition is needed
to select one authorized dataset without allowing an arbitrary caller-provided
identifier or accidentally emitting both mutually exclusive selectors.

## Decision

1. Add an explicit constructor over `TokenResponseWithAuthorizationDetails`.
2. Select one recognized Authorization Detail and one of its Credential Dataset
   identifiers by checked source-order indices.
3. Require the selected detail's `credential_configuration_id` to exactly
   match a configuration in the already matched Credential Offer.
4. Emit exactly `credential_identifier` for this route. Preserve exactly
   `credential_configuration_id` for the existing route.
5. Share Bearer-token validation, proof validation, authorization construction,
   complete-body bounds, deterministic encoding and redaction privately.
6. Return fieldless stable errors for missing indices and configuration
   mismatch. Never place selected identifiers in diagnostics.
7. Establish only local state correlation. Do not infer token, issuer,
   metadata, credential or dataset trust.

## Consequences

- A headless wallet can construct both Final Credential Request selector
  branches from typed prior state.
- Arbitrary strings cannot enter the authorized-dataset selector API.
- Configuration and dataset selectors cannot coexist in one generated body.
- No new dependency or public limit type is introduced.
- HTTP execution, request correlation, authorization-code flow, trust,
  selection policy, storage and downstream adoption remain later work.

## Rejected alternatives

- Accepting a raw credential identifier would discard the authorization proof
  state and permit confused-deputy request construction.
- Adding an optional identifier to the existing constructor would admit
  ambiguous selector combinations and weaken the type boundary.
- Duplicating the full request builder would allow bounds and security behavior
  to drift between selector branches.

## Rollback

Remove the additive constructor, selector helper, stable errors and tests. The
existing configuration-ID request constructor and typed Authorization Details
state remain compatible.

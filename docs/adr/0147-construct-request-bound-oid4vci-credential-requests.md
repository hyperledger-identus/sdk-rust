# ADR 0147: construct request-bound OID4VCI Credential Requests

- **Status:** Accepted
- **Date:** 2026-09-25
- **Issue:** [#364](https://github.com/hyperledger-identus/sdk-rust/issues/364)
- **Decision authority:** ADR 0105, ADR 0142, ADR 0146 and issue #364

## Context

Issue #362 correlates a successful Authorization Code Token Response to the
exact issuer, server and Credential Configuration selected by the originating
request. It owns the Token Response core and the source-ordered authorized
Credential Dataset identifiers. The older Credential Request constructors are
useful compatibility surfaces, but accept detached metadata and response-local
state and therefore cannot preserve that exact request authority by type.

OpenID4VCI 1.0 Final permits a Credential Request to select one authorized
dataset using `credential_identifier`. Its access token is carried through the
RFC 6750 Bearer authentication scheme. The existing serializer already
implements this wire shape, proof limits, token validation and redaction.

## Decision

1. Add a consuming transition on
   `CorrelatedAuthorizationCodeTokenResponse`; do not add a public constructor
   or detached replacement inputs.
2. Accept only a source-order identifier index, holder-produced JWT proofs and
   the existing positive `JwtCredentialRequestLimits`.
3. Derive the Credential Endpoint from retained issuer metadata, the access
   token and token type from the retained Token Response, and the selector from
   the correlated authorized identifier collection.
4. Reuse one private serializer for the new transition and both existing
   compatibility constructors. Do not fork JSON, proof, bearer-token or limit
   behavior.
5. Preserve the existing `CredentialRequestIdentifierMissing` diagnostic for
   an out-of-range index and every existing deterministic validation error.
6. Return the existing owned, redaction-safe `JwtCredentialRequest`; duplicate
   only its public endpoint and keep Authorization/body values zeroizing.
7. Add no error variant, dependency, feature, HTTP execution, response
   handling, trust, storage, retry, recovery, polling, format, chain, consumer
   or product policy.
8. Defer consuming request-bound Credential Endpoint response classification
   to focused issue [#366](https://github.com/hyperledger-identus/sdk-rust/issues/366).

## Consequences

A wallet can no longer substitute metadata, a token response, configuration or
dataset identifier between exact token-response correlation and request
construction when it uses the new path. The state is one-shot by ownership,
and validation failure destroys the consumed secret-bearing state. Existing
unpublished constructors remain available without behavior changes.

The transition proves structural continuity only. It does not establish token,
issuer, dataset, proof or Credential trust, freshness or authorization, and it
does not perform transport.

## Reconsideration and rollback

Reconsider the compatibility constructors only through a separately reviewed
breaking API decision. Rollback removes the additive consuming method, private
decomposition helper, tests and evidence while leaving the shared serializer
and earlier constructors intact.

# ADR 0151: bind OID4VCI Pre-Authorized Token responses

- **Status:** Accepted
- **Date:** 2026-09-25
- **Issue:** [#375](https://github.com/hyperledger-identus/sdk-rust/issues/375)
- **Decision authority:** ADR 0048, ADR 0145 and issue #375

## Context

The pre-authorized flow constructs a bounded request containing a
Pre-Authorized Code and optional Transaction Code, but its request state did
not retain the matched issuer, selected server or offered configuration scope.
The shared Token Response cores deliberately prove no HTTP or request
correlation. Retaining the complete Credential Offer to restore authority
would also retain the Pre-Authorized Code in its exact JSON.

OpenID4VCI 1.0 Final delegates Token Endpoint success/error behavior to OAuth
2.0 and identifies `invalid_request` and `invalid_grant` for pre-authorized
failures. The current request has no client-authentication authority.

## Decision

1. During request construction, retain the exact matched Credential Issuer
   Metadata, selected Authorization Server Metadata and ordered offered
   Credential Configuration IDs. Drop the grant-bearing offer after building
   the zeroizing form; never retain its JSON as lineage.
2. Consume one `PreAuthorizedTokenRequest` to bind one response. Erase its
   zeroizing form before inspecting supplied remote response metadata.
3. Accept exact HTTP `200` as success and exact `400` as OAuth error. Reject
   all other statuses before header/body parsing. Do not borrow the
   Authorization Code flow's `401` behavior without authenticated-client
   authority.
4. Require independently bounded JSON Content-Type, bare `no-store` and bare
   `no-cache`, then reuse the existing status-selected bounded response core.
5. Share only a crate-private Token Endpoint HTTP-header validator with the
   Authorization Code flow. Keep flow-specific public lineage, outcomes,
   limits and append-only diagnostics.
6. Preserve whether a Transaction Code was present as non-secret request
   evidence, but retain neither that value nor the Pre-Authorized Code.
7. Advance the Final matrix row to implemented and IDR-023 to focused issue
   [#376](https://github.com/hyperledger-identus/sdk-rust/issues/376). Do not
   claim M4 complete until the vector/provenance evidence is resolved.

## Consequences

Consumers receive a one-shot, status-first response seam whose success and
error data cannot be detached from the matched offer scope and server. The
existing response-core and Authorization Code behavior remains compatible,
and no dependency, feature, transport or product policy is introduced.

The result proves bounded syntax and typed lineage only. It does not prove
HTTP origin, TLS, issuer/server trust, token validity, authorization, cache
compliance outside supplied fields, retries, storage or successful issuance.

## Reconsideration and rollback

Reconsider `401` only with an explicit authenticated-client request state.
Reconsider retained scope if a later Final profile gives pre-authorized flows
a narrower server-authorized dataset. Rollback removes the additive binder,
limits, outcomes, diagnostics and retained request lineage while leaving the
existing request and core parsers intact.

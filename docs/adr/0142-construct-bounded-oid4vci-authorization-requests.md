# ADR 0142: construct bounded OID4VCI Authorization Requests

- **Status:** Accepted
- **Date:** 2026-09-25
- **Issue:** [#354](https://github.com/hyperledger-identus/sdk-rust/issues/354)
- **Decision authority:** ADR 0083, ADR 0102, ADR 0141 and issue #354

## Context

Issue #352 owns validated Authorization Code server, credential selection,
client, redirect, CSRF state and SDK-derived PKCE S256 inputs. Wallet adapters
still need deterministic protocol bytes without importing browser, transport or
OAuth-client policy into the reusable crate.

OID4VCI Final permits credential intent through `authorization_details` or
scope, while RFC 8707 resource indicators address a different authorization
resource concern. Emitting several mechanisms silently would make intent and
server behavior ambiguous. Authorization Endpoints may already contain query
fields, so blind string concatenation would permit duplicate OAuth parameters
and alternate-intent smuggling.

The evaluated `oauth2 5.0.0` crate remains an oracle under ADR 0102: adopting
its broader URL, HTTP, RNG, time and serialization cone does not improve this
narrow deterministic boundary. ADR 0083 already accepts a strict local HTML
form codec for exact OID4VCI behavior.

## Decision

1. Consume `CredentialOfferWithAuthorizationRequestInput` into an owned
   `AuthorizationRequest`; retain the complete predecessor for later exact
   response correlation and code exchange.
2. Serialize exactly one OID4VCI Final Authorization Details object with type
   `openid_credential` and the selected `credential_configuration_id`. Do not
   emit scope or RFC 8707 `resource` in this slice.
3. Add `locations` containing the Credential Issuer exactly when issuer
   metadata explicitly advertised one or more Authorization Servers. Omitted
   `authorization_servers` retains the issuer-as-default behavior without a
   redundant location.
4. Append parameters in the fixed order `response_type`, `client_id`,
   `redirect_uri`, `state`, `code_challenge`, `code_challenge_method`,
   `authorization_details`, then optional `issuer_state`.
5. Preserve an existing Authorization Endpoint query byte-for-byte only after
   strict bounded form decoding. Reject empty/malformed fields, decoded
   duplicate names, managed OAuth names, `scope`, `resource`, `request`, and
   `request_uri` collisions.
6. Require positive explicit limits for Authorization Details bytes, existing
   query count/name/value bytes, and the complete request URI. Apply bounds
   before unbounded retained allocation.
7. Own the exact request URI in zeroizing storage, expose it only through an
   explicitly sensitive accessor, and redact it from Debug and static errors.
8. Factor the already accepted exact form mechanics into one private module;
   preserve predecessor request bytes and admit no new dependency.
9. Defer response parsing, state/issuer correlation, browser/PAR/HTTP and code
   exchange to focused successor [#356](https://github.com/hyperledger-identus/sdk-rust/issues/356).

## Consequences

Consumers receive a deterministic GET request target with an explicit chosen
credential-intent mechanism and no hidden execution. Existing safe endpoint
query extensions survive exactly, while alternate OAuth intent and duplicate
fields fail closed. The complete request remains sensitive because it carries
state and can carry issuer state; adapters must avoid logs, telemetry,
referrers and generic persistence.

The narrow local implementation keeps coupling and dependency growth at zero,
but the SDK owns its exact codec and URI-construction tests. It does not claim
successful delivery, client/redirect registration, server trust, returned
state or issuer correlation, authorization, or token issuance.

## Reconsideration and rollback

Reconsider a shared OAuth crate only when a released, feature-sliced candidate
meets ADR 0102 and provides multiple protocol benefits without weakening
bounds or diagnostics. Reconsider scope/resource/PAR only in separately
specified interoperable profiles. Rollback removes the additive request type,
private factoring, diagnostics and evidence while preserving #352 inputs and
all prior wire behavior.

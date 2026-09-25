# ADR 0141: derive bounded OID4VCI PKCE request inputs

- **Status:** Accepted
- **Date:** 2026-09-25
- **Issue:** [#352](https://github.com/hyperledger-identus/sdk-rust/issues/352)
- **Decision authority:** ADR 0102, ADR 0137 and issue #352

## Context

Issue #350 proves that one offered Authorization Code flow is bound to a
capable advertised server. A later Authorization Request still needs a client
identifier, redirect URI, CSRF state, credential selection and PKCE pair.
Accepting a caller-provided verifier and unrelated challenge would create a
misleading security state. Generating entropy in the protocol crate would
instead import runtime policy and adapters.

ADR 0102 keeps exact `oauth2 5.0.0` as an oracle because its otherwise useful
PKCE mechanics are inseparable from broad URL, clock, RNG, HTTP and JSON
dependencies and its verifier constructor is under-validated/panicking. The
SDK already owns bounded SHA-256 and base64url primitives in `identus-crypto`.

## Decision

1. Add a consuming `CredentialOfferWithAuthorizationRequestInput` state after
   the issue #350 server-bound predecessor.
2. Select one already validated offered Credential Configuration by index,
   retaining no duplicate caller string.
3. Require positive configurable bounds for client identifier, redirect URI
   and state; validate their RFC 6749 grammar/URI structure before retention.
4. Require the fixed RFC 7636 verifier grammar of 43 through 128 unreserved
   ASCII characters.
5. Derive S256 internally from the verifier. Do not accept a challenge input,
   support `plain`, or generate entropy.
6. Reuse `identus-crypto` through a direct default-disabled dependency with
   only `hash` and `base64` features. No crypto type crosses the public
   OID4VCI facade.
7. Preserve all inputs under redacted types and zeroize retained state and
   verifier values.
8. Defer scope versus `authorization_details`, URL serialization, PAR and
   Authorization Response/callback correlation to focused successor #354.

## Consequences

Consumers receive a non-panicking, bounded state that proves the verifier/S256
relationship and preserves the exact offered configuration and `issuer_state`.
The direct internal cone intentionally adds `identus-crypto`; no new external
package or runtime is admitted. The state does not claim entropy quality,
client or redirect registration, successful request delivery, returned-state
comparison, server mix-up defense, code exchange or authorization.

## Reconsideration and rollback

Reconsider external OAuth reuse only when a released feature-sliced candidate
satisfies ADR 0102's bounded facade requirements and proves multi-mechanic
payoff. Rollback removes the additive state, feature-minimal dependency,
diagnostics and evidence while preserving server binding and all prior wire
behavior.

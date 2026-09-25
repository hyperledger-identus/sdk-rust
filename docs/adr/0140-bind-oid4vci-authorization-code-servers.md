# ADR 0140: Bind OID4VCI Authorization Code offers to capable servers

- Status: Accepted
- Date: 2026-09-25
- Decision owners: issue #350 under #7 and #20
- Normative sources: OpenID4VCI 1.0 Final sections 4.1, 5.1 and 12.3;
  RFC 8414 section 2

## Context

The SDK validates Credential Offers, their grants, Credential Issuer Metadata,
and a bounded partial Authorization Server Metadata core. A wallet choosing the
Authorization Code flow still had to duplicate the cross-document rules for
selecting an advertised server, honoring an offer hint, interpreting omitted
grant metadata, and requiring an Authorization Endpoint.

The existing metadata error catalogue already contains the standing maximum
of 39 records. Adding the transition's diagnostics there would weaken a
repository maintainability guard.

## Decision

Add `CredentialOfferWithAuthorizationCodeServer` as an owned state reached by
consuming `CredentialOfferWithMetadata` and one caller-selected
`AuthorizationServerMetadataCore`.

The transition checks, in order:

1. the offer contains an Authorization Code grant;
2. the selected server is effectively advertised;
3. the selected server exactly matches any grant hint;
4. effective grant types contain exact `authorization_code`, using the RFC
   8414 omitted-list default; and
5. an Authorization Endpoint is present.

The state preserves optional `issuer_state` only through its existing
zeroizing owner. A Token Endpoint is deliberately not required until a later
code-exchange boundary. Four new static errors live in a focused seventh
private catalogue, leaving every catalogue at or below 39 records.

## Consequences

- Wallets share one deterministic, redacted server-selection contract.
- Later Authorization Request work receives a least-authority typed
  predecessor without JSON reparsing or network authority.
- No dependency, feature, lockfile, wire format, parser or target changes.
- Selection does not prove metadata provenance, trust, reachability, client
  eligibility, PKCE/CSRF safety, callback issuer binding, authorization or
  issuance.

## Rejected alternatives

- Reuse the Pre-Authorized Code state: it proves a different grant and Token
  Endpoint capability.
- Require a Token Endpoint now: it exceeds what is needed to start
  authorization.
- Add four records to the full metadata catalogue: it violates the 39-record
  review ceiling.
- Build the Authorization Request in the same slice: client, redirect, state,
  PKCE and credential-selection inputs need their own bounded contract.

## Rollback

Remove the additive state, module export, four suffix errors, focused error
catalogue, tests and documentation. Existing offer and metadata contracts are
unchanged.

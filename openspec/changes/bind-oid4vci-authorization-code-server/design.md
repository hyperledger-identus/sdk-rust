# Design

## Owned semantic transition

Add `CredentialOfferWithAuthorizationCodeServer`, privately owning one
`CredentialOfferWithMetadata` and one `AuthorizationServerMetadataCore`.
Expose both predecessors by shared reference. Optional `issuer_state` remains
available through the existing nested Authorization Code grant; no new copy or
parallel owner is introduced.

Add `CredentialOfferWithMetadata::try_with_authorization_code_server`, which
consumes both inputs and applies checks in this order:

1. Require an Authorization Code grant.
2. Require the selected server issuer to be effectively advertised by issuer
   metadata.
3. Require exact agreement with the grant's optional server hint.
4. Require effective exact `authorization_code` grant support, including the
   existing RFC 8414 omitted-list default.
5. Require an Authorization Endpoint.

The ordered precedence keeps diagnostics deterministic and does not inspect a
server when the selected flow was never offered.

## Error and compatibility surface

Append four static variants: Authorization Code grant missing, selected server
hint mismatch, selected server does not support the grant, and Authorization
Endpoint required. Reuse the existing generic unadvertised-server diagnostic.
No prior error row, code, category, Display text or help URL changes.

## Least authority and redaction

The transition requires only the endpoint used by the next authorization
step. Token Endpoint capability is deliberately deferred until code exchange.
Debug emits only the type name and a non-exhaustive marker; consumers reach
bounded remote values through explicit borrowed accessors.

## Risks and mitigations

- Defaulting drift: use `effective_grant_type` rather than reimplementing RFC
  8414 defaults.
- Hint ambiguity: compare exact validated identifiers and preserve existing
  issuer-metadata multiplicity checks.
- Excess authority: do not require or expose a transport client, Token
  Endpoint, redirect, PKCE or callback behavior.
- Mix-up overclaim: document that selection does not satisfy Final section
  12.3 response validation.
- Sensitive leakage: retain existing zeroizing owners and static diagnostics.

## Rollback

Remove the additive module, export, variants, tests, ADR and capability. No
existing parser, type, wire, storage or migration contract is changed.

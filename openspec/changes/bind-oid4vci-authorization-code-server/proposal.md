# Bind OID4VCI Authorization Code Offers to Capable Servers

## Why

The SDK already validates Credential Offers, their Authorization Code grant,
Credential Issuer Metadata, and a bounded partial Authorization Server
Metadata core. Wallets still have to reproduce the cross-document rules that
select one advertised server, honor an offer-level server hint, apply RFC 8414
grant defaults, and require an Authorization Endpoint. That duplicated policy
is an unsafe foundation for later Authorization Request and PKCE work.

## What changes

- Consume a matched offer/issuer-metadata state and one caller-selected
  Authorization Server Metadata core.
- Require an Authorization Code grant before evaluating server capabilities.
- Prove exact effective advertisement and exact agreement with any grant-level
  Authorization Server hint.
- Require effective `authorization_code` support and an Authorization
  Endpoint.
- Return an owned, redacted typed predecessor that preserves the existing
  grant, optional issuer state, issuer metadata and selected server metadata.
- Add stable append-only diagnostics, exact defaulting/mismatch/capability
  tests, ADR 0140, architecture evidence and the next bounded roadmap handoff.

## What does not change

No Authorization Request URL, client identifier, redirect URI, state/CSRF
value, PKCE value, PAR request, scope, authorization details, browser/callback,
code exchange, discovery, HTTP, DNS, TLS, trust, persistence or product policy
is added.

## Capabilities

### New capabilities

- `oid4vci-authorization-code-server`: bounded cross-document selection of one
  capable Authorization Server for an offered Authorization Code flow.

### Modified capabilities

- `oid4vci-error-contracts`: append stable static diagnostics needed by the
  new state transition without changing prior entries.
- `ssi-upstream-program`: record #350 as the current IDR-023 delivery slice and
  hand the live pointer to a focused successor before integration.

## Authority

Issue #350, OpenID4VCI 1.0 Final sections 4.1, 5.1 and 12.3, RFC 8414 section
2, and the standing SDK delivery mandate.

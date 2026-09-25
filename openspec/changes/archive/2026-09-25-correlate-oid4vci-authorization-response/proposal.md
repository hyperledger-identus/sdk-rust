# Correlate a Bounded OID4VCI Authorization Response

## Why

Issue #354 constructs a deterministic Authorization Request and retains the
state, selected server, redirect, PKCE verifier and credential intent needed by
later stages. A wallet still cannot safely accept the front-channel result:
raw query fields are attacker-controlled, OAuth success and error branches can
be confused, state must match exactly, and multi-server clients need explicit
issuer-identification evidence before code exchange.

## What changes

- Extend the existing partial Authorization Server Metadata projection with
  RFC 9207 `authorization_response_iss_parameter_supported`, defaulting false.
- Consume one `AuthorizationRequest` plus one already-extracted bounded query
  payload using strict `application/x-www-form-urlencoded` semantics.
- Reject malformed/duplicate parameters, ambiguous success/error branches,
  missing or mismatched state, and invalid RFC 9207 issuer behavior.
- Return either a redacted correlated code state retaining the complete request
  lineage, or a bounded terminal OAuth Authorization Error outcome.
- Preserve explicit evidence that RFC 9207 issuer identification was verified
  or was not advertised; never imply the latter is mix-up protection.
- Keep callback routing, browser/mobile adapters, JARM/form-post, code exchange,
  HTTP and product policy outside the crate.

## Capabilities

- `oid4vci-authorization-response`: add bounded query parsing, exact state and
  issuer correlation, and closed success/error outcomes.
- `oid4vci-authorization-server-metadata`: retain the RFC 9207 support flag.
- `oid4vci-error-contracts`: append static response-correlation diagnostics.
- `ssi-upstream-program`: hand IDR-023 to a focused code-exchange-request
  successor before this slice integrates.

## Impact

- Public additive OID4VCI types, accessors, limits, outcomes and errors.
- One additive optional projection in existing Authorization Server Metadata.
- Private OAuth grammar factoring and strict form-query parsing.
- No dependency, feature, lockfile, unsafe/native, target, stored-data,
  downstream, chain-specific or release change.

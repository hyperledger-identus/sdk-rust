# Construct a Bounded OID4VCI Authorization Request

## Why

Issue #352 now owns validated OAuth, redirect, state and PKCE inputs, but a
wallet still has to combine them with issuer-controlled endpoint query data and
credential intent. That combination is security-sensitive: duplicate managed
parameters, lossy form handling, ambiguous scope/detail intent, or an unbounded
URI would create a request that the typed predecessor does not actually prove.

## What changes

- Consume the issue #352 predecessor into one owned, transport-neutral
  Authorization Request description.
- Choose exactly one `authorization_details` entry using the already selected
  offered Credential Configuration; emit the issuer `locations` value exactly
  when issuer metadata explicitly advertises Authorization Servers.
- Strictly validate and retain any pre-existing form-encoded Authorization
  Endpoint query, rejecting malformed, duplicate or reserved names.
- Append mandatory OAuth, state, S256 PKCE, credential-intent and optional
  `issuer_state` parameters in a fixed order with the existing strict local
  form mechanics.
- Bound endpoint-query fields, Authorization Details JSON and the final request
  URI; keep diagnostics and Debug output data-free.

## What does not change

No entropy generation, caller extensions, scope/resource selection, PAR, JAR,
browser launch, callback listener, Authorization Response parsing/correlation,
redirect registration, code exchange, HTTP, DNS, TLS, trust, persistence,
product policy, format-profile interpretation or chain behavior is added.

## Capabilities

### New capabilities

- `oid4vci-authorization-request`: deterministic bounded construction of one
  authorization-details-based request URI from the typed predecessor.

### Modified capabilities

- `oid4vci-error-contracts`: append static diagnostics for request limits,
  endpoint query validation/collisions and output bounds.
- `ssi-upstream-program`: record #354 as the current IDR-023 slice and require
  a focused response-correlation successor before integration.

## Authority

Issue #354; OpenID4VCI 1.0 Final sections 5.1, 5.1.1 and 5.1.3; RFC 6749
sections 3.1 and 4.1.1 plus Appendix B; RFC 9396 sections 2 and 3; RFC 7636
section 4.3; RFC 9700 section 2.1.1; ADRs 0083, 0102 and 0141; and the standing
SDK delivery mandate.

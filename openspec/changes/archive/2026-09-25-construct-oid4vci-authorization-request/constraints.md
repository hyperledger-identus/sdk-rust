# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/354
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. The B09 roadmap and issue #354 already direct this exact
generic, bounded OID4VCI wire transition.

## Introduced or changed constraints

- The request consumes only the issue #352 predecessor and uses exactly one
  Authorization Details credential-intent mechanism.
- `locations` contains the exact Credential Issuer identifier only when issuer
  metadata explicitly contains `authorization_servers`; scope and resource are
  absent.
- Existing Authorization Endpoint query parameters are strictly decoded for
  validation, independently bounded and retained byte-for-byte only when names
  are non-empty, unique and outside the reserved set.
- Managed parameters use fixed ordering and strict local form encoding; the
  final URI and Authorization Details JSON have positive byte ceilings.
- The request URI is zeroizing and exposed only through an explicitly
  sensitive accessor; errors and Debug contain no values.
- New diagnostics append after every existing live error row.

## Introduced or changed limitations

No effective limitation is removed. The request description does not execute
PAR/HTTP/browser behavior, accept caller extensions, parse or correlate a
response, validate returned state/issuer/redirect, prevent mix-up by itself,
exchange a code, establish endpoint/client trust, or create authorization or
issuance completion evidence.

## Consumer and product impact

Additive unpublished API and deterministic wire bytes only. Consumers gain a
headless request URI while retaining transport and product policy. No stored
data, migration, product, chain, format, release or support promise changes.

## Activation and rollback

Activation requires issue #354's local review, canonical OpenSpec archive,
green hosted fast lane and merge to `develop`. Rollback removes the additive
request state, private serializer factoring, diagnostics, tests, ADR and
capability without changing the issue #352 predecessor.

## Evidence

Normative examples, conditional locations, exact issuer-state inclusion,
endpoint-query retention/collision/duplicate/malformed/boundary tests, fixed
ordering, strict percent encoding, output limits, redaction, error-contract,
factory, full Rust gates, signed+DCO and exact-head hosted CI evidence are
mandatory.

# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/348
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. Generic OID4VCI state transitions belong in sdk-rust;
bounded inputs, explicit sensitive ownership, Rust 1.98.1, issue-first delivery
and honest target claims do not change.

## Introduced or changed constraints

- Deferred payload-error validation reuses the existing exact status-400 JSON
  envelope and bounded Credential Error Response core.
- Exact `invalid_transaction_id` and `credential_request_denied` receive
  deferred-specific classifications; other codes retain the existing core and
  classification without reinterpretation.
- Stop-polling guidance is true only for exact `credential_request_denied` and
  performs no side effect.
- The wrapper and request-bound method add no new parser, limits, fieldful
  error, generic serialization, Clone, dependency, networking or authority.
- RFC 6750 authorization challenge handling remains outside this slice.

## Introduced or changed limitations

No effective limitation is removed. Validation does not prove transport or
issuer provenance, correlation, token validity, truth, retry safety,
transaction invalidation, persistence, encryption, credential validity,
localization, display safety or product support.

## Consumer and product impact

Additive unpublished API only. Consumers can delete deferred payload-error
classification glue after adoption. No downstream repository, wire shape,
stored data, migration, feature, dependency or product policy changes.

## Activation and rollback

Activation requires issue #348's PR to pass local and hosted gates and merge to
`develop`. Rollback removes the deferred-specific wrapper, request method,
tests, ADR and capability while preserving all existing successful-response
and generic Credential Error APIs.

## Evidence

Exact `invalid_transaction_id`, `credential_request_denied`, inherited known
and extension codes, status/media/body boundaries, stop-polling guidance,
repeatability, diagnostic redaction, compatibility, factory, Nix, signed+DCO
and exact-head hosted review/CI evidence is mandatory.

# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/345
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. Generic OID4VCI state transitions belong in sdk-rust;
bounded state, explicit sensitive ownership, Rust 1.98.1, issue-first delivery
and honest target claims do not change.

## Introduced or changed constraints

- A constructed request retains a private zeroizing copy of its originating
  transaction identifier for exact pending-response correlation.
- Only status `200` and `202` are successful outcomes for this unencrypted
  transition; other statuses fail before body parsing.
- Content-Type is independently bounded and must be `application/json` under
  the existing media-type semantics.
- Status `200` uses the existing bounded immediate response parser; status
  `202` uses the existing bounded deferred response parser and must return the
  exact request transaction identifier.
- Limits, status, media type and mismatch failures use static fieldless errors.
- No generic serialization, Clone, networking or ambient authority is added.

## Introduced or changed limitations

No effective limitation is removed. Validation does not prove transport
provenance, issuer control, TLS/redirect safety, token validity, response
freshness, interval compliance, retry/invalidation policy, error semantics,
encryption, credential validity/storage, proof cardinality or product support.

## Consumer and product impact

Additive unpublished API only. Consumers can delete their successful-response
status/media-type/correlation glue after adoption, but no downstream repository
is changed and no wire, storage, migration or product policy is activated.

## Activation and rollback

Activation requires issue #345's PR to pass local and hosted gates and merge to
`develop`. Rollback removes the response limits, outcome, request validator,
private retained identifier, errors, tests, ADR and capability while preserving
the existing request and response parsers.

## Evidence

Exact 200/202 examples, unsupported status, missing/oversized/invalid media
type, immediate/deferred body boundaries, exact and mismatched transaction
identifiers, repeated validation, diagnostics redaction, compatibility,
factory, Nix, signed+DCO and exact-head hosted review/CI evidence is mandatory.

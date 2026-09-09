# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/250
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007` remain
effective. Generic OID4VCI request construction belongs in sdk-rust; bounded
state, explicit sensitive access, Rust 1.98.1, issue-first delivery and honest
target claims do not change.

## Introduced or changed constraints

- Construction requires an existing bounded deferred response transaction and
  an advertised validated Deferred Credential Endpoint.
- The unencrypted body contains exactly one `transaction_id` member and is
  serialized as JSON under a positive caller-supplied complete-body ceiling.
- The request reports POST, `application/json`, and that an OAuth access token
  is required without retaining or constructing that token.
- Body storage is zeroizing and accessible only through an explicitly
  sensitive byte accessor; Debug and errors expose shape/length only.
- No generic serialization, Clone, networking or ambient authority is added.

## Introduced or changed limitations

No effective limitation is removed. Typed construction does not prove endpoint
provenance, issuer control, reachability, transport/TLS safety, access-token or
transaction validity, interval compliance, replay/invalidation, encryption,
response correlation or product support.

## Consumer and product impact

Additive unpublished API only. Consumers can delete hand-written unencrypted
request shaping after adoption, but no downstream repository is changed and no
wire, storage, migration or product policy is activated here.

## Activation and rollback

Activation requires issue #250's PR to pass local and hosted gates and merge to
`develop`. Rollback removes the request value, request limits, construction
transition, errors, tests, ADR and canonical capability; all existing response
and metadata behavior remains compatible.

## Evidence

Final example, exact escaping, endpoint omission, positive/exact/one-over body
limits, repeated construction, sensitive accessor, redaction, compatibility,
factory, Nix, signed+DCO and exact-head hosted review/CI evidence is mandatory.

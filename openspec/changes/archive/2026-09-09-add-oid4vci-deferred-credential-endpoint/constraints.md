# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/241
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007` remain
effective. Generic OID4VCI metadata belongs in sdk-rust; bounds, redaction,
Rust 1.98.1, issue-first delivery and honest target claims do not change.

## Introduced or changed constraints

- A present Deferred Credential Endpoint is a non-empty bounded HTTPS URL with
  a host and without userinfo or fragment.
- Port, path and query remain permitted and exact accepted text is retained.
- The existing endpoint byte budget applies independently to the Credential,
  Nonce and Deferred Credential Endpoint values.
- Omission remains valid and exposes no inferred fallback.
- New diagnostics are fieldless, static and redaction-safe.
- Existing metadata parsing and the public limits constructor remain compatible.

## Introduced or changed limitations

No effective limitation is removed. Endpoint syntax does not prove retrieval
provenance, issuer control, reachability, network safety, token validity,
transaction validity, polling correctness, trust or product support.

## Consumer and product impact

Additive unpublished API only. No existing caller, stored data, product policy,
wire producer or downstream repository changes.

## Activation and rollback

Activation requires issue #241's PR to pass local and hosted gates and merge to
`develop`. Rollback removes the optional field/type/accessor/errors/tests/spec;
metadata that includes the member returns to extension-discard behavior without
migrating existing callers.

## Evidence

Exact/omission/type/syntax/duplicate/bound/redaction/compatibility tests,
factory gates, full Nix, signed+DCO commits and exact-head hosted review/CI are
mandatory.

# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/362
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. This is chain-neutral bounded OID4VCI protocol behavior; the
Rust 1.98.1, zeroizing ownership, no-I/O, issue-first and honest-target rules
remain unchanged.

## Introduced or changed constraints

- Only a #360 request-bound success may enter correlation, and it is consumed
  exactly once on success or failure.
- Existing positive Token Authorization Details and retained JSON budgets are
  reused; no parallel resource-limit vocabulary is introduced.
- Every recognized `openid_credential` entry must reference the exact selected
  Credential Configuration. Any mismatch fails before duplicate ambiguity.
- Exactly one recognized matching entry is required; repeated matching entries
  fail instead of selecting by position.
- Unknown detail types remain bounded and counted but confer no credential
  authority.
- The correlated state retains exact issuer/server/configuration lineage,
  issuer-identification evidence, secret Token Response core and only the
  matching entry's unique source-ordered dataset identifiers.
- Debug, Display and public errors expose no identifier, token, JSON, endpoint,
  issuer/server/configuration or remote value.
- No dependency, feature, unsafe/native code, network or downstream behavior
  is introduced.

## Introduced or changed limitations

The result proves response-local syntax plus exact selected-configuration
correlation only. It does not prove token or dataset validity, server/issuer
trust, response origin, authorization beyond the supplied response, product
selection, proof possession, Credential Request readiness, storage or
successful issuance.

## Consumer and product impact

Additive unpublished Rust API only. Consumers gain a stronger alternative to
detached response-local validation. Existing public validation and Credential
Request helpers remain unchanged during this slice. No stored data, migration,
target, product, chain, credential-format or release promise changes.

## Activation and rollback

Activation requires issue #362's PR to pass local and hosted gates and merge
to `develop`. Rollback removes the additive correlated state, consuming method,
two diagnostics, tests, ADR and private decomposition without changing #360 or
the existing response-local public API.

## Evidence

Missing/malformed details, mismatched configuration, repeated matching entry,
unknown-type coexistence, exact dataset order, one-shot ownership, retained
lineage/evidence, redaction, stable error contract, portable compilation,
factory, Nix, signed+DCO and exact-head hosted CI evidence are mandatory.

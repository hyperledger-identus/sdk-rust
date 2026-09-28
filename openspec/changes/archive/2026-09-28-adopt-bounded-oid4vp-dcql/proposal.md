# Why

The signed OID4VP Request Object transition proves integrity and correlation but
leaves `dcql_query` as opaque bytes. A headless wallet still has to reimplement
Final-profile bounds, structural validation, claim-path evaluation, credential
selection, and credential-set combinations. ADR 0156 found that exact
`siros-dcql 0.3.0` can own the generic engine, but only behind a strict
Identus-owned facade.

# What changes

- Consume a `VerifiedRequestObject` into a bounded, signature-proven DCQL query
  state without claiming full Authorization Request or verifier trust.
- Validate the complete DCQL structure and Final identifier/reference rules
  before private-engine execution.
- Adopt exact `siros-dcql 0.3.0` privately for path, candidate, and
  credential-set evaluation.
- Expose only Identus-owned credential, path, outcome, error, and limit types.
- Repair the IDR-024 live roadmap owner from closed #396 to #429.

# Capabilities

## New capabilities

- `oid4vp-dcql-query`: bounded extraction, validation, and private-engine
  evaluation of a Final-profile `dcql_query`.

## Modified capabilities

- None. Signature validity remains distinct from request/DCQL validity.

# Non-goals

- No client-prefix authorization, verifier trust, audience/time/replay policy,
  scope expansion, response construction, consent, credential verification,
  JWE/JWS JSON, DC API, transport runtime, release, or downstream adoption.

# Delivery

Issue #429 and ADR 0165 control this unpublished slice. Planning and immutable
preflight evidence precede production code. The signed/DCO PR targets
`develop` and may merge only after exact-head hosted gates are green.

# Why

The bounded DID Resolution HTTP adapter now implements its fixed GET route,
content negotiation and query options, but hosts cannot obtain a matching
machine-readable contract. NeoPRISM carries an optional Utoipa document for
the older binding, so leaving documentation downstream would preserve avoidable
duplication and allow the runtime and published description to drift.

Issue #205 defines a small child of #10: expose deterministic OpenAPI 3.1
metadata for behavior already implemented by the adapter, without coupling
`identus-did` to HTTP documentation or enabling the dependency by default.

# What changes

- Add an opt-in `openapi` feature to `identus-did-resolver-http`.
- Adopt exact `utoipa 5.5.0` without default macro features and construct the
  document through its typed model.
- Describe the fixed `GET /{did}` operation, common query options, supported
  media types, status outcomes, bounds and `Vary: Accept` behavior.
- Keep the returned document composable by a host and deterministic under
  serialization.
- Record ADR 0092, exact dependency evidence, tests and rollback.

# Capabilities

## Modified capabilities

- `did-resolution-http`: add an optional OpenAPI 3.1 description for the
  existing fixed router contract.

# Non-goals

- No generated documentation UI, listener, runtime, middleware or server
  configuration.
- No OpenAPI dependency or annotations in `identus-did`.
- No dynamic externally mounted path, POST, DID URL dereferencing, publication
  or downstream adoption.
- No claim that a free-form method extension can be enumerated in OpenAPI.

# Delivery

Issue #205 owns this bounded child of #10. Research and constraints must pass
before implementation. Focused default/feature tests, dependency inspection,
exact-diff review and hosted CI are mandatory before merge to `develop`.

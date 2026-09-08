# Why

`identus-did` now owns the bounded, transport-neutral W3C DID Resolution
contract, but every HTTP service must still recreate routing, content
negotiation, status mapping and serialization. NeoPRISM has a useful Axum
adapter to extract, while its permissive `Accept` handling, ignored query
options and dynamic route construction do not meet this SDK's resource and
failure-boundary rules.

Issue #201 defines the smallest reusable transport slice: a state-closed Axum
router over any `DidResolver`. It provides standards-shaped HTTP behavior
without moving framework, runtime or network concerns into DID Core.

# What changes

- Add unpublished outer-boundary crate `identus-did-resolver-http` with one
  public router constructor and explicit media-type/resource constants.
- Bind a fixed `GET /{did}` route to an injected `Arc<dyn DidResolver>`; the
  consumer chooses and nests the service prefix.
- Negotiate `application/did-resolution`, `application/did` and
  `application/json` using exact `headers-accept 0.3.0` privately, preceded by
  SDK-owned byte/range and strict `q` preflight.
- Map invalid inputs, W3C resolution failures and deactivation to the pinned
  DID Resolution HTTP Binding statuses and bounded JSON envelopes.
- Add exact `axum 0.8.9` with defaults disabled and only `json` for the
  production adapter. Server execution remains consumer-owned.
- Record ADR 0090, dependency and provenance evidence, donor divergences,
  deterministic conformance tests and rollback.

# Capabilities

## New capabilities

- `did-resolution-http`: reusable bounded Axum GET binding over the generic
  DID resolver port.

# Non-goals

- No DID method, ledger, HTTP client, server listener, TLS, authentication,
  authorization, rate limiting, tracing or deployment policy.
- No DID URL dereferencing endpoint, registration endpoint, OpenAPI surface or
  resolver implementation.
- No resolution query-option decoding in this slice; non-empty query strings
  fail closed instead of being silently ignored.
- No public Axum extractor, `headers-accept`, `mediatype` or donor type.
- No portable-target, publication, release, certification or downstream
  adoption claim.

# Delivery

Issue #201 owns this bounded child of #10. Specification, research,
constraints and ADR land in a signed/DCO commit before Cargo or Rust changes.
Focused tests, full local gates and a distinct exact-diff review are required
before an issue-linked PR. The PR may merge into `develop` only after every
required hosted gate is green.

# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/391
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` preserve cohesive protocol crates and keep
dependency types private. `SDK-COMPAT-001` through `SDK-COMPAT-005` require the
exact compiler and explicit target evidence. `SDK-SEC-001` through
`SDK-SEC-003` require no authored unsafe, bounded untrusted inputs, and
redacted errors. `SDK-DELIVERY-001` requires issue-first reviewed delivery.
`SDK-LIM-001`, `SDK-LIM-003`, `SDK-LIM-005`, `SDK-LIM-006`, `SDK-LIM-007`, and
`SDK-LIM-009` keep unpublished features, resource bounds, and target claims
explicit.

## Introduced or changed constraints

- `siros-dcql` is pinned to exact `0.3.0` in a separately locked research
  fixture only.
- Candidate strings, vectors, JSON values, errors, and result types cannot
  cross a future Identus public boundary.
- The fixture must reject over-limit query bytes before candidate parsing and
  map candidate diagnostics to a bounded, redacted local error.
- No candidate enters the root workspace, root lock, release graph, or fast CI.
- Any production adoption requires a separate issue, OpenSpec, ADR amendment,
  consumer-shaped target evidence, and Identus-owned facade.

## Introduced or changed limitations

- Candidate behavior is non-normative evidence; OpenID4VP 1.0 Final remains
  authoritative.
- The fixture does not prove presentation construction, cryptographic
  verification, transport, trust, consent UX, deployed interoperability,
  runtime support, certification, or production security.
- Host compilation does not imply WASM, iOS, or Android support.

## Consumer and product impact

None. Existing OID4VCI and presentation APIs remain unchanged. The outcome can
shape a future generic OID4VP capability only through a separate delivery.

## Activation and rollback

This research activates only as repository evidence when its reviewed PR
merges. Rollback removes the fixture, checker, ADR, and research records; no
runtime, public API, persisted data, release, or downstream migration exists.

## Evidence

The assessment must record immutable provenance, protocol version, license,
MSRV, features, direct/resolved cone, unsafe/native reach, target results,
resource and diagnostic behavior, maintenance, compatibility, exact commands,
unrun checks, and objective reconsideration triggers.

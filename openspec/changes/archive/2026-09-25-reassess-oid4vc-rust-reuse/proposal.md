# Why

The SDK already owns bounded OID4VCI 1.0 Final behavior and has a quarantined
OpenID4VC placeholder, while several Rust projects now claim OID4VCI,
OID4VP, or DCQL coverage. Before IDR-024 starts an OID4VP implementation, the
repository needs current evidence showing which code should remain local,
which implementations are useful only as conformance oracles, and whether a
narrow engine can remove risky selection logic without importing a framework.

# What changes

- Reassess current Rust OID4VCI, OID4VP, OAuth, and DCQL candidates at exact
  releases or revisions.
- Exercise exact `siros-dcql 0.3.0` through a separately locked, unpublished
  compatibility fixture with bounded caller-owned inputs and errors.
- Record protocol currency, license, MSRV, dependency cone, targets,
  unsafe/native reach, resource behavior, maintenance, compatibility, and
  rollback evidence.
- Decide whether to retain `identus-oid4vci`, adopt any full OID4VC framework,
  or conditionally reuse a private DCQL engine behind future Identus types.
- Keep all production manifests, public APIs, wire behavior, and fast CI
  unchanged.

# Capabilities

## New capabilities

- `oid4vc-rust-reuse-assessment`: reproducible evidence and decision rules for
  Rust OID4VC reuse before the OID4VP roadmap is activated.

# Non-goals

- No production dependency, OID4VP API, transport, wallet flow, trust policy,
  persistence, release claim, or downstream repository mutation.
- No retirement or replacement of `identus-oid4vci`.
- No claim that compile evidence proves browser or mobile runtime support.

# Delivery

Issue #391 owns this research. The OpenSpec and constraints must pass readiness
before the executable fixture is added. A distinct exact-diff review, signed
and DCO-compliant PR to `develop`, and green hosted gates are required.

# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/235
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` keep candidate/framework types out of generic
crates and public APIs. `SDK-SEC-001` through `SDK-SEC-003` require first-party
safe code, redacted diagnostics and bounded inputs. `SDK-COMPAT-001` through
`SDK-COMPAT-005` require exact Rust 1.98.1 and honest target evidence.
`SDK-DELIVERY-001`, `SDK-LIM-001`, `SDK-LIM-005`, `SDK-LIM-006` and
`SDK-LIM-007` preserve issue-first delivery, unpublished state and incomplete
target/resource claims.

## Introduced or changed constraints

- Exact `identity_did 1.5.1` is permitted only in a nested conformance fixture.
- W3C DID Core 1.0 and RFC 3986 remain normative; IOTA Identity is an oracle.
- Case identifiers and result classes are retained, but candidate diagnostics
  and caller content are not logged.
- The root manifest, root lock, release graph, public API and wire behavior do
  not change.

## Introduced or changed limitations

No effective limitation changes. The oracle does not establish DID method,
document, resolution, IOTA network, mobile/browser runtime or production
framework support. Compile results remain evidence only.

## Consumer and product impact

No consumer or product behavior changes. Oxid, Midnight, NeoPRISM and other
donor/downstream repositories remain unchanged.

## Activation and rollback

The evidence activates only after issue #235's PR passes local and hosted gates
and merges to `develop`. Rollback removes research artifacts; no API, wire,
data or downstream migration exists.

## Evidence

Issue #235 and ADR 0069 authorize the oracle. Final evidence must record
normative sources, release/tag/checksum/license provenance, exact version and
features, MSRV, direct/resolved dependency cone, unsafe/native and supply-chain
posture, target results, compatibility, maintenance, limitations, commands and
unrun checks.

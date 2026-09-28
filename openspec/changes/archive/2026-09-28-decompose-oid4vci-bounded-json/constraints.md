# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/403
Constraint blockers: none

## Existing entries affected

`SDK-SEC-001` continues to prohibit unsafe first-party code. `SDK-SEC-003`
continues to require bounded untrusted inputs, and `SDK-LIM-007` continues to
describe allocation before typed SDK entry. ADR 0115 requires a semantic
responsibility ratchet. The Rust/MSRV, dependency, target, publication, and
protocol-support entries remain unchanged.

## Introduced or changed constraints

No effective constraint changes. The private module map must preserve one
scanner/root ingress owner and must not move limit or error authority away
from the protocol object that owns it.

## Introduced or changed limitations

None. This slice neither expands nor narrows supported OID4VCI messages. It
does not claim external transport allocation, generic JSON parsing, or OID4VP
reuse, and it does not resolve the separate `limits.rs` attention signal.

## Consumer and product impact

Consumers observe no API, wire, error, allocation, dependency, feature, MSRV,
target, performance-budget, or certification change. Maintainers receive
smaller protocol-cohesive review units and one explicit parser-invariant owner.

## Activation and rollback

Activation requires issue #403's signed/DCO PR, pre/post characterization
tests, exact code-health evidence, distinct architecture/security review, and
green protected CI. Rollback recombines private modules at the same facade;
no consumer migration or data transition is required.

## Evidence

The issue, Discussion #399, OpenSpec design/specification, pre-move and
post-move OID4VCI suites, immutable error golden, code-health report, strict
Rust/factory/Nix gates, exact-diff review, and hosted exact-head receipt are
required evidence.

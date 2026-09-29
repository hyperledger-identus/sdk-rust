## 1. Contract and readiness

- [x] 1.1 Pin issue #447, Final/errata sources, existing typestate, and candidate decisions.
- [x] 1.2 Define composed state, supported routing, limits, error order, trust boundary, and non-scope.
- [x] 1.3 Record ADR 0168 and pre-implementation semantic/security review.
- [x] 1.4 Pass research/constraint readiness and retain the immutable preflight receipt.

## 2. Implementation

- [x] 2.1 Add bounded routing limits, safe enums, static error contracts, and public exports.
- [x] 2.2 Add the composed request transition with one shared request map and preserved JAR evidence.
- [x] 2.3 Refactor the private DCQL constructor without changing the public DCQL-only transition.
- [x] 2.4 Add clean-room positive, negative, precedence, resource, redaction, and compatibility tests.
- [x] 2.5 Update crate, architecture, boundary inventory, and IDR-024 documentation.

## 3. Review and delivery

- [x] 3.1 Run focused, workspace, factory, dependency, MSRV, and portable compile gates.
- [x] 3.2 Complete and record one distinct exact-diff architecture/security review.
- [x] 3.3 Complete verification, readiness, archive, and immutable receipt.
- [x] 3.4 Prepare the signed/DCO issue-linked PR and publish factory metrics.

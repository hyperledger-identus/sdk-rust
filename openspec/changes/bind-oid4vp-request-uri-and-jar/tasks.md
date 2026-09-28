## 1. Contract and readiness

- [x] 1.1 Create issue #396 and pin Final/RFC/security evidence.
- [x] 1.2 Define retrieval, JAR, limits, states, trust boundary, and non-scope.
- [x] 1.3 Record material authority and ADR 0158.
- [x] 1.4 Pass research/constraint readiness and write the immutable preflight receipt.

## 2. Implementation

- [x] 2.1 Accept explicit `request_uri_method=get` and update ingress vectors.
- [x] 2.2 Implement bounded GET/POST request construction and response binding.
- [x] 2.3 Implement signed compact JAR parse/verify/correlation states via `identus-jose`.
- [x] 2.4 Add stable redacted errors and positive, negative, resource, and misuse tests.
- [x] 2.5 Update crate, architecture, inventory, and follow-up documentation.

## 3. Review and delivery

- [x] 3.1 Run focused, workspace, factory, dependency, MSRV, and portable compile gates.
- [x] 3.2 Complete and record one distinct exact-diff architecture/security review.
- [ ] 3.3 Complete verification, readiness, archive, and immutable receipt.
- [ ] 3.4 Prepare the signed/DCO issue-linked PR and publish factory metrics.

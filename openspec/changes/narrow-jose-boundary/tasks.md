# Tasks

## 1. Planning

- [x] 1.1 Create issue #398 and record the exact boundary/problem statement.
- [x] 1.2 Assess current JOSE crates and record version, MSRV, cone, semantic,
      coupling, target, and reconsideration evidence.
- [x] 1.3 Record OpenSpec, constraints, design, and superseding ADR decision.
- [x] 1.4 Pass research/constraint/strict readiness and commit planning only.

## 2. Implementation

- [x] 2.1 Move OID4VCI proof construction and verifier modules/tests into
      `identus-oid4vci` without semantic growth.
- [x] 2.2 Move profile-specific errors/contracts to the OID4VCI capability and
      preserve static redacted bridges.
- [x] 2.3 Remove the JOSE DID dependency, add the OID4VCI DID dependency, and
      add a regression guard for the intended dependency direction.
- [x] 2.4 Update crate documentation, architecture documentation, and the
      dependency not-adopted ledger.

## 3. Verification and delivery

- [x] 3.1 Run moved profile/error tests plus workspace fmt, Clippy, docs, and
      tests.
- [ ] 3.2 Run factory dependency, code-health, primary/MSRV, and portable target
      evidence; perform a clean diff review.
- [ ] 3.3 Open a signed/DCO issue-linked PR to `develop`, publish local factory
      metrics, and merge only after required CI is green.

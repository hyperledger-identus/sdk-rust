# Tasks

## 1. Planning gate

- [x] 1.1 Record proposal, research, constraints, design, capability delta,
      risks, and rollback for issue #305.
- [x] 1.2 Pass research readiness, constraint readiness, strict OpenSpec, and
      write the immutable planning receipt.

## 2. Implementation

- [x] 2.1 Pin the complete ordered slow-blocker set exactly.
- [x] 2.2 Pin the two decomposition thresholds and advisory action exactly.
- [x] 2.3 Add mutation tests for omission, reorder, insertion, and both
      directions of threshold drift.

## 3. Verification and delivery

- [x] 3.1 Run focused Node tests and `./bootstrap.sh --check`.
- [ ] 3.2 Complete a fresh local review and exact-diff target plan.
- [ ] 3.3 Archive the guarded change, push signed commits, open an issue-linked
      PR to `develop`, and require green exact-head integration evidence.
- [ ] 3.4 Retain and publish terminal metrics after merge.

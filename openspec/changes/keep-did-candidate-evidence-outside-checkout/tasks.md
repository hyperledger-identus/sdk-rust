# Tasks

## 1. Contract

- [x] 1.1 Record #480 failure evidence, alternatives, constraints, design,
      rollback, and verification boundary.
- [x] 1.2 Pass research, constraint, and strict OpenSpec readiness; commit the
      planning-only contract and persist immutable preimplementation evidence.

## 2. Workflow repair

- [ ] 2.1 Move primary and MSRV lane outputs to the runner temporary root.
- [ ] 2.2 Assert checkout cleanliness between lanes and retain the existing
      artifact name, content, exact-SHA identity, and seven-day retention.

## 3. Regression policy

- [ ] 3.1 Require external output/upload paths and the clean-source boundary in
      the offline release-candidate checker.
- [ ] 3.2 Add mutations that reject checkout-local output and a missing
      cleanliness assertion.

## 4. Verification and delivery

- [ ] 4.1 Run focused tests, actionlint, Nix, factory, and OpenSpec gates.
- [ ] 4.2 Complete local architecture/security/workflow review, exact-head CI,
      discovery review, merge, metrics, #388 update, and worktree closeout.

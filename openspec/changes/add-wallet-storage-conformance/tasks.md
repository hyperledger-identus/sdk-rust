## 1. Contract

- [x] 1.1 Create issue #91 under #20 / `IDR-010` at the exact develop base.
- [x] 1.2 Decide verification ownership, direct port coverage, fixture bounds,
  revision invalidation, pagination progress, redaction and downstream receipts.
- [x] 1.3 Record ADR 0033 and complete pre-implementation review.

## 2. Implementation

- [ ] 2.1 Add and classify the `identus-wallet-conformance` workspace crate.
- [ ] 2.2 Implement static failure/report vocabulary and exact lifecycle checks.
- [ ] 2.3 Implement bounded list checks and five port-specific public entry points.
- [ ] 2.4 Add a test-only memory implementation for all entry points and failures.

## 3. Evidence

- [ ] 3.1 Add and run the ignored release diagnostic.
- [ ] 3.2 Update human/machine inventory and the IDR-010 issue linkage.
- [ ] 3.3 Pass focused, workspace, factory and pinned Nix validation.
- [ ] 3.4 Complete post-implementation review and archive this change.

## 4. Delivery

- [ ] 4.1 Create a signed/DCO PR linked to #91 against `develop`.
- [ ] 4.2 Resolve hosted review, pass every CI gate and merge when green.
- [ ] 4.3 Record effort/evidence, update #20, sync develop and remove the branch
  and worktree; leave `IDR-010` specified pending downstream adapter receipts.

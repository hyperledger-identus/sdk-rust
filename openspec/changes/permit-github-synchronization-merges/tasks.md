# Tasks

## 1. Planning

- [x] 1.1 Record issue #342, ADR 0136, research, constraints, specification and
      design.
- [x] 1.2 Pass research readiness, constraint readiness and strict validation;
      commit the planning-only contract and write its exact preflight receipt.

## 2. Hosted policy

- [x] 2.1 Extend workflow commit records and event context with parent,
      committer and base/head evidence.
- [x] 2.2 Add the fail-closed synchronization classifier and exempt only its
      authored subject and DCO checks while retaining signature validation.
- [x] 2.3 Keep the local range path fail-closed and ordinary-commit behavior
      unchanged.

## 3. Tests and documentation

- [x] 3.1 Add the PR #340 regression and negative structural boundaries to the
      contribution-policy tests.
- [x] 3.2 Update contribution/DCO documentation and the canonical factory
      specification.

## 4. Evidence and delivery

- [x] 4.1 Run focused Node, factory, OpenSpec, actionlint and applicable Nix
      checks; report every unrun gate.
- [ ] 4.2 Complete a distinct local review and resolve blocking findings.
- [ ] 4.3 Complete factory readiness and receipt, then archive the change.

## Delivery boundary

Push, pull-request creation and merge follow the standing issue-linked
`develop` authority only after the local candidate is complete and reviewed.
The release SHA remains unfrozen until this change is merged.

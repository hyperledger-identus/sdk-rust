## 1. Research and specification

- [x] 1.1 Reproduce and inspect the zero-exit/no-op archive behavior and current wrapper contract.
- [x] 1.2 Compare exit-status, output-parsing and filesystem-postcondition strategies.
- [x] 1.3 Record issue #194, research, constraints, design and canonical behavior before implementation.

## 2. Fail-closed implementation

- [ ] 2.1 Reject the deterministic dated-archive destination collision before invoking OpenSpec.
- [ ] 2.2 Verify active removal, archive creation and mandatory artifact preservation before reporting success.
- [ ] 2.3 Preserve the existing readiness, loss-prevention and semantic post-archive gates.

## 3. Verification and delivery

- [ ] 3.1 Add hermetic no-op, collision and successful-transition contract tests.
- [ ] 3.2 Run focused shell/factory checks and complete local Nix validation.
- [ ] 3.3 Perform a distinct exact-diff review, record immutable evidence and archive the change safely.

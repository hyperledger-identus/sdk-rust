## 1. Research and contract

- [x] 1.1 Record issue #218 evidence, alternatives, constraints and the exact archive receipt behavior.
- [x] 1.2 Pass research, constraint and strict OpenSpec readiness before implementation.

## 2. Timezone-safe receipt

- [x] 2.1 Snapshot matching archive entries and require exactly one newly created entry after OpenSpec returns.
- [x] 2.2 Reject non-directory, symlink, absent and ambiguous results while preserving existing validation gates.
- [x] 2.3 Update the factory operator documentation to describe the observed-state receipt.

## 3. Verification and delivery

- [x] 3.1 Add deterministic mismatched-date regression coverage and retain collision, no-op, incomplete and canonical protection tests.
- [x] 3.2 Run focused shell/factory tests and complete local Nix validation.
- [x] 3.3 Perform a distinct exact-diff review, record evidence and archive the completed change safely.

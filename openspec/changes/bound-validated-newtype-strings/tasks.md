## 1. Research and specification

- [x] 1.1 Create issue #207 under #168 with compatibility, security and acceptance scope.
- [x] 1.2 Inventory every validated `String` newtype and validator signature at the base revision.
- [x] 1.3 Evaluate allocation-order, macro-contract, method-ceiling and dependency alternatives.
- [x] 1.4 Add ADR 0093 and research-ready, constraint-ready OpenSpec artifacts before Rust implementation.

## 2. Implementation

- [ ] 2.1 Validate borrowed string input before allocation in generated `parse` and `FromStr` paths.
- [ ] 2.2 Expose and enforce the compatible DID method byte ceiling before ASCII grammar traversal.
- [ ] 2.3 Replace caller-bearing DID method validation detail with static diagnostics.
- [ ] 2.4 Narrow the machine-readable and narrative `SDK-LIM-007` evidence without removing the outer-allocation limitation.

## 3. Verification and delivery

- [ ] 3.1 Add macro ordering, exact/one-over, precedence, constructor, redaction, DID and Registration compatibility tests.
- [ ] 3.2 Pass focused, workspace, factory and available Rust 1.98/Nix gates.
- [ ] 3.3 Perform a distinct exact-diff security, compatibility and architecture review.
- [ ] 3.4 Synchronize canonical specs, archive safely, sign/DCO commits, open the issue-linked PR and merge only after every hosted gate is green.

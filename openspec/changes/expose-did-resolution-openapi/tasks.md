## 1. Research and specification

- [x] 1.1 Create issue #205 under #10 with scope, compatibility and acceptance.
- [x] 1.2 Inspect current SDK/NeoPRISM behavior and pin OpenAPI, DID Resolution
      and Utoipa revisions.
- [x] 1.3 Evaluate Utoipa typed-model, macro and handwritten-document choices,
      including MSRV, features, dependency cone, license and unsafe/native code.
- [x] 1.4 Add ADR 0092 and this research-ready, constraint-ready contract before
      Rust implementation.

## 2. Implementation

- [ ] 2.1 Add exact optional Utoipa workspace/member dependency with only the
      upstream-required macro compile feature and no macro use.
- [ ] 2.2 Build a fixed mount-relative OpenAPI document for implemented GET
      parameters, representations, statuses, bounds and response headers.
- [ ] 2.3 Keep `identus-did`, default features and router/wire behavior unchanged.

## 3. Verification and delivery

- [ ] 3.1 Add deterministic structural tests and default/feature graph checks.
- [ ] 3.2 Pass focused, workspace, factory and available Rust 1.98 gates; record
      unavailable Nix/security wrapper gates exactly.
- [ ] 3.3 Perform distinct exact-diff protocol/architecture/dependency review.
- [ ] 3.4 Synchronize the canonical spec, archive safely, sign/DCO commits, open
      the issue-linked PR and merge only after every hosted gate is green.

## 1. Research and specification

- [x] 1.1 Pin issue #189 with Rust 1.98.1 behavior, first-party inventory, options and stop conditions.
- [x] 1.2 Reproduce expansion-span behavior and reject blanket caller-spanning after its 21-error compatibility regression.
- [x] 1.3 Specify structured direct-output validation, residual limitation, activation and rollback before implementation.
- [x] 1.4 Correct ADR 0088 to select fail-closed syntax validation.

## 2. Enforcement

- [ ] 2.1 Parse and recursively reject unsafe constructs/attributes in completed direct `Newtype` output.
- [ ] 2.2 Add unit evidence for every prohibited construct, nesting and safe output.
- [ ] 2.3 Narrow `SDK-LIM-008` and strengthen the canonical unsafe-policy specification after focused evidence passes.

## 3. Verification and delivery

- [ ] 3.1 Pass focused derive/conformance suites, formatting, strict Clippy, feature/target gates and complete compatible Nix checks.
- [ ] 3.2 Perform and record a distinct exact-diff security/architecture review, including residual limitations and intentionally unrun surfaces.
- [ ] 3.3 Synchronize canonical specs, archive the change, create an immutable receipt and deliver a signed/DCO issue-linked PR to `develop` through hosted gates.

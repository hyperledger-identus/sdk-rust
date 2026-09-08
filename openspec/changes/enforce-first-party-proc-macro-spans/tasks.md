## 1. Research and specification

- [x] 1.1 Pin issue #189 with the Rust 1.98.1 behavior, first-party inventory, options, selected direction and stop conditions.
- [x] 1.2 Reproduce default, call-site, mixed-site, keyword-only and caller-spanned expansion behavior on exact Rust 1.98.1.
- [x] 1.3 Specify scope, residual limitation, compatibility, activation and rollback before implementation.
- [x] 1.4 Add ADR 0088 for caller-spanned direct first-party macro output.

## 2. Enforcement

- [ ] 2.1 Convert all direct `Newtype` expansion templates to caller-spanned generation without changing emitted semantics.
- [ ] 2.2 Add an isolated stable compile-fail fixture for caller-spanned unsafe expansion output.
- [ ] 2.3 Narrow `SDK-LIM-008` and strengthen the canonical unsafe-policy specification after focused evidence passes.

## 3. Verification and delivery

- [ ] 3.1 Pass focused derive and conformance suites, formatting, strict Clippy, feature/target gates and complete compatible Nix checks.
- [ ] 3.2 Perform and record a distinct exact-diff security/architecture review, including residual limitations and intentionally unrun surfaces.
- [ ] 3.3 Synchronize canonical specs, archive the change, create an immutable receipt and deliver a signed/DCO issue-linked PR to `develop` through hosted gates.

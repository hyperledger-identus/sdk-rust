## 1. Contract and dependency admission

- [x] 1.1 Record issue #226, base SHA, exact UniFFI source/license/cones and the host-only support boundary.
- [x] 1.2 Specify ABI values, stable errors/versioning, bounds, panic/ownership/memory/threading behavior and rollback.
- [x] 1.3 Pass FFI research and constraint readiness before implementation.

## 2. Production foundation

- [ ] 2.1 Add the isolated `identus-uniffi-did` runtime crate and root dependency/inventory records without changing domain crates or the placeholder.
- [ ] 2.2 Add stable version/value/error exports plus Rust success, bound, redaction and panic-containment tests.
- [ ] 2.3 Add the separately locked exact-version bindgen tool, deterministic generation and reviewed Swift/Kotlin API snapshots.
- [ ] 2.4 Compile and execute Swift and Kotlin/JVM host smoke tests against the built library.

## 3. Review and delivery

- [ ] 3.1 Record the production host-foundation ADR and update architecture/support/reuse evidence while retaining `SDK-LIM-002`.
- [ ] 3.2 Run focused dependency/security checks, full factory/Cargo/Nix gates and distinct exact-diff architecture/security review.
- [ ] 3.3 Sync canonical specs, complete the receipt, archive the change, open the signed/DCO issue-linked PR and integrate only after green required CI.

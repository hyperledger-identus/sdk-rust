## 1. Research and specification

- [x] 1.1 Refresh issue #169 with the current 17-package inventory, exact Rust 1.98.1 behavior, options, exception contract and stop conditions.
- [x] 1.2 Record authoritative Cargo/rustc behavior and empirically probe library, binary, test, example, bench, build-script and proc-macro targets.
- [x] 1.3 Specify first-party scope, drift checks, negative evidence, limitations, material authority, activation and rollback before implementation.
- [x] 1.4 Add ADR 0087 with the default-forbid decision and separate future exception path.

## 2. Enforcement

- [x] 2.1 Add exact `unsafe_code = "forbid"` to the inherited workspace Rust lint table without removing crate-local defense in depth.
- [x] 2.2 Add an isolated conformance guard that derives every workspace member and rejects a missing/changed root forbid or missing member lint inheritance.
- [x] 2.3 Add dependency-free offline compile-fail probes for every supported authored first-party Cargo target class, including proc-macro implementations.
- [x] 2.4 Strengthen `SDK-SEC-001` enforcement and narrow `SDK-LIM-008` atomically after reproducing the proc-macro expansion exception under #189.

## 3. Verification and delivery

- [ ] 3.1 Pass focused positive/negative guard tests, format, strict Clippy, all feature/target/compiler gates and complete compatible Nix checks.
- [ ] 3.2 Perform a distinct exact-diff security/architecture review and record intentionally unrun surfaces plus residual limitations.
- [ ] 3.3 Prepare canonical synchronization, safe archive and immutable receipt for a signed/DCO issue-linked PR to `develop`; hosted CI, merge and issue/program receipts remain GitHub evidence.

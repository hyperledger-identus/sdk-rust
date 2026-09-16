# Tasks

## 1. Contract and pre-implementation gate

- [x] 1.1 Inspect v1 scanner, tests, ADR 0115, OpenSpec, fast/weekly workflows,
      workspace parser versions, and conformance dependency boundary.
- [x] 1.2 Decide bounded classifier protocol, AST population semantics, module
      reachability, conservative fallback, and rollback in OpenSpec/ADR 0126.
- [ ] 1.3 Pass research, constraints, strict OpenSpec, and immutable preflight
      receipt before production implementation.

## 2. Rust classifier

- [ ] 2.1 Add the non-published conformance binary and bounded JSON protocol.
- [ ] 2.2 Implement cfg/cfg_attr evaluation, AST span collection, production-
      conservative line projection, and actionable parse/span diagnostics.
- [ ] 2.3 Implement ordinary/raw/path-attributed module resolution and
      production-wins fixed-point reachability.
- [ ] 2.4 Add focused and adversarial Rust fixtures for all acceptance syntax.

## 3. Orchestration and migration

- [ ] 3.1 Remove Rust boundary parsing from Python and invoke the classifier for
      working-tree, Git-tree, fast-check, and baseline-regeneration inputs.
- [ ] 3.2 Upgrade the closed policy/report schema and mutation tests with exact
      classifier identity and protocol fields.
- [ ] 3.3 Regenerate the baseline from the implementation commit and record an
      exhaustive v1-to-v2 population delta.
- [ ] 3.4 Update pinned Nix/workflow/factory execution and human documentation.

## 4. Verification and delivery

- [ ] 4.1 Run focused tests, fmt, strict Clippy, workspace tests, factory,
      OpenSpec, fast-equivalent checks, and the compatible Nix closure.
- [ ] 4.2 Complete architecture/security/API review and archive the change.
- [ ] 4.3 Open an issue-linked PR, resolve exact-head review findings, and merge
      to protected `develop` only after every required check is green.

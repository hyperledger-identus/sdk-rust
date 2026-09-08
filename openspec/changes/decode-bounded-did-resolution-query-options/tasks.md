## 1. Research and specification

- [x] 1.1 Create issue #203 under #10 with scope, bounds, compatibility, tests,
      rollback and stop conditions.
- [x] 1.2 Refresh the W3C GET/query/security baseline and inspect the SDK and
      NeoPRISM implementations.
- [x] 1.3 Evaluate strict local reuse and crate candidates; preserve ADR 0083's
      negative form-codec decision.
- [x] 1.4 Add ADR 0091 and this research-ready, constraint-ready OpenSpec change
      before Rust implementation.

## 2. Implementation

- [ ] 2.1 Add public raw-query/member/name/value ceiling constants and a private
      strict single-pass decoder without changing dependencies.
- [ ] 2.2 Project common typed fields and string-valued extensions, reject
      header-only/conflicting options, and merge negotiated representation.
- [ ] 2.3 Preserve the current redacted response/error boundary and prevent
      resolver invocation on invalid input.

## 3. Verification and delivery

- [ ] 3.1 Add deterministic boundary, malformed-input, projection, no-call and
      redaction tests while preserving existing HTTP tests.
- [ ] 3.2 Pass focused, workspace, factory, Rust 1.98, dependency/security and
      applicable Nix gates; record exact evidence.
- [ ] 3.3 Perform a distinct exact-diff architecture/security/protocol review
      with no unresolved blocker.
- [ ] 3.4 Synchronize canonical spec/ADR, archive safely, sign/DCO commits,
      create the issue-linked PR, and merge only after hosted gates are green.

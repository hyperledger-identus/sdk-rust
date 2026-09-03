## 1. Contract and provenance

- [x] 1.1 Reconcile issue #35 with exact base, standards/tool revisions,
      downstream receipts, compatibility, threat/resource policy, budgets,
      non-scope, rollback and effort estimate
- [x] 1.2 Complete and strictly validate the OpenSpec delta, ADR 0018, and
      pre-implementation semantic/security/API review before production edits

## 2. Independent fuzz harness

- [x] 2.1 Add standalone pinned cargo-fuzz workspace and separate `Did` and
      `DidUrl` public-invariant targets
- [x] 2.2 Add original bounded corpora, grammar dictionaries, artifact ignores,
      and crash/minimization guidance

## 3. Reproducible automation

- [x] 3.1 Add cargo-fuzz to the locked Nix shell and one validated wrapper for
      replay, deterministic smoke, and bounded soak modes
- [x] 3.2 Add path-scoped Ubuntu PR/push smoke plus scheduled/manual soak and
      failure-only artifact retention
- [x] 3.3 Run corpus replay/fixed smoke, record execution rate/wall time, and
      triage every sanitizer or invariant finding

## 4. Verification and delivery

- [x] 4.1 Run focused/workspace all/no-feature, strict Clippy/rustdoc/fmt,
      factory, MSRV, WASM/mobile, architecture, supply-chain and full Nix gates
- [x] 4.2 Complete a distinct exact-head semantic/security/API review and prove
      downstream HEAD/status receipts are unchanged
- [x] 4.3 Produce ready/receipt, synchronize the canonical DID Core spec,
      archive the change, and deliver the signed issue-linked all-green PR

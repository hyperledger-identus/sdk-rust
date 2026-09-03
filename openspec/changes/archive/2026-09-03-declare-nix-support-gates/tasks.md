## 1. Contract and baseline

- [x] 1.1 Record issue #24, exact base, predecessor architecture, #23 timing
      baseline, scope/non-scope and rollback
- [x] 1.2 Complete and strictly validate the OpenSpec delta, ADR 0020 and
      pre-implementation semantic/architecture review

## 2. Declarative execution contract

- [x] 2.1 Add the versioned fail-closed gate manifest with every current Rust
      gate and structured operation, toolchain, package, feature and target data
- [x] 2.2 Generate reachable Crane checks from the manifest and remove the
      duplicated hand-written Rust gate definitions

## 3. Validation and adversarial evidence

- [x] 3.1 Refactor the offline support-policy validator to consume the manifest
      without parsing Nix Cargo-selection semantics
- [x] 3.2 Translate and extend #23 regressions for malformed/duplicate data,
      dynamic/interpolated Nix, quote/comment edges and dead-code decoys
- [x] 3.3 Add the 20-sample benchmark and record macOS/Linux cold/warm p50/p95
      comparison evidence without creating a compatibility promise

## 4. Verification and delivery

- [x] 4.1 Run focused validator/tests, factory, formatting/linting and full Nix
      gates; record every applicable and unrun command exactly
- [x] 4.2 Complete a distinct exact-head architecture/security review and prove
      consumer repositories and reserved `main` remain unchanged
- [x] 4.3 Produce ready/receipt, synchronize canonical specs, archive the change
      and deliver the signed issue-linked all-green PR to `develop`

## 1. Contract and provenance

- [x] 1.1 Create issue #58 with exact base, standards/tool pins, donor and
      consumer receipts, compatibility, threats/resources, non-scope,
      rollback, stopping point, and effort estimate
- [x] 1.2 Complete and strictly validate the OpenSpec delta, ADR 0019, and
      pre-implementation semantic/security/API review before production edits

## 2. Public-key fuzz harness

- [x] 2.1 Generalize the standalone fuzz package without changing the DID
      target contract; add separate JWK and COSE public-invariant targets
- [x] 2.2 Add original standards-shaped positive/negative corpora, binary seed
      transport, dictionaries, and crypto-specific triage guidance

## 3. Reproducible automation

- [x] 3.1 Add one validated crypto wrapper for replay, fixed smoke, and bounded
      soak under the pinned runner and resource controls
- [x] 3.2 Add path-scoped Ubuntu PR/push smoke plus scheduled/manual soak,
      supply-chain checks, and failure-only artifact retention
- [x] 3.3 Run DID/crypto corpus replay and fixed smoke; record performance and
      triage every sanitizer or invariant finding

## 4. Verification and delivery

- [x] 4.1 Run focused crypto feature/workspace, strict Clippy/rustdoc/fmt,
      factory, MSRV, WASM/mobile, architecture, supply-chain, and full Nix gates
- [x] 4.2 Complete a distinct exact-head semantic/security/API review and prove
      all downstream HEAD/status receipts are unchanged
- [x] 4.3 Produce ready/receipt, synchronize the canonical crypto spec, archive
      the change, and deliver the signed issue-linked all-green PR

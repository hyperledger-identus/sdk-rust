## 1. Contract and provenance

- [x] 1.1 Reconcile issue #38 with exact base, standards, donor revisions,
      compatibility, threat/resource policy, non-scope, and rollback
- [x] 1.2 Complete and strictly validate the OpenSpec delta, ADR, and
      pre-implementation semantic/security review before production edits

## 2. Raw JSON and error boundary

- [ ] 2.1 Add the reusable crate-private streaming duplicate-name scanner with
      depth, node, object-member, live-key, malformed, and trailing-data limits
- [ ] 2.2 Apply it ahead of DID document deserialization and map duplicate and
      resource failures to stable redaction-safe document errors
- [ ] 2.3 Prove top-level and recursively nested duplicate rejection, escaped
      name equality, object-local scope, and every scanner edge

## 3. URI and generative conformance

- [ ] 3.1 Pin NeoPRISM-aligned `uriparse` 0.6.4 as dev-only and add classified
      RFC 3986 differential vectors without changing the runtime parser
- [ ] 3.2 Add deterministic generated URI/document/extension/cardinality and
      native/unique-wire equivalence suites with minimized regressions
- [ ] 3.3 Record release throughput and resource/allocation-shape evidence while
      keeping cargo-fuzz #35 and resolution-envelope #41 out of scope

## 4. Verification and delivery

- [ ] 4.1 Run focused coverage plus workspace all/no-feature, strict Clippy,
      rustdoc, format, factory, MSRV, WASM/mobile, supply-chain, and full Nix
- [ ] 4.2 Complete a distinct exact-head semantic/security/API review and
      confirm all downstream HEAD/status receipts remain unchanged
- [ ] 4.3 Produce ready/receipt, synchronize the canonical DID Core spec,
      archive the change, and deliver the signed issue-linked all-green PR

## 1. Contract and architecture

- [x] 1.1 Create child issue #34 under IDR-005 parent #5 with immutable
      standard, donor, license, scope, performance and rollback evidence
- [x] 1.2 Add the OpenSpec proposal, did-core delta, design and ADR 0008 before
      implementation; record semantic review with no uncleared blocker

## 2. DID lexical boundary

- [x] 2.1 Add immutable `Did` and `DidUrl` types with cached component offsets
- [x] 2.2 Implement bounded single-pass DID Core and RFC 3986 validation
- [x] 2.3 Add zero-allocation component accessors and allocation-aware
      conversions
- [x] 2.4 Add validating native/serde construction and stable redacted errors

## 3. Conformance and performance

- [x] 3.1 Add standards-derived and donor-shaped positive vectors
- [x] 3.2 Add delimiter, encoding, bounds and adversarial negative vectors
- [x] 3.3 Prove native/serde equivalence, exact round trips and component views
- [x] 3.4 Run and record a release parser throughput diagnostic
- [x] 3.5 Create bounded fuzzing follow-up #35 linked to parent #5
- [ ] 3.6 Run focused, workspace, wasm/mobile, lint, formatting, docs,
      OpenSpec, supply-chain and Nix gates

## 4. Delivery evidence

- [ ] 4.1 Sync the canonical did-core spec, archive the OpenSpec change and
      record exact verification plus distinct local semantic/security review
- [ ] 4.2 Prepare the issue-linked PR receipt and record exact-head CI, hosted
      review and green-only merge evidence

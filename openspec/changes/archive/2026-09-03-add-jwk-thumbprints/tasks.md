## 1. Contract and architecture

- [x] 1.1 Create child issue #32 under IDR-004 parent #9 with immutable
      standards, donor, license, scope, performance and rollback evidence
- [x] 1.2 Add the OpenSpec proposal, crypto delta, design and ADR 0007 before
      implementation; record semantic review with no uncleared blocker

## 2. RFC 7638 thumbprints

- [x] 2.1 Add the typed `JwkThumbprint` API and streaming SHA-256 computation
- [x] 2.2 Add digest-byte and canonical unpadded base64url accessors
- [x] 2.3 Add compositional `jwk-thumbprint` feature wiring and public exports

## 3. Conformance and misuse resistance

- [x] 3.1 Add RFC 8037 Appendix A.3 and independent EC vectors
- [x] 3.2 Prove extensions are ignored and required key material is sensitive
- [x] 3.3 Prove canonical input order and zero canonicalization allocation
- [x] 3.4 Run focused, workspace, minimal-feature, wasm, conformance, lint,
      formatting, docs, OpenSpec and factory gates under Nix

## 4. Delivery evidence

- [x] 4.1 Sync the canonical crypto spec, archive the OpenSpec change and
      record exact verification plus distinct local crypto/security review
- [x] 4.2 Prepare the issue-linked PR receipt and record exact-head CI, hosted
      review and green-only merge evidence

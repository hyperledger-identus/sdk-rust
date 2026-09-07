## 1. Contract and dependency decision

- [x] 1.1 Complete issue-linked proposal, crypto delta, design, research,
      constraint record and ADR 0078 before production code
- [x] 1.2 Pass strict OpenSpec, research-readiness and constraint-readiness gates

## 2. SDK-owned Cardano V2 facade

- [x] 2.1 Add workspace dependency and isolated `cardano-bip32` feature wiring
- [x] 2.2 Implement redacted, zeroizing extended-private key construction,
      export, private derivation and public conversion
- [x] 2.3 Implement redacted extended-public key construction, export and
      fallible soft derivation with hardened rejection
- [x] 2.4 Export only SDK-owned types and errors without changing `EdHDKey`

## 3. Conformance and security evidence

- [x] 3.1 Add Apollo/upstream hard and soft private derivation vectors
- [x] 3.2 Add private/public soft-equivalence and hardened-public rejection tests
- [x] 3.3 Add formatting-redaction, explicit-zeroization and invalid-key tests
- [x] 3.4 Prove minimal/default feature graphs and public dependency encapsulation

## 4. Validation and delivery

- [x] 4.1 Run focused tests, formatting, clippy, docs, MSRV, primary, portable
      targets, dependency policy, audit and complete factory gates
- [x] 4.2 Perform distinct correctness/security review and record findings
- [x] 4.3 Sync the canonical crypto spec, archive the OpenSpec change and produce
      the exact-head delivery receipt
- [x] 4.4 Prepare the signed/DCO issue-linked PR packet; hosted CI, review,
      green-only merge to `develop`, issue closure and cleanup remain PR evidence
- [x] 4.5 Create a focused upstream-hardening follow-up for secret formatting and
      zeroization, retaining the exact trigger and temporary-fork fallback

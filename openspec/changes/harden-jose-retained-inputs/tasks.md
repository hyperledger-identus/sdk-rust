# Tasks

## 1. Planning gate

- [x] 1.1 Inventory workspace and named downstream callers, alternatives,
      compatibility, threats, supply chain, targets, and rollback.
- [x] 1.2 Pass research/constraint/OpenSpec readiness, commit the planning-only
      contract, and write the immutable preimplementation receipt.

## 2. Implementation

- [x] 2.1 Add opaque validated key-ID, X.509-chain, and proof-client payloads;
      remove direct raw retained construction.
- [x] 2.2 Route protected-header and proof parsing, building, serialization,
      verification, and workspace callers through the validated APIs.
- [x] 2.3 Add exact/one-over, malformed, parity, tighter-limit, redaction, and
      public API closure evidence.

## 3. Governance and delivery

- [x] 3.1 Add the compatibility ADR and migration guidance; convert the JOSE
      inventory row and narrow only the corresponding `SDK-LIM-007` clause.
- [ ] 3.2 Run format, focused/minimal/all-feature tests, strict workspace gates,
      API/SBOM evidence, compatible target/Nix checks, and exact-diff plan.
- [ ] 3.3 Complete fresh security/API/architecture review, archive the change,
      prepare the signed PR, and publish bounded metrics locally plus to the PR
      and issue.

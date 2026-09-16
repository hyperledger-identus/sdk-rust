# Tasks

## 1. Planning gate

- [x] 1.1 Inventory workspace and named downstream callers, alternatives,
      compatibility, threats, supply chain, targets, and rollback.
- [x] 1.2 Pass research/constraint/OpenSpec readiness and write the immutable
      preimplementation receipt.

## 2. Implementation

- [x] 2.1 Add bounded fallible byte construction for both canonical codecs and
      remove the blanket infallible `From` surface.
- [x] 2.2 Migrate bounded parser and fixed-size JWK callers through private
      trusted encoding without changing successful output.
- [ ] 2.3 Add exact/one-over, ownership-form, canonical parity, redaction, and
      public source/API regression evidence.

## 3. Governance and delivery

- [ ] 3.1 Add the compatibility ADR and migration guidance; narrow the codec
      inventory row and only the corresponding `SDK-LIM-007` clause.
- [ ] 3.2 Run format, focused/minimal/all-feature tests, strict workspace gates,
      API/SBOM evidence, compatible Nix closure, and exact-diff plan.
- [ ] 3.3 Complete fresh security/API/architecture review, archive the change,
      prepare the signed PR, and publish bounded metrics locally plus to the PR
      and issue.

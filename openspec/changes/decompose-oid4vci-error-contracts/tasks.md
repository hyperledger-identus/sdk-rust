# Tasks

## 1. Planning contract

- [x] 1.1 Bind #271/#277 to exact `develop`, assess prior ADR applicability,
  and record research, constraints, architecture, security, target limits,
  #7/#168 protection, and rollback.
- [x] 1.2 Add ADR 0119, the OID4VCI-error delta specification, and the
  immutable exact-base 171-row golden with its recorded SHA-256.
- [x] 1.3 Independently review planning semantics/API/security and cross-check
  every committed golden row against source.
- [x] 1.4 Commit planning-only signed+DCO evidence; write, validate, and commit
  the durable preimplementation receipt before production edits.

## 2. Implementation

- [x] 2.1 Copy the golden byte-for-byte to the stable fixture before source
  edits and verify its hash.
- [x] 2.2 Add the three-field record and six protocol catalogue modules, then
  route all 171 variants exhaustively without changing public declarations,
  typed wire models, or behavior.
- [x] 2.3 Add exact golden/order/const/source/redaction tests, explicitly cover
  the 21 named gaps, and extend the existing checker, mutation suite, factory
  fixture, and Nix source contract with one bounded binding.

## 3. Evidence and delivery

- [ ] 3.1 Run focused/default/minimal/all-feature tests, strict Clippy, docs,
  format, public API/dependency/spec diffs, and honest maintainability metrics.
- [ ] 3.2 Run direct and authoritative Nix WASM/Android/iOS checks plus
  applicable workspace Nix/factory gates; preserve #7/#168 and state target
  limits exactly.
- [ ] 3.3 Resolve independent architecture/API/security and adversarial review,
  complete verification, ready/receipt/archive, and open one signed+DCO
  issue-linked PR to protected `develop`.

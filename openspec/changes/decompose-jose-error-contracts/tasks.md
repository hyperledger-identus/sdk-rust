# Tasks

## 1. Planning contract

- [x] 1.1 Bind #271/#280 to exact `develop`, assess prior ADR applicability,
  and record research, constraints, architecture, security and rollback.
- [x] 1.2 Add ADR 0118, the JOSE-error delta specification, and immutable
  exact-base 51-row golden with recorded SHA-256.
- [x] 1.3 Independently review planning semantics/API/security and cross-check
  every golden row against source.
- [ ] 1.4 Commit planning-only signed+DCO evidence; write, validate, and commit
  the durable preimplementation receipt before production edits.

## 2. Implementation

- [ ] 2.1 Copy the golden byte-for-byte to the stable fixture before source
  edits and verify its hash.
- [ ] 2.2 Add the three-field record and four catalogue modules, then route all
  variants exhaustively without changing public declarations or behavior.
- [ ] 2.3 Add exact golden/source/const/redaction tests and extend the existing
  checker, mutation suite, factory fixture, and Nix source contract.

## 3. Evidence and delivery

- [ ] 3.1 Run focused/default/minimal/all-feature tests, Clippy, docs, format,
  public API/dependency diffs and honest maintainability measurements.
- [ ] 3.2 Run direct WASM/Android/iOS plus applicable workspace Nix/factory
  gates; preserve #7/#168 and state target limits exactly.
- [ ] 3.3 Resolve independent architecture/API/security and adversarial review,
  complete verification, ready/receipt/archive, and open one signed+DCO
  issue-linked PR to protected `develop`.

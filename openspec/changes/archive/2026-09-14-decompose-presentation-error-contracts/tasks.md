# Tasks

## 1. Planning contract

- [x] 1.1 Bind #271/#279 to exact `develop`, assess ADR 0116 applicability,
  and record research, constraints, architecture, security and rollback.
- [x] 1.2 Add ADR 0117, the presentation-error delta specification, and the
  immutable exact-base 48-row golden with recorded SHA-256.
- [x] 1.3 Independently review the planning semantics/API/security and
  cross-check every golden row against source.
- [x] 1.4 Commit planning-only evidence with signed+DCO provenance and write
  and validate the durable preimplementation receipt.

## 2. Implementation

- [x] 2.1 Copy the golden byte-for-byte to the stable test fixture before
  production edits and verify its hash.
- [x] 2.2 Add the private lean record and five catalogue modules, then route all
  variants exhaustively without changing public declarations or behavior.
- [x] 2.3 Add exact golden/source/const/redaction tests and extend the existing
  immutable checker, mutation suite, factory and Nix source-filter contracts.

## 3. Evidence and delivery

- [x] 3.1 Run focused/default/minimal/all-feature tests, Clippy, docs, format,
  public API/dependency diffs and maintainability measurements.
- [x] 3.2 Run direct WASM/Android/iOS checks plus applicable workspace Nix and
  factory gates; preserve #7/#168 behavior and state target limits exactly.
- [x] 3.3 Resolve independent architecture/API/security and adversarial review,
  complete verification, ready/receipt/archive, and open one signed+DCO
  issue-linked PR to protected `develop`.

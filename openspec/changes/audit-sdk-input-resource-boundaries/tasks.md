## 1. Planning and evidence

- [x] 1.1 Audit implemented-package input families and identify avoidable versus
      owner-delegated work.
- [x] 1.2 Record BIP-39, compatibility, security, dependency and residual-limit
      research.
- [x] 1.3 Specify the inventory schema, enforcement, material constraint change,
      consumer impact and rollback before implementation.

## 2. Implementation

- [x] 2.1 Add the human/machine repository-wide input-resource inventory and
      narrow `SDK-LIM-007` atomically.
- [x] 2.2 Add an offline bounded checker, factory integration and structural
      mutation tests.
- [x] 2.3 Enforce BIP-39 entropy, word and passphrase limits before allocation or
      expensive work, with exact-boundary/redaction/KMP tests.
- [x] 2.4 Enforce count, depth, node and text budgets for retained standalone JWK
      extensions, with native/serde boundary and redaction tests.
- [ ] 2.5 Inventory the bounded DID method registry and separate DID cache
      policy limits from injected cache/clock adapter QoS ownership.
- [ ] 2.6 Iteratively dismantle every rejected native JWK extension tree,
      including errors that precede resource validation, with hostile-depth
      regression coverage.

## 3. Verification and delivery

- [x] 3.1 Run focused and full factory/Nix gates and complete a distinct
      security/architecture review.
- [x] 3.2 Prepare the completed change for guarded archive, protected CI and an
      issue-linked PR to `develop`.
- [ ] 3.3 Re-run focused, factory, candidate and compatible Nix gates; update
      review evidence and resolve every hosted review thread before integration.

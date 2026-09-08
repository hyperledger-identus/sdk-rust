# Tasks

- [x] 1.1 Create issue #197 as a focused child of #168 and #9.
- [x] 1.2 Inspect SDK, Apollo and NeoPRISM codec implementations, consumers,
      fixtures, standards and existing resource budgets.
- [x] 1.3 Decide the 4,096-byte parser ceiling, trusted-encoding asymmetry,
      error compatibility, JWK behavior, activation and rollback.
- [x] 2.1 Export `MAX_CRYPTO_TEXT_BYTES = 4_096` and reject oversized hex and
      base64url parser input before decoder work.
- [x] 2.2 Preserve the public error category/bridge and add redaction-safe local
      length diagnostics without retaining input.
- [x] 2.3 Add exact, one-over, valid-oversize, canonicalization, precedence and
      JWK inheritance tests.
- [x] 2.4 Update canonical crypto specification, public documentation and
      `SDK-LIM-007` atomically.
- [ ] 3.1 Run focused tests, formatting, Clippy, features, dependency equality,
      unsafe/native scans and complete Nix validation.
- [ ] 3.2 Perform distinct exact-diff/security review and resolve findings.
- [ ] 3.3 Record verification, archive the change, and validate the resulting
      factory state.
- [ ] 4.1 Prepare a signed, DCO-compliant PR to `develop` linked to #197 and
      preserve #168/#9 for their remaining scope.
- [ ] 4.2 Monitor hosted gates, fix real failures and merge only when green.

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
- [x] 3.1 Run focused tests, formatting, Clippy, features, dependency equality,
      unsafe/native scans and complete Nix validation.
- [x] 3.2 Perform distinct exact-diff/security review and resolve findings.
- [x] 3.3 Record local verification evidence and prepare the completed change
      for an immutable receipt and archive validation.
- [x] 4.1 Prepare a signed, DCO-compliant branch and issue-linked PR body for
      `develop`, preserving #168/#9 for their remaining scope.
- [x] 4.2 Record that hosted gates must pass before merge; hosted execution,
      fixes, merge and issue closure remain GitHub delivery evidence.

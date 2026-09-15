## 1. Planning and evidence

- [x] 1.1 Capture the exact failed canary step, package, prompt and license
      identity after all preceding macOS gates passed.
- [x] 1.2 Verify the stable API-35 default ARM64 package in Google's current
      AOSP catalog and document why Google services are unnecessary.
- [x] 1.3 Select the AOSP image without blanket license acceptance and pass
      strict research/constraint/OpenSpec readiness before implementation.

## 2. Implementation

- [x] 2.1 Add ADR 0122 and change workflow installation to the exact API-35
      default ARM64 package.
- [x] 2.2 Change verifier package/directory identity atomically without changing
      emulator behavior or support claims.
- [x] 2.3 Extend ordered structural checks and mutation cases for AOSP identity,
      install/execution parity and absence of broad license acceptance.

## 3. Verification and delivery

- [x] 3.1 Run focused support-policy, script syntax, factory/OpenSpec and
      proportional Nix checks; complete architecture/security review.
- [x] 3.2 Prepare the guarded archive and issue-linked protected PR.

## Post-merge issue acceptance

Issue #276 owns merge, immediate reopen, a complete exact-merged-head manual
canary and the first natural scheduled-run receipt. Hosted macOS execution is
the authoritative runtime proof because this development host has no Android
SDK installation.

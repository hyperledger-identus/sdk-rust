## 1. Planning and evidence

- [x] 1.1 Isolate the exact canary failure after AOSP installation and attach
      the run evidence to issue #276.
- [x] 1.2 Verify the current hosted runner's ambient NDK root and the official
      side-by-side NDK installation model.
- [x] 1.3 Decide exact SDK-relative selection with metadata plus ELF evidence
      and pass research/constraint/OpenSpec readiness before implementation.

## 2. Implementation

- [ ] 2.1 Add ADR 0123 and bind the verifier to the exact installed NDK path.
- [ ] 2.2 Validate exact `source.properties`, bind child-process aliases, and
      record metadata checksum evidence.
- [ ] 2.3 Extend structural policy and mutations for ambient-input rejection,
      exact revision metadata and child-process binding.

## 3. Verification and delivery

- [ ] 3.1 Run focused policy/script/factory gates and proportional Nix checks;
      complete exact-diff architecture/security review.
- [ ] 3.2 Archive the completed change and deliver an issue-linked protected PR.

## Post-merge issue acceptance

Issue #276 owns a complete manual canary on the exact merged SHA and the later
first natural scheduled-run receipt. This host lacks an Android SDK, so hosted
macOS is authoritative for native execution.

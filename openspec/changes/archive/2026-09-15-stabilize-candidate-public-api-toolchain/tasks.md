## 1. Contract and evidence

- [x] 1.1 Capture the exact canary revision, failed job and upstream toolchain
      selection behavior.
- [x] 1.2 Decide the stable rustdoc-JSON/parser separation and pass strict
      research and constraint readiness before implementation.

## 2. Implementation

- [x] 2.1 Generate rustdoc JSON explicitly with the primary Nix toolchain and
      scope `RUSTC_BOOTSTRAP=1` to that subprocess.
- [x] 2.2 Parse only the verified JSON artifact with locked
      `cargo-public-api`, without invoking a rustup-owned nightly compiler.
- [x] 2.3 Extend structural and mutation tests and add successor ADR 0121 while
      preserving accepted ADR 0113 byte-for-byte.

## 3. Verification and delivery

- [x] 3.1 Run focused candidate tests and one complete candidate generation in
      an environment without an installed rustup nightly.
- [x] 3.2 Run factory/OpenSpec and proportional Nix gates; complete an
      independent architecture/security review.
- [x] 3.3 Prepare the guarded archive for an issue-linked repair PR targeting
      protected `develop` without treating local evidence as hosted acceptance.

## Post-merge issue acceptance

Issue #276 owns protected PR review/CI, merge into `develop`, immediate reopen,
the Android-license follow-up repair, one complete exact-merged-head canary and
the first natural scheduled-run receipt. Those live operations cannot be
completed inside the pre-PR OpenSpec archive.

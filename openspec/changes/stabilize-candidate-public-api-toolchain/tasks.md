## 1. Contract and evidence

- [x] 1.1 Capture the exact canary revision, failed job and upstream toolchain
      selection behavior.
- [x] 1.2 Decide the stable rustdoc-JSON/parser separation and pass strict
      research and constraint readiness before implementation.

## 2. Implementation

- [ ] 2.1 Generate rustdoc JSON explicitly with the primary Nix toolchain and
      scope `RUSTC_BOOTSTRAP=1` to that subprocess.
- [ ] 2.2 Parse only the verified JSON artifact with locked
      `cargo-public-api`, without rustup or implicit nightly selection.
- [ ] 2.3 Extend structural and mutation tests and amend ADR 0113.

## 3. Verification and delivery

- [ ] 3.1 Run focused candidate tests and one complete candidate generation in
      an environment without an installed rustup nightly.
- [ ] 3.2 Run factory/OpenSpec and proportional Nix gates; complete an
      independent architecture/security review.
- [ ] 3.3 Open an issue-linked PR, resolve exact-head review/CI, merge into
      `develop`, reopen #276 and run the complete exact-merged-head canary.

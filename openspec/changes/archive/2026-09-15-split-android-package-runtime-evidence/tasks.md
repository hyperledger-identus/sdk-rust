## 1. Planning and evidence

- [x] 1.1 Isolate the exact canary boundary and attach the run report to #276.
- [x] 1.2 Research hosted virtualization, Android acceleration, exact AOSP
      packages, and package/runtime separation candidates.
- [x] 1.3 Specify the selected roles, limitations, diagnostics, acceptance, and
      rollback; pass research/constraint/OpenSpec readiness before implementation.

## 2. Implementation

- [x] 2.1 Add ADR 0124 and parameterize Android verification into explicit
      ARM64 package and test-only x86_64 runtime roles.
- [x] 2.2 Add the dedicated accelerated Linux runtime job, exact inputs,
      artifact retention, final receipt binding, and the isolated Rust target.
- [x] 2.3 Extend offline support-policy checks and mutations for runner, ABI,
      package, KVM, role-separation, diagnostics, and artifact contracts.

## 3. Verification and delivery

- [x] 3.1 Run focused script/policy/factory gates and proportional compatible
      Nix checks; complete a distinct architecture/security review.
- [x] 3.2 Prepare the completed change for guarded archive and an issue-linked
      protected PR.

## Post-merge issue acceptance

Issue #276 owns a complete canary on the exact merged SHA and the first natural
scheduled-run receipt. Hosted macOS/Linux remain authoritative for their
respective package/runtime roles because this host lacks Android SDK/KVM.

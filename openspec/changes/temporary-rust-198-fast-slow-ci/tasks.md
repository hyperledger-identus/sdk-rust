## 1. Contract and policy

- [ ] 1.1 Add ADR 0081 and supersede the active parts of ADR 0064 for the temporary phase.
- [ ] 1.2 Update support-policy and dependency-research specifications before implementation.
- [ ] 1.3 Update the machine support policy, constraint index and human guidance atomically.

## 2. Toolchain and Nix gates

- [ ] 2.1 Raise workspace `rust-version` to 1.98.1 and bind ordinary providers to that exact compiler.
- [ ] 2.2 Isolate the pinned nightly as `fuzzToolchain` and add a manifest-derived workspace `rust-build` gate.
- [ ] 2.3 Preserve every existing exhaustive check in the weekly/manual full flake.

## 3. Workflow split

- [ ] 3.1 Convert the Linux factory workflow into stable `fast` build/lint/test evidence for PRs and develop.
- [ ] 3.2 Move the Linux/macOS full Nix matrix to weekly/manual `slow` evidence.
- [ ] 3.3 Move all sanitizer workflows to staggered weekly/manual experimental evidence.

## 4. Enforcement and documentation

- [ ] 4.1 Update the offline support-policy checker and negative tests for one compiler and two CI lanes.
- [ ] 4.2 Update contributor, release, repository-settings, blueprint and factory guidance without overstating live protection.
- [ ] 4.3 Record baseline timing, review date, release prohibition and the 20-run post-change measurement follow-up.

## 5. Verification and review

- [ ] 5.1 Run focused policy, constraint, factory, action and file-hygiene checks.
- [ ] 5.2 Run the selected local fast derivations and complete local Nix flake slow matrix.
- [ ] 5.3 Complete a distinct exact-diff review, archive the change and open the ready PR to `develop`.

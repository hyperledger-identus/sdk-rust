## 1. Contract and research

- [x] 1.1 Create issue #230 and record the immutable develop base.
- [x] 1.2 Research Android, NDK, AGP, Gradle, UniFFI and JNA package choices.
- [x] 1.3 Specify deterministic AAR, explicit dependency, emulator, security,
  limitation and rollback contracts.
- [x] 1.4 Commit specification and ADR before implementation.

## 2. Android package proof

- [x] 2.1 Add Android std only to the dedicated bindings toolchain.
- [x] 2.2 Add deterministic ephemeral AAR assembly and tracked consumer
  template with exact Gradle dependencies.
- [x] 2.3 Prove ELF, ABI/API, symbols, complete-tree equality and path hygiene.
- [x] 2.4 Execute the behavior contract in an isolated arm64 emulator.
- [x] 2.5 Add the proof only to the weekly/manual slow macOS lane.

## 3. Review and delivery

- [x] 3.1 Update architecture/support evidence while retaining limitations.
- [ ] 3.2 Run focused, factory, Cargo, dependency and compatible Nix gates plus
  distinct exact-diff review.
- [ ] 3.3 Complete receipts, archive the delta and prepare a signed/DCO
  issue-linked PR for green-only integration.

# Tasks

## 1. Contract and decision

- [x] 1.1 Record #387 scope, exact base, sources, alternatives,
      package-by-package decision, constraints, threats, rollback, and evidence.
- [ ] 1.2 Pass research, constraint, and strict OpenSpec readiness; commit the
      planning-only contract and persist the immutable preimplementation receipt.
- [ ] 1.3 Accept ADR 0155 for staged-source qualification and the host-only HTTP
      adapter boundary.

## 2. Closed matrix policy

- [ ] 2.1 Add native-host and portable-target records to the DID descriptor,
      narrowing but never broadening the global support policy.
- [ ] 2.2 Extend offline validation for exact compilers, packages, profiles,
      hosts, targets, tiers, operations, limitations, and staged identity.
- [ ] 2.3 Add drift/overclaim mutations for compiler, package, target, host,
      operation, receipt, and unsupported HTTP portability.

## 3. Staged execution and receipt

- [ ] 3.1 Add primary/MSRV matrix modes to the existing bounded DID builder and
      retain unchanged archive/API/SBOM evidence.
- [ ] 3.2 Emit closed, bounded, atomic lane receipts and aggregate exactly four
      clean exact-SHA lanes into one matrix receipt.
- [ ] 3.3 Add pinned primary/MSRV Nix apps without adding compiler or dependency
      versions.

## 4. Slow-line integration

- [ ] 4.1 Run both compiler lanes on native Linux/macOS in weekly/manual slow CI
      and upload attempt-scoped lane artifacts.
- [ ] 4.2 Aggregate the lane artifacts in the immutable evidence job and bind the
      matrix outcome/file into the slow receipt.
- [ ] 4.3 Preserve the one-job required fast architecture and do not dispatch or
      rerun slow evidence from this slice.

## 5. Verification and delivery

- [ ] 5.1 Run focused mutations, synthetic receipts, local macOS lanes, Nix,
      format, workflow, factory, and OpenSpec checks.
- [ ] 5.2 Complete local architecture, security, compatibility, claim-boundary,
      workflow, and decomposition review.
- [ ] 5.3 Archive, open the issue-linked PR, pass exact-head hosted fast, merge,
      publish metrics, update M5, and close the managed worktree.

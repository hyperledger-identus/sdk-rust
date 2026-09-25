# Tasks

## 1. Contract and decision

- [x] 1.1 Record #384 scope, exact base, sources, alternatives, constraints,
      security/resource boundaries, rollback and verification plan.
- [ ] 1.2 Pass research, constraints and strict OpenSpec readiness; commit the
      planning-only contract and persist the exact preimplementation receipt.
- [ ] 1.3 Accept ADR 0154 for the first-candidate API/SemVer origin.

## 2. Closed evidence configuration

- [ ] 2.1 Add exact evidence tools and API baseline paths to the descriptor and
      fail-closed checker.
- [ ] 2.2 Generate and commit deterministic staged public-API baselines for
      exactly both candidate packages.
- [ ] 2.3 Extend mutation tests for tool, baseline and compatibility-state drift.

## 3. Supply-chain artifacts

- [ ] 3.1 Generate bounded CycloneDX 1.5 JSON for exactly both staged packages.
- [ ] 3.2 Validate component/version/license/dependency identity and reject Git
      sources, malformed/duplicate/oversize artifacts.
- [ ] 3.3 Bind API/SBOM digests, sizes, exact tools, repository policy and
      explicit limitations into the atomic receipt.
- [ ] 3.4 Reproduce the #382 candidate archive hashes and keep existing release
      evidence green.

## 4. Review and delivery

- [ ] 4.1 Run focused mutations, candidate, API, SBOM, Taplo, factory/OpenSpec
      and compatible cleaned-source checks.
- [ ] 4.2 Complete local architecture/security/decomposition/release review.
- [ ] 4.3 Archive, open an issue-linked PR, pass exact-head hosted CI, merge,
      publish metrics and close the managed worktree.

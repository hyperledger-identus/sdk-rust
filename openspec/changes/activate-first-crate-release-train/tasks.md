# Tasks

## 1. Planning and release controls

- [ ] 1.1 Record the package, credential, environment, approval, immutable
      identity, failure, and rollback decisions in OpenSpec and ADR 0134.
- [ ] 1.2 Pass research, constraint, and strict OpenSpec readiness and commit
      the planning-only contract.
- [ ] 1.3 Create the protected `crates-io` GitHub environment with independent
      maintainer approval and protected-ref deployment policy.

## 2. Package and publisher implementation

- [ ] 2.1 Activate explicit metadata for exactly the three release crates and
      exact path-plus-version internal dependencies.
- [ ] 2.2 Extend deterministic candidate evidence with a clean publication
      workspace and release-eligible receipt while preserving the no-publish
      candidate boundary.
- [ ] 2.3 Add a constrained, idempotent, dependency-ordered publisher and
      protected workflow with bootstrap-token and trusted-publishing modes.
- [ ] 2.4 Add structural and mutation tests for scope, versions, tag/SHA,
      signatures, environment controls, credentials, action pins, and order.
- [ ] 2.5 Update release policy, public handbook, changelog, limitations,
      ownership/recovery, and operator runbook.

## 3. Evidence and publication

- [ ] 3.1 Run factory, Rust, package, workflow, release-train, and dry-run
      evidence from a clean focused branch; complete a distinct local review.
- [ ] 3.2 Merge the issue-linked PR only after required CI and an independent
      maintainer approval; freeze the resulting protected `develop` revision.
- [ ] 3.3 Create and verify the signed release tag, obtain protected-environment
      approval, and publish derive, core, then crypto using the bootstrap token.
- [ ] 3.4 Verify crates.io packages, owners, checksums, docs.rs, and GitHub
      release evidence; configure all trusted publishers and revoke the token.
- [ ] 3.5 Close #3 and #326 only after immutable publication and ownership
      receipts are attached; archive this OpenSpec change.

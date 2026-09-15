# Tasks

## 1. Planning and preflight

- [x] 1.1 Reproduce the hosted failure and identify the differing native Cargo
      archive members.
- [x] 1.2 Record research, constraints, design, and the modified candidate
      contract before implementation.
- [x] 1.3 Commit the planning-only change and pass the exact issue #276
      preflight receipt.

## 2. Repair and focused evidence

- [ ] 2.1 Separate VCS-independent build scratch from destination-local atomic
      output staging and fail closed on repository-contained scratch.
- [ ] 2.2 Add focused behavioral and policy regressions for the staging
      boundary.
- [ ] 2.3 Run the real package-only and complete candidate evidence paths at an
      exact clean source revision.

## 3. Delivery and activation evidence

- [ ] 3.1 Run factory, formatting, Rust, archive, and issue-specific gates and
      record every unrun check.
- [ ] 3.2 Complete a distinct local review and resolve all findings.
- [ ] 3.3 Archive the OpenSpec change, open the issue-linked PR, and merge only
      after required CI is green and review threads are clear.
- [ ] 3.4 Rerun the protected `develop` slow canary and attach its exact receipt
      to issue #276; leave the issue open for the first natural schedule.

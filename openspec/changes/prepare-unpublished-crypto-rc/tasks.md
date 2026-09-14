# Tasks

## 1. Decision and package contract

- [ ] 1.1 Record the package-brand, candidate-scope, Rust evidence, and
  publication-boundary ADR.
- [ ] 1.2 Add strict candidate descriptor and crate-level package documentation.
- [ ] 1.3 Add the unpublished-candidate capability specification.

## 2. Candidate evidence

- [ ] 2.1 Implement deterministic staging and three-package archive assembly.
- [ ] 2.2 Verify normalized archives and a clean extracted consumer across the
  required feature profiles.
- [ ] 2.3 Generate pinned CycloneDX, API, checksum, and provenance evidence.
- [ ] 2.4 Add fail-closed tests for scope, versions, metadata, paths, archive
  contents, and publication prohibitions.

## 3. Delivery

- [ ] 3.1 Run and measure the candidate gate at the exact source head.
- [ ] 3.2 Run existing factory, Rust, package, and hosted protected gates.
- [ ] 3.3 Complete a distinct local architecture/security/release-boundary
  review and resolve findings.
- [ ] 3.4 Archive the OpenSpec change and merge the issue-linked PR while
  preserving the planning commit.

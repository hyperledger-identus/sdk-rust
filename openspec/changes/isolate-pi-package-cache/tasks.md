## 1. Cache contract

- [x] 1.1 Implement deterministic runtime/package identity and a canonical
      repository-private cache path outside registered working trees.
- [x] 1.2 Populate and verify a complete cache without lifecycle-script
      approval, then link Pi's project path without replacing operator data.

## 2. Bootstrap and operations

- [x] 2.1 Prepare and audit the cache before every bootstrap Pi launch while
      retaining the exact current runtime and package versions.
- [x] 2.2 Add the raw-Pi ignore fallback and document ownership, concurrency,
      retention, cleanup and recovery.

## 3. Evidence and integration

- [x] 3.1 Add focused tests for cache identity, external containment,
      malformed paths/sources, symlink safety, concurrency and clean status.
- [ ] 3.2 Run the factory/bootstrap gates and a cache-reuse smoke, record local
      review and verification, and archive the completed OpenSpec change.
- [ ] 3.3 Open the issue-linked exact-head PR, pass hosted `fast`, merge to
      `develop` and leave `main` unchanged.

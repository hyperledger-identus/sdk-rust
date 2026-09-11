## 1. Planning contract

- [x] 1.1 Record retrospective evidence, candidate dispositions, constraints,
      portable-audit behavior and the exact `develop` base.
- [x] 1.2 Pass strict OpenSpec, research and constraint readiness before any
      factory implementation edit.

## 2. Portable health check

- [x] 2.1 Make the direct factory audit enter the pinned Nix environment once
      outside a Nix shell and preserve direct execution inside it.
- [x] 2.2 Add effective runtime audit to the bootstrap `--check` composition
      with fail-fast exit propagation.

## 3. Evidence and integration

- [x] 3.1 Add known-good and known-bad routing/composition tests and update the
      factory operations documentation.
- [x] 3.2 Run focused and end-to-end factory, audit, Pi, file-hygiene and Nix
      gates; record timings and complete a distinct local review.
- [x] 3.3 Archive the reviewed change and prepare the signed issue-linked PR,
      exact-head CI and guarded `develop` integration evidence.

# Tasks

## 1. Planning

- [x] 1.1 Reproduce the CVSS 4 parser and offline yank-index failure modes.
- [x] 1.2 Inspect the pinned RustSec, Crane, nixpkgs, and advisory-db sources.
- [x] 1.3 Record the OpenSpec contract, material constraint decision, and ADR.
- [x] 1.4 Pass research, constraint, and strict planning readiness; commit the
      planning-only change and write the exact-head preflight receipt.

## 2. Implementation

- [ ] 2.1 Add a deterministic CVSS 4 audit compatibility fixture and stable
      result adapter with success, vulnerability, incompatible-tool, and yank
      unavailable states.
- [ ] 2.2 Pin/assert `cargo-audit 0.22.2`, override Crane with `--no-yanked`,
      and retain structured quiet evidence from the full pinned database.
- [ ] 2.3 Add focused mutation tests and factory/Nix manifest enforcement.
- [ ] 2.4 Update security-evidence documentation, the effective constraint
      index, and Discussion #399.

## 3. Verification and delivery

- [ ] 3.1 Run the focused fixtures, full pinned-db audit, Nix evaluation/build,
      formatting, and factory checks.
- [ ] 3.2 Perform a distinct exact-diff security/factory review and resolve all
      blocking findings.
- [ ] 3.3 Archive the completed OpenSpec change, open a signed/DCO PR to
      `develop`, publish metrics, and merge only after required CI is green.

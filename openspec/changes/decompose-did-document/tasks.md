# Tasks

## 1. Planning and characterization

- [x] 1.1 Inventory public exports, wire/cardinality behavior, JSON budgets,
      verification/service validation, aggregate invariants, and cleanup paths.
- [x] 1.2 Record private module ownership, dependency direction, compatibility,
      non-goals, risk, and rollback.
- [ ] 1.3 Commit planning alone, rebase it onto the merged issue #407
      `develop` tip, pass readiness, and write issue #408's exact-head receipt.
- [ ] 1.4 Run the complete pre-move DID all-feature suite and capture public,
      wire, source, and iterative-cleanup invariants.

## 2. Implementation

- [ ] 2.1 Move scalar-or-array cardinality without serde or construction drift.
- [ ] 2.2 Move context and extension-tree budgets with iterative cleanup intact.
- [ ] 2.3 Move verification methods/relationships with exact public-material,
      suite-neutrality, duplicate, redaction, and cleanup behavior.
- [ ] 2.4 Move service types/endpoints with exact cardinality, endpoint-map,
      extension, duplicate, and cleanup behavior.
- [ ] 2.5 Move the document aggregate/builder without changing raw preflight,
      cross-resource validation, serde construction, or rejection cleanup.
- [ ] 2.6 Prove public/wire/error/source equivalence and refresh code-health
      evidence without weakening unrelated ownership.

## 3. Verification and delivery

- [ ] 3.1 Run DID/workspace tests, strict Clippy/format, public/error/source
      contracts, factory checks, portable targets, and Nix.
- [ ] 3.2 Perform a distinct exact-diff architecture/security/Rust review and
      resolve every blocking finding.
- [ ] 3.3 Archive the change, open an issue-linked signed/DCO PR to `develop`,
      publish metrics, and merge only after required CI is green.

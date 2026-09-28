# Tasks

## 1. Planning and characterization

- [x] 1.1 Inventory all public limit types, exports, constructors, accessors,
      defaults, validation errors, compositions, consumers, and governance.
- [x] 1.2 Record the central policy invariant, lifecycle module map, routine
      constraint impact, rejected abstractions, and rollback boundary.
- [x] 1.3 Pass research/constraint/planning checks, commit planning alone, and
      write the exact-head pre-implementation receipt for issue #404.
- [x] 1.4 Run the complete existing OID4VCI characterization suite before
      moving policy or public type mechanics.

## 2. Implementation

- [x] 2.1 Move every existing numeric default and the configurable depth
      ceiling unchanged into one private policy module.
- [x] 2.2 Move limit types into offer, token, credential, and metadata modules
      while preserving the private facade and crate-root public exports.
- [x] 2.3 Prove exact public signatures, defaults, validation predicates,
      errors, and compositions against the base inventory.
- [ ] 2.4 Refresh code-health evidence and remove the completed limits hotspot
      without weakening unrelated ownership or governance.

## 3. Verification and delivery

- [ ] 3.1 Run post-move OID4VCI tests, error/conformance evidence, strict
      Clippy, workspace tests, factory checks, and relevant Nix gates.
- [ ] 3.2 Perform a distinct exact-diff architecture/security/Rust review and
      resolve every blocking finding.
- [ ] 3.3 Archive the completed change, open an issue-linked signed/DCO PR to
      `develop`, publish metrics, and merge only after required CI is green.

# Tasks

## 1. Planning contract

- [x] 1.1 Bind the slice to umbrella issue #271, delivery issue #278, and the
  exact `develop` base; record
  research, constraints, scope, compatibility, security, and rollback evidence.
- [x] 1.2 Add ADR 0116, the `public-error-contracts` delta specification, and
  the immutable 47-row pre-refactor credentials golden.
- [x] 1.3 Complete the pre-implementation semantic/architecture/API/security
  review and cross-check the golden against the exact base sources.
- [x] 1.4 Commit the planning-only contract with signed+DCO provenance and
  write and validate its durable preimplementation receipt.

## 2. Credentials pilot

- [x] 2.1 Copy the planning golden byte-for-byte to the stable credentials
  test-fixture path and verify the recorded SHA-256 before changing production
  error code.
- [x] 2.2 Add the private crate-local `ErrorContract` and the five cohesive
  envelope, metadata, status, verification, and verifier catalogue modules.
- [x] 2.3 Route both existing credentials error enums exhaustively through the
  private records while preserving every public declaration and behavior.
- [x] 2.4 Add the independent golden regression, source/redaction and const-use
  checks, plus a temporary local compile mutation proving incomplete routing
  fails; commit no mutation.

## 3. Verification evidence

- [ ] 3.1 Run focused credentials tests, Clippy, docs, and default, minimal,
  and all-feature checks with the repository-pinned Rust toolchain.
- [ ] 3.2 Record an empty base/head public API diff, unchanged dependency and
  feature inventory, and before/after mapping SLOC, duplication, and largest
  function evidence.
- [ ] 3.3 Run applicable native, WASM, mobile, Nix, architecture, factory, and
  full-workspace gates without changing issue #7 or issue #168 behavior.

## 4. Review and delivery

- [ ] 4.1 Perform a distinct architecture/API/security self-review against the
  exact implementation head and resolve every finding.
- [ ] 4.2 Complete verification evidence, sync canonical specs if required,
  pass factory readiness, archive the change, and retain signed+DCO history.
- [ ] 4.3 Open one ready PR linked to delivery issue #278 and umbrella #271 to
  protected `develop`; merge only after required CI/review gates pass. Do not
  publish, adopt in consumers, or expand into other error crates.

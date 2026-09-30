# Tasks

## 1. Specification and planning

- [x] 1.1 Audit current `identus-did` values/errors and the pinned SDK-TS DID,
      DID URL, parser, and legacy error surfaces.
- [x] 1.2 Decide canonical direction, loss classification, version window,
      migration, observability, rollback, and removal rules.
- [x] 1.3 Specify four bounded DID value/error seed records and the explicit
      consumer-implementation stop boundary.
- [x] 1.4 Commit this planning packet and bind an exact preimplementation
      receipt to issue #505.

## 2. Mapping implementation

- [x] 2.1 Add the versioned language-neutral mapping registry and four
      SDK-TS DID seed records.
- [x] 2.2 Add strict offline validation and mutation tests.
- [x] 2.3 Add deterministic human documentation generation and drift checks.
- [x] 2.4 Integrate the registry, renderer, and checks into the factory
      contract.

## 3. Verification and closeout

- [x] 3.1 Run focused validator, mutation, and renderer tests.
- [x] 3.2 Run OpenSpec readiness and repository factory gates.
- [ ] 3.3 Perform a fresh-diff architecture/security review, archive the
      change, publish metrics, and close issue #505 through a protected PR.

## 4. Explicit stop boundary

- [x] 4.1 Confirm no consumer repository, language adapter implementation,
      public Rust DTO/error, binding, release, or target-support claim changed.

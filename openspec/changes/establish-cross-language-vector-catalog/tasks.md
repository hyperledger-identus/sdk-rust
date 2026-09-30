# Tasks

## 1. Specification and planning

- [x] 1.1 Audit W3C DID Core, current SDK-Rust DID behavior, and pinned
      SDK-TS/Swift/KMP generic DID examples.
- [x] 1.2 Decide authority, provenance, licensing, packet, mutation,
      supersession, and offline-validation rules.
- [x] 1.3 Specify the bounded DID/DID URL seed and explicit stop boundary.
- [x] 1.4 Commit this planning packet and bind an exact preimplementation
      receipt to issue #420.

## 2. Catalog implementation

- [ ] 2.1 Add the versioned catalog and explanatory documentation.
- [ ] 2.2 Add the immutable DID/DID URL packet with positive, negative,
      boundary, redaction, and consumer-regression cases.
- [ ] 2.3 Add the strict offline validator and mutation tests.
- [ ] 2.4 Add a Rust conformance loader/test without a production dependency.
- [ ] 2.5 Integrate all records and checks into the factory contract.

## 3. Verification and closeout

- [ ] 3.1 Run focused validator, mutation, and Rust packet tests.
- [ ] 3.2 Run OpenSpec readiness and repository factory gates.
- [ ] 3.3 Perform a fresh-diff architecture/security review, archive the
      change, publish metrics, and close issue #420 through a protected PR.

## 4. Explicit stop boundary

- [ ] 4.1 Confirm no consumer repository, language adapter, DID method,
      credential/protocol engine, release, or target-support claim changed.

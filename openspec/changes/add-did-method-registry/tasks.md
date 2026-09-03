## 1. Contract and architecture

- [x] 1.1 Create focused issue #44 after #43 with scope, dependencies,
      standards behavior, acceptance boundary and rollback
- [x] 1.2 Inspect immutable donor revisions and record the reusable
      intersection without copying code or modifying downstream trees
- [x] 1.3 Add OpenSpec proposal, did-core delta, design, threat contract,
      pre-implementation review and ADR 0012 before implementation

## 2. Registry model

- [ ] 2.1 Add bounded method bindings, builder and immutable registry
- [ ] 2.2 Add duplicate/capacity errors and stable redaction-safe bridging
- [ ] 2.3 Add deterministic narrow introspection without adapter exposure

## 3. Dispatch and conformance

- [ ] 3.1 Implement exact method resolution through `DidResolver`
- [ ] 3.2 Implement independent dereferencing through `DidUrlDereferencer`
- [ ] 3.3 Cover PRISM/Midnight routing, unknown/unsupported standards failures,
      duplicate/capacity limits, exact matching and concurrent clones
- [ ] 3.4 Record release dispatch performance and run focused/workspace/Nix
      verification

## 4. Delivery evidence

- [ ] 4.1 Sync canonical did-core specification and record exact evidence
- [ ] 4.2 Complete a distinct local semantic/security/API review
- [ ] 4.3 Produce ready/receipt, archive, open the issue-linked PR, and merge
      only after all exact-head hosted CI gates are green

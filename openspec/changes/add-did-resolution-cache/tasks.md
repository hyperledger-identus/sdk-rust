## 1. Contract and architecture

- [x] 1.1 Reconcile and update issue #45 after #43 and #44 with the current
      W3C cache/no-cache and metadata boundary
- [x] 1.2 Inspect immutable donor revisions and record the reusable
      intersection without copying code or modifying downstream trees
- [x] 1.3 Add OpenSpec proposal, foundation-time and did-core deltas, threat
      contract, pre-implementation review and ADR 0013 before implementation

## 2. Foundation time and options

- [x] 2.1 Add bounded time values plus split fallible clock ports to core
- [x] 2.2 Add exact optional `noCache` input behavior and round-trip tests

## 3. Cache contract and decorator

- [x] 3.1 Add normalized bounded cache key, policy, entry, status and errors
- [x] 3.2 Add object-safe bounded cache lookup/store/invalidation port
- [x] 3.3 Add opt-in caching resolver with exact hit/miss/bypass/failure rules
- [x] 3.4 Cover fake clocks/backends, metadata non-TTL behavior, poisoning,
      invalidation, concurrent misses and release performance

## 4. Delivery evidence

- [x] 4.1 Run focused, conformance, workspace, feature, lint and Nix gates
- [x] 4.2 Complete a distinct semantic/security/API review and sync canonical
      specifications with exact verification evidence
- [x] 4.3 Produce ready/receipt, archive, and prepare the issue-linked PR under
      the exact-head all-green merge policy

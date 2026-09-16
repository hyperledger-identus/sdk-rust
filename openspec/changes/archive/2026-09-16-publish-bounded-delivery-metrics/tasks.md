# Tasks

## 1. Contract and preflight

- [x] 1.1 Record issue #306, research, material constraint direction, ADR 0128,
      design, complete delta requirement, and rollback.
- [x] 1.2 Pass research readiness, constraint readiness, strict OpenSpec, and
      immutable planning preflight before implementation.

## 2. Dual-retention publisher

- [x] 2.1 Make publish persist or confirm the exact private record before any
      remote mutation and reject conflicting retained content.
- [x] 2.2 Add PR-first auto targeting, issue fallback/override, exact hosted
      historical PR-head verification, and authoritative closing-issue binding
      without weakening current-head writes.
- [x] 2.3 Add one bounded mutation retry and closed, redacted telemetry-debt
      failure behavior.

## 3. Contract tests and operator flow

- [x] 3.1 Add positive and negative tests for target routing, hosted identity,
      local-before-remote ordering, conflicts, duplicates, retry bounds, and
      privacy.
- [x] 3.2 Update factory policy, agent/supervisor guidance, metrics handbook,
      command help, and examples with the terminal dual-receipt flow.
- [x] 3.3 Prove v1/v2 rendering and existing markers remain compatible.

## 4. Verification, backfill and delivery

- [x] 4.1 Run focused Node tests, factory/OpenSpec checks, and available local
      Rust fast equivalents; leave unavailable exact Nix derivations to hosted
      `fast` evidence.
- [x] 4.2 Complete a distinct privacy/architecture review, archive the change,
      and prepare the signed/DCO issue-linked PR.
- [x] 4.3 Validate the retained records for #246/#247/#250/#257/#259 and
      prepare their exact issue-targeted backfill. Execute it only after merge,
      then verify idempotency and publish #306 metrics to its PR.

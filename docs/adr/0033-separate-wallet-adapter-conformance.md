# ADR 0033: separate wallet adapter conformance from repository guards

- **Status:** Accepted for experimental implementation
- **Date:** 2026-09-05
- **Related work:** issue #91, predecessor #89, `IDR-010`, OpenSpec change
  `add-wallet-storage-conformance`

## Context

The wallet storage ports are implemented, but each consumer would otherwise
write a different test interpretation. The existing `identus-conformance`
crate protects this repository's structure and intentionally has no domain
dependency. Consumer behavioral testing has a different dependency direction
and lifecycle.

## Decision

1. Add `identus-wallet-conformance` in the verification ring with
   `identus-wallet` as its only runtime dependency.
2. Invoke each of the five actual production ports through public,
   executor-neutral async checks.
3. Share exact-record lifecycle behavior internally while preserving separate
   secret/status and list-capable public entry points.
4. Require successful replacement to invalidate the preceding per-record
   revision so compare-and-swap prevents lost updates.
5. Accept preseeded consumer list fixtures and verify page bounds, cursor
   progress, termination, duplicates and expected membership without defining
   indexing policy.
6. Return only static failure classes and aggregate receipt counts; require no
   formatting or serialization from consumer-owned types.
7. Keep the local memory implementation test-only. Require independent
   downstream in-memory and encrypted receipts before delivering `IDR-010`.

## Consequences

- Oxid, midnight-identity and future wallets can generate comparable evidence
  without adopting an SDK runtime or storage implementation.
- The workspace gains one focused verification crate and one inward dependency
  edge, while production dependency cones remain unchanged.
- Test fixtures need clone/equality bounds that production storage values do
  not need.
- Encryption, crash consistency, custody and platform behavior remain outside
  generic conformance.

## Rejected alternatives

- **Put the harness in `identus-conformance`:** mixes reusable consumer
  behavior with repository self-policing and changes that crate's narrow role.
- **Put test support behind a wallet feature:** makes verification code part of
  the production orchestration package and complicates feature policy.
- **Ship a reference storage adapter:** would establish backend and persistence
  policy without consumer evidence.
- **Copy tests downstream:** permits semantic drift and defeats crystallization.

## Rollback

Before publication, remove the focused verification crate and revert the
revision clarification. No production adapter or stored representation exists.

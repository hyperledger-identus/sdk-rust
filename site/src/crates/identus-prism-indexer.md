# `identus-prism-indexer`

Deterministic PRISM transaction planning and atomic commit contracts.

## Responsibility

- validate one complete bounded, ordered source transaction;
- derive a domain-separated transition digest over exact records;
- probe immutable transition receipts before mutable-state reads;
- plan DID and VDR mutations in wire order; and
- submit the complete mutation set plus next cursor through one atomic commit.

Exact replay is idempotent. Identity reuse with different content, stale
cursors, incomplete batches, malformed operations, invalid state, and commit
conflicts fail closed.

## Dependency direction

This is the outer portable composition crate. It depends on
`identus-prism-vdr`, which depends on `identus-prism-did`. It does not pull in
an async executor, network client, database driver, Cardano client, or native
system library.

## Ports and non-goals

Adopters implement cursor, transition-receipt, DID/VDR lookup, and atomic
commit ports. They also own authoritative transaction-boundary extraction,
polling, cancellation, retry, telemetry, rollback policy, and process
lifecycle. The SDK owns no internal queue or background task.

- [Source](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/prism-indexer)
- [Executable walkthrough](../architecture/prism-portable-stack.md)
- [Atomic indexing decision](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0177-use-synchronous-atomic-prism-indexing-core.md)

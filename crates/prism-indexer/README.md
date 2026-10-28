# identus-prism-indexer

Deterministic, effect-neutral PRISM transaction indexing primitives.

This crate validates complete ordered batches, plans DID/VDR state transitions,
recognizes exact replay, and presents one caller-owned atomic commit boundary.
It owns no source polling, channel, async runtime, database, Cardano client,
HTTP service, or retry loop.

See the
[portable PRISM stack](../../site/src/architecture/prism-portable-stack.md) for
the dependency model and executable E2E command.

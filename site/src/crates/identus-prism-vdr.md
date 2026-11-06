# `identus-prism-vdr`

Bounded, effect-neutral PRISM verifiable-data-registry semantics.

## Responsibility

- parse bounded create, update, and deactivate storage operations;
- authorize exact operation bytes with the active PRISM VDR key;
- represent root ownership, current head, and terminal state; and
- traverse ancestry locally first under finite depth and lookup budgets.

The crate returns typed, redacted outcomes for malformed, missing, cyclic,
stale, terminal, unauthorized, and resource-exhausted operations.

## Dependency direction

The crate depends on `identus-prism-did` for method state and exact signed
envelopes. `identus-prism-indexer` composes VDR mutations; no generic crate
depends on this method-specific layer.

## Ports and non-goals

`VdrLookup` is a caller-implemented read port. The crate does not supply a
Cardano client, SQL repository, cache, retry loop, runtime, HTTP service, CLI,
or deployment. Cancellation belongs between bounded calls.

- [Source](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/prism-vdr)
- [Portable stack walkthrough](../architecture/prism-portable-stack.md)
- [Quality baseline](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/prism-vdr-indexer-quality-baseline.md)

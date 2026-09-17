# Ownership boundaries

![Ownership boundaries around the SDK](../diagrams/ownership-boundaries.svg)

## SDK-Rust owns

- generic primitives and domain values;
- reusable cryptographic operations and bounded representations;
- protocol-neutral ports and validation contracts;
- conformance and compatibility evidence attached to those surfaces.

## Chain and identity repositories own

- DID method semantics and operation construction;
- ledger clients, indexing, submission, synchronization, and finality;
- Compact contracts, proving systems, and chain-native cryptosuites;
- chain deployment and operational policy.

## Wallet and application products own

- key custody, secure storage, consent, account recovery, and authorization;
- trust decisions, credential selection, user experience, and telemetry;
- backend deployment, availability, privacy, and regulatory controls;
- the decision to adopt an SDK revision or release.

An SDK abstraction is reusable only when these product and chain decisions can
remain outside it.

# SDK-TS peer DID facade quality evidence

Issue [#603](https://github.com/hyperledger-identus/sdk-rust/issues/603)
adds an unpublished, opt-in peer DID method surface to the existing
capability-named TypeScript facade. Canonical semantics remain in
`identus-did-peer`; the TypeScript layer performs closed structural ingress,
ownership-safe projection, target loading, and redacted error mapping.

## Selected profiles

- numalgo 4 creation, long-form parsing, short-form parsing, and short/long
  correlation for local resolution;
- explicitly named numalgo 2 compatibility creation and parsing from caller
  supplied public keys and services;
- browser, Node.js ESM/CommonJS, React SSR, and hydrated React execution of the
  same assertion body;
- a frozen pairwise `{from, to}` value boundary without DIDComm packing,
  transport, storage, key generation, or custody.

Historical numalgos 0, 1, and 3, React Native, publication, support, default
selection, accepted performance budgets, and external SDK-TS adoption are not
claimed.

## Local gates

Run the canonical core and binding gates through the pinned Nix environments:

```text
nix develop --command cargo test -p identus-wasm-did
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh all
```

The target gate builds deterministic API-version-4 browser and Node artifacts,
checks declarations and package boundaries, executes the shared peer vector
assertions on every applicable runtime, and emits exact-head target receipts.
The existing peer DID fuzz target and native conformance packet remain the
authoritative malformed-input and cross-implementation evidence for the Rust
core.

## Security and ownership

The TypeScript ingress rejects accessors, exotic prototypes, cycles, unknown
fields, and unsafe integers before serialization. Rust independently rejects
secret-shaped numalgo-4 members and revalidates all domain invariants and
resource ceilings.
Generated WASM values are copied and freed exactly once, projections are frozen,
and observations exclude identifiers, documents, public keys, services, raw
errors, stacks, and WASM memory.

## Rollback

Provider selection is explicit and immutable after initialization. Rollback
selects the existing legacy provider before initialization; no canonical Rust
rejection triggers an automatic parser, profile, resolver, or legacy fallback.
Removing this unpublished facade does not remove the Rust peer DID crate or alter
the external SDK-TS repository.

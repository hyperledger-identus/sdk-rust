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
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh adapter-fuzz
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh differential
```

The target gate builds deterministic API-version-4 browser and Node artifacts,
checks declarations and package boundaries, executes the shared peer vector
assertions on every applicable runtime, and emits exact-head target receipts.
The deterministic adapter fuzz lane executes 2,048 cases at seed `0x605f0078`
through both TypeScript peer input serializers and both generated-value
projection functions. It covers bounded nested values, accessors, cycles,
exotic prototypes, unsafe numbers, unknown fields, invalid purposes and
multibase values, bare service IDs, malformed generated fields, frozen output,
and exact-once generated-value release. Node.js/V8 exception handling and the
explicit ownership counters are the adapter oracle; this lane makes no ASAN
claim.

The existing peer DID libFuzzer/ASAN lane remains separate, authoritative
core-only evidence. Its 300-second soak exercises the Rust parser,
create/resolve and hash-linkage boundary; it is not relabelled as TypeScript
adapter fuzz evidence.

The native differential test executes every applicable negative packet and
compares exact identifiers, authentication and key-agreement references,
service ID/type/URI, routing keys and accept values. Accessor, cycle,
exotic-prototype, unsafe-number and closed-object-field rejection are explicit
TypeScript-only preconditions that occur before Rust and are therefore not
attributed to the native differential runner.

The peer evidence-completion implementation boundary is commit
`3482ab32d5ee10c5e4b7766c0c78dbeedfe95ae5`. The original facade implementation
boundary remains `5fb02d1bf7507adbdf8bd0255ebe15c2ea146756`. At that revision, the pinned
Rust 1.99.1 environment passed 29 `identus-did-peer` unit tests, the portable
peer packet, and 17 `identus-wasm-did` tests. The isolated TypeScript target
gate passed package/type checks plus the same 13 peer vectors in browser,
Node.js ESM/CommonJS, React SSR, and hydrated React. React Native remains
explicitly inapplicable for this private web canary.

The retained exact-head target receipt digests are `8bad9557` for package/types,
`3503a29c` for browser, `7b1cc25d` for Node.js, and `09071a97` for React web.
The standalone Rust/WASM browser fixture compiled after its canonical/legacy
numalgo-2 assertion was repaired, but local execution did not start because the
available Chrome 157 binary did not match ChromeDriver 152; Firefox was absent.
This local driver limitation does not replace the passing browser target receipt
and must be rechecked by the isolated hosted browser lane.

The deterministic target assembly SHA-256 is
`13c003e72af9af636fabb53f6ec8e323d57d7d69ce0d376c77497fee64d8d60c`;
the generated WASM module is 507,680 bytes. These measurements are descriptive,
not an accepted support or performance budget. On Apple arm64 with Node 24.19.0,
101 samples after 21 warm-ups measured ordinary numalgo-4 creation at 0.033 ms
p50 / 0.043 ms p95 and exact-limit creation at 602.257 ms p50 / 772.474 ms p95.
The respective normalized inputs were 17 and 16,384 bytes; projected outputs
were 1,013 and 161,874 bytes; process RSS after the complete measurement set was
86,130,688 bytes. The exact Rust 1.99.1 ASAN peer-DID soak completed 6,784,994
executions in 301 seconds at seed 4279326073, with 22,541 average executions per
second, 695 MiB peak RSS, and no crash or timeout.

## Security and ownership

The TypeScript ingress rejects accessors, exotic prototypes, cycles, unknown
fields, unsafe integers, and over-limit JSON byte budgets before serialization.
Rust independently rejects
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

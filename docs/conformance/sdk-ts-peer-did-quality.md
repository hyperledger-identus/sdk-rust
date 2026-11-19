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
nix develop .#typescript --command ./scripts/check-sdk-ts-did-target.sh measurements
```

The measurement command additionally requires
`SDK_TS_DID_BASELINE_PACKAGE_ROOT` to name a clean, already built worktree at
`bd1981372fcb08772d1196131a0cf26291fad9cb`,
`SDK_TS_DID_BASELINE_REVISION` to contain that SHA, and
`SDK_TS_DID_CANDIDATE_REVISION` to contain the clean current checkout SHA.
The baseline and candidate package roots must each contain artifacts and
`dist/node.js` produced by the pinned TypeScript Nix environment. The runner
verifies both worktree revisions and rejects tracked modifications before
measuring.

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

The measurement lane compares the exact pre-peer PR #604 base
`bd1981372fcb08772d1196131a0cf26291fad9cb` with a caller-declared exact
candidate revision built on the same host and pinned toolchain. For each web
and Node target it sums the identical manifest-ordered files both uncompressed
and as individual deterministic RFC 1952 level-9 streams with zero mtime, then
records signed candidate-minus-baseline deltas. Cold Node initialization is
dynamic module import plus first provider creation in 11 isolated processes;
warm initialization is cached dynamic import plus provider creation in one
process, with provider construction measured separately over 101 samples after
21 warm-ups. The receipt also retains p50/p95 operation latency, input/output
bytes, RSS, exact revisions, environment, aggregation, and limitations.

The final exact-revision receipt is private local evidence and its digest and
summary are published with the pull request. Browser and React receipts remain
functional proof only; this Node measurement does not establish their runtime
performance. No value is an accepted support or regression budget. The exact
Rust 1.99.1 ASAN peer-DID soak completed 6,784,994
executions in 301 seconds at seed 4279326073, with 22,541 average executions per
second, 695 MiB peak RSS, and no crash or timeout.

The retained measurement compares baseline
`bd1981372fcb08772d1196131a0cf26291fad9cb` with implementation boundary
`b1ea7101655f0a29964df9a35bfc8be312162fde`; its SHA-256 is
`bf9b3d95c15e45b4206ba18a27405d3682fa1dc673a0cd4c1b71d138faf438c5`.
Web artifacts changed from 377,545 to 573,733 raw bytes and 136,885 to
197,444 deterministic gzip bytes. Node artifacts changed from 370,084 to
565,143 raw bytes and 135,481 to 195,996 deterministic gzip bytes. On Apple
arm64 with Node 24.19.0, cold import-plus-provider p50/p95 changed from
14.106/18.327 ms to 18.016/21.783 ms; warm cached import-plus-provider changed
from 0.005/0.020 ms to 0.015/0.033 ms. These are directional results from one
host and remain non-budget evidence.

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

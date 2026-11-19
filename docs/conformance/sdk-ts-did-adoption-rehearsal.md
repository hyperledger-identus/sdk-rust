# SDK-TS DID facade adoption rehearsal

Issue [#609](https://github.com/hyperledger-identus/sdk-rust/issues/609)
defines a private local rehearsal of the capability-named DID facade. The
consumer fixture is pinned to SDK-TS 8.1.4 revision
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`; the SDK-Rust revision is the
full `headSha` in each target receipt.

## Journey and provider routes

`packages/identus-sdk/tests/adoption-journey.mjs` owns one immutable pairwise
journey and one assertion function. It composes:

- deterministic numalgo 2 compatibility creation and local resolution;
- DID, DID URL, DID Document, authentication, key-agreement, and service
  projections;
- injected resolution and dereferencing with request/result correlation; and
- frozen request-shaped pairwise `from` and `to` values.

The function executes unchanged against a fixed SDK-TS compatibility fixture
and the Rust-backed provider. The fixture recognizes only the reviewed inputs
and outputs for this journey, and fails closed otherwise. It is not a second
DID implementation. Numalgo 4 is recorded in the same assertion function as a
Rust-only addition and an immutable legacy `not-applicable` disposition.

## Selection, failure, and rollback

Provider selection happens before generated initialization. The selected Rust
route receives a deliberate stable invalid-DID rejection while a compatibility
provider invocation counter remains unchanged. There is no catch-and-fallback
path. Rollback is then exercised explicitly by selecting the compatibility
provider at a fresh boundary and rerunning the known value.

## Exact receipts

The Node.js, browser, and React web target drivers retain private mode-0600
JSON receipts under `target/sdk-ts-did-receipts/adoption-<target>.json`. Each
receipt binds the exact SDK-Rust head, donor revision, source digest, required
`legacy` and `rust` routes, route-instance count, 23 assertions per route,
aggregate assertion count, zero skipped/focused/quarantined cases, terminal
selection, and explicit rollback. Node.js records ESM and CommonJS route pairs;
React web records SSR and hydrated-client pairs. React Native remains
structurally not applicable.

Run the full gate with the pinned toolchain:

```text
nix develop path:.#typescript --command ./scripts/check-sdk-ts-did-target.sh all
```

## Non-claims

This evidence closes only repository-local consumer-proof and rollback gaps.
It does not mutate or activate external SDK-TS, select a default, publish a
package, establish support or accepted budgets, qualify React Native, or claim
DIDComm packing/transport, credential workflows, storage, custody, agent
lifecycle, or mdoc delivery.

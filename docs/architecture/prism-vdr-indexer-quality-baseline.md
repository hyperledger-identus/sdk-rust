# PRISM VDR and indexer quality baseline

Issue: [#522](https://github.com/hyperledger-identus/sdk-rust/issues/522)  
Harness revision: `46608ea16d45fe6efb2a4835a7ad285912b92cdb`  
Recorded: 2026-10-27

This receipt covers the chain-neutral `identus-prism-vdr` and
`identus-prism-indexer` cores. It does not claim Cardano, SQL, HTTP, runtime,
deployment, device execution, FFI, or downstream NeoPRISM adoption.

## Dependency and target evidence

The exact internal graph guard accepts only
`identus-prism-indexer -> identus-prism-vdr -> identus-prism-did` plus the
declared generic foundation edges. `cargo tree --edges normal` shows no async
executor, network client, database driver, Cardano client, or native system
library introduced by either crate.

The following checks passed with Rust 1.98.1 on an Apple Silicon macOS host:

```text
cargo check -p identus-prism-vdr -p identus-prism-indexer --all-features
cargo check -p identus-prism-vdr -p identus-prism-indexer --no-default-features
cargo check -p identus-prism-vdr -p identus-prism-indexer --target wasm32-unknown-unknown
cargo check -p identus-prism-vdr -p identus-prism-indexer --target aarch64-apple-ios
cargo check -p identus-prism-vdr -p identus-prism-indexer --target aarch64-linux-android
```

These are compile-only receipts. They do not imply browser or device runtime
support, linking, packaging, or performance.

## Directional benchmark

Command: `cargo bench -p identus-prism-indexer --bench directional`.

| Operation | Median nanoseconds | Samples | Iterations per sample |
| --- | ---: | ---: | ---: |
| VDR parse | 700 | 101 | 100 |
| VDR traversal, 100 nodes | 8,680 | 101 | 100 |
| Indexer batch digest, 128 records | 59,441 | 101 | 100 |
| Plan same-transaction DID + VDR create | 120,804 | 101 | 100 |

This is a directional local baseline, not a regression threshold. The harness
reports that allocation observation is unavailable; a later stable Linux
baseline may add thresholds and allocation instrumentation.

## Property and fuzz evidence

Deterministic property tests exhaust all 24 orderings of a four-record batch
and every accepted ancestry length from zero through 200 predecessor hops. Focused VDR and
indexer unit, conformance, and clippy gates pass.

Both libFuzzer targets passed a 4,096-run AddressSanitizer smoke on the local
host:

- `prism_vdr`: 4,096 executions, peak RSS 43 MiB.
- `prism_indexer_batch`: 4,096 executions, peak RSS 75 MiB.

The weekly/manual Linux slow lane owns the required 300-second run per target:
`nix develop .#fuzz --command ./scripts/fuzz-prism-vdr-indexer.sh soak all`.
Until an immutable successful Linux run is recorded, that obligation remains
open under issue #522 and is not represented as completed evidence.

# Peer DID quality baseline

Issue [#523](https://github.com/hyperledger-identus/sdk-rust/issues/523)
owns the first bounded quality baseline for `identus-did-peer`.

## Directional benchmark

The source-bound benchmark uses 101 samples after 500 warm-up iterations and
500 operations per sample. On 2026-10-27, Rust 1.98.1 on Apple Silicon macOS
produced these advisory medians:

| Operation | Median nanoseconds per iteration |
| --- | ---: |
| numalgo-4 create | 54,604 |
| numalgo-4 parse and resolve | 57,274 |
| numalgo-2 parse and resolve | 34,635 |

The baseline detects order-of-magnitude drift; it is not a cross-host release
threshold and does not measure allocation. Linux and portable-target results
remain separate evidence.

## Evidence routes

- Property-style deterministic generation covers every supported numalgo-2
  purpose and numalgo-4 input sizes from zero through 512 bytes.
- The portable packet covers the pinned numalgo-4 reference output and a
  numalgo-2 public Rust-oracle vector.
- `scripts/fuzz-did-peer.sh` replays or fuzzes both profile parsers, creation,
  hash linkage and resolution under the public 128 KiB lexical ceiling.
- The package remains effect-free and accepts only caller-owned public
  material; secret-shaped members are rejected before retention.

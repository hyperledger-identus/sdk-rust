# DID parser parity harness

This isolated, unpublished Cargo workspace compares the SDK parser with exact
`did_url_parser 0.3.0`. It is not part of the SDK workspace or dependency
graph. Its lockfile makes the research replay deterministic.

Run from this directory with the repository Rust 1.98.1 toolchain:

```sh
cargo run --locked --release
DID_PARSER_BENCH=1 cargo run --locked --release
```

The first command asserts every recorded curated outcome and component view,
then enumerates 17,284 ASCII/percent/terminal-colon comparisons. The optional
second command adds a machine-specific throughput diagnostic; timings are not
an SDK performance promise or an adoption criterion.

Candidate source provenance is independently pinned by the crates.io checksum
and release commit in the associated OpenSpec research and ADR. The harness
uses the registry artifact because its packaged Rust source is byte-identical
to that commit.

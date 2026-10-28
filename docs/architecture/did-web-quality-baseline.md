# did:web quality baseline

Issue [#559](https://github.com/hyperledger-identus/sdk-rust/issues/559)
owns this source-bound baseline for `did.web.profile.v1`. Measurements are
directional development evidence, not portable release SLAs.

## Evidence routes

```sh
cargo test -p identus-did-web --all-features
cargo bench -p identus-did-web --bench directional
nix develop .#fuzz --command ./scripts/fuzz-did-web.sh soak
```

## Directional benchmark

The benchmark measures strict parse/derive, bounded successful document
binding, and early oversized-identifier rejection over 101 samples after warm
up. The functional byte/work ceilings are blocking; latency numbers are
diagnostic and host-specific.

The 2026-10-28 Rust 1.98.1 Apple Silicon run recorded these medians over 101
samples with 500 iterations per sample:

| Operation | Median |
| --- | ---: |
| parse and derive | 187 ns |
| bind a bounded response | 374 ns |
| reject an oversized identifier | 2 ns |

## Sanitizer fuzz soak

The 2026-10-28 AddressSanitizer soak completed 21,360,099 executions in 301
seconds without a finding. It reached the full 140,000-byte input limit,
reported 70,963 average executions per second, and peaked at 947 MiB RSS under
the 1,024 MiB ceiling. This local Darwin result complements, but does not
replace, the protected Linux slow lane.

## Dependency and effect boundary

The production crate depends only on `identus-core`, `identus-derive`, and
`identus-did`. It contains no HTTP, DNS, TLS, URL-runtime, executor, timer,
cache, persistence, or broad SSI dependency. The in-memory E2E proves the
public injected port without claiming live-network behavior.

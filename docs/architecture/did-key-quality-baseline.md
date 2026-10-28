# did:key quality baseline

Issue [#557](https://github.com/hyperledger-identus/sdk-rust/issues/557)
owns this source-bound baseline for the strict `did.key.profile.v1` capability.
The figures are directional development evidence, not cross-host release
thresholds.

## Directional benchmark

Command:

```text
cargo bench -p identus-did-key --bench directional
```

Environment: Rust 1.98 etalon on Apple Silicon macOS, 2026-10-28. Each result
is the median of 101 samples after 500 warm-up iterations; every measured
sample contains 500 iterations.

| Operation | Median ns/iteration |
| --- | ---: |
| create Ed25519 DID | 1,946 |
| parse Ed25519 DID | 5,348 |
| resolve local document | 5,142 |
| reject oversized identifier | 95 |

The oversized path stays ahead of generic DID allocation and base58 decoding.
Future measurements should compare order of magnitude and allocation/boundary
behavior on the same host, not treat these nanosecond values as portable SLAs.

## Quality routes

- Property: deterministic Ed25519/X25519 public material round-trips through
  create and parse for 64 generated seed classes.
- Differential: the attributed packet covers normative Ed25519, independent
  direct-X25519 oracle behavior, and strict base64url rejection.
- E2E: an opt-in generic registry creates, resolves, serializes, reparses, and
  checks relationships without I/O or global registration.
- Fuzz: the bounded parser/creator/resolver harness replays checked-in corpus,
  runs a deterministic smoke route, and reserves a 300-second ASan slow route.

The canonical commands and freshness state are in
`quality-evidence-declarations.toml`; this file records reproducible local
source evidence only.

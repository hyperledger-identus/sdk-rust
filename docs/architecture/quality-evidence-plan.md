# Quality evidence plan

Generated deterministically from `docs/architecture/quality-evidence-declarations.toml`.
Commands are declared evidence routes; rendering this plan never executes them.

- Registry version: `1.0.0`
- Evidence evaluated: `2026-09-30`
- Declaration count: `1`

## `did.syntax.quality.v1`

- Capability: `did.syntax`
- Slice: A1 shared compatibility foundation
- Owner: SDK-Rust DID maintainers (issue #501)
- Risk: Malformed or divergent DID syntax handling can break portable identity flows or exhaust bounded parser resources.
- Targets: `package:identus-did`, `host:rust`, `consumer:portable-sdk`
- Vector IDs: `22` local catalog entries
- Limitations: Lexical DID and DID URL syntax only; DID method semantics, resolution, and dereferencing are outside this declaration.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #501 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `max-age-days (8 days)` | #501 |
| `benchmark` | `not-applicable` | `none` | `not-applicable` | `not-applicable` | #501 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #501 |

### `property`

- Command: `cargo test -p identus-did --test did_syntax`
- Budget: Exhaust ASCII bytes 0..127 in each grammar position and verify exact 2,048-byte DID and 4,096-byte DID URL limits.
- Receipt: `source:crates/did/tests/did_syntax.rs`
- Evidence revision: `d3543e3eec42dd519c57a7250950c01fa628145a`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-did.sh soak all`
- Budget: Run each DID and DID URL fuzz target for 300 seconds with max_len=8192, timeout=5, and rss_limit_mb=1024.
- Receipt: `https://github.com/hyperledger-identus/sdk-rust/actions/runs/36517752658`
- Evidence revision: `d27e455901c501d9611abbe0065e6a9b7270dd57`

### `benchmark`

- Rationale: No A1 consumer has declared a DID syntax latency or throughput objective; add a benchmark only with a stable fixture, comparable environment, statistic, sample budget, and threshold.

### `differential`

- Command: `cargo test -p identus-conformance --test cross_language_did_vectors`
- Budget: Evaluate all 22 immutable DID syntax packet cases without network access or an unpublished consumer dependency.
- Receipt: `source:crates/conformance/tests/cross_language_did_vectors.rs`
- Evidence revision: `d3543e3eec42dd519c57a7250950c01fa628145a`

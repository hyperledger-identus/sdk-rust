# Quality evidence plan

Generated deterministically from `docs/architecture/quality-evidence-declarations.toml`.
Commands are declared evidence routes; rendering this plan never executes them.

- Registry version: `1.0.0`
- Evidence evaluated: `2026-10-26`
- Declaration count: `2`

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
- Budget: Exhaust ASCII bytes 0..127 in each grammar position and verify exact 131,072-byte DID and 139,264-byte DID URL limits.
- Receipt: `source:crates/did/tests/did_syntax.rs`
- Evidence revision: `a2683961fb07305d12e96fbcabdb07bb7c0cbab8`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-did.sh soak all`
- Budget: Run each DID and DID URL fuzz target for 300 seconds with max_len=8192, timeout=5, and rss_limit_mb=1024.
- Receipt: `https://github.com/hyperledger-identus/sdk-rust/actions/runs/39364833087`
- Evidence revision: `fb1c7011897ba766f82ea5957feaeb95df179418`

### `benchmark`

- Rationale: No A1 consumer has declared a DID syntax latency or throughput objective; add a benchmark only with a stable fixture, comparable environment, statistic, sample budget, and threshold.

### `differential`

- Command: `cargo test -p identus-conformance --test cross_language_did_vectors`
- Budget: Evaluate all 22 immutable DID syntax packet cases without network access or an unpublished consumer dependency.
- Receipt: `source:crates/conformance/tests/cross_language_did_vectors.rs`
- Evidence revision: `a2683961fb07305d12e96fbcabdb07bb7c0cbab8`

## `did.prism.quality.v1`

- Capability: `did.prism`
- Slice: Portable chain-neutral PRISM DID method
- Owner: SDK-Rust PRISM DID maintainers (issue #521)
- Risk: Wire reconstruction, invalid key profiles, non-atomic updates or unstable projection can split PRISM history and expose unauthorized DID state.
- Targets: `package:identus-prism-did`, `host:rust`, `target:wasm32-unknown-unknown`
- Vector IDs: `4` local catalog entries
- Limitations: The declaration covers portable DID operations and projection only; Cardano indexing, VDR payload semantics, persistence and downstream adoption remain separate capabilities.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #521 |
| `fuzz` | `required` | `slow` | `weekly-or-manual` | `pending` | #521 |
| `benchmark` | `required` | `slow` | `weekly-or-manual` | `pending` | #521 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #521 |

### `property`

- Command: `cargo test -p identus-prism-did property`
- Budget: Generate deterministic create/update/deactivate traces covering ordered rollback, tombstones, active-only 50-item limits and final master-key invariants.
- Receipt: `source:crates/prism-did/src/state.rs`
- Evidence revision: `5c5a90d9f3f22c39f76fbde19239ee451f9fec3f`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-did-prism.sh soak all`
- Budget: Bound raw inputs at the public envelope and identifier ceilings, five seconds per case, 1024 MiB RSS and 300 seconds per target.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #521

### `benchmark`

- Command: `cargo bench -p identus-prism-did`
- Budget: Measure ordinary and maximum-profile parse, signature verification, projection and state transition paths with fixed fixtures and allocation observations.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #521

### `differential`

- Command: `cargo test -p identus-prism-did verifies_pinned_preprod`
- Budget: Verify both pinned public Preprod create envelopes and compare exact operation ID, DID suffix, signature and projected document evidence offline.
- Receipt: `source:crates/prism-did/src/state.rs`
- Evidence revision: `5f4ea13c1635db5466d5fcae353d0897f09abdf5`

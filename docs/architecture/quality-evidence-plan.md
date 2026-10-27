# Quality evidence plan

Generated deterministically from `docs/architecture/quality-evidence-declarations.toml`.
Commands are declared evidence routes; rendering this plan never executes them.

- Registry version: `1.0.0`
- Evidence evaluated: `2026-10-27`
- Declaration count: `7`

## `crypto.quality.v1`

- Capability: `crypto.secp256k1`
- Slice: RustCrypto advisory remediation
- Owner: SDK-Rust cryptography maintainers (issue #525)
- Risk: A curve-family security upgrade can silently change key encodings, signature acceptance, target portability or cross-implementation verification behavior.
- Targets: `package:identus-crypto`, `host:rust`, `consumer:portable-sdk`
- Vector IDs: `1` local catalog entries
- Limitations: This declaration proves the existing SDK cryptographic facade across the RustCrypto 0.14 migration; it does not make upstream tuple-field types stable or add a new algorithm claim.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #525 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `max-age-days (8 days)` | #525 |
| `benchmark` | `satisfied` | `release` | `per-candidate` | `source-bound` | #525 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #525 |

### `property`

- Command: `cargo test -p identus-crypto --all-features --test curves`
- Budget: Exercise deterministic lifecycle and exact encoding round trips for every supported curve with tamper rejection.
- Receipt: `source:crates/crypto/tests/curves.rs`
- Evidence revision: `67c30d599a538b873646a6e857809f2106391bb1`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-crypto.sh soak all`
- Budget: Run public JWK and COSE fuzz targets for 300 seconds each with max_len=8192, timeout=5 and rss_limit_mb=1024.
- Receipt: `https://github.com/hyperledger-identus/sdk-rust/actions/runs/39196675834`
- Evidence revision: `fb1c7011897ba766f82ea5957feaeb95df179418`

### `benchmark`

- Command: `./scripts/benchmark-crypto.sh --samples 20 --output artifacts/crypto-baseline.json`
- Budget: Measure 20 samples for the maintained representative crypto operations and validate the bounded baseline artifact.
- Receipt: `source:scripts/benchmark-crypto.sh`
- Evidence revision: `67c30d599a538b873646a6e857809f2106391bb1`

### `differential`

- Command: `cargo test -p identus-crypto --all-features --test secp256k1_compat`
- Budget: Verify all 50 published JVM signatures plus the deciding bitcoin-transcode fallback regression.
- Receipt: `source:crates/crypto/tests/secp256k1_compat.rs`
- Evidence revision: `67c30d599a538b873646a6e857809f2106391bb1`

## `oid4vp.direct-post-response.quality.v1`

- Capability: `oid4vp.direct-post-response`
- Slice: Bounded OpenID4VP Final plain direct-post success and protocol-error construction
- Owner: SDK-Rust OID4VP maintainers (issue #487)
- Risk: Response construction can detach success or error material from validated request lineage, emit malformed wire values, leak credentials or developer context, or amplify hostile input.
- Targets: `package:identus-oid4vp`, `host:rust`, `consumer:portable-sdk`
- Vector IDs: `2` local catalog entries
- Limitations: This declaration covers plain OpenID4VP Final direct_post success and explicitly selected protocol-error construction only; it does not prove HTTP delivery, presentation authenticity, error truth, End-User interaction, consent, privacy safety, trust, encryption, direct_post.jwt, SIOPv2, DC API or HAIP.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #487 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #487 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #487 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #487 |

### `property`

- Command: `cargo test -p identus-oid4vp --test direct_post_response`
- Budget: Generate bounded success matrices and every closed error code with optional detail forms; round-trip accepted output and reject independent boundary violations.
- Receipt: `source:crates/oid4vp/tests/direct_post_response.rs`
- Evidence revision: `86efc29b517ad4fe53c2a345c6cc47076556509e`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-oid4vp.sh soak direct_post_response`
- Budget: Run both success and error direct-post constructors for 300 seconds with max_len=65536, timeout=5 and rss_limit_mb=1024.
- Receipt: `source:fuzz/fuzz_targets/direct_post_response.rs`
- Evidence revision: `86efc29b517ad4fe53c2a345c6cc47076556509e`

### `benchmark`

- Command: `cargo bench -p identus-oid4vp --bench direct_post_response`
- Budget: Measure the complete deterministic pipeline for ordinary and near-limit success presentations and error descriptions; this diagnostic does not isolate construction or measure allocations.
- Receipt: `source:crates/oid4vp/benches/direct_post_response.rs`
- Evidence revision: `86efc29b517ad4fe53c2a345c6cc47076556509e`

### `differential`

- Command: `cargo test -p identus-oid4vp --test direct_post_response`
- Budget: Reproduce attributed Final success and error response shapes and round-trip form bodies through an independent strict decoder.
- Receipt: `source:docs/conformance/fixtures/oid4vp-direct-post/v1/vectors.json`
- Evidence revision: `86efc29b517ad4fe53c2a345c6cc47076556509e`

## `oid4vp.direct-post-response.encrypted.quality.v1`

- Capability: `oid4vp.direct-post-response`
- Slice: Bounded OpenID4VP Final direct_post.jwt response construction through an SDK-owned JWE capability port
- Owner: SDK-Rust OID4VP and JOSE maintainers (issue #535)
- Risk: Encrypted response construction can select an attacker-controlled or ambiguous key, detach plaintext from verified request lineage, expose sensitive response material, accept malformed compact output, or amplify hostile metadata.
- Targets: `package:identus-jose`, `package:identus-oid4vp`, `host:rust`, `consumer:portable-sdk`
- Vector IDs: `1` local catalog entries
- Limitations: This declaration covers Final direct_post.jwt metadata selection, bounded SDK-owned P-256 ECDH-ES/A128GCM port input, opaque compact-output validation and response-only form construction; it does not prove a production JWE engine, HTTP delivery, wallet authentication, consent, trust, presentation verification or foreign DTO shape.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `required` | `focused` | `on-change` | `pending` | #535 |
| `fuzz` | `required` | `slow` | `weekly-or-manual` | `pending` | #535 |
| `benchmark` | `required` | `slow` | `weekly-or-manual` | `pending` | #535 |
| `differential` | `required` | `fast` | `per-pull-request` | `pending` | #535 |

### `property`

- Command: `cargo test -p identus-jose --test jwe_port && cargo test -p identus-oid4vp --test encrypted_direct_post_response`
- Budget: Generate bounded public keys, metadata orderings, optional fields, presentation maps and compact segments; preserve exact selection and form round trips while rejecting every independent boundary violation.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #535

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-oid4vp.sh soak encrypted_direct_post_response`
- Budget: Run metadata selection and returned compact-material validation for 300 seconds with max_len=65536, timeout=5 and rss_limit_mb=1024.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #535

### `benchmark`

- Command: `cargo bench -p identus-oid4vp --bench encrypted_direct_post_response`
- Budget: Measure ordinary and near-limit verified request metadata selection, response JSON construction, one port invocation and compact/form validation without claiming cryptographic engine performance.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #535

### `differential`

- Command: `cargo test -p identus-oid4vp --test encrypted_direct_post_response`
- Budget: Reproduce the attributed Final encrypted-response shape, assert the exact SDK port request, and decrypt one isolated P-256/A128GCM oracle artifact outside the production dependency graph.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #535

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
- Vector IDs: `3` local catalog entries
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
- Evidence revision: `5ce2deaf9ca61642fc8b81ee4baa593bababe98f`

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

- Command: `cargo test -p identus-prism-did verifies_pinned_preprod && cargo test -p identus-conformance --test cross_language_prism_vectors`
- Budget: Verify both pinned public Preprod create envelopes and compare exact operation ID, DID suffix, signature and projected document evidence offline.
- Receipt: `source:crates/conformance/tests/cross_language_prism_vectors.rs`
- Evidence revision: `5ce2deaf9ca61642fc8b81ee4baa593bababe98f`

## `did.prism.vdr-indexing.quality.v1`

- Capability: `did.prism.vdr-indexing`
- Slice: Portable chain-neutral PRISM VDR and deterministic indexer cores
- Owner: SDK-Rust PRISM VDR and indexer maintainers (issue #522)
- Risk: Unbounded ancestry, ambiguous replay, partial commits, or wire drift can authorize invalid storage history or split deterministic index state.
- Targets: `package:identus-prism-vdr`, `package:identus-prism-indexer`, `host:rust`, `target:wasm32-unknown-unknown`, `target:aarch64-apple-ios`, `target:aarch64-linux-android`
- Vector IDs: `2` local catalog entries
- Limitations: Effect-neutral VDR and indexing semantics only; no Cardano source, persistence, SQL, HTTP, scheduler, retry loop, device runtime, FFI, deployment, or downstream NeoPRISM adoption is claimed.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #522 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `max-age-days (8 days)` | #522 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #522 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #522 |

### `property`

- Command: `cargo test -p identus-prism-vdr property && cargo test -p identus-prism-indexer property`
- Budget: Exercise every accepted ancestry length from zero through 200 predecessor hops and all 24 permutations of four framed transaction records.
- Receipt: `source:docs/architecture/prism-vdr-indexer-quality-baseline.md`
- Evidence revision: `fd808b1df5e25923d60eea5f8d276e6c7a00ac27`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-prism-vdr-indexer.sh soak all`
- Budget: Run each target for 300 seconds with 139264-byte inputs, five seconds per case and 1024 MiB RSS; promote failures into checked-in regression seeds.
- Receipt: `https://github.com/hyperledger-identus/sdk-rust/actions/runs/40463418925`
- Evidence revision: `fd808b1df5e25923d60eea5f8d276e6c7a00ac27`

### `benchmark`

- Command: `cargo bench -p identus-prism-indexer --bench directional`
- Budget: Measure VDR parse, 100-node traversal, 128-record digest construction and same-transaction DID plus VDR planning over 101 samples.
- Receipt: `source:docs/architecture/prism-vdr-indexer-quality-baseline.md`
- Evidence revision: `fd808b1df5e25923d60eea5f8d276e6c7a00ac27`

### `differential`

- Command: `cargo test -p identus-prism-vdr accepts_the_schema_default_empty_nonce && cargo test -p identus-prism-indexer canonical_transition_digest_matches_the_portable_vector && cargo test -p identus-prism-indexer neoprism_shaped_metadata`
- Budget: Compare exact empty-nonce storage parsing, the canonical transition digest and NeoPRISM-shaped transaction metadata mapping without donor dependencies.
- Receipt: `source:docs/conformance/fixtures/did-prism-vdr-indexing/v1/vectors.json`
- Evidence revision: `fd808b1df5e25923d60eea5f8d276e6c7a00ac27`

## `did.peer.profiles.quality.v1`

- Capability: `did.peer.profiles`
- Slice: Bounded numalgo-4 and explicit numalgo-2 compatibility profiles
- Owner: SDK-Rust peer DID maintainers (issue #523)
- Risk: Ambiguous encodings, malformed public material, secret-bearing input or unbounded projection can split peer DID identity state or expose private material in public identifiers.
- Targets: `package:identus-did-peer`, `host:rust`, `target:wasm32-unknown-unknown`, `target:aarch64-apple-ios`, `target:aarch64-linux-android`
- Vector IDs: `2` local catalog entries
- Limitations: Effect-free public-material construction, parsing and local resolution only; no custody, private keys, networking, persistence, DIDComm engine, language bindings or historical numalgo 0, 1 and 3 support is claimed.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #523 |
| `fuzz` | `required` | `slow` | `weekly-or-manual` | `pending` | #523 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #523 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #523 |

### `property`

- Command: `cargo test -p identus-did-peer property`
- Budget: Exercise every supported numalgo-2 purpose and deterministic numalgo-4 JSON sizes from zero through 512 bytes.
- Receipt: `source:docs/architecture/did-peer-quality-baseline.md`
- Evidence revision: `fcf7083d712baa1c4ff45c2933b7bec48c44531e`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-did-peer.sh soak`
- Budget: Run both peer DID profiles for 300 seconds with 131072-byte inputs, five seconds per case and 1024 MiB RSS.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #523

### `benchmark`

- Command: `cargo bench -p identus-did-peer --bench directional`
- Budget: Measure numalgo-4 create, numalgo-4 parse/resolve and numalgo-2 parse/resolve over 101 samples.
- Receipt: `source:docs/architecture/did-peer-quality-baseline.md`
- Evidence revision: `fcf7083d712baa1c4ff45c2933b7bec48c44531e`

### `differential`

- Command: `cargo test -p identus-did-peer --test conformance`
- Budget: Compare exact numalgo-4 output and numalgo-2 parse/create/resolve behavior against two pinned public oracle cases.
- Receipt: `source:docs/conformance/fixtures/did-peer/v1/vectors.json`
- Evidence revision: `fcf7083d712baa1c4ff45c2933b7bec48c44531e`

# Quality evidence plan

Generated deterministically from `docs/architecture/quality-evidence-declarations.toml`.
Commands are declared evidence routes; rendering this plan never executes them.

- Registry version: `1.0.0`
- Evidence evaluated: `2026-10-28`
- Declaration count: `15`

## `oid4vp.redirect-response.quality.v1`

- Capability: `oid4vp.direct-post-response`
- Slice: Bounded redirect response modes and direct-post acknowledgement
- Owner: SDK-Rust OID4VP maintainers (issue #546)
- Risk: Redirect encoding or acknowledgement parsing can lose request lineage, leak presentation data, create ambiguous destinations, or authorize effects.
- Targets: `package:identus-oid4vp`, `host:rust`, `consumer:portable-sdk`
- Vector IDs: `1` local catalog entries
- Limitations: Inert protocol material only; no HTTP, browser navigation, HTML, leakage policy, response-code redemption, entropy proof, storage, publication, or certification claim.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #546 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #546 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #546 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #546 |

### `property`

- Command: `cargo test -p identus-oid4vp --test redirect_response property`
- Budget: Sweep every supported mode, query/fragment output boundaries, form-post collision bounds, all reserved fields, and the exact acknowledgement body boundary.
- Receipt: `source:crates/oid4vp/tests/redirect_response.rs`
- Evidence revision: `30b90193044bdabdafb97a4f309c249e30f275cc`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-oid4vp.sh soak direct_post_response`
- Budget: Run for 300 seconds with max_len=131072, timeout=5 and rss_limit_mb=1024.
- Receipt: `source:fuzz/fuzz_targets/direct_post_response.rs`
- Evidence revision: `30b90193044bdabdafb97a4f309c249e30f275cc`

### `benchmark`

- Command: `cargo bench -p identus-oid4vp --bench direct_post_response`
- Budget: Measure ordinary and near-limit redirect construction plus acknowledgement parsing.
- Receipt: `source:crates/oid4vp/benches/direct_post_response.rs`
- Evidence revision: `30b90193044bdabdafb97a4f309c249e30f275cc`

### `differential`

- Command: `cargo test -p identus-oid4vp --test redirect_response`
- Budget: Compare clean-room fragment, query, form_post, and acknowledgement cases to pinned rules.
- Receipt: `source:docs/conformance/fixtures/oid4vp-redirect-response/v1/vectors.json`
- Evidence revision: `30b90193044bdabdafb97a4f309c249e30f275cc`

## `oid4vp.direct-post-response.request-object-security.quality.v1`

- Capability: `oid4vp.direct-post-response`
- Slice: Bounded Request Object audience, freshness and atomic replay evidence
- Owner: SDK-Rust OID4VP maintainers (issue #544)
- Risk: A signed Request Object can target the wrong Wallet, be accepted outside its validity window, or be replayed before typed authorization routing.
- Targets: `package:identus-oid4vp`, `host:rust`, `consumer:portable-sdk`
- Vector IDs: `1` local catalog entries
- Limitations: This declaration proves local Final audience, integral temporal policy, atomic replay decision and typed state lineage; it does not prove verifier trust, persistence, transport, consent, credential validity or product policy.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #544 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #544 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #544 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #544 |

### `property`

- Command: `cargo test -p identus-oid4vp --test request_object_security && cargo test -p identus-oid4vp --doc`
- Budget: Sweep generated age/skew/expiry/not-before boundaries and exercise a stateful atomic replay decision with consuming-state compile/runtime canaries.
- Receipt: `source:crates/oid4vp/tests/request_object_security.rs`
- Evidence revision: `4fc60ef685a035e56e9b79ad73c41924047e95ec`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-oid4vp.sh soak request_object_security`
- Budget: Run the bounded claims and policy path for 300 seconds with max_len=131072, timeout=5 and rss_limit_mb=1024.
- Receipt: `source:fuzz/fuzz_targets/request_object_security.rs`
- Evidence revision: `643773773b40a3571f94e676a22d01cd43393aff`

### `benchmark`

- Command: `cargo bench -p identus-oid4vp --bench request_object_security`
- Budget: Measure deterministic ordinary and near-limit local security transitions over a stable sample set.
- Receipt: `source:crates/oid4vp/benches/request_object_security.rs`
- Evidence revision: `21f16a0fc91244a0ff6304f14d2b80cb6e7c2fab`

### `differential`

- Command: `cargo test -p identus-oid4vp --test direct_post_response packet_executes_full_invocation_jar_dcql_and_response_lineage`
- Budget: Execute the pinned clean-room Final/RFC packet and compare normalized result classes rather than foreign DTOs or error strings.
- Receipt: `source:docs/conformance/fixtures/oid4vp-direct-post/v1/vectors.json`
- Evidence revision: `547ef60adea10a8d97c2948043e51c82f793d9be`

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
| `fuzz` | `required` | `slow` | `weekly-or-manual` | `pending` | #525 |
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
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #525

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
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #535 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #535 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #535 |
| `differential` | `required` | `fast` | `per-pull-request` | `pending` | #535 |

### `property`

- Command: `cargo test -p identus-jose --test jwe_port && cargo test -p identus-oid4vp --test encrypted_direct_post_response`
- Budget: Exercise deterministic key-identifier and plaintext boundaries, required and optional metadata, wrong-mode constructors, private or ambiguous keys, unsupported algorithms, presentation lineage, every compact-header rule and segment grammar.
- Receipt: `source:crates/oid4vp/tests/encrypted_direct_post_response.rs`
- Evidence revision: `15d500b639c67e824e1ac29459a9038730c0cb1b`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-oid4vp.sh soak encrypted_direct_post_response`
- Budget: Run metadata selection and returned compact-material validation for 300 seconds with max_len=65536, timeout=5 and rss_limit_mb=1024.
- Receipt: `source:fuzz/fuzz_targets/encrypted_direct_post_response.rs`
- Evidence revision: `dd1996086d72510ae9e9c806886823614a8a1567`

### `benchmark`

- Command: `cargo bench -p identus-oid4vp --bench encrypted_direct_post_response`
- Budget: Measure ordinary and near-limit verified request metadata selection, response JSON construction, one port invocation and compact/form validation without claiming cryptographic engine performance.
- Receipt: `source:crates/oid4vp/benches/encrypted_direct_post_response.rs`
- Evidence revision: `dd1996086d72510ae9e9c806886823614a8a1567`

### `differential`

- Command: `cargo test -p identus-oid4vp --test encrypted_direct_post_response`
- Budget: Reproduce the attributed Final encrypted-response shape, then independently encrypt and decrypt the exact SDK request with a qualified adapter on native and portable targets without comparing randomized ciphertext bytes.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #538

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

## `siopv2.did-authentication.quality.v1`

- Capability: `siopv2.did-authentication`
- Slice: Pinned SIOPv2 draft-13 DID-subject direct_post protocol core
- Owner: SDK-Rust OpenID maintainers (issue #514)
- Risk: Ambiguous request binding, signature lineage, token claims or lifecycle retries can authenticate the wrong relying party, disclose identity material or duplicate a response.
- Targets: `package:identus-siopv2`, `host:rust`, `target:wasm32-unknown-unknown`, `target:aarch64-apple-ios`, `target:aarch64-linux-android`
- Vector IDs: `1` local catalog entries
- Limitations: Experimental draft-13 DID-subject direct_post profile only; no transport, durable replay storage, trust, custody, UI, DID resolution, language adapter, publication or certification claim.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `required` | `focused` | `on-change` | `pending` | #514 |
| `fuzz` | `required` | `slow` | `weekly-or-manual` | `pending` | #514 |
| `benchmark` | `required` | `slow` | `weekly-or-manual` | `pending` | #514 |
| `differential` | `required` | `fast` | `per-pull-request` | `pending` | #514 |

### `property`

- Command: `cargo test -p identus-siopv2 --test property`
- Budget: Exercise every accepted event-ID length plus deterministic terminal idempotency, conflict rejection and state preservation.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #514

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-siopv2.sh soak`
- Budget: Run bounded SIOPv2 parsing and lifecycle inputs for 300 seconds with finite input, timeout and RSS limits.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #514

### `benchmark`

- Command: `cargo bench -p identus-siopv2 --bench protocol`
- Budget: Measure deterministic ordinary signed-request verification over a stable sample set; response planning remains an unmeasured follow-up.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #514

### `differential`

- Command: `cargo test -p identus-siopv2 --test conformance`
- Budget: Execute the pinned clean-room draft-13 vector and independently verify normalized request/response claims; foreign implementations remain research oracles.
- Receipt: `pending`
- Evidence revision: `pending`
- Delivery debt: #514

## `oid4vp.direct-post-response.verifier-authorization.quality.v1`

- Capability: `oid4vp.direct-post-response`
- Slice: Final client identifier schemes, verifier metadata and exact-key trust policy
- Owner: SDK-Rust OID4VP maintainers (issue #545)
- Risk: A syntactically valid signed request can route with a key unauthorized for its full client identifier or with malformed or mis-sourced verifier evidence.
- Targets: `package:identus-oid4vp`, `package:identus-jose`, `host:rust`, `consumer:portable-sdk`
- Vector IDs: `1` local catalog entries
- Limitations: The declaration proves bounded classification, evidence lineage and one caller-owned trust decision; it does not prove any concrete DID, PKI, Federation or attestation adapter, transport, consent or product policy.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #545 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #545 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #545 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #545 |

### `property`

- Command: `cargo test -p identus-oid4vp --test verifier_authorization`
- Budget: Exercise every Final scheme, first-colon disposition, exact key lineage, metadata source rule, verifier-info reference and independent collection limit.
- Receipt: `source:crates/oid4vp/tests/verifier_authorization.rs`
- Evidence revision: `0c194d4a365ae9dc8b51efe2f8abb40eb54ac6ba`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-oid4vp.sh soak request_object_security`
- Budget: Run security and verifier authorization over arbitrary bounded claims and metadata for 300 seconds with max_len=131072, timeout=5 and rss_limit_mb=1024.
- Receipt: `source:fuzz/fuzz_targets/request_object_security.rs`
- Evidence revision: `0c194d4a365ae9dc8b51efe2f8abb40eb54ac6ba`

### `benchmark`

- Command: `cargo bench -p identus-oid4vp --bench request_object_security`
- Budget: Measure ordinary and near-limit request security followed by exact verifier authorization with one deterministic accepting port.
- Receipt: `source:crates/oid4vp/benches/request_object_security.rs`
- Evidence revision: `0c194d4a365ae9dc8b51efe2f8abb40eb54ac6ba`

### `differential`

- Command: `cargo test -p identus-oid4vp --test verifier_authorization`
- Budget: Compare every Final-defined prefix and metadata source disposition against the pinned normative matrix using normalized SDK states and errors.
- Receipt: `source:crates/oid4vp/tests/verifier_authorization.rs`
- Evidence revision: `0c194d4a365ae9dc8b51efe2f8abb40eb54ac6ba`

## `oid4vp.direct-post-response.transaction-format.quality.v1`

- Capability: `oid4vp.direct-post-response`
- Slice: Bounded transaction data and credential-format capability ports
- Owner: SDK-Rust OID4VP maintainers (issue #547)
- Risk: Unbounded or detached transaction data can amplify parser work, bypass credential selection, disclose authorization context, or couple the protocol core to one proof engine.
- Targets: `package:identus-oid4vp`, `host:rust`, `consumer:portable-sdk`
- Vector IDs: `3` local catalog entries
- Limitations: This declaration proves protocol envelope, resource, capability and selected-query linkage only; it does not create or verify SD-JWT VC, mdoc, W3C VC or extension proofs, execute transport, authorize disclosure, or certify a wallet.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #547 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #547 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #547 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #547 |

### `property`

- Command: `cargo test -p identus-oid4vp --test authorization_request transaction`
- Budget: Exercise exact unpadded base64url, resource boundaries, unique references, alternative selections, every selected compatible reference, response-result linkage and error-response recovery.
- Receipt: `source:crates/oid4vp/tests/authorization_request.rs`
- Evidence revision: `58057823d2c05830a1fb05271fbfad5f05c4b485`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-oid4vp.sh soak transaction_data`
- Budget: Run the bounded transaction parser and linkage path for 300 seconds with max_len=131072, timeout=5 and rss_limit_mb=1024.
- Receipt: `source:fuzz/fuzz_targets/transaction_data.rs`
- Evidence revision: `58057823d2c05830a1fb05271fbfad5f05c4b485`

### `benchmark`

- Command: `cargo bench -p identus-oid4vp --bench direct_post_response`
- Budget: Measure deterministic ordinary and maximum-string transaction validation, binding and response construction over a stable sample set.
- Receipt: `source:crates/oid4vp/benches/direct_post_response.rs`
- Evidence revision: `58057823d2c05830a1fb05271fbfad5f05c4b485`

### `differential`

- Command: `cargo test -p identus-oid4vp --test authorization_request transaction_format_packet`
- Budget: Execute clean-room SD-JWT-shaped, mdoc-shaped and generic-extension packets and compare exact retained encoding plus stable binding results.
- Receipt: `source:docs/conformance/fixtures/oid4vp-transaction-format/v1/vectors.json`
- Evidence revision: `58057823d2c05830a1fb05271fbfad5f05c4b485`

## `did.key.profile.quality.v1`

- Capability: `did.key.profile`
- Slice: Bounded effect-free did:key creation, parsing and local resolution
- Owner: SDK-Rust DID maintainers (issue #557)
- Risk: Non-canonical multicodec or curve encodings, unusable public material, unbounded base58 work or relationship drift can split portable identity state or expose key content.
- Targets: `package:identus-did-key`, `host:rust`, `target:wasm32-unknown-unknown`, `target:aarch64-apple-ios`, `target:aarch64-linux-android`
- Vector IDs: `6` local catalog entries
- Limitations: Effect-free direct public-key creation, parsing and local resolution only; no private keys, generation, derivation, signing, custody, network resolution, persistence, runtime, bindings or global registry is claimed.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #557 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #557 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #557 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #557 |

### `property`

- Command: `cargo test -p identus-did-key --all-features`
- Budget: Sweep 64 deterministic Ed25519 and X25519 seed classes, then create, opt-in resolve, serialize and reparse one complete generic DID document.
- Receipt: `source:crates/did-key/tests/conformance.rs`
- Evidence revision: `9d032d8397e8afb1ed7133b95119ab522755e25f`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-did-key.sh soak`
- Budget: Run bounded parse, canonical recreation and local resolution for 300 seconds with max_len=144, timeout=5 and rss_limit_mb=1024.
- Receipt: `source:fuzz/fuzz_targets/did_key.rs`
- Evidence revision: `9d032d8397e8afb1ed7133b95119ab522755e25f`

### `benchmark`

- Command: `cargo bench -p identus-did-key --bench directional`
- Budget: Measure create, parse, local resolve and early oversized rejection over 101 samples with 500 iterations per sample.
- Receipt: `source:docs/architecture/did-key-quality-baseline.md`
- Evidence revision: `9d032d8397e8afb1ed7133b95119ab522755e25f`

### `differential`

- Command: `cargo test -p identus-did-key --test conformance attributed_vector_packet_matches_selected_profile`
- Budget: Compare exact Ed25519 and X25519 DIDs, semantic relationship counts, strict encoding/category rejection and the public identifier ceiling against the attributed packet.
- Receipt: `source:docs/conformance/fixtures/did-key/v1/vectors.json`
- Evidence revision: `9d032d8397e8afb1ed7133b95119ab522755e25f`

## `did.web.profile.quality.v1`

- Capability: `did.web.profile`
- Slice: Bounded did:web parsing, request derivation, injected retrieval and exact document binding
- Owner: SDK-Rust DID maintainers (issue #559)
- Risk: Ambiguous authority parsing, unbounded remote data, transport-policy leakage, or document mismatch can redirect identity resolution or amplify work.
- Targets: `package:identus-did-web`, `host:rust`, `target:wasm32-unknown-unknown`, `target:aarch64-apple-ios`, `target:aarch64-linux-android`
- Vector IDs: `1` local catalog entries
- Limitations: Portable method core only; concrete HTTP, DNS, TLS, redirect, proxy, cache, retry, cancellation, SSRF policy, registration, persistence, bindings, consumer migration and publication remain separate.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #559 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #559 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #559 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #559 |

### `property`

- Command: `cargo test -p identus-did-web --all-features`
- Budget: Sweep deterministic valid and invalid authority, port and path classes plus complete in-memory fetch-port E2E.
- Receipt: `source:crates/did-web/tests/conformance.rs`
- Evidence revision: `9ccffe38ca4f91ec0a36acb3729edd4814bd2d5e`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-did-web.sh soak`
- Budget: Run bounded parse and response binding for 300 seconds with timeout 5 seconds and RSS limit 1024 MiB.
- Receipt: `source:fuzz/fuzz_targets/did_web.rs`
- Evidence revision: `9ccffe38ca4f91ec0a36acb3729edd4814bd2d5e`

### `benchmark`

- Command: `cargo bench -p identus-did-web --bench directional`
- Budget: Measure parse/derive, response binding, and early oversized rejection over deterministic samples.
- Receipt: `source:docs/architecture/did-web-quality-baseline.md`
- Evidence revision: `9ccffe38ca4f91ec0a36acb3729edd4814bd2d5e`

### `differential`

- Command: `cargo test -p identus-did-web --test conformance pinned_url_mapping_examples_match_the_selected_profile`
- Budget: Compare exact bare, path and encoded-port URLs plus exact document ID binding against attributed pinned examples.
- Receipt: `source:docs/conformance/fixtures/did-web/v1/vectors.json`
- Evidence revision: `9ccffe38ca4f91ec0a36acb3729edd4814bd2d5e`

## `did.jwk.profile.quality.v1`

- Capability: `did.jwk.profile`
- Slice: Bounded effect-free did:jwk creation, duplicate-safe parsing and local resolution
- Owner: SDK-Rust DID maintainers (issue #558)
- Risk: Ambiguous JSON, invalid public material, unbounded decode work or relationship drift can split portable identity state or expose content.
- Targets: `package:identus-did-jwk`, `host:rust`, `target:wasm32-unknown-unknown`, `target:aarch64-apple-ios`, `target:aarch64-linux-android`
- Vector IDs: `2` local catalog entries
- Limitations: Effect-free public-JWK creation, parsing and local resolution only; no private keys, key_ops policy, generation, signing, custody, network resolution, persistence, runtime, bindings, consumer migration or publication.

| Class | Disposition | Lane | Cadence | Freshness | Owner issue |
| --- | --- | --- | --- | --- | ---: |
| `property` | `satisfied` | `focused` | `on-change` | `source-bound` | #558 |
| `fuzz` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #558 |
| `benchmark` | `satisfied` | `slow` | `weekly-or-manual` | `source-bound` | #558 |
| `differential` | `satisfied` | `fast` | `per-pull-request` | `source-bound` | #558 |

### `property`

- Command: `cargo test -p identus-did-jwk --all-features`
- Budget: Sweep 64 deterministic Ed25519 and X25519 seed classes, retain one alternate JSON identifier exactly, and complete one opt-in registry document round trip.
- Receipt: `source:crates/did-jwk/tests/conformance.rs`
- Evidence revision: `cca5a33cc861436ae6830dd3e42ba08d9424b074`

### `fuzz`

- Command: `nix develop .#fuzz --command ./scripts/fuzz-did-jwk.sh soak`
- Budget: Run bounded parse, exact retention and local resolution for 300 seconds with max_len=22000, timeout=5 and rss_limit_mb=1024.
- Receipt: `source:fuzz/fuzz_targets/did_jwk.rs`
- Evidence revision: `cca5a33cc861436ae6830dd3e42ba08d9424b074`

### `benchmark`

- Command: `cargo bench -p identus-did-jwk --bench directional`
- Budget: Measure create, parse, local resolve and early oversized rejection over 101 samples with 500 iterations per sample.
- Receipt: `source:docs/architecture/did-jwk-quality-baseline.md`
- Evidence revision: `cca5a33cc861436ae6830dd3e42ba08d9424b074`

### `differential`

- Command: `cargo test -p identus-did-jwk --test conformance pinned_normative_examples_match_the_selected_profile`
- Budget: Compare exact pinned P-256 and X25519 DIDs, validated curves and semantic relationship counts against the attributed packet.
- Receipt: `source:docs/conformance/fixtures/did-jwk/v1/vectors.json`
- Evidence revision: `cca5a33cc861436ae6830dd3e42ba08d9424b074`

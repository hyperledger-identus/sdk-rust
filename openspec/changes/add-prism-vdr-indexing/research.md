# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-27
Source retrieval date: 2026-10-27
Research blockers: none

## Problem and existing implementation

The current implementation at SDK-Rust revision
`67c30d599a538b873646a6e857809f2106391bb1` recognizes and bounds PRISM VDR
operation envelopes but intentionally leaves their payload opaque. NeoPRISM at
revision `a2ddbc5a8a316841e2c8df3831e7904fef01c356` contains working storage
classification, root traversal and transaction grouping, but its current
implementation couples reusable rules to Tokio channels, async traits, Cardano
metadata, UUID records, tracing, SQL-oriented repositories and `anyhow`.

The consumer need is deterministic classification and atomic persistence, not
that concrete effect graph. The SDK facade must own idiomatic Rust values and
stable redacted errors while adapters own runtime and infrastructure choices.

## Normative sources

- PRISM protobuf storage schema already pinned in `identus-prism-did` from
  `hyperledger-identus/neoprism@a2ddbc5a8a316841e2c8df3831e7904fef01c356`.
- NeoPRISM donor paths `lib/did-prism-indexer` and
  `lib/did-prism-ledger` at that exact revision.
- ADR 0174 and the accepted architecture decision on issue #493.
- Canonical SDK specification `openspec/specs/prism-vdr-indexing/spec.md`.

Primary source URLs are
https://github.com/hyperledger-identus/neoprism/tree/a2ddbc5a8a316841e2c8df3831e7904fef01c356/lib/did-prism-indexer
and
https://github.com/hyperledger-identus/sdk-rust/blob/67c30d599a538b873646a6e857809f2106391bb1/docs/adr/0174-separate-peer-and-prism-did-method-ownership.md.
The pinned source revision is immutable evidence. The PRISM storage protocol is
draft/version-sensitive; a new wire revision triggers a new compatibility
decision rather than silent permissive decoding.

The donor and SDK have Apache-2.0 license terms; license and provenance are
recorded separately. No
new third-party runtime crate is selected: exact versions and features are the
workspace `identus-*` crates plus existing `prost` decoding inside the method
crate. The final direct and resolved dependency cone must be captured from the
lockfile. The donor's `async-trait`, Tokio, chrono, uuid, anyhow, tracing, SQLx,
Pallas and Cardano feature dependencies are explicitly rejected.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Separate `identus-prism-vdr` | `adopt` | Cohesive method-specific wire/state/root responsibility. | Another method proves a genuinely shared VDR semantic core. |
| Separate `identus-prism-indexer` | `adopt` | Deterministic batch planning is reusable without source/storage effects. | The API cannot remain executor- and chain-neutral. |
| NeoPRISM indexing behavior | `conditional-adopt` | Preserve operation classification and same-batch/cross-batch root behavior, but correct silent ignore/partial-progress cases. | Donor behavior conflicts with normative protocol evidence. |
| One-call atomic commit port | `adopt` | Avoids exposing transaction lifetimes and prevents partial SDK-visible writes. | A supported adapter cannot provide atomic compare-and-set semantics. |
| Sync bounded core API | `adopt` | Portable to native/WASM/mobile; caller owns async scheduling and cancellation. | Evidence proves unavoidable internal concurrency. |
| Tokio/async-channel engine | `not-adopt` | Adds executor ownership and hidden backpressure to a portable crate. | Separately approved runtime crate owns orchestration. |
| Donor repository traits | `not-adopt` | Leak UUID, Cardano metadata, `String`/`anyhow` errors and SQL-shaped queries. | Never without a compatibility adapter outside the canonical facade. |
| Donor implementation as differential oracle | `oracle` | Useful positive vectors and grouping behavior, not architecture authority. | Donor is deprecated after independently pinned conformance replaces it. |
| Third-party universal VDR crate | `not-adopt` | No reviewed crate matches PRISM storage wire and finite root semantics. | A maintained crate demonstrates exact compatibility and smaller risk. |

NeoPRISM's V1 processor authorizes storage operations with an active
secp256k1 `VdrKey` over the exact operation bytes. The storage protobuf comments
still say “issuing key”; that text is stale relative to the dedicated key usage
in the schema/domain and executable processor. SDK-Rust therefore selects
`KeyUsage::Vdr`, records the deviation, and requires a DID-state snapshot at the
operation's exact ordered position.

### Reusable-module assessment: VDR

- Cohesion: high after isolating create/update/deactivate payload decoding,
  predecessor identity and finite root traversal.
- Coupling: low with one inward dependency on `identus-prism-did`; protobuf
  generated structs remain private to the owning method boundary.
- Orthogonality: no cursor, batching, chain metadata or persistence behavior.
- Portability: pure Rust, no executor, OS, time, network or database service.
- Security: explicit sizes/depth, cycle detection, exact hashes and redaction.
- Decision: `conditional-adopt` donor behavior, redesigned as an SDK crate.

### Reusable-module assessment: indexer

- Cohesion: high when limited to ordered batch planning, replay/cursor checks
  and a single atomic commit request.
- Coupling: depends only on VDR/method/core values; source/store ports use SDK
  identifiers rather than Cardano/UUID/SQL types.
- Orthogonality: chain fetch, persistence, retries, cancellation and scheduling
  remain caller-owned.
- Portability: synchronous finite calls work on native, WASM, iOS and Android.
- Security: finite batch/operation work, no partial commit, stable errors.
- Decision: `conditional-adopt` donor rules and reject its runtime graph.

## Compatibility and dependency evidence

The public and wire change is additive. `identus-prism-did` remains the exact
wire owner; the VDR crate consumes its bounded payload accessors and returns
validated domain values. The indexer facade accepts SDK-owned ordered records
and emits a deterministic commit plan. No TypeScript, donor repository or SQL
DTO crosses the public boundary.

The intended direct dependency cone is:

`identus-prism-indexer -> identus-prism-vdr -> identus-prism-did ->
identus-did/identus-crypto/identus-core`.

No new external dependency is expected. Existing `prost 0.14.4` has MSRV 1.85,
below the repository MSRV 1.89 and Rust 1.98 etalon. Target evidence will cover
native, `wasm32-unknown-unknown`, iOS and Android library compilation. The
facade boundary uses owned or borrowed plain Rust domain types and never exposes
generated protobuf messages, async runtimes or persistence handles.

`cargo search identus-prism-vdr` and `cargo search identus-prism-indexer`
returned no matches on 2026-10-27. This is availability evidence only; Identus
publication authority and ownership remain release-time checks.

## Security, privacy and maintenance evidence

The implementation adds no unsafe or native code. Supply-chain surface should
remain unchanged apart from the two local packages. Every untrusted byte and
collection boundary is finite; batch-local maps are bounded by batch capacity,
and ancestry tracks visited fixed-size operation hashes up to the depth limit.
Malformed, unauthorized, orphan, cyclic and too-deep chains fail
deterministically. Candidate
plans are fully computed before commit, preventing partial persistence after a
late validation error.

The donor's root traversal is not sufficient as an update-state check: a
root-reachable predecessor may no longer be the current entry head. SDK-Rust
adds an explicit committed plus batch-local head invariant and rejects forks.
The same entry state retains a terminal bit because donor protocol state rejects
updates and repeated deactivation after revocation; a terminal hash is not a
new mutable head.
Replay identity is a domain-separated digest of the complete exact ordered
transition, not the opaque transaction label alone. A committed-receipt probe
must run before state-dependent planning; otherwise an ambiguous retry can be
rejected as a duplicate create or stale predecessor after its first commit.
The atomic commit repeats the digest check to close the probe/commit race.

Public errors expose stable categories and bounded indices, never raw payloads,
DIDs, keys, signatures or adapter diagnostics. The SDK owns neither secrets
nor personal-data storage. Maintenance posture is two narrow independently
versioned crates with in-memory conformance adapters. Release remains disabled
until quality evidence and authority receipts are complete. Rollback before
publication is removal of additive packages; after publication, compatibility
rules apply.

## Rejected or deferred candidates

Cardano sources, Blockfrost, db-sync, Oura, cardano-wallet, SQL, HTTP, CLI,
daemon lifecycle, retry loops, background channels, wall-clock timestamps and
NeoPRISM mutation are deferred downstream. Splitting one transaction across
multiple commits, silently ignoring orphan storage children and continuing
after insertion failure are rejected because they weaken atomicity or hide
invalid state. A universal VDR abstraction is rejected as premature.

## Open questions and blockers

No research blocker remains. Concrete default/max sizes will be declared in the
API ADR and proven by tests before implementation activation. The design uses
batch capacity as backpressure rather than an SDK-owned channel. Cancellation
belongs to the caller between finite calls; adapters may cancel their own I/O.
The reconsideration trigger is evidence from a supported downstream adapter
that these boundaries cannot preserve atomic ordered replay without importing
an executor or concrete storage concern.

## Evidence commands

Planning commands are `scripts/factory research-ready add-prism-vdr-indexing`,
`scripts/factory constraints-ready add-prism-vdr-indexing`, OpenSpec strict
validation and `scripts/factory preflight add-prism-vdr-indexing --issue 522
--write`. Implementation commands will include focused/all-feature tests,
clippy, rustdoc, dependency guards, property/fuzz/differential vectors,
directional benchmarks and native/WASM/mobile checks. Official Nix, exact-head
independent CLI review and protected CI are unrun and are not claimed here.

Independent review uses Claude when available. If Claude is missing,
unauthenticated or quota-limited, Codex CLI runs immediately against the same
exact commit; waiting for quota is prohibited. Findings remain advisory until
verified locally, and the reviewer identity, command and commit are retained.

The issue-owned pending quality declaration is
`did.prism.vdr-indexing.quality.v1`; implementation must activate its property,
fuzz, benchmark and differential routes before consumer-visible activation.

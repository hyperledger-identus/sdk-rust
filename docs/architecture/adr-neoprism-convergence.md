# ADR: NeoPRISM Convergence

**Status**: Accepted

**Date**: 2026-06-13

## Context

NeoPRISM is the existing Rust implementation in the Identus platform that
manages PRISM DIDs, Cardano anchoring, indexing, submission, DID resolution,
VDR operations, node storage, HTTP resolver endpoints, and node deployment
modes.

The local NeoPRISM source baseline checked for this ADR is:

| NeoPRISM path | Current crate or role | sdk-rust target |
|---|---|---|
| `lib/apollo` | `identus-apollo` crypto/key helpers | `identus-crypto` |
| `lib/did-core` | `identus-did-core` DID core model and resolver traits | `identus-did` |
| `lib/did-prism` | `identus-did-prism` PRISM method operations and protobuf model | `identus-did` plus `identus-crypto` |
| `lib/did-prism-ledger` | `identus-did-prism-ledger` indexing and submission facade | `identus-adapters` plus `identus-did` |
| `lib/did-prism-indexer` | Cardano data source indexing | `identus-adapters` |
| `lib/did-prism-submitter` | Cardano wallet and embedded-wallet submission | `identus-adapters` |
| `lib/did-resolver-http` | Universal-resolver compatible HTTP endpoint | `identus-adapters` |
| `lib/node-storage` | PostgreSQL and SQLite node storage | `identus-adapters` plus `identus-wallet` |
| `bin/neoprism-node` | Service binary and Web UI/API host | Future thin service over sdk-rust crates |

NeoPRISM already carries quality signals that `sdk-rust` should preserve:
GitHub checks, conformance tests, coverage, SonarCloud, OpenSSF Scorecard,
OpenSSF Best Practices, release automation, Docker images, Nix packaging,
file hygiene, markdown linting, YAML linting, Rust formatting, and dependency
automation.

## Decision

`sdk-rust` becomes the long-term home for reusable NeoPRISM libraries. NeoPRISM
service binaries remain deployable while their domain modules move behind
stable sdk-rust crates and adapter ports.

Migration must be incremental:

1. Stabilize crate boundaries and conformance fixtures in `sdk-rust`.
2. Port or re-export low-level PRISM and DID primitives behind `identus-did`
   and `identus-crypto`.
3. Move Cardano, HTTP, database, and storage integrations behind
   `identus-adapters` and `identus-wallet`.
4. Keep `neoprism-node` as a thin service that composes sdk-rust crates.
5. Remove duplicate NeoPRISM domain logic only after conformance parity is
   proven.

## Ownership Map

### `identus-crypto`

`identus-crypto` owns reusable crypto primitives from `identus-apollo`:

- secp256k1.
- Ed25519.
- X25519.
- JWK encoding.
- Hashing.
- Hex and base64 codecs.
- Signer and key-agreement ports.

NeoPRISM-specific API names must not leak into public sdk-rust crypto APIs.

### `identus-did`

`identus-did` owns reusable DID and PRISM method behavior:

- DID Core models.
- DID URL parsing and dereferencing.
- PRISM DID short-form and long-form parsing.
- PRISM operation model.
- PRISM protobuf serialization.
- Create, update, deactivate, and resolve state transitions.
- DID Document translation.
- Resolver traits and method-specific resolver ports.

The PRISM `AtalaOperation` protobuf serializer must be ported before the
deterministic PRISM DID method-specific id can be pinned.

### `identus-adapters`

`identus-adapters` owns integrations that must not become core protocol logic:

- Cardano Oura data source.
- Cardano DBSync data source.
- Blockfrost data source.
- Cardano-wallet submitter.
- Embedded-wallet submitter.
- Universal-resolver HTTP adapter.
- NeoPRISM HTTP API adapter.
- PostgreSQL and SQLite infrastructure adapters.
- Docker, Nix, and deployment integration hooks.

Adapters must be feature-gated and must not block default
`cargo test --workspace`.

### `identus-wallet`

`identus-wallet` owns storage and backup abstractions shared with NeoPRISM:

- Node storage records.
- SQLite and PostgreSQL storage boundaries through adapter ports.
- Backup and restore policy.
- Secure storage integration points.
- Migration metadata for future service thinning.

### `identus-trust`

`identus-trust` owns trust and status decisions consumed by NeoPRISM and
OpenID4VC:

- PRISM root trust policy.
- VDR trust decisions.
- OpenID Federation-backed trust.
- X.509 and IACA trust anchors.
- Status and revocation decisions.

## VDR Ownership

NeoPRISM VDR support is part of the sdk-rust roadmap. Ownership is split:

- `identus-did`: VDR key-purpose semantics and DID-bound authorization checks.
- `identus-trust`: VDR trust and status policy.
- `identus-adapters`: Cardano transaction metadata, HTTP endpoints, and
  infrastructure integrations.
- Future Rust services: product-specific API composition and deployment.

VDR operations must use typed ports and conformance fixtures before service
APIs claim parity.

## Quality Gates

`sdk-rust` must preserve or improve NeoPRISM quality attributes:

- Rust 2024 workspace.
- `unsafe_code = "forbid"` unless explicitly justified by ADR.
- `cargo fmt --all -- --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo test --workspace`.
- Specification conformance tests.
- Coverage reporting.
- SonarCloud or equivalent code-quality reporting.
- OpenSSF Scorecard.
- OpenSSF Best Practices.
- Release automation.
- File hygiene.
- Markdown and YAML linting.
- Nix or reproducible environment support where required by service builds.

## Conformance Requirements

Before moving a NeoPRISM module into sdk-rust:

- The target crate owner must be named in this ADR or a follow-up ADR.
- Existing behavior must have a fixture, transcript, vector, or static-model
  test in `identus-conformance`.
- The migration must identify Docker-free tests and opt-in infrastructure
  tests separately.
- The old NeoPRISM service path and new sdk-rust crate path must both be
  runnable during the transition.
- Public behavior must have a parity gate before duplicate logic is removed.
- Error rendering must be redaction-safe.

## Migration Phases

### Phase A: Fixture And Port Alignment

- Pin deterministic PRISM DID vectors.
- Add PRISM operation protobuf fixtures.
- Add DID lifecycle fixtures for create, update, deactivate, resolve, and VDR.
- Add Cardano source adapter contracts for Oura, DBSync, and Blockfrost.
- Add storage adapter contracts for PostgreSQL and SQLite.

### Phase B: Core Module Port

- Move DID Core and PRISM method models behind `identus-did`.
- Move crypto helpers behind `identus-crypto`.
- Keep compatibility shims for NeoPRISM crate imports.
- Prove deterministic DID and operation serialization parity.

### Phase C: Adapter Port

- Move indexer, submitter, ledger, resolver HTTP, and node storage behind
  `identus-adapters` and `identus-wallet`.
- Keep Cardano, database, and HTTP dependencies out of core crates.
- Gate infrastructure tests separately from default workspace tests.

### Phase D: Service Thinning

- Rebuild `neoprism-node` as a composition layer over sdk-rust crates.
- Remove duplicate domain logic after conformance and integration parity.
- Keep deployment, Web UI, OpenAPI, Docker, and Nix packaging in the service
  repository until service ownership is explicitly moved.

## Consequences

- NeoPRISM remains useful and deployable during migration.
- sdk-rust gains the reusable Rust DID/VDR foundation required by Cloud-Service,
  Mediator, and wrappers.
- Core crates stay free of Cardano, HTTP, database, and deployment dependencies.
- Service thinning happens only after conformance parity is visible.

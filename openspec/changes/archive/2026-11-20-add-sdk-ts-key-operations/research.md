# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-11-20
Source retrieval date: 2026-11-20
Research blockers: none

## Problem and existing implementation

`identus-crypto` already owns portable Ed25519 and X25519 lifecycle semantics,
strict errors, validated public JWK/COSE representations, caller-budgeted
borrowed-message work, zeroizing private construction, and injected entropy.
`identus-adapters-entropy` supplies the reviewed `getrandom` system adapter for
native and browser WASM. The missing capability is a narrow TypeScript/WASM
adapter and its cross-target evidence, not a new cryptographic engine.

The private TypeScript package currently composes only the DID WASM artifact.
It already supplies strict values, generated-owner cleanup, provider selection,
stable errors, observations, deterministic artifact assembly, and isolated
Browser/Node/React gates. Those structural patterns are reused, while secret
material is separated into its own generated artifact and lifecycle.

## Normative sources

- Primary source URLs are <https://www.rfc-editor.org/rfc/rfc8032> for
  Ed25519 and <https://www.rfc-editor.org/rfc/rfc7748> for X25519. Protocol or
  draft currency is the final RFC publication plus the repository-pinned
  implementation contract; a revision triggers reconsideration.
- SDK-Rust base `d291a446ece0e932f7f00cbe19da34b72b36bb17` is the selected Apache-2.0
  implementation authority at Rust 1.99.1. The crypto spec, Apollo parity
  registry, input-resource inventory, and exact Cargo/Nix locks define the
  accepted current implementation and direct and resolved dependency cone.
  `identus-crypto` exact version is workspace `0.0.0` with default features
  enabling Ed25519, X25519, secp256k1, P-256, hash, JWK, COSE and derivation;
  this train selects only the Ed25519/X25519 subset. The exact MSRV is Rust
  1.99.1.
- RFC 8032 is the Ed25519 profile authority and RFC 7748 is the X25519 profile
  authority. The repository-pinned RustCrypto/dalek behavior and accepted
  vectors are implementation evidence; normative revisions trigger explicit
  reconsideration.
- SDK-TS revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e`
  (`8.1.4`, Apache-2.0) is consumer-regression evidence only. Its Apollo entry,
  Ed25519/X25519 utilities, key values, and tests are donor journeys, not API
  or implementation authority.

No new npm runtime engine, native library, remote service, or crypto dependency
is selected. License and provenance remain Apache-2.0 and source-bound to the
pinned SDK-Rust/SDK-TS revisions. The resolved supply-chain evidence remains
Cargo.lock, package-lock, source-distribution checks, Cargo audit/deny, and the
Nix closure. Affected crates forbid unsafe code; the selected dalek/RustCrypto
cone adds no repository-owned unsafe block or new native-code dependency.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| SDK-TS Ed25519 generation/sign/verify journeys | `adapt` | Preserve applicable outcomes behind capability-named opaque Rust keys. |
| SDK-TS X25519 generation/public derivation journeys | `adapt` | Preserve applicable public outcomes and add the Rust-owned DH profile. |
| SDK-TS generic key classes and historical component names | `not-adopt` | They erase curve/purpose and retain the compatibility vocabulary being retired. |
| Private-key export | `not-adopt` | Conflicts with opaque lifecycle and is unnecessary for the selected consumer canary. |
| Existing `identus-crypto` implementation | `adopt` | It is the accepted portable semantic owner. |
| Separate `identus-wasm-crypto` artifact | `adopt` | Isolates secret-bearing generated APIs from public-material DID bindings. |
| Add crypto APIs to `identus-wasm-did` | `not-adopt` | Couples unrelated security and artifact boundaries. |
| Browser `crypto.getRandomValues` through the entropy adapter | `adopt` | Reuses the reviewed injected port and target backend. |
| TypeScript/WebCrypto algorithm fallback | `not-adopt` | Creates split semantic authority and target divergence. |

## Compatibility and dependency evidence

The facade is additive and private. Public and wire compatibility are closed
to purpose-specific fixed-byte inputs and immutable public projections; API v1
for the new crypto artifact does not alter DID binding API v5. Existing package
exports remain compatible. The adapter mapping is one-way for import/public
projection and operation-specific for sign/verify/derive; generated owners do
not cross the package facade.

The new artifact depends inward on `identus-crypto`, `identus-adapters-entropy`
with feature `getrandom`, and `wasm-bindgen`. Their direct and resolved cone is
already locked, audited, wasm-compiled, and Nix-pinned. No dependency points
from a domain crate to the binding or platform layer.

Rollback removes the unreleased crypto artifact/provider or selects the legacy
provider before initialization. It never exports, copies, translates, or
automatically migrates a private key and does not change canonical Rust.

Maintenance, release and security posture remain the repository support,
unsafe-code, source-distribution, advisory, signed+DCO, protected-CI, and
private-package policies. This change makes no publication, stable support,
production, or certification claim.

## Security, privacy and maintenance evidence

The selected boundary never returns private bytes. Import is explicit and
overwrites the supplied mutable buffer in a `finally` path; aliases and prior
copies remain caller-owned. Rust types keep zeroizing construction/storage.
Generated owners are freed exactly once. Stable errors and observations omit
key, message, signature, and shared-secret bytes. Debug and declaration
snapshots expose only curve, purpose, lifecycle state, and public bytes.

Fixed-size key/signature limits precede Rust parsing. A facade message ceiling
precedes linear sign/verify work. X25519 rejects noncanonical or
non-contributory peers and all-zero outcomes. Fuzz/property tests cover ingress,
lifecycle, use-after-dispose, mutation, redaction, and target equivalence.

The reconsideration triggers are a normative algorithm revision, a proved
semantic gap in `identus-crypto`, a target entropy failure, a secret-copy leak,
or evidence that separate artifacts violate an accepted budget. HD derivation,
additional curves, keystores/KMS, native bindings, publication, and mdoc require
separate decisions.

## Rejected or deferred candidates

HD derivation, mnemonic and seed handling, secp256k1, P-256, private export,
Ed25519-to-X25519 conversion, non-exportable host keys, keystores, KMS/hardware,
persistence, wallet custody, ambient networking, React Native, external
SDK-TS activation, publication, stable support, release, and mdoc are rejected
or deferred to separately accepted owners.

## Evidence commands

Planning uses `scripts/factory research-ready add-sdk-ts-key-operations`,
`scripts/factory constraints-ready add-sdk-ts-key-operations`, strict
validation, and `scripts/factory preflight add-sdk-ts-key-operations --issue
611 --write`. Implementation will retain focused crypto/WASM/package/target,
artifact, vector, mapping, secret-handling, factory, and complete Nix receipts.
Exact implementation commands will include focused Cargo/WASM/package/target
commands plus `nix flake check --print-build-logs`; unrun checks will be
recorded explicitly rather than inferred from narrower gates.

## Open questions and blockers

There are no blockers to the private repository-local Ed25519/X25519 train.
External activation, accepted artifact/latency budgets, additional curves,
native bindings, custody, publication/support, and release remain separate.

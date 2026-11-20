# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-11-20
Source retrieval date: 2026-11-20
Research blockers: none

## Problem and existing implementation

`identus-crypto` already owns bounded derivation paths, hardened-only
secp256k1 BIP-32, hardened-only Ed25519 SLIP-0010, Cardano/IOG
Ed25519-BIP32 V2 private and public derivation, zeroizing secret storage, and
accepted vectors. The missing capability is a profile-explicit TypeScript/WASM
adapter and cross-target evidence, not a new derivation engine.

The private TypeScript package and `identus-wasm-crypto` artifact now supply
strict values, opaque disposable private handles, generated-owner cleanup,
provider selection, stable errors, bounded observations, deterministic
artifact assembly, and isolated Browser/Node/React gates. This train extends
those boundaries without exposing raw derived secrets or generated classes.

## Full evidence matrix

- Current implementation: the in-tree path, BIP-32, SLIP-0010, Cardano V2,
  WASM crypto, and private TypeScript package sources named above.
- Consumer evidence: the pinned SDK-TS wallet-key journey and the repository's
  TS1 capability/conformance matrices.
- Primary source URL: <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki>,
  <https://github.com/satoshilabs/slips/blob/master/slip-0010.md>, and
  <https://cips.cardano.org/cip/CIP-0003>.
- Pinned source revision: SDK-Rust base
  `48b3c1dc34969016fe8fbf7bba1a68756b312d7a`, SDK-TS
  `4bf86ebf69d5e96616a148e4c973f831f95fa38e`, and the exact Cargo lock.
- Exact version and features: unpublished workspace version `0.0.0` with
  `identus-crypto` features `derivation`, `cardano-bip32`, `secp256k1`, and
  `ed25519`; the WASM crate selects only the required subset.
- License and provenance: repository and SDK-TS donor sources are Apache-2.0;
  the private Cardano primitive remains MIT OR Apache-2.0 under its existing
  locked provenance record.
- MSRV: exact Rust 1.99.1 from Cargo, Nix, and support-policy evidence.
- Target evidence: Rust host/WASM plus private Browser, Node.js, and React Web;
  React Native remains inapplicable.
- Direct and resolved dependency cone: the existing locked
  `identus-wasm-crypto -> identus-crypto` cone, with no new package.
- Unsafe and native-code evidence: no authored unsafe block and no new native
  dependency; existing dependency evidence remains controlling.
- Supply-chain evidence: Cargo/npm locks, source distribution, cargo-deny,
  advisory, and Nix closure gates remain unchanged.
- Public and wire compatibility: additive private DTO/handle APIs only; no
  existing wire form or public package export changes.
- Facade boundary: profile/path/seed ingress and immutable public projection
  only; all derivation semantics remain in Rust.
- Rollback: explicit legacy selection before initialization; no fallback or
  secret migration.
- Maintenance, release and security posture: private, unpublished, pre-stable,
  signed+DCO, protected-CI delivery with no release claim.
- Protocol or draft currency: each named profile is pinned independently;
  revisions or errata trigger explicit reconsideration rather than silent drift.
- Reconsideration trigger: profile revision, semantic gap, label mismatch,
  secret leak, or a consumer requirement outside the closed operation set.
- Exact commands and unrun checks: factory readiness/preflight, focused
  Rust/WASM/TypeScript/target evidence, complete Nix gate, and explicit
  recording of every unrun command.

## Normative sources

- BIP-0032 is the selected secp256k1 profile authority. This train intentionally
  exposes the current Rust hardened-only private subset and does not claim
  general soft or public BIP-32 derivation.
- SLIP-0010 is the selected Ed25519 hardened-only profile authority.
- Cardano/IOG Ed25519-BIP32 V2 is the compatibility profile selected by the
  repository-pinned `ed25519-bip32` implementation and accepted Apollo parity
  vectors. It supports hard or soft private derivation and soft-only public
  derivation.
- SDK-Rust base `48b3c1dc34969016fe8fbf7bba1a68756b312d7a` is the selected
  Apache-2.0 implementation authority at Rust 1.99.1. The crypto spec, Apollo
  parity registry, input-resource inventory, exact Cargo/Nix locks, and issue
  #612 define the accepted implementation and dependency cone.
- SDK-TS revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e`
  (`8.1.4`, Apache-2.0) is consumer-regression evidence only. Its Apollo
  `createPrivateKey`, `DerivationPath`, and derivation tests are donor journeys,
  not naming, error, custody, or algorithm authority.

No new npm engine, native library, remote service, crypto primitive, or
dependency is selected. The direct and resolved supply-chain evidence remains
the existing Cargo/npm locks, source-distribution checks, audit/deny evidence,
and Nix closure. Affected authored crates forbid unsafe code.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Existing SDK-Rust derivation implementations | `adopt` | They are the accepted portable semantic owners with stronger profile labeling and bounds. |
| SDK-TS seed-to-secp256k1 wallet-key journey | `adapt` | Preserve the applicable deterministic outcome under the named hardened BIP-32 profile. |
| SDK-TS Ed25519-BIP32 journey | `adapt` | Preserve compatibility only under the explicitly named Cardano V2 profile. |
| Ed25519 SLIP-0010 | `adopt` | Expose the distinct standards profile without presenting it as Cardano V2 compatibility. |
| SDK-TS generic key classes and historical component names | `not-adopt` | They erase profile, curve, purpose, and custody boundaries. |
| SDK-TS permissive path parsing through `parseInt` | `not-adopt` | Rust's bounded canonical parser rejects partial, empty, signed, fractional, and overflow axes. |
| Mnemonic acquisition and BIP-39 seed creation | `defer` | The issue keeps mnemonic sources and recovery-material custody host-owned. |
| Private or chain-code export to TypeScript | `not-adopt` | It conflicts with opaque lifecycle and is unnecessary for the consumer-shaped public-key fixture. |
| TypeScript derivation fallback | `not-adopt` | It would create split semantic authority and target drift. |

## Compatibility and dependency evidence

The facade is additive, private, and profile-explicit. Seed ingress and path
text are the only caller inputs. Derived private and chain-code material remain
inside opaque Rust owners; TypeScript receives immutable profile-specific
public projections and metadata only. Generated owners do not cross the
package facade.

The artifact retains its existing inward dependencies on `identus-crypto`,
`identus-adapters-entropy`, and `wasm-bindgen`. This change uses only existing
derivation features and adds no upward or sideways domain dependency.

Rollback selects the legacy host route before provider initialization or
removes the unreleased HD capability. It never exports, copies, translates, or
automatically migrates a seed or derived private key and does not change the
canonical Rust profiles.

## Security, privacy and maintenance evidence

Seed inputs are independently bounded by profile before derivation and are
overwritten synchronously in a `finally` path after the Rust owner copies them.
Callers remain responsible for aliases and prior copies. Paths are bounded by
4,096 UTF-8 bytes and 255 axes before cryptographic work. Profile-specific
hardened/soft rules fail before child derivation. Derived private and chain-code
bytes never appear in DTOs, errors, observations, debug text, artifacts, or
receipts.

Opaque handles are freed exactly once and reject use after disposal. Public
projections are copied and frozen. Stable errors and observations contain only
operation, profile, route, result category, stable code, depth, and bounded
duration. Property/fuzz/differential tests cover parsing, seed bounds,
profile separation, derivation rules, lifecycle, mutation, redaction, and
target equivalence.

Reconsideration triggers are a normative profile revision, a proved semantic
gap in `identus-crypto`, a profile-label mismatch, a secret-copy leak, or a
consumer requirement for a currently excluded public/private operation.

## Rejected or deferred candidates

Mnemonic generation/validation, BIP-39 seed creation, X25519 conversion,
general soft secp256k1 BIP-32, secp256k1 public derivation, private export,
keystore/KMS/hardware, persistence, wallet custody, ambient networking, React
Native, external SDK-TS activation, publication, stable support, release, and
mdoc are rejected or deferred to separately accepted owners.

## Evidence commands

Planning uses `scripts/factory research-ready add-sdk-ts-hd-derivation`,
`scripts/factory constraints-ready add-sdk-ts-hd-derivation`, strict
validation, and `scripts/factory preflight add-sdk-ts-hd-derivation --issue
612 --write`. Implementation will retain focused derivation/WASM/package/target,
artifact, vector, mapping, secret-handling, factory, and complete Nix receipts.
Unrun checks will be recorded explicitly rather than inferred.

## Open questions and blockers

There are no blockers to the private repository-local profile-explicit train.
Mnemonic UX, additional profiles/operations, external activation, accepted
artifact or latency budgets, native bindings, custody, publication/support,
and release remain separate.

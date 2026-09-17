# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

At base `4c2942fc11124348f5a330334294a91e221306eb`, `JwsKeyReference`
has public `KeyId(String)` and `X5c(Vec<String>)` variants. The protected-header
builder validates those values later against `JwsLimits`, but a caller can first
create and retain an arbitrarily large standalone enum. The analogous
`Oid4vciProofJwtClient::Identified(String)` variant bypasses its already-present
fallible `identified` constructor until claims are built or verified.

The wire parser already caps the enclosing compact/header bytes, X.509 chain
cardinality, and individual strings before producing a `ProtectedHeader`.
Builders and verifiers revalidate against their supplied limits. The missing
property is construction closure: every inhabitant of the three retained public
alternatives must already satisfy the applicable typed limit contract.

Workspace direct-construction call sites are confined to `identus-jose` source
and tests plus `identus-oid4vci` tests. No direct symbol usage was found in the
inspected consumer worktrees:

| Repository | Inspected HEAD | Status note | Direct usage |
| --- | --- | --- | --- |
| Oxid | `183664aeca500c25d6d27a22fa402b4d40c649d3` | existing `.pi`/UI changes, read-only | none |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | existing untracked harness state, read-only | none |
| Midnight Identity | `3e1672baf0a237d2360d393a124611d6452b8192` | existing submodule/doc state, read-only | none |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` | clean local branch, read-only | none |

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Opaque validated payload newtypes inside the existing enums | `adopt` | Removes raw retained construction while preserving the enum alternatives, builder parameters, wire shape, and exhaustive internal modeling. | A post-publication major version deliberately replaces the enum family. |
| Replace each enum with a public struct and private internal enum | `reject` | Closes construction but causes broader pattern-match and API churn than needed for the bounded pre-release migration. | A separate API simplification has consumer evidence. |
| Make the enums non-public and expose only headers/claims | `reject` | Prevents reusable caller selection of key-reference and client modes and conflicts with current builders. | The builders themselves are redesigned in a major release. |
| Keep raw variants and rely on later builder validation | `reject` | Leaves the audited retained-state bypass reachable and cannot close `SDK-LIM-007`. | Never while the typed values are represented as bounded. |
| Add a bounded-string/collection dependency | `reject` | Existing limits are profile-specific and the small private-field wrappers need no external mechanism or dependency cone. | Multiple crates converge on an identical public value contract. |

## Compatibility and dependency evidence

The accepted API adds `JwsKeyId`, `JwsX5c`, and
`Oid4vciProofJwtClientId` wrappers with private storage, redacted diagnostics,
validated constructors, and borrowed accessors. `JwsKeyReference` keeps
`KeyId`, `Jwk`, and `X5c`; `Oid4vciProofJwtClient` keeps `Identified` and
`AnonymousPreAuthorized`. Direct source construction migrates from raw payloads
to named fallible constructors. Existing `ProtectedHeader::new`,
`ProtectedHeader::with_key_reference`, proof builder/verifier entry points,
successful JSON, and compact bytes stay unchanged.

The crate is unpublished at `0.0.0`; the source-distribution policy prohibits a
release commitment. `cargo tree -p identus-jose --edges normal` shows the
existing `base64`, `identus-core`, `identus-crypto`, `identus-did`, `serde`, and
`serde_json` cone. No manifest, package, feature, native library, build script,
registry source, or lockfile change is required. Rust 1.98.1 remains the sole
active-development compiler and compatibility etalon.

## Security, privacy and maintenance evidence

Constructors validate byte length before retaining a new accepted value. X.509
construction rejects empty chains, cardinality above eight, empty/oversized or
malformed standard-base64 entries before wrapping the caller-owned vector.
Values accepted under one limit are revalidated when used under a tighter
builder/parser limit. Errors and `Debug` expose only static classifications or
cardinality, never key identifiers, client identifiers, certificates, compact
tokens, or claims.

Caller-owned allocation can occur before SDK entry and remains disclosed. Flat
rejected vectors do not create a recursive-drop claim. Parsing keeps its bounded
wire-first path and Serde/native construction parity is proven with identical
accepted and rejected boundaries. The change adds no unsafe code, cryptographic
primitive, secret handling, trust decision, runtime, chain, or product coupling.

## Normative sources

- Current implementation and pinned source revision:
  `hyperledger-identus/sdk-rust@4c2942fc11124348f5a330334294a91e221306eb`.
- Primary source URL for JOSE: https://www.rfc-editor.org/rfc/rfc7515.
- Primary source URL for the proof profile:
  https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html.
- Sponsor-directed issue #299, parent audit #168, ADR 0125, and
  `SDK-SEC-003`/`SDK-LIM-007`.
- RFC 7515 protected-header `kid`/`x5c` shapes and OpenID4VCI 1.0 Final proof
  JWT behavior already pinned by the canonical specs.
- sdk-rust source at
  `4c2942fc11124348f5a330334294a91e221306eb`, especially
  `crates/jose/src/header.rs`, `oid4vci.rs`, `oid4vci_verifier.rs`, and their
  tests.
- The four local consumer repositories listed above, inspected read-only on
  2026-09-17.

No donor code or fixture is copied. Repository source remains Apache-2.0 and
the resolved third-party supply chain is unchanged.

Exact version and feature evidence is the unchanged `identus-jose 0.0.0`
workspace manifest and lockfile. License and provenance remain Apache-2.0 SDK
source with no imported donor material. MSRV, development compiler, and etalon
remain Rust 1.98.1. Target evidence remains the repository's Linux fast line
plus eligible WASM, iOS ARM64, and Android ARM64 slow/local checks. The direct
and resolved dependency cone is unchanged. Reachable unsafe and native-code
evidence is unchanged and no new such code is introduced. Supply-chain
evidence therefore consists of the unchanged manifest, lockfile, SBOM, and
advisory/license gates. Public and wire compatibility is bounded by the opaque
Identus facade boundary; no dependency type becomes the policy surface.
Maintenance, release, and security posture stay pre-release and unpublished.
Protocol/draft currency is unchanged: RFC 7515 and OpenID4VCI 1.0 Final remain
the pinned profiles rather than a moving draft.

## Rejected or deferred candidates

The candidate table records the rejected whole-enum redesigns, continued raw
variants, and unnecessary new dependency. A generic bounded-string/collection
framework is deferred until independent crates demonstrate the same semantic
contract; introducing it for three JOSE-profile wrappers would reduce cohesion.

## Open questions and blockers

No blocker remains. Issue #299 explicitly authorizes the pre-release source
migration, the narrow newtype design preserves the modeled alternatives, and
no downstream sdk-rust caller requires coordinated mutation.

## Evidence commands

Commands run before implementation: `scripts/factory doctor`; workspace and
named-consumer `rg` inventories; `cargo tree -p identus-jose --edges normal`;
issue #299, ADR 0125, canonical JOSE/OID4VCI/input-governance specs, constraint
index, and machine input-boundary review. Commands intentionally unrun before
implementation are new exact/one-over and compile-fail tests, focused and
workspace Rust gates, API/SBOM evidence, portable-target/Nix closure, final
factory checks, fresh review, and protected exact-head CI.

## Evidence plan and rollback

Evidence will include exact/one-over string and chain limits, empty and ninth
chain rejection, malformed base64, native constructor/parser parity, tighter-
limit revalidation, redacted diagnostics, compile-fail raw-construction guards,
workspace/minimal/all-feature tests, strict Clippy, rustdoc, public API/SBOM,
portable target checks, factory gates, and a fresh security/API/architecture
review. Slow exhaustive evidence remains the weekly/manual production line.

Rollback restores raw enum payloads, workspace call sites, the inventory row,
and the JOSE clause in `SDK-LIM-007` atomically. No wire or persisted-data
migration is needed.

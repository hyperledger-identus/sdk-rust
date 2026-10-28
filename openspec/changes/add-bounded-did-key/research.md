# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The SDK can represent canonical DIDs and public verification material but has
no `did:key` method owner. The peer method contains private method-local
multicodec parsing that is not a public reuse contract. Reusing it would couple
two method profiles before identical semantics are demonstrated.
The current implementation therefore ends at generic DID/crypto primitives;
consumer evidence from SDK-TS remains migration input, not normative authority.

## Normative sources

- `w3c-ccg/did-key-spec@2cc490c38c5aacf58497a87fc0cf8794668bf716`
  is the pinned normative v0.9 profile.
- `decentralized-identity/did-key.rs@eb00da6074d8bc61e5d4c8129fbdd9dc05735cbf`
  is a historical differential oracle.
- `spruceid/ssi@886a6cf694dfdb6b019d5f69a04dd16f01cb58a4`
  is an independent Rust differential oracle.
- `impierce/did-manager@183f3eb061938404060d75817de53dd99adb18b0`
  is an independent method oracle.
- SDK-TS is compatibility evidence only; its DTO and error shapes are not a
  Rust contract.

Primary source URL:
https://github.com/w3c-ccg/did-key-spec/tree/2cc490c38c5aacf58497a87fc0cf8794668bf716.
The pinned source revision is
`2cc490c38c5aacf58497a87fc0cf8794668bf716`; the exact draft version is v0.9.
The production feature set is the crate's default-free core profile. Source
licenses and provenance are recorded in `provenance.md`.

The draft's syntax prose permits `z` or `u`, but its creation and decoding
algorithms require `z`. The first SDK profile follows the executable algorithm
and rejects `u`. The four chosen public multicodecs already map to validated
SDK key types. Signing keys receive signing/capability relationships; direct
X25519 receives only `keyAgreement`. No implicit key conversion occurs.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| SDK-owned `identus-did-key` | `adopt` | Exact narrow profile, SDK types, small cone, effect-free targets. | A maintained narrow crate satisfies every profile, bound, target, and error contract. |
| DIF `did-key` | `oracle` | Archived and tied to older crypto generations. | Maintained release resumes with the exact SDK profile. |
| Spruce SSI | `oracle` | Broad SSI/JWK/JSON-LD/multicodec cone; probe resolved 295 packages. | A narrow package removes unrelated facade ownership. |
| Impierce stack | `oracle` | IOTA, Stronghold, Tokio, and forked SSI coupling. | Its method core becomes independently maintained and portable. |
| peer-DID internal decoder | `not-adopt` | Private profile-local code with different accepted codecs and errors. | Two delivered method owners prove an identical abstraction. |
| General multicodec/multibase crate | `not-adopt` | Larger grammar and dependency surface than four fixed codecs require. | A second owner proves identical maintained semantics. |

No oracle is a production or development dependency. Frozen attributed vectors
may be adapted only with exact provenance, transformation, and expected result.

## Compatibility and dependency evidence

`identus-did-key` depends inward on `identus-core`, `identus-did`,
`identus-crypto`, and workspace `bs58`. Generic DID and crypto never depend on
the method crate. Parsing recognizes a canonical unsigned-varint codec before
key allocation, checks exact key length, and delegates curve-point validation
to SDK public-key constructors. Errors are SDK-owned, non-exhaustive, stable,
and redact identifier and key bytes.

The direct dependency cone is those four packages; the resolved dependency
cone is captured from locked Cargo metadata and must contain no oracle. The
public and wire compatibility contract is the canonical DID/fingerprint and
SDK-owned Rust facade, not a donor DTO. Supply-chain evidence includes the
locked source graph, license inventory, and explicit oracle-absence check.

The crate is synchronous, deterministic, no-std-unclaimed, and effect-free.
Target evidence covers native, Rust 1.89 MSRV, no-default, WASM compile, iOS,
Android, rustdoc, strict Clippy, and dependency/source policy. Publication is
disabled under the workspace release constraint.

## Security, privacy and maintenance evidence

The method-specific identifier has a public 128-byte text ceiling and a
67-byte decoded ceiling, enough for a two-byte codec plus a 65-byte SEC1 point;
the first profile emits compressed 33-byte EC points. Canonical creation emits
compressed EC encodings. Parsing rejects private codecs/material, trailing or
overlong varints, invalid points, empty payloads, and base58 amplification
before projection. The crate owns no entropy, secrets, signing, logs, I/O,
clock, executor, global state, or policy.
Unsafe and native-code evidence must show no new unsafe block or native
dependency in the reachable method cone. Maintenance belongs to SDK-Rust DID
maintainers; release remains disabled, and the security posture is fail-closed
bounded parsing with redacted errors. Rollback removes the additive unreleased
crate and evidence without runtime migration. Protocol/draft currency is pinned
to v0.9; a newer normative revision is a reconsideration trigger, not an
automatic upgrade.

## Open questions and blockers

No implementation blocker remains. If a pinned normative example cannot be
reproduced without accepting a broader encoding or implicit conversion, stop
and record the conflict rather than weakening the selected profile.

## Rejected or deferred candidates

Broad SSI facades, an archived production dependency, a generic multicodec
abstraction, `u` encoding, additional curves, implicit key conversion, secret
handling, effects, and consumer mutation are rejected or deferred for the
reasons and triggers in the candidate table and ADR 0188.

## Evidence commands

Planning gates are `scripts/factory research-ready add-bounded-did-key`,
`scripts/factory constraints-ready add-bounded-did-key`, strict validation,
and exact issue #557 preflight. Implementation must add focused/workspace tests,
fuzzing, benchmarks, dependency/source receipts, the declared target matrix,
Codex-or-Claude exact-diff review, protected CI, and terminal metrics.
Those Rust, target, fuzz, benchmark, dependency, review, CI, and metrics
commands are unrun at planning time and cannot be claimed by preflight.

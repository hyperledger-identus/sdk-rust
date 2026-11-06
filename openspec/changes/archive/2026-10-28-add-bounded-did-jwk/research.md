# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The protected base is `develop@9e0530084f5a7a7e1cde954995af4fc10708dc4e`.
The current implementation has no `did:jwk` method owner.
`identus-did` owns bounded DID syntax, documents and resolver composition.
`identus-crypto` owns `PublicKeyJwk`, canonical coordinate encodings, bounded
public extensions and the four required curve implementations. The JWK type is
deliberately structural and does not prove EC point validity, Ed25519 weakness,
or X25519 contributory behavior; the method profile must revalidate those
properties before an identifier becomes usable.

No SDK crate currently owns `did:jwk`. The newly delivered
`identus-did-key` is an architecture and registry-composition pattern, not a
runtime dependency. SDK-TS and other consumers remain compatibility evidence,
not normative Rust API or error authorities.

## Normative sources

- `quartzjer/did-jwk@44e186cf4cc215fc726afeecad1727cc9f9a66e8`
  is the pinned method syntax, document projection, relationship, and security
  contract.
- RFC 4648 section 5 defines base64url; this profile selects canonical
  unpadded encoding.
- RFC 7517 and RFC 8037 define JWK and OKP public-key semantics.
- W3C DID Core 1.0 defines the projected DID document.
- `spruceid/ssi@886a6cf694dfdb6b019d5f69a04dd16f01cb58a4`
  and `did-jwk 0.4.0` are independent Rust differential oracles.
- `impierce/did-manager@183f3eb061938404060d75817de53dd99adb18b0`
  is an independent method oracle.

Primary source URL:
https://github.com/quartzjer/did-jwk/tree/44e186cf4cc215fc726afeecad1727cc9f9a66e8.
On 2026-10-28, GitHub reports `main` at that exact revision and the `spec.md`
blob remains `9407c141d7e4892f6c2518cd4e357c0715a517f6`.
Crates.io reports `did-jwk 0.4.0` as latest, published 2026-02-13 with no
declared MSRV. Source licenses and dispositions are recorded in
`provenance.md`.
The exact assessed version and features are the published `did-jwk 0.4.0`
default package surface and the source-pinned Spruce implementation; neither
provides a feature set narrow enough to replace the SDK-owned method facade.

The pinned method requires one `JsonWebKey2020` verification method at `#0`.
`use=sig` removes `keyAgreement`; `use=enc` retains only `keyAgreement`; an
absent `use` retains all five relationship families. The method says curve and
explicit `use` should match. This SDK profile makes explicit mismatches invalid;
the absent-`use` projection remains the pinned method's deliberate compatibility
rule and is not an assertion that every curve performs every relationship's
operation. RFC 7517 requires `use` and `key_ops` to be consistent when both are
present, while the pinned method defines no `key_ops` mapping. The first profile
therefore rejects `key_ops` instead of inventing application policy. Other
extensions, including `kid`, are retained semantically but cannot select the
fragment or relationship policy.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| SDK-owned `identus-did-jwk` | `adopt` | Exact narrow profile, existing SDK values, effect-free behavior and zero new production dependency. | A maintained narrow crate satisfies every profile, bound, target and error contract. |
| `identus_crypto::PublicKeyJwk` | `adopt` | Public-only structural owner with bounded extensions and canonical coordinates. | The method requires a public key family absent from the SDK. |
| Spruce `did-jwk 0.4.0` | `oracle` | Current and useful, but its resolved 286-package graph, SSI facade, missing MSRV and projection choices exceed this method boundary. | A narrow release removes unrelated facade ownership and matches the selected profile. |
| Impierce method stack | `oracle` | IOTA, Stronghold, Tokio, storage and forked SSI coupling are disproportionate. | A standalone portable method package is published and qualified. |
| Generic JOSE/JWK facade | `not-adopt` | The SDK already owns the required bounded public JWK domain; another facade would duplicate policy and types. | The SDK JWK cannot represent a normative required profile. |
| Shared generic JSON parser extraction | `not-adopt` | Method-local duplicate-safe parsing avoids expanding a public foundation before identical reuse is proven. | Two delivered owners require the same public parser semantics and errors. |

No oracle is a production or development dependency. Frozen attributed
vectors may be adapted only with exact source, path, license, transformation,
retrieval date, and expected result.

## Compatibility and dependency evidence

`identus-did-jwk` depends inward on `identus-core`, `identus-did`, and
`identus-crypto`, plus existing workspace Base64/Serde JSON support. Generic
DID and crypto never depend on the method crate, and the static method crates
do not depend on each other. No external JOSE, DID or SSI type crosses the
public boundary.
Public and wire compatibility are additive and unreleased: existing packages
retain their behavior, while the new crate owns only its strict identifier,
document projection, errors, and Rust API.

Creation consumes an SDK `PublicKeyJwk`, validates the method profile, emits a
deterministic compact JSON serialization, enforces the same 16,384-byte JSON
and 21,846-byte carrier ceilings as parsing, and applies canonical unpadded
base64url. Parsing preserves the exact DID string, validates the carrier and
bounded duplicate-free UTF-8 JSON, deserializes the public JWK, revalidates the
actual curve point or OKP public material, and applies `use` compatibility.
Because valid JSON ordering and whitespace may differ, parsing does not demand
that the decoded JSON equal the SDK's deterministic serialization.

The direct dependency cone and locked resolved cone must contain no oracle.
Supply-chain evidence includes license/source policy, RustSec state, unsafe and
native-code inspection, and explicit oracle absence. The crate is synchronous,
deterministic, no-std-unclaimed, and effect-free. Target evidence covers host,
Rust 1.89 MSRV, no-default, WASM, iOS, Android, rustdoc, strict Clippy, and
workspace/Nix checks. Publication remains outside this issue.

## Security, privacy and maintenance evidence

The method-specific carrier is limited to 21,846 ASCII bytes and decoded JWK
JSON to 16,384 bytes. JSON is limited to 16 levels, 1,024 nodes, 64 top-level
members, and the stricter inherited `PublicKeyJwk` extension budgets. Length
and alphabet checks occur before proportional decode; duplicate object names
at every retained level fail before typed construction. Known private JWK
members (`d`, `k`, `p`, `q`, `dp`, `dq`, `qi`, and `oth`) fail closed.
`key_ops` is also rejected by the first method profile because accepting it
without a normative operation-to-relationship mapping could contradict `use`
or the projected DID relationships.

Ed25519 values must be canonical and non-weak; X25519 values must be canonical
and contributory; P-256 and secp256k1 coordinates must form valid curve points.
Errors use fixed categories and never echo the DID, carrier, decoded JSON,
extensions, coordinates, or key material. The crate owns no entropy, secret,
signing, proof, I/O, storage, clock, executor, global state, or trust policy.

Maintenance belongs to SDK-Rust DID maintainers. Rollback removes the additive
unreleased crate and evidence without runtime migration. A normative source
change, a qualifying narrow crate, or interoperability evidence conflicting
with the strict profile triggers reconsideration rather than silent widening.
Protocol or draft currency is pinned to revision `44e186c`; currency review is
a reconsideration trigger, not permission to float behavior with `main`.

## Open questions and blockers

No implementation blocker remains. The normative P-256 example contains an
apparent stray `$` in one authentication reference; tests use the DID and JWK
material but normalize document expectations to the algorithm text rather
than reproducing that editorial typo. Any further normative conflict stops the
affected behavior instead of weakening the selected profile.

## Rejected or deferred candidates

Broad SSI or JOSE facades, permissive private-member handling, padded or
non-canonical carriers, unsupported curves, secret handling, JWK Set, proof or
trust selection, effects, language DTO parity, and consumer mutation are
rejected or deferred for the reasons and triggers above and in ADR 0189.

## Evidence commands

Planning gates are `scripts/factory research-ready add-bounded-did-jwk`,
`scripts/factory constraints-ready add-bounded-did-jwk`, strict validation,
and exact issue #558 preflight. Implementation must add focused and workspace
tests, fuzzing, benchmarks, dependency/source receipts, target builds,
Codex-or-Claude exact-diff review, protected CI, and terminal metrics. Those
implementation gates are unrun at planning time and cannot be claimed by the
preflight receipt. Delivery also requires a manifest/public-API dependency
guard, a repository-diff stop-boundary audit, and a distinct registry E2E run;
portable compile evidence cannot substitute for that behavioral E2E route.

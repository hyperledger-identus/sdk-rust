# SDK bootstrap inventory

This inventory answers what exists on `develop` now. It is not a release
catalogue and does not reserve future crate names. The normative local data is
[`sdk-bootstrap-inventory.toml`](sdk-bootstrap-inventory.toml), checked by
`scripts/check-bootstrap-inventory.py` on every factory run.

## Repository and authority status

| Concern | Current contract |
| --- | --- |
| Repository | `hyperledger-identus/sdk-rust` |
| Integration | issue-linked, reviewed, green-CI pull requests into `develop` |
| Reserved branch | `main`; intentionally outside integration and release |
| Release state | unreleased; every package is Cargo `publish = false` |
| Maintainers | inherited from the canonical Hyperledger Identus policy |
| Release authority | assigned human release manager plus a second maintainer |
| Security response | Identus security response team through private reporting |
| Namespace/publishing | not activated; governed separately by issue #3 |
| Live repository controls | external action required under issue #26 |

The repository-local governance packet includes the Apache-2.0 license,
maintainer inheritance, governance, contribution, DCO, security, release,
conduct, ownership, issue/PR and desired repository-settings records. The
machine inventory pins the canonical Identus policy blob SHAs observed for this
iteration so provenance does not depend on a moving branch name alone.

Read-only GitHub API evidence observed on 2026-09-03 showed a private
repository, protected reserved `main`, unprotected `develop`, no GitHub
environments and disabled repository security features. This file does not
turn those observations into approval or claim they are fixed. Human
maintainers own the public-readiness and settings work in issue #26; `IDR-001`
therefore remains `in_progress` after this local slice.

## Maturity vocabulary

| Class | Meaning |
| --- | --- |
| `implemented` | Real code and tests exist, but its public surface is experimental, unreleased and subject to its focused component issue. |
| `verification` | Repository guard/test code exists; it is not a consumer runtime capability. |
| `placeholder` | Only a component marker remains from the seed. The name, API and dependency graph are not accepted commitments. |

Every placeholder depends only on `identus-core` for its component marker.
Package presence, the layer rulebook and version `0.0.0` do not imply support.

## Implemented experimental foundations

| Package | Current public surface | Features and boundary | Stabilization |
| --- | --- | --- | --- |
| `identus-core` | `Component`, `CapabilityId`, `ErrorCode`, `ErrorKind`, `IdentusError`, `IdentusResult`, `Url`, `UrlError` | Foundation values and redaction-safe errors; not an umbrella SDK | issue #25 now; later component contracts may narrow it |
| `identus-derive` | `DeriveNewtype` and inert `port` procedural macros | Compile-time helper only; diagnostics and generated API remain experimental | issue #25 |
| `identus-crypto` | encodings, JWK conversion, curve key types, sign/verify traits, hashes, secure-random port, BIP/SLIP derivation | Default, minimal and `kmp-compat` surfaces are tested; no custody, JOSE or Midnight Jubjub | issue #9 |
| `identus-did` | initial `DidMethod`, `Version`, `Multihash` and error values | Not yet the DID Core model, DID URL, document or resolver contract | issue #5 |
| `identus-credentials` | bounded format identifier, opaque artifacts/envelope, metadata/schema descriptors, status bindings/freshness/query/evidence, canonical staged verification evidence, async verifier port and exact-format registry | no claim values, serde/wire codecs, concrete formats/protocols/verifiers, trust decisions, schema/status implementations, holder bindings, status proof payloads, display/localization or storage | issues #71, #73, #75, #77 and #87; later credential/verification slices remain experimental |
| `identus-presentations` | bounded request/query/claim/challenge, request-validated candidate, validated disclosure-plan, opaque generated-artifact, value-free receipt-input, and allocation-free protocol lifecycle contracts | no protocol wire or transport, candidate lookup/ranking, selection, consent or authorization policy, proof execution or verification, verifier acceptance, receipt outcome/persistence/policy, storage, FFI, chain or product behavior | issues #79, #81, #83 and #85; later presentation slices remain experimental |
| `identus-wallet` | bounded opaque storage revision/cursor/page vocabulary, explicit single-record mutation receipts, and separate associated-type `SecretStore`, `CredentialStore`, `DidStore`, `ProtocolStateStore` and `StatusCacheStore` async ports | no wallet product, SDK-owned records, concrete storage, encryption, codecs, cross-record transactions, synchronization, custody, policy, protocol, chain or FFI behavior; runtime cone is `identus-core` plus the `identus-derive` port marker | issue #89; production adapters and cross-consumer conformance remain downstream |
| `identus-adapters-entropy` | `GetrandomSystemRandomAdapter`, `DeterministicRandomAdapter` | `getrandom` and `deterministic` are opt-in; deterministic entropy is test-only | issue #25 and later ports convergence |

`identus-conformance` is verification-only. Its public `COMPONENT` marker does
not make architecture guards a supported runtime API.

## Quarantined placeholders

| Package | Current meaning | Next decision |
| --- | --- | --- |
| `identus-trust` | marker only; no trust-policy or registry API | issue #6 or a narrower trust/status issue |
| `identus-messaging` | marker only; no DIDComm implementation | conditional row under issue #20 |
| `identus-openid4vc` | marker only; not an umbrella protocol commitment | focused OID4VCI issue #7 and later protocol issues |
| `identus-agent` | marker only; no agent runtime contract | conditional row under issue #20 |
| `identus-bindings` | marker only; FFI remains unsupported | binding work only after accepted component APIs |

No consumer should depend on these placeholders. A focused issue must first
replace the marker contract, choose the real crate/API boundary and provide two
consumer-shaped proofs. Apollo and NeoPRISM reduction remain downstream
outcomes after compatible SDK components are released; their repositories are
not dependencies of this inventory.

## Deliberate 70–80% boundary

The fast contract validates local evidence, Cargo publication denial, exact
explicit workspace coverage, in-tree path-dependency membership, layer
classification and placeholder shape, including Cargo targets and executable
documentation. Generated
rustdoc item inventories, API-diff baselines, signed canonical-policy
snapshots, remote GitHub polling and live control mutation are deferred. These
would add cost without improving the next component's core architecture today.

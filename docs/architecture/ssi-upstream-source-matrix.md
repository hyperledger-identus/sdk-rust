# SSI upstream source matrix

This matrix supports the canonical
[SDK upstream backlog](../roadmap/ssi-upstream-dependency-backlog.csv). It is a
source audit, not permission to bulk-copy a repository. Every component port
still requires its own issue, OpenSpec contract, file-level Apache-2.0
provenance and conformance evidence.

## Inspected references

| Alias | Repository and revision | License evidence | Working-tree note |
| --- | --- | --- | --- |
| `apollo` | [`hyperledger-identus/apollo@ccee22b`](https://github.com/hyperledger-identus/apollo/tree/ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c) | Root `LICENSE`: Apache-2.0 | Kotlin Multiplatform source; local checkout had a pre-existing dirty nested secp256k1 submodule |
| `neoprism` | [`hyperledger-identus/neoprism@8becb22`](https://github.com/hyperledger-identus/neoprism/tree/8becb225132efb1d9302b2c5f6ed4d87b84e8685) | Root `LICENSE` and workspace metadata: Apache-2.0 | Remote `main` inspected without switching or editing the local checkout |
| `midnight-identity` | [`MediaNoxLabs/midnight-identity@427f857`](https://github.com/MediaNoxLabs/midnight-identity/tree/427f8571950c42967a18726cbcbefecc19ef8d79) | Root `LICENSE` and workspace metadata: Apache-2.0 | Local checkout had a pre-existing dirty `third_party/midnight-did` submodule |
| `lace-id-portal` | [`input-output-hk/lace-id-portal@804de0a`](https://github.com/input-output-hk/lace-id-portal/tree/804de0a9e58cf48ece3cc6c24b2245bb70bc80f1) | No repository license file or workspace license metadata found; evidence-only until resolved | Local checkout had pre-existing untracked agent and temporary paths |
| `oxid` | [`MediaNoxLabs/oxid@685f967`](https://github.com/MediaNoxLabs/oxid/tree/685f9670af4846d52697a4cfeb94779758ae1075) | Root `LICENSE` and workspace metadata: Apache-2.0 | Remote `integration` inspected; local checkout had pre-existing untracked agent paths |

The original CSV was an uncommitted file in an Oxid roadmap worktree based on
`b21eb326727cb49aecf3eb47d11a5a8136fce573`. That commit does not contain the
file. The SDK copy introduced by issue #20 is therefore the first immutable
repository record of the SDK-owned rows.

## Classification rule

| Class | Meaning |
| --- | --- |
| `extract` | Chain-neutral Rust behavior already has a coherent source unit |
| `adapt` | Useful generic behavior exists but its API or dependencies cross the SDK boundary |
| `conformance-only` | Behavior or vectors inform compatibility; its implementation is not a Rust source port |
| `remain-downstream` | Chain, ledger, Compact, product, policy, custody, UI, deployment or concrete storage behavior |

## Candidate surfaces

| Source | Path | Class | SDK rows | Boundary and intended result |
| --- | --- | --- | --- | --- |
| Apollo | `apollo/src/commonMain/.../base64`, `derivation`, `hashing`, `secp256k1`, `securerandom`, `utils` | `conformance-only` | `IDR-004`, `IDR-044` | Preserve cross-language encodings, derivation and curve behavior as vectors; do not port Kotlin APIs directly |
| NeoPRISM | `lib/apollo` | `adapt` | `IDR-004` | Reconcile Rust encodings, SHA-256, JWK and curve behavior with the existing SDK crypto foundation; isolate PRISM legacy verification as an explicit compatibility feature |
| NeoPRISM | `lib/did-core` | `adapt` | `IDR-005`, `IDR-006` | Converge W3C DID model and resolution ports; the SDK contract must also express Midnight, Lace and Oxid fixtures |
| NeoPRISM | `lib/did-resolver-http` | `extract` | `IDR-006` | Optional chain-neutral HTTP binding follows the core resolver contract under issue #10 |
| NeoPRISM | `lib/did-prism` | `adapt` | `IDR-040` | A future `did:prism` method slice may own method grammar and operation semantics after the generic DID contract stabilizes |
| NeoPRISM | `lib/did-prism-indexer`, `lib/did-prism-ledger`, `lib/did-prism-submitter`, `bin/neoprism-node`, `lib/node-storage` | `remain-downstream` | none | Cardano observation/submission, node composition and concrete storage remain NeoPRISM responsibilities |
| midnight-identity | `crates/midnight-did-domain` | `adapt` | `IDR-005`, `IDR-006` | Generic DID/JWK/document/resolution vocabulary converges upstream; Midnight network and method behavior do not |
| midnight-identity | `crates/midnight-vc-domain` | `adapt` | `IDR-007`, `IDR-008`, `IDR-009`, `IDR-010`, `IDR-027` | Extract only format-neutral envelopes, status vocabulary and ports proven by non-Midnight consumers |
| midnight-identity | `crates/midnight-did-method`, `crates/midnight-did-runtime`, `crates/midnight-did-indexer`, `crates/midnight-vc-runtime` | `remain-downstream` | none | `did:midnight`, Compact codegen/runtime, proving, ledger and indexer integration remain in midnight-identity |
| Lace ID Portal | `crates/core`, `crates/issuer-ports` | `adapt` | `IDR-005`, `IDR-006`, `IDR-010` | Reuse generic DID/JWK, clock, randomness and issuer port evidence without importing service or storage policy |
| Lace ID Portal | `crates/issuer-services`, protocol fixtures and integration tests | `conformance-only` | `IDR-009`, `IDR-023`, `IDR-024`, `IDR-025` | Prove issuer/verifier wire behavior and negative paths against SDK protocol engines |
| Lace ID Portal | `crates/did-midnight`, `crates/credential-digital-passport`, `crates/jubjub-schnorr`, `crates/contracts` | `remain-downstream` | none | Midnight method, Compact passport and Jubjub suite behavior move only to midnight-identity or remain product-specific |
| Oxid | `crates/identity/domain` | `adapt` | `IDR-005`, `IDR-006` | Port consumer-shaped DID/JWK/document fixtures; Midnight network types remain downstream |
| Oxid | `crates/credential/domain` | `adapt` | `IDR-007`, `IDR-009`, `IDR-010`, `IDR-027` | Generalize holder envelopes, private-material handles and staged verification without wallet IDs or trust policy |
| Oxid | `crates/presentation/domain` | `adapt` | `IDR-008`, `IDR-009` | Generalize request, candidate, disclosure and state vocabulary without consent/UI policy |
| Oxid | `crates/protocol/domain`, `crates/adapters/openid4vci`, `crates/adapters/openid4vp`, `crates/adapters/siopv2` | `conformance-only` | `IDR-023`, `IDR-024`, `IDR-025`, `IDR-026`, `IDR-031` | Preserve holder-side wire/state and negative fixtures; SDK engines cannot import wallet orchestration |
| Oxid | `crates/platform/ports` | `adapt` | `IDR-004`, `IDR-010` | Consider clock/random contracts; QR, screen privacy and platform UI ports remain product-owned |
| Oxid | application services, custody, storage adapters, trust decisions, UI and composition | `remain-downstream` | none | Product policy and implementation stay in Oxid behind thin SDK adapters |

## Resulting repository shape

```text
Apollo compatibility vectors ───────┐
NeoPRISM generic Rust surfaces ─────┼──> sdk-rust generic components
Midnight/Lace/Oxid generic evidence ┘              │
                                                    ├──> midnight-identity adapters
                                                    ├──> NeoPRISM PRISM/Cardano adapters
                                                    ├──> Lace issuer/verifier application
                                                    └──> Oxid wallet application
```

The desired reductions happen only after compatible upstream candidates:

- Apollo can enter a separately governed deprecation path after its supported
  consumers have equivalent SDK/binding behavior and migration evidence.
- NeoPRISM can delete or repoint `lib/apollo`, `lib/did-core` and the generic
  resolver binding after released SDK components pass its historical fixtures.
- midnight-identity, Lace ID Portal and Oxid can replace duplicate generic
  types with thin adapters while retaining their chain and product boundaries.

# Remaining portable DID-method disposition

**Date:** 2026-10-28

**Planning issue:** [#555](https://github.com/hyperledger-identus/sdk-rust/issues/555)

**Decision:** [ADR 0187](../../adr/0187-sequence-portable-static-did-methods-before-web-effects.md)

**Delivery:** [did:key #557](https://github.com/hyperledger-identus/sdk-rust/issues/557) → [did:jwk #558](https://github.com/hyperledger-identus/sdk-rust/issues/558) → [did:web #559](https://github.com/hyperledger-identus/sdk-rust/issues/559)

## Executive decision

Implement three independently selectable SDK-owned method crates. Reuse the
existing generic DID and public-key domains, keep the static methods
effect-free, and place `did:web` retrieval behind the smallest bounded injected
port. Current Rust implementations are valuable oracles but are not selected as
production dependencies.

## Source ledger

| Source | Immutable revision | Purpose |
| --- | --- | --- |
| [did:key specification](https://github.com/w3c-ccg/did-key-spec/tree/2cc490c38c5aacf58497a87fc0cf8794668bf716) | `2cc490c38c5aacf58497a87fc0cf8794668bf716` | syntax, multicodec, generation, and resolution |
| [did:jwk specification](https://github.com/quartzjer/did-jwk/tree/44e186cf4cc215fc726afeecad1727cc9f9a66e8) | `44e186cf4cc215fc726afeecad1727cc9f9a66e8` | JWK encoding and document projection |
| [did:web specification](https://github.com/w3c-ccg/did-method-web/tree/ea423c114e6f2537498ee6f94e8d794c64f60c18) | `ea423c114e6f2537498ee6f94e8d794c64f60c18` | HTTPS mapping and document binding |
| [Spruce SSI](https://github.com/spruceid/ssi/tree/886a6cf694dfdb6b019d5f69a04dd16f01cb58a4) | `886a6cf694dfdb6b019d5f69a04dd16f01cb58a4` | independent Rust oracle |
| [DIF did-key.rs](https://github.com/decentralized-identity/did-key.rs/tree/eb00da6074d8bc61e5d4c8129fbdd9dc05735cbf) | `eb00da6074d8bc61e5d4c8129fbdd9dc05735cbf` | historical Rust oracle |
| [Impierce did-manager](https://github.com/impierce/did-manager/tree/183f3eb061938404060d75817de53dd99adb18b0) | `183f3eb061938404060d75817de53dd99adb18b0` | independent method oracle |

Revisions describe the inspected state; they are not floating dependency pins.
No code or fixture is copied by this planning change.

## Method matrix

| Method | First SDK profile | Existing SDK reuse | Effects | Deferred |
| --- | --- | --- | --- | --- |
| `did:key` | base58-btc `z`; Multikey; Ed25519, X25519, P-256, secp256k1; direct X25519 agreement | DID values/documents and validated public keys | none | `u`, P-384, BLS, RSA, implicit Ed→X conversion, private material |
| `did:jwk` | unpadded base64url public JWK; `JsonWebKey2020`; `#0`; exact `use` relationship rules | `PublicKeyJwk`, DID values/documents | none | private members, alternate VM suites, uniqueness claims, custody |
| `did:web` | FQDN/encoded port/path to HTTPS; well-known/path rules; exact document ID | DID values/documents and resolver error vocabulary | injected bounded fetch | concrete HTTP, IP hosts, DNS/TLS/redirect/proxy/cache/retry/SSRF policy |

The `did:key` draft contains a profile ambiguity: syntax prose permits `z` or
`u`, while the creation and decoding algorithms explicitly require base58-btc
`z`. The first SDK profile follows the executable algorithm and fails closed on
`u`; expansion requires a later profile decision.

`did:jwk` resolution cannot claim one-key/one-DID uniqueness because ordinary
JSON admits multiple serializations. SDK generation may be deterministic, but
resolution preserves the supplied DID and validates its public JWK.

## Build-versus-adopt matrix

| Candidate | Evidence | Disposition |
| --- | --- | --- |
| SDK-owned narrow crates | Existing bounded SDK types, target gates, and method-crate architecture | adopt |
| Spruce `did-method-key` 0.5.0 | 295-package all-target resolved probe with defaults disabled; SSI/JWK/JSON-LD/multicodec coupling | oracle |
| Spruce `did-jwk` 0.4.0 | 286-package resolved probe; current projection defaults differ from pinned `JsonWebKey2020` profile | oracle |
| Spruce `did-web` 0.5.1 | 333-package resolved probe; embeds Reqwest/TLS and leaves host/TLS policy concerns in method code | oracle |
| DIF `did-key` 0.2.1 | 117-package resolved probe; archived and old crypto generations | oracle |
| Impierce method stack | Rust 1.75 declaration but IOTA Git, Stronghold, Tokio, and forked Spruce coupling | oracle |

Resolved-package counts were produced from disposable all-target Cargo metadata
probes. They include target-specific alternatives and are structural comparison
evidence, not compiled-closure or binary-size measurements.

## Architecture and E2E plan

```text
identus-did-key ──┐
                  ├──> identus-did + identus-crypto
identus-did-jwk ──┘

identus-did-web ─────> identus-did + BoundedFetch port
                                      ▲
                                      │
                         runtime-specific adapter (later)
```

Every method child must prove positive and negative normative vectors,
constructor/parser equivalence, explicit resource ceilings, no secret-bearing
errors, property invariants, bounded fuzzing, deterministic benchmarks, and the
declared host/WASM/mobile compile matrix. Differential tests compare oracles
without importing their public types.

The `did:web` child must add a deterministic in-process E2E covering DID-to-URL
mapping, injected retrieval, bounded parsing, exact document-ID correlation,
and failure redaction. External Internet access is neither necessary nor
sufficient evidence for the portable core.

## Ownership and stop boundary

The active owner moves serially from #557 to #558 to #559. Planning does not
create Cargo packages or imply support. No SDK-TS, SDK-Swift, SDK-KMP,
NeoPRISM, Midnight, Oxid, wallet, agent, cloud service, or binding repository is
changed. Those systems may later adopt immutable SDK candidates through their
own migration issues and thin adapters.

## Reconsideration triggers

- a normative method revision changes identifier or projection semantics;
- a maintained narrow Rust crate matches the selected profile, bounds, targets,
  dependency policy, and redacted error contract;
- two delivered methods prove an identical reusable multicodec abstraction;
- a named runtime consumer needs a concrete `did:web` transport adapter;
- conformance evidence reveals an interoperability conflict in a selected
  strict profile.

Until one of those triggers is documented, the ADR and child issue contracts
remain the implementation baseline.

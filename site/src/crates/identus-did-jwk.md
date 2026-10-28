# `identus-did-jwk`

`identus-did-jwk` is an independently selectable, effect-free DID-method
package. It turns caller-owned public JWKs into deterministic `did:jwk` values,
parses bounded duplicate-safe JSON, and projects local DID documents without a
network, runtime, store, custody layer, or global registry.

![did:jwk ownership and evidence](../diagrams/did-jwk.svg)

## Supported profile

| Surface | Included | Excluded |
| --- | --- | --- |
| Carrier | canonical unpadded base64url over UTF-8 JSON | padding, alternate alphabets, duplicate names |
| Public keys | Ed25519, X25519, P-256, secp256k1 | private/symmetric keys, unsupported curves |
| JWK policy | absent `use`, explicit `sig` or `enc`, bounded public extensions | `key_ops` and extension-driven relationship policy |
| Verification method | `JsonWebKey2020` with `publicKeyJwk` at fixed `#0` | `kid` fragments and donor DTOs |
| Operations | create, parse, deterministic local resolve | update, deactivate, network resolution |
| Representations | context-free DID JSON; DID Core + JWS 2020 contexts for DID JSON-LD | JSON-LD processing or expansion |

Carrier text is bounded at 21,846 bytes and decoded JSON at 16,384 bytes. The
duplicate-safe parser additionally limits depth to 16, nodes to 1,024, and
top-level members to 64. Creation applies the same serialized ceilings, and
both entry paths preflight generic DID document projection limits. Accepted
values therefore remain resolvable rather than deferring an extension-shape
failure until projection. All four curve families receive actual public-key
validation before use.

## Explicit composition

```rust
use identus_did::DidMethodRegistry;
use identus_did_jwk::DidJwkResolver;

let registry = DidMethodRegistry::builder()
    .register(DidJwkResolver.into_method_binding())?
    .build();
# Ok::<(), identus_did::Error>(())
```

Linking the package never changes a global default. Products continue to own
custody, trust, persistence, transport, consent, telemetry, and runtime policy.

## Evidence

The attributed fixture packet executes the pinned P-256 and X25519 examples.
Property tests, alternate-serialization tests, security negatives, an opt-in
registry E2E, bounded fuzz target, and directional benchmark cover the profile.

Run the readable local E2E:

```bash
cargo test -p identus-did-jwk --test registry_e2e
```

- [ADR 0189](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0189-bind-did-jwk-to-a-bounded-public-profile.md)
- [Profile and adoption guide](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/did-jwk-profile.md)
- [Quality baseline](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/did-jwk-quality-baseline.md)
- [Conformance packet](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/conformance/fixtures/did-jwk/v1/vectors.json)
- [Source](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/did-jwk)

The manifest remains experimental at `0.0.0` with publication disabled.
Portable compile evidence does not claim browser/device packaging,
certification, consumer adoption, or production support.

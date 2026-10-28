# `identus-did-key`

`identus-did-key` is an independently selectable, effect-free DID-method
package. It turns caller-owned public keys into canonical `did:key` values,
parses a strict bounded profile, and projects local DID documents without a
network, runtime, store, or global registry.

## Supported profile

| Surface | Included | Excluded |
| --- | --- | --- |
| Encoding | canonical base58-btc `z` | base64url `u`, non-canonical varints |
| Public keys | Ed25519, X25519, P-256, secp256k1 | private codecs, P-384, BLS, RSA |
| Verification method | `Multikey` with `publicKeyMultibase` | donor DTOs and alternate suites |
| Relationships | contributory direct X25519 key agreement; four signing/capability relationships for signing curves | low-order X25519 and implicit Ed25519-to-X25519 conversion |
| Operations | create, parse, deterministic local resolve | update, deactivate, network resolution |
| Representations | context-free DID JSON; DID Core + Multikey contexts for DID JSON-LD | JSON-LD processing or expansion |

The method-specific identifier is limited to 128 bytes and decoded public
material to 67 bytes. Parsing uses a fixed stack buffer, canonical unsigned
varints, exact key lengths, SDK curve validation, canonical re-encoding, and
content-free error categories. X25519 public keys must additionally use a
canonical field encoding and produce a contributory shared secret.

## Explicit composition

```rust
use identus_did::DidMethodRegistry;
use identus_did_key::DidKeyResolver;

let registry = DidMethodRegistry::builder()
    .register(DidKeyResolver.into_method_binding())?
    .build();
# Ok::<(), identus_did::Error>(())
```

Linking the package never changes a global default. Products choose the
method explicitly and continue to own custody, trust, persistence, transport,
consent, telemetry, and runtime policy.

## Evidence

The fixture-driven conformance suite binds the pinned did:key v0.9 draft to
independent DIF, Spruce, and Impierce Rust oracles without adding those crates
as dependencies. Property tests, a registry E2E, a bounded fuzz target, and a
directional benchmark cover the selected profile.
The registry E2E additionally proves representation negotiation: DID JSON is
context-free while DID JSON-LD carries the DID Core and Multikey contexts.

Run the readable local E2E:

```bash
cargo test -p identus-did-key --test registry_e2e
```

- [ADR 0188](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0188-bind-did-key-to-a-strict-public-key-profile.md)
- [Profile and adoption guide](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/did-key-profile.md)
- [Quality baseline](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/did-key-quality-baseline.md)
- [Source](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/did-key)

The canonical manifest remains experimental at `0.0.0` with publication
disabled. Compile evidence for native, WASM, iOS, and Android is not a claim of
browser/device packaging, certification, or production support.

# did:key profile and adoption guide

Issue [#557](https://github.com/hyperledger-identus/sdk-rust/issues/557)
owns the first bounded `identus-did-key` profile. ADR 0188 fixes the supported
surface; the pinned did:key v0.9 draft remains normative.

## Purpose and ownership

```text
caller-owned public key
          |
          v
  identus-did-key -----> identus-crypto validation
          |
          +------------> identus-did document + opt-in registry binding
```

The crate owns canonical method encoding, public-key classification, and local
document projection. It owns no private keys, generation, signing, custody,
storage, transport, executor, product policy, or global registration.

## Supported profile

| Surface | Supported | Deliberately absent |
| --- | --- | --- |
| Multibase | canonical base58-btc `z` | base64url `u` |
| Public keys | Ed25519, X25519, P-256, secp256k1 | P-384, BLS, RSA, private codecs |
| Verification method | one `Multikey` with `publicKeyMultibase` | donor-specific DTOs and alternate suites |
| Relationships | contributory X25519: key agreement; signing curves: authentication, assertion, capability invocation/delegation | low-order X25519 and implicit Ed25519-to-X25519 derivation |
| Operations | create, parse, deterministic local resolve | update, deactivate, network resolution |
| Representations | context-free `application/did+json`; DID Core + Multikey contexts for `application/did+ld+json` | JSON-LD processing or expansion |

The method-specific identifier is limited to 128 bytes and decoded material to
67 bytes. Base58 is decoded into a fixed stack buffer. Codec varints, exact key
length, curve points, and canonical re-encoding are checked before projection.
X25519 inputs must use a canonical field encoding and be contributory, so two
fingerprints cannot represent one effective agreement key. Errors expose
categories, never identifier or key content.

## Explicit registry composition

```rust
use identus_did::DidMethodRegistry;
use identus_did_key::DidKeyResolver;

let registry = DidMethodRegistry::builder()
    .register(DidKeyResolver.into_method_binding())?
    .build();
# Ok::<(), identus_did::Error>(())
```

No global default changes when the crate is linked. The deterministic E2E in
`crates/did-key/tests/registry_e2e.rs` creates a DID, resolves it through this
opt-in registry, serializes/reparses the generic document, and proves the
fingerprint and relationships survive without I/O. It also negotiates the
JSON-LD representation and proves the DID Core and Multikey contexts are
present only on that representation.

## Evidence and adoption

Normative and oracle inputs are immutable and attributed; oracle crates are not
dependencies. Conformance/property tests, fuzzing, directional benchmarks,
strict Clippy, rustdoc, dependency checks, native/MSRV/no-default and portable
target builds are the delivery evidence. Consumers should adopt only an
immutable SDK candidate through their own adapter issue; Rust DTOs and errors
remain canonical, with language-specific migration shapes kept thin.

# did:jwk profile and adoption guide

Issue [#558](https://github.com/hyperledger-identus/sdk-rust/issues/558)
owns the first bounded `identus-did-jwk` profile. ADR 0189 fixes the supported
surface; the pinned `quartzjer/did-jwk@44e186c` draft remains normative.

## Purpose and ownership

```text
caller-owned PublicKeyJwk
          |
          v
  identus-did-jwk -----> identus-crypto key validation
          |
          +------------> identus-did document + opt-in registry binding
```

The crate owns method encoding, bounded duplicate-safe JSON parsing, public-key
classification, and local document projection. It owns no private keys,
generation, signing, proof/trust policy, custody, storage, transport, executor,
language binding, consumer migration, publication, or global registration.

## Supported profile

| Surface | Supported | Deliberately absent |
| --- | --- | --- |
| Carrier | canonical unpadded base64url of duplicate-free UTF-8 JSON | padding, alternate alphabets, duplicate names |
| Public keys | Ed25519, X25519, P-256, secp256k1 | RSA, symmetric `k`, unsupported curves, private parameters |
| JWK policy | absent `use`, `sig`, `enc`; bounded public extensions | `key_ops` and extension-driven relationship policy |
| Verification method | one `JsonWebKey2020` at fixed `#0` | `kid` fragments and donor DTOs |
| Operations | create, parse, deterministic local resolve | update, deactivate, network resolution |
| Representations | context-free DID JSON; DID Core + JWS 2020 contexts for DID JSON-LD | JSON-LD processing or expansion |

Carrier text is limited to 21,846 bytes and decoded JSON to 16,384 bytes. JSON
work is limited to 16 levels, 1,024 nodes, and 64 top-level members before
typed construction. Creation enforces the same serialized ceilings as parsing.
Both paths also preflight the generic DID document extension rules, so an
accepted JWK cannot later fail solely because its arrays or objects are too
large to project.
Ed25519 values must be canonical and non-weak; X25519 values canonical and
contributory; EC coordinates must form valid points. Errors expose categories,
never identifier, JSON, extension, coordinate, or key content.

Absent `use` deliberately preserves the pinned draft's five relationships.
Explicit `sig` produces the four signing/capability relationships and explicit
`enc` only key agreement. This compatibility rule is not a claim that every
curve performs every relationship operation. The v1 profile rejects `key_ops`
instead of inventing an operation-to-relationship policy.

## Explicit registry composition

```rust
use identus_did::DidMethodRegistry;
use identus_did_jwk::DidJwkResolver;

let registry = DidMethodRegistry::builder()
    .register(DidJwkResolver.into_method_binding())?
    .build();
# Ok::<(), identus_did::Error>(())
```

No global default changes when the crate is linked. The deterministic E2E in
`crates/did-jwk/tests/registry_e2e.rs` creates a DID, resolves it through the
opt-in registry, serializes and reparses the generic document, and verifies the
public JWK without I/O. DID JSON-LD context negotiation is tested separately.

## Evidence and adoption

The attributed fixture packet records the pinned P-256 and X25519 examples.
Unit, conformance, property, fuzz, benchmark, E2E, dependency, MSRV, portable
target, rustdoc, strict Clippy, workspace, and Nix receipts make the profile
adoptable. Oracle crates are evidence only and never dependencies. Consumers
should adopt an immutable SDK candidate through a separate adapter issue.

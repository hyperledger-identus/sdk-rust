# Fixture Policy

Fixtures are part of the SDK contract. They must be stable, reviewable, and
shared by Rust crates and future TypeScript, Swift, Kotlin, WASM, Node, and
UniFFI bindings.

The governing schema decision is
[`docs/architecture/adr-fixture-schema-policy.md`](../architecture/adr-fixture-schema-policy.md).
It requires schema versioning, source evidence, owner crate metadata, expected
states or typed errors, and a redaction policy for every checked-in fixture.

## Directory Layout

```text
fixtures/
  schema/
  conformance/
    static-model/
    vector/
    transcript/
    interop/
    infrastructure/
```

## Fixture Types

| Type | Purpose | Default CI |
|---|---|---|
| `static-model` | Parser, model-shape, and negative structural tests | yes |
| `vector` | Deterministic input/output vectors | yes |
| `transcript` | Protocol replay without external services | yes |
| `interop` | Reference-suite or cross-SDK interoperability | opt-in until hermetic |
| `infrastructure` | Minimal Docker, ledger, database, cloud, or device adapters | opt-in |

## File Policy

- JSON is the default fixture format.
- Each fixture directory must include a `README.md` describing its source,
  normative reference, owner crate, and update process.
- Fixtures that come from a specification or external test suite must include
  source URI and license notes.
- Negative fixtures must describe the expected typed error family.
- Infrastructure fixtures must have a Docker-free alternative where practical
  and must never be required by the default `cargo test --workspace` path.
- Protocol transcripts must include participants, ordered messages, expected
  states, source evidence, and fixture-safe redacted attachments.
- Credential and presentation fixtures must include verification assumptions and
  negative cases for expiry, audience/domain/challenge mismatch, holder binding,
  status, tampering, malformed schema, and unsupported formats.

## Naming

Use the conformance catalog id as the first path component:

```text
fixtures/conformance/vector/did-prism/<case-name>.json
fixtures/conformance/vector/credential-verification/<case-name>.json
fixtures/conformance/transcript/didcomm-issue-credential-3/<case-name>.json
```

The `identus-conformance` crate owns the catalog and validates the root fixture
layout. Protocol crates own the behavior-specific tests that consume fixtures.
Machine-readable schema files live under `fixtures/schema/` and are enforced in
the default conformance test path for checked-in JSON fixtures.

## Seed Fixtures

The first checked-in vector is
`fixtures/conformance/vector/did-prism/deterministic-master-key.json`, copied
from the Identus deterministic PRISM DID docs. It pins mnemonic, seed,
derivation path, private key, compressed public key, key id, curve, and protobuf
key encoding. The final method-specific DID hash remains a follow-up because it
requires the PRISM `AtalaOperation` protobuf serializer.

`fixtures/conformance/vector/credential-verification/negative-cases.json`
defines the first credential and presentation verification negative-case matrix.
It is synthetic and redacted by default; implementation crates must preserve the
stable case ids when they add executable credential, presentation, status, and
disclosure vectors.

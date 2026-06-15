# Specification Conformance Tests

`identus-conformance` is the executable catalog for every specification planned
for `sdk-rust`. The first layer is metadata conformance: every specification
must have a stable id, owner crate, source URI, support stage, conformance mode,
and backlog task before production implementation starts.

The catalog currently covers:

- Crypto and key material: BIP-39, JWK, JOSE, COSE.
- DID: W3C DID Core, DID URL, PRISM DID, Peer DID, Web DID, Key DID,
  JWK DID, PKH DID, and fixture-only Example DID.
- Credentials: W3C VC Data Model, VC JSON Schema, JWT VC/VP, SD-JWT, SD-JWT VC,
  AnonCreds, OpenBadges, ISO mdoc.
- Presentations: Presentation Exchange and DCQL.
- DIDComm and mediation: DIDComm v2, OOB, BasicMessage, Issue Credential,
  Present Proof, Report Problem, Revocation Notification, Coordinate Mediation,
  Message Pickup, Trust Ping, Routing.
- OpenID4VC and trust: OID4VCI, OID4VP, SIOPv2, OpenID Federation, HAIP,
  Bitstring Status List, IETF Token Status List, X.509/IACA.
- Platform exchange: Digital Credentials API and BLE/NFC/QR proximity handoff.

Run it with:

```bash
cargo test -p identus-conformance
```

The CI conformance gate checks the same default contract and also blocks
wrapper API parity drift. Third-party GitHub Actions must be pinned to
immutable SHAs. The expected checkout layout is:

```text
repos/sdk-rust
repos/sdk-ts
repos/sdk-swift
repos/sdk-kmp
```

From `repos/sdk-rust`, CI runs:

```bash
.specify/scripts/bash/check-prerequisites.sh --json --include-tasks
node tools/generate-wrapper-api-parity.mjs --check
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

When an upstream wrapper export changes, the parity gate must fail until
`fixtures/conformance/interop/wrapper-api-parity.json` is regenerated and the
affected backlog task or fixture coverage is updated.

Later phases should replace each metadata-only entry with deeper tests according
to its conformance mode:

| Mode | Expected test form |
|---|---|
| `StaticModel` | Model/API shape tests and negative parsing tests |
| `Vector` | Deterministic vectors checked in `fixtures/` |
| `Transcript` | Protocol transcript replay without external services |
| `Interop` | Cross-implementation tests against reference suites or partner SDKs |
| `Infrastructure` | Optional tests gated behind minimal Docker or device adapters |

Fixture layout and file rules are defined in
[`docs/testing/fixtures.md`](fixtures.md), with schema policy governed by
[`docs/architecture/adr-fixture-schema-policy.md`](../architecture/adr-fixture-schema-policy.md).

The first behavior vector is the deterministic PRISM DID master-key fixture in
`fixtures/conformance/vector/did-prism/deterministic-master-key.json`.

The integration runner contract for joining the existing cross-component
release matrix is defined in
[`docs/testing/integration-runner-contract.md`](integration-runner-contract.md).

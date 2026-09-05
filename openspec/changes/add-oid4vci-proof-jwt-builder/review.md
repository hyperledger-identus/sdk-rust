# Review: OID4VCI proof JWT holder builder

## Pre-implementation review — 2026-09-05

### Architecture and API

- **Result:** pass.
- The profile stays in the blueprint-assigned `identus-jose` crate and does not
  activate the quarantined umbrella OpenID crate.
- Existing header construction remains source-compatible; the sum type makes
  ambiguous key references unrepresentable for callers.
- Builder and verifier remain distinct staged APIs. The second PR may add the
  accepted JOSE-to-DID dependency without forcing DID into holder construction.

### Standards and interoperability

- **Result:** pass with bounded optional-feature deferral.
- Required Appendix F.1 fields, exact `typ`, anonymous `iss` omission, audience,
  integer `iat`, nonce and the three signature-key references are covered.
- Optional `key_attestation` and `trust_chain` are rejected rather than ignored
  and require a separate capability contract. This does not change the common
  proof shape or claim support promised by #99.
- Fully specified `Ed25519`, ES256 and explicit legacy EdDSA follow the already
  accepted #98 algorithm policy.

### Security and privacy

- **Result:** pass.
- Public JWK rejects private material; X.509 input is bounded and structurally
  decoded; exact reference exclusivity prevents key-selection ambiguity.
- Invalid local state and impossible output bounds precede external signing.
- Static errors and redacted Debug representations exclude attacker-controlled
  values and cryptographic material.
- The builder makes no verification, authorization, freshness, replay or trust
  claim.

### Performance and portability

- **Result:** pass.
- Work is linear in bounded header/claim/certificate input and performs one JSON
  serialization plus existing compact encoding. Maximum certificate entries are
  eight; no maps, network, clock, async runtime or new dependency is introduced.
- One release diagnostic will record preparation/signing throughput without a
  machine-specific threshold. Existing Rust 1.85, WASM and mobile gates remain
  applicable.

### Blockers

None. Implementation may begin after strict structural/factory validation.

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

## Post-implementation review — 2026-09-05

### Architecture and API

- **Result:** pass.
- The implementation matches the staged holder-only boundary: validated claims
  and one closed key reference produce exact signing bytes, and only an external
  `JwsSigner` can produce the typed signed-but-unverified result.
- Existing `ProtectedHeader::new` callers and compact wire values remain
  compatible. No dependency, feature, DID, runtime, clock or custody edge was
  added.
- The builder and key-reference types are exported from `identus-jose`; the
  quarantined umbrella OpenID crate remains untouched.

### Standards and interoperability

- **Result:** pass for the specified baseline.
- Tests cover exact `openid4vci-proof+jwt`, all three exclusive key references,
  identified and anonymous pre-authorized claim shapes, explicit integer `iat`,
  optional nonce, and the existing Ed25519/ES256 algorithm policy.
- `key_attestation`, `trust_chain`, certificate trust, DID authorization and
  issuer policy remain explicit verifier/follow-up work and are rejected or
  unclaimed here.

### Security and privacy

- **Result:** pass after one resolved finding.
- **Resolved finding:** the first draft serialized a protected header into an
  unconstrained intermediate `Vec` before checking its configured ceiling. A
  caller-constructed inline JWK extension could therefore allocate above the
  advertised boundary. Compact serialization now uses a bounded writer that
  stops at `max_protected_header_bytes`; regression coverage proves failure
  occurs before signing.
- Public JWK private material, ambiguous references, malformed or excessive
  certificate chains, algorithm confusion, invalid claims and impossible token
  bounds fail closed. Static errors and redacted Debug output do not expose
  attacker-controlled values or signature material.

### Performance and portability

- **Result:** pass.
- The ignored release diagnostic prepared 100,000 proofs in 79.423416 ms,
  approximately 1,259,075 operations/second on the local machine. This is an
  observation, not a portable threshold.
- Focused, workspace, minimal-feature, Rust 1.85, Android, iOS, browser-WASM,
  supply-chain and Nix checks passed. Nix correctly omitted incompatible local
  `x86_64-linux`; hosted CI owns that independent lane.

### Blockers

None. The implementation is ready for spec synchronization, archive and hosted
review.

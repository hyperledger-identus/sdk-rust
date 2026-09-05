# Review: OID4VCI proof JWT issuer verifier

## Pre-implementation review — 2026-09-05

### Architecture and API

- **Result:** pass.
- The verifier remains in the blueprint-assigned `identus-jose` crate and uses
  the allowed inward DID/core dependencies without activating the quarantined
  umbrella OpenID crate.
- Parsed, verified and authorized states separate syntax, cryptography/key
  authorization and issuer policy. External resolution, certificate, clock and
  replay concerns enter through narrow ports.
- Existing compact, signature and holder-builder APIs remain unchanged.

### Standards and interoperability

- **Result:** pass with explicit narrow `kid` support.
- Appendix F.1/F.4 proof type, asymmetric algorithm, exclusive key reference,
  signature binding, claims, nonce and freshness rules are covered.
- DID URL fragments with public JWK material cover the current reusable
  Midnight/NeoPRISM/Oxid seam. Non-DID identifiers and multibase conversion
  fail explicitly rather than guess a resolver or codec.
- Optional `key_attestation` and `trust_chain` remain rejected under #104.

### Security and privacy

- **Result:** pass.
- Profile and policy checks precede provider work; exact authentication
  dereferencing and returned-ID checks prevent key/relationship substitution.
- X.509 trust remains in an injected provider while the SDK independently
  checks algorithm/key and proof-signature binding.
- Replay is an atomic final gate with no permissive default. Invalid proofs
  cannot consume replay state.
- Static errors and redacted state/provider Debug output omit all
  attacker-controlled values, cryptographic bytes and timestamps.

### Performance and portability

- **Result:** pass.
- Parsing and comparisons are linear in already bounded input. One attempt has
  at most one key-provider call, signature-suite call, clock read and replay
  call.
- Manual boxed futures add no executor/runtime dependency. The existing Rust
  1.85, native, WASM and mobile checks remain applicable.
- One ignored release diagnostic will record full in-memory verification
  throughput without creating a portable threshold.

### Blockers

None. Implementation may begin after strict OpenSpec/factory validation.

## Post-implementation review — pending

Record the distinct implementation review, resolved findings and exact-head
evidence here before delivery.

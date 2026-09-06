# Review: OID4VCI attestation and trust-chain capabilities

## Pre-implementation review — 2026-09-06

### Architecture and API

- **Result:** pass.
- The change remains in `identus-jose`, introduces no dependency edge and
  preserves existing method signatures.
- Parsed, cryptographically verified, trust-evaluated and issuer-authorized
  states expose the exact guarantees reached.
- Platform trust, transport and product policy enter only through injected
  ports.

### Standards and interoperability

- **Result:** pass with one deliberate narrowing.
- OID4VCI 1.0 and OpenID Federation 1.0 are Final and define the two protected
  members plus their binding requirements.
- This profile requires `kid` whenever outer `trust_chain` is present. That is
  stricter than metadata-only carriage and prevents ambiguous proof key
  selection while satisfying the Final signature-verification rule.

### Security and privacy

- **Result:** pass.
- Total header size, per-string size, compact-token shape and chain depth are
  checked before providers. Trust-chain keys are re-bound to the outer
  algorithm; attestation validation receives the already verified proof key.
- Missing/unavailable/rejecting providers fail closed. No ambient trust store,
  clock, network or fallback key source is selected.
- Errors and Debug expose only static classes, lengths, presence and counts.

### Portability and performance

- **Result:** pass.
- Parsing remains linear in the existing bounded header. Each applicable
  provider is called at most once. Manual boxed futures preserve executor
  neutrality and Rust 1.85.
- Existing native, Android, iOS and browser-WASM gates cover the affected crate.

### Blockers

None. Implementation may begin after strict OpenSpec and factory validation.

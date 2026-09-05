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

## Post-implementation review — 2026-09-05

### Architecture and API

- **Result:** pass.
- The implementation keeps holder construction synchronous while introducing
  async only at issuer-owned DID, X.509 and replay boundaries. The one new
  runtime edge is the blueprint-approved `identus-jose -> identus-did` edge;
  no protocol umbrella, chain, product, transport, storage or executor enters
  the dependency cone.
- Private constructors preserve the parsed, verified and authorized state
  transitions. Existing JWS and holder APIs remain source-compatible.

### Standards and interoperability

- **Result:** pass for the specified OpenID4VCI Final baseline.
- Tests cover exact type and algorithm allowlisting, duplicate-safe recognized
  claims, inline JWK, exact DID URL plus `authentication`, injected `x5c`,
  identified and anonymous client modes, audience, nonce, integer time and
  replay handling.
- Non-DID `kid`, DID path/query selectors, substituted DID resources and
  multibase-only material fail explicitly. Optional `key_attestation` and
  `trust_chain` remain issue #104 rather than being guessed here.

### Security and privacy

- **Result:** pass after two resolved findings.
- **Resolved finding:** the initial negative matrix relied on the generic DID
  adapter and did not independently exercise a hostile dereferencer returning
  another method ID or unsupported multibase-only material. Dedicated cases
  now prove both paths fail closed.
- **Resolved finding:** new verifier error variants were individually observed
  at behavior boundaries but their SDK error-code/kind bridge was not
  exhaustive. A table-driven regression now binds every variant to its stable,
  static contract.
- Claim policy is checked before expensive providers on the convenience path;
  signature verification still precedes replay. Debug and Display canaries
  prove claims, DID/certificate material, compact bytes and timestamps are not
  copied into diagnostics.

### Performance and portability

- **Result:** pass.
- The first diagnostic draft used a recording replay double and therefore
  measured mutex/string logging. It now uses an allocation-free accepting
  provider. A release run verified and authorized 20,000 inline-JWK proofs in
  835.960833 ms, approximately 23,925 operations/second on this machine. This
  is informational and not a portable threshold.
- Focused and workspace all-feature/no-default-feature tests, strict Clippy and
  warning-denied documentation pass. The staged prospective Git tree also
  passes all 28 compatible local Nix checks, including Rust 1.85 MSRV,
  Android/iOS/WASM, supply-chain, docs and release Nextest lanes; hosted Linux
  remains the independent platform confirmation.

### Blockers

None. Proceed to the exact-head receipt, spec sync and hosted review.

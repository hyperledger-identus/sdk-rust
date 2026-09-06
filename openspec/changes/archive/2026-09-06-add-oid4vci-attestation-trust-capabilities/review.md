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

## Post-implementation review — 2026-09-06

### Exact-diff architecture and API review

- **Result:** pass.
- Existing `prepare`, verifier construction, `authorize`,
  `verify_and_authorize`, and authorized-state access remain source compatible.
  The new evidence value, provider ports, and trusted state are additive.
- Trust-chain key selection has one exclusive provider path and no DID/X.509
  fallback. The SDK independently re-binds the returned JWK to the protected
  algorithm and verifies the exact outer signing input.
- Key attestation runs only after a valid outer signature and receives the
  exact verified key plus the validated optional proof nonce.

### Security, privacy and resource review

- **Result:** pass after two test-only findings were resolved.
- Finding 1: raw parser coverage did not directly exercise `trust_chain` with
  `jwk`. A negative case now proves that this ambiguous key source fails as
  `InvalidProofEvidence` before any provider.
- Finding 2: the new attestation input's custom `Debug` implementation was not
  protected by a canary assertion. The accepting-provider test now proves that
  neither the nested token nor nonce appears in its Debug output.
- Evidence parsing is bounded by the existing complete-header and per-string
  limits plus an eight-statement ceiling. Empty, malformed, duplicate,
  oversized and unknown members fail closed. Provider failures remain static
  and value-free.

### Standards, portability and dependency review

- **Result:** pass.
- The implementation matches the pinned Final specifications and preserves the
  deliberate `kid`-only trust-chain profile narrowing documented above.
- No dependency, feature, executor, network, storage, certificate, chain or
  platform coupling was added. Rust 1.85, native, Android, iOS and browser-WASM
  builds passed through the repository-pinned Nix matrix.

### Final findings

No unresolved finding remains.

## Hosted review cycle — 2026-09-06

- Hosted Codex reviewed exact head `3838d7fc264a6bacba9618ce5611551edfc4a6f5`
  and found one P2: alphabet-only checking admitted undecodable Base64url
  segments such as `A.A.A`.
- Resolution: every nested compact-token segment is now decoded and
  canonically re-encoded with unpadded Base64url before evidence is accepted.
  Constructor and raw-parser regressions cover the counterexample.
- A fresh hosted review is required on the replacement head. No finding is
  treated as resolved until the focused and full gates pass on that head.

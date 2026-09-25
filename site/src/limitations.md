# Current limitations

The handbook is intentionally explicit about what has **not** been proven.

- Only `identus-derive`, `identus-core`, and `identus-crypto` are activated for
  the protected `0.1.0-rc.1` crates.io train; use its registry receipt to
  determine live publication state.
- `identus-did` and `identus-did-resolver-http` have isolated, reproducible
  `0.1.0-rc.1` candidate archives but no crates.io release. Their canonical
  manifests remain `0.0.0` with `publish = false`; evaluate them only by exact
  source revision.
- Every package outside those two train closures also remains `0.0.0` with
  `publish = false`.
- Rust 1.89.0 is the `0.1.x` release MSRV; Rust 1.98.1 remains the primary and
  compatibility-etalon compiler.
- WASM, iOS, and Android cryptography evidence is currently compile-oriented;
  it is not a blanket browser/device/runtime support claim.
- Experimental DID language-binding evidence does not automatically support
  the crypto or DID release train on those platforms.
- DID candidate API snapshots are a diffable compatibility origin, not a
  stable API promise. Candidate SBOMs are dependency evidence, not an advisory
  scan, signed provenance, vulnerability-free claim, or certification.
- Test vectors, fuzzing, coverage, static analysis, and review reduce risk but
  are not security or compliance certification.
- The SDK does not provide key custody, hardware/KMS management, secure wallet
  storage, trust policy, consent, chain integration, or product operations.
- Consumer adoption remains independently owned and reversible.
- Apollo deprecation and downstream duplicate removal require later releases,
  adoption evidence, and maintainer governance.

The machine-readable constraint index and component evidence remain normative:

- [Constraint index](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/governance/sdk-constraints.toml)
- [Support policy](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/sdk-support-policy.md)
- [Crypto completion report](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/crypto-foundation-completion.md)

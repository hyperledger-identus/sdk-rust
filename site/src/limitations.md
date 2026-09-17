# Current limitations

The handbook is intentionally explicit about what has **not** been proven.

- No crate is currently published to crates.io.
- Canonical workspace versions remain `0.0.0` with `publish = false`.
- Rust 1.89.0 is the `0.1.x` release MSRV; Rust 1.98.1 remains the primary and
  compatibility-etalon compiler.
- WASM, iOS, and Android cryptography evidence is currently compile-oriented;
  it is not a blanket browser/device/runtime support claim.
- Experimental DID language-binding evidence does not automatically support
  the crypto release train on those platforms.
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

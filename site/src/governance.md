# Governance and contribution

SDK-Rust is AI-first but evidence-gated. Agents may plan, implement, review,
push, and merge routine issue-linked work into protected `develop` after the
required checks pass. Humans retain authority over release, publication,
repository administration, security disclosure, legal-risk acceptance, and
promotion to `main`.

## Specification-driven flow

```text
issue → research → constraints → OpenSpec contract → preflight
      → implementation → verification → review → protected develop
      → production evidence → human release decision
```

Important repository references:

- [Contributing](https://github.com/hyperledger-identus/sdk-rust/blob/develop/CONTRIBUTING.md)
- [Governance](https://github.com/hyperledger-identus/sdk-rust/blob/develop/GOVERNANCE.md)
- [Security](https://github.com/hyperledger-identus/sdk-rust/blob/develop/SECURITY.md)
- [Release policy](https://github.com/hyperledger-identus/sdk-rust/blob/develop/RELEASING.md)
- [AI Software Factory](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/factory/README.md)
- [Architecture blueprint](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/sdk-rust-blueprint.md)
- [Constraints and limitations](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/governance/constraints-and-limitations.md)

Documentation changes use the same issue, OpenSpec, signed/DCO commit, local
review, exact-head CI, and protected merge controls as code changes.

# Identus SDK for Rust

Chain-agnostic Rust foundations for decentralized identity and verifiable
credential products in the
[Hyperledger Identus](https://github.com/hyperledger-identus) ecosystem.

> **Work in progress:** `develop` is a pre-release integration line. No crate
> in this repository should yet be treated as a stable production SDK release.

## Direction

The repository owns generic SSI domain models, protocol engines, cryptographic
utilities, ports and conformance evidence. Midnight, PRISM/Cardano and future
chain families consume the SDK and keep ledger-specific behavior outside it.
Wallet products keep custody, storage, consent, trust and UI policy.

- [Technical blueprint and component sequence](docs/architecture/sdk-rust-blueprint.md)
- [Bootstrap branch decision](docs/adr/0001-bootstrap-branch-selection.md)
- [Roadmap](ROADMAP.md)
- [Governance](GOVERNANCE.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)
- [Release policy](RELEASING.md)

## Branch model and current baseline

`develop` starts from `yet-another-seed` revision `662f8d7` and is the active
integration branch. It contains working validated-newtype, derivation,
cryptography, DID, entropy-adapter and conformance foundations. The baseline
also contains known bootstrap debt: placeholder crates, version `0.0.0`, an
MSRV/Nix toolchain mismatch and a compiler-diagnostic-sensitive UI snapshot.
These are stabilization items, not evidence of a production release.

`main` remains intentionally minimal and is not an integration or release
target until maintainers explicitly activate it through a later decision.
Feature branches and pull requests target `develop`.

## Getting started

### Prerequisite

- [Nix](https://nixos.org/) with flakes enabled for the reproducible toolchain;
  or a compatible stable Rust toolchain for plain-Cargo development.

```bash
# Enter the devshell
nix develop

# Build and test
nix develop -c cargo build --workspace
nix develop -c cargo test --workspace

# Format Rust and Nix
nix run .#format

# Run the complete repository gate
nix flake check
```

The flake supports `x86_64-linux` and `aarch64-darwin`. CI runs the flake gate
on both platforms for pull requests and pushes to `develop`.

## Development workflow

We use [OpenSpec](https://github.com/Fission-AI/OpenSpec) for changes that need
a recorded proposal, design, specification delta and task list.

```text
   /opsx:explore
        │
        ▼
   /opsx:propose <idea> ──► openspec/changes/<idea>/
        │
        ▼
   /openspec-review
        │
        ▼
   /opsx:apply ──► /opsx:verify ──► /opsx:archive
```

Use an OpenSpec change for new behavior, public API, architecture, protocol or
multi-step work. Direct edits are suitable for typos, non-behavioral bug fixes,
formatting, dependency chores and behavior-preserving refactors. Archived seed
decisions are historical evidence: branch, release and architecture rules in
the current governance documents take precedence.

## Resources

- [Identus documentation](https://hyperledger-identus.github.io/)
- [Hyperledger Identus GitHub](https://github.com/hyperledger-identus)
- [Hyperledger governing documents](https://toc.hyperledger.org/governing-documents/)

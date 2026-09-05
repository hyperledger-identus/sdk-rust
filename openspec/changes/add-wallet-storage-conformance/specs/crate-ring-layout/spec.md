## MODIFIED Requirements

### Requirement: In-source layer rulebook

`identus-conformance` SHALL encode the layer rules as an in-source
`pub(crate) const LAYER_RULES` typed Rust value. Every workspace crate SHALL
appear in exactly one of the seven layers. The verification layer SHALL contain
`identus-conformance` and `identus-wallet-conformance` and SHALL permit direct
dependencies on foundation or orchestration. The wallet conformance crate SHALL
use only the orchestration allowance; the repository conformance crate SHALL
retain only its existing foundation dependency. Every member SHALL carry its
`proc_macro` flag and `identus-derive` SHALL remain the sole true member.

#### Scenario: Wallet verification points inward to its contract

- **WHEN** the rulebook and workspace manifests are inspected
- **THEN** `identus-wallet-conformance` appears once in verification, its edge
  to `identus-wallet` is accepted, and all production edges to verification
  remain rejected

## ADDED Requirements

### Requirement: Wallet conformance is a separate verification leaf

`identus-wallet-conformance` SHALL be a verification-layer crate depending
only on the inward orchestration-layer `identus-wallet` crate. It SHALL expose
consumer behavioral test support and SHALL not be a dependency of any
production SDK crate. `identus-conformance` SHALL retain repository structure
and architecture enforcement; neither crate SHALL become a production adapter.

#### Scenario: Verification dependency direction is inspected

- **WHEN** the workspace dependency guard reads both conformance manifests
- **THEN** the wallet conformance edge to wallet is accepted and every
  production-to-verification edge remains rejected

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

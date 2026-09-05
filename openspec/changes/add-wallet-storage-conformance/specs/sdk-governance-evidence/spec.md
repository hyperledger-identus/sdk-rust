## ADDED Requirements

### Requirement: Wallet conformance support is inventoried separately

The human and machine inventories SHALL classify
`identus-wallet-conformance` as verification-only, identify issue #91 and its
wallet-only runtime dependency, and state that local memory proof is not a
production adapter or downstream adoption receipt.

#### Scenario: Consumer inspects test-support status

- **WHEN** a consumer evaluates wallet adapter integration
- **THEN** the inventory identifies the reusable conformance entry points and
  preserves the experimental, unreleased and no-adapter limitations

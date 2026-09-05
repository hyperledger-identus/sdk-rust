## MODIFIED Requirements

### Requirement: In-source layer rulebook

`identus-conformance` SHALL encode the layer rules as an in-source
`pub(crate) const LAYER_RULES` (typed Rust data), porting the seed's
`layer_rules` and `allowed_target_layers_by_source_layer`: the 7 layers
(foundation, domain-primitives, credential-semantics, protocol-semantics,
orchestration, outer-boundary, verification), each layer's `identus-*` crate
membership, and each source layer's allowed inward target layers. The
rulebook SHALL be the guard's source of truth for layer membership and
direction. Each member entry SHALL carry a `proc_macro: bool` flag indicating
whether the crate is a proc-macro crate (`[lib] proc-macro = true`);
`proc_macro = true` crates are exempt from the inward-direction policy (see
"Rust dep-graph guard enforces layer rules"). The `identus-derive` crate SHALL
be a foundation-layer member flagged `proc_macro = true`. No `.json` fixture
file SHALL be introduced for this purpose.

The verification layer SHALL contain `identus-conformance` and
`identus-wallet-conformance` and SHALL permit direct dependencies on
foundation or orchestration. `identus-wallet-conformance` SHALL use only the
orchestration allowance; `identus-conformance` SHALL retain only its existing
foundation dependency.

#### Scenario: Rulebook defines all 7 layers and all workspace crates

- **WHEN** `LAYER_RULES` is inspected
- **THEN** it SHALL list the foundation, domain-primitives,
  credential-semantics, protocol-semantics, orchestration, outer-boundary, and
  verification layers, and every `identus-*` workspace crate SHALL appear in
  exactly one layer's membership, and `identus-derive` SHALL appear in the
  foundation layer flagged `proc_macro = true`

#### Scenario: Rulebook carries a proc_macro flag per member

- **WHEN** a member entry in `LAYER_RULES` is inspected
- **THEN** it SHALL expose a `proc_macro: bool` field; `identus-derive`'s entry
  SHALL set it `true` and every other existing member SHALL set it `false`

#### Scenario: Rulebook encodes the accepted inward-direction policy

- **WHEN** `LAYER_RULES`'s allowed-inward lists are inspected
- **THEN** each source layer's allowed target layers SHALL match the seed's
  `allowed_target_layers_by_source_layer` (foundation → none;
  domain-primitives → foundation + domain-primitives; credential-semantics →
  those plus credential-semantics; protocol-semantics → those plus
  protocol-semantics; orchestration → those plus orchestration;
  outer-boundary → foundation through orchestration), while verification
  SHALL allow foundation + orchestration for its two narrowly guarded members

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

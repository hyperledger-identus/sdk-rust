## MODIFIED Requirements

### Requirement: In-source layer rulebook

`identus-conformance` SHALL encode the layer rules as an in-source
`pub(crate) const LAYER_RULES` (typed Rust data), porting the seed's
`layer_rules` and `allowed_target_layers_by_source_layer`: the 7 layers
(foundation, domain-primitives, credential-semantics, protocol-semantics,
orchestration, outer-boundary, verification), each layer's `identus-*` crate
membership, and each source layer's allowed inward target layers. The rulebook
SHALL be the guard's source of truth for layer membership and direction. Each
member entry SHALL carry a `proc_macro: bool` flag indicating whether the crate
is a proc-macro crate (`[lib] proc-macro = true`); `proc_macro = true` crates
are exempt from the inward-direction policy. `identus-derive` SHALL be a
foundation-layer member flagged `proc_macro = true`. No `.json` fixture file
SHALL be introduced for this purpose.

The verification layer SHALL contain `identus-conformance` and
`identus-wallet-conformance` and SHALL permit direct dependencies on
foundation or orchestration. `identus-wallet-conformance` SHALL use only the
orchestration allowance; `identus-conformance` SHALL retain only its existing
foundation dependency.

The credential-semantics layer SHALL contain `identus-credentials`,
`identus-presentations` and `identus-jose`. The JOSE member MAY depend inward
on `identus-core`, `identus-crypto` and `identus-did`; it SHALL NOT depend on a
protocol, orchestration, outer-boundary, chain or product crate.

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

#### Scenario: JOSE is reusable credential semantics

- **WHEN** the rulebook and `identus-jose` manifest are inspected
- **THEN** `identus-jose` appears exactly once in credential-semantics and its
  only `identus-*` runtime dependencies are the inward `identus-core`,
  `identus-crypto` and `identus-did` crates

#### Scenario: Rulebook encodes the accepted inward-direction policy

- **WHEN** `LAYER_RULES`'s allowed-inward lists are inspected
- **THEN** each source layer's allowed target layers SHALL match the seed's
  `allowed_target_layers_by_source_layer` (foundation to none;
  domain-primitives to foundation plus domain-primitives;
  credential-semantics to those plus credential-semantics;
  protocol-semantics to those plus protocol-semantics; orchestration to those
  plus orchestration; outer-boundary to foundation through orchestration),
  while verification SHALL allow foundation plus orchestration for its two
  narrowly guarded members

#### Scenario: Wallet verification points inward to its contract

- **WHEN** the rulebook and workspace manifests are inspected
- **THEN** `identus-wallet-conformance` appears once in verification, its edge
  to `identus-wallet` is accepted, and all production edges to verification
  remain rejected

## ADDED Requirements

### Requirement: Exact DID dependency cone is guarded

The manifest conformance test SHALL assert that the current internal normal
dependencies of `identus-did` are exactly `identus-core` and
`identus-derive`. The broad domain-layer rule MAY permit other inward edges,
but no unused edge SHALL enter this reusable crate without a focused contract
that deliberately updates the exact assertion.

#### Scenario: Unused crypto coupling returns

- **WHEN** the DID manifest adds `identus-crypto` without updating an accepted
  component contract and its exact dependency assertion
- **THEN** repository conformance fails even though the broad ring direction
  would otherwise permit the domain-to-domain edge

#### Scenario: Current DID manifest remains minimal

- **WHEN** repository conformance inspects the DID manifest
- **THEN** its internal dependency set equals `identus-core` plus the
  `identus-derive` proc macro and no crypto feature is transitively selected by
  DID

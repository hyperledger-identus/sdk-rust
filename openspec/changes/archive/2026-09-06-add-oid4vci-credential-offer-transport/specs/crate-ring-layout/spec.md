## ADDED Requirements

### Requirement: Focused OID4VCI protocol semantics are independently activated

The workspace SHALL classify `identus-oid4vci` as an implemented experimental
protocol-semantics crate owned by issue #111. It SHALL appear exactly once in
`LAYER_RULES` and in the root workspace dependency map. The quarantined
`identus-openid4vc` umbrella placeholder SHALL remain a separate marker-only
package and SHALL NOT become a dependency or facade for the focused crate.

`identus-oid4vci` SHALL depend at runtime only on `identus-core` among workspace
packages. It SHALL NOT depend on JOSE, DID, crypto, wallet, agent, adapter,
binding, verification, chain, or product crates until a focused later contract
accepts a narrower protocol capability requiring such an inward edge.

#### Scenario: focused crate and umbrella marker remain distinct

- **WHEN** the workspace manifests, rulebook, and bootstrap inventory are
  inspected
- **THEN** `identus-oid4vci` is implemented protocol semantics while
  `identus-openid4vc` remains a quarantined marker with no dependency edge
  between them

#### Scenario: transport dependency cone stays minimal

- **WHEN** conformance inspects the new package manifest and resolved runtime
  dependency graph
- **THEN** its only internal runtime dependency is `identus-core` and every
  external dependency is workspace-pinned

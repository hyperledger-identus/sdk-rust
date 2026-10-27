# ADR 0176: allow only the in-tree portable PRISM method package

- **Status:** Accepted as issue #521 implementation constraint
- **Date:** 2026-10-26
- **Decision authority:** ADR 0170 and issue
  [#521](https://github.com/hyperledger-identus/sdk-rust/issues/521)
- **Related:** ADR 0110, ADR 0162, ADR 0170, ADR 0174, ADR 0175, issue #524
- **Constraint impact:** material; narrows one dependency-identity prohibition
  without admitting an external chain, donor, product, or ledger dependency

## Context

The dependency guard predates the accepted platform-core direction. It treats
every PRISM-shaped package name as evidence of chain coupling, including the
exact local name `identus-prism-did`. ADR 0170 subsequently decided that a
method-specific Rust crate may own portable protobuf and deterministic DID
method semantics while ledger observation/submission, persistence and node
composition remain downstream.

Issue #521 follows that boundary: the proposed crate depends inward on generic
SDK DID and crypto primitives and general-purpose crates.io libraries. It does
not depend on NeoPRISM, Cardano, Pallas, Oura, Blockfrost, a database, a
network runtime, or a product repository. The old name-only rule therefore
rejects the accepted architecture while providing no additional source
protection.

## Decision

Allow the exact package identity `identus-prism-did` only when Cargo proves it
is a source-less local workspace package:

- the root declaration uses an in-repository path beneath `crates/`;
- member declarations inherit that workspace dependency;
- the resolved lockfile entry has no registry or Git source.

Continue rejecting:

- a version-only or registry-sourced `identus-prism-did` dependency;
- any Git source in the NeoPRISM or other prohibited donor families;
- `identus-prism-did-*`, `prism-*`, Cardano/Pallas/Oura/Blockfrost and other
  expanded method/ledger package identities;
- local paths outside the repository or workspace `crates/` directory.

The allowlist is closed, exact, and enforced at direct declarations and the
resolved lockfile. It authorizes a package location, not arbitrary PRISM
functionality. The crate's own OpenSpec, resource, layer and dependency-cone
contracts still exclude ledger and product behavior.

## Consequences

- The portable method crate can live under the canonical `identus-*` namespace
  without weakening donor-source or chain-family enforcement.
- A same-named crates.io package cannot silently replace the workspace crate.
- Future method crates need their own architecture decision; prefix matching
  does not create a wildcard exception.
- The guard becomes slightly more complex because it evaluates provenance as
  well as spelling, but that is the property the policy actually intends to
  protect.

## Alternatives rejected

- Rename the crate to hide `prism`: obscures ownership and evades rather than
  resolves the policy conflict.
- Remove PRISM from the denied families: would admit unrelated registry and
  ledger packages.
- Allow every in-tree prohibited family: too broad and not justified by issue
  #521.
- Keep PRISM method semantics in NeoPRISM: contradicts ADR 0170 and preserves
  duplicated portable behavior.

## Verification and rollback

Conformance tests cover the accepted root path, workspace inheritance and
source-less lock entry, plus rejected registry and expanded-package cases. The
real workspace must pass through the same functions.

Before publication, rollback removes `identus-prism-did`, its inventory entry
and this exact allowlist. After publication, rollback follows the consumer
change ledger and release policy; it does not widen the exception.

# Constraint impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/253
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`: the layered dependency direction is refined into mandatory
  module-level acceptance gates.
- `SDK-DEP-001`: third-party and donor implementations remain replaceable
  behind Identus-owned facades with explicit provenance and rollback.
- `SDK-SEC-001`: unsafe Rust remains prohibited without a dedicated safety
  decision.
- `SDK-SEC-003`: new or materially changed untrusted input boundaries remain
  explicitly bounded.
- `SDK-LIM-005`: product, chain, consent, custody, and trust policy remain
  downstream.
- `SDK-LIM-006`: SDK delivery and NeoPRISM adoption remain separately
  authorized, evidenced, and reversible.

## Introduced or changed constraints

The change introduces a reusable-module architecture guardrail. A candidate
cannot enter sdk-rust merely because its source is written in Rust, is small,
or appears generic. It must pass every hard ownership, dependency, cohesion,
portability, safety, provenance, conformance, consumer-evidence, and release-
independence gate in ADR 0110.

The guardrail is material because it controls cross-repository ownership,
public API boundaries, consumer responsibility, and future code removal. The
project sponsor directed the outcome in issue #253. No protected release,
publication, or repository setting is changed.

## Introduced or changed limitations

- The criteria do not certify any current NeoPRISM module as compatible.
- The existing `codex/sdk-rust-beta` branch is an experiment pinned to an old
  integration SHA; it is not a published SDK dependency or green adoption
  receipt.
- Two consumer-shaped proofs may use SDK-local examples before production
  adoption, so reusability evidence is not a claim that two products already
  ship the component.
- Apparent generic helpers remain downstream or rejected until a named SDK
  capability and credible independent use are evidenced.

## Consumer and product impact

NeoPRISM is explicitly authorized by issue #321 as the downstream adoption
laboratory, but this ADR phase remains read only. Future adoption occurs one
component slice at a time on the beta line, pinned to immutable SDK revisions.
Oxid, Midnight Identity, Lace ID Portal, and other consumers are unaffected and
remain read only unless separately authorized.

## Activation and rollback

The architecture rule becomes effective when the issue-linked PR merges to
`develop`. Reverting that PR removes the documentation rule without changing
runtime behavior. Each future extraction and adoption remains an independently
revertible PR. NeoPRISM keeps its compatibility facade or local implementation
until its replacement passes the downstream evidence gate; rollback repoints
the facade to the last proven implementation without rewriting SDK history.

## Evidence

Issue #253 is the exact durable sponsor direction. ADRs 0061 and the existing
SSI program establish the owned-facade and downstream-isolation constraints.
NeoPRISM issue #321 supplies separate downstream authorization. Exact SHA,
license, branch ancestry, diff size, manifests, and current candidate surfaces
are recorded in `research.md`.

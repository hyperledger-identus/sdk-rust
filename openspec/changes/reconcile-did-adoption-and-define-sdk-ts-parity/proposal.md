# Reconcile DID adoption and define SDK-TS parity

## Why

The SDK-Rust DID candidate passed its complete exact-revision slow line and the
NeoPRISM consumer adoption, fixed-boundary parity harness, E2E hardening, and
ownership documentation are merged. The public SDK-Rust readiness page,
machine backlog, and platform milestone registry still describe older pending
states. That makes completed engineering evidence look unfinished and obscures
the remaining administrator-owned publication decision.

The next roadmap also needs a sharper SDK-Rust/SDK-TS target. Repository-wide
or historical-module parity is misleading: portable semantics can converge in
Rust, while TypeScript must retain browser/Node integration, npm packaging,
plugin hosting, lifecycle, permissions, and product-facing ergonomics. The
roadmap therefore needs an explicit parity relation and conformance state for
every normalized capability rather than one percentage or an assumption that
the implementations and APIs should look alike.

## What changes

- Reconcile DID candidate readiness with the exact slow receipt and merged
  NeoPRISM adoption while keeping publication, stable API, and `main`
  promotion separate and unauthorized.
- Mark the delivered DID Core and DID port backlog rows truthfully and align
  completed A0/A1 plus current A2 milestone state.
- Add a machine-readable SDK-Rust/SDK-TS parity milestone covering every
  normalized SDK-TS capability.
- Classify each capability as `semantic-equivalent`, `compatible-adapter`,
  `platform-owned`, `intentionally-different`, `deferred`, or `retired`.
- Track conformance as `delivered`, `partial`, `blocked`, `excluded`, or
  `not-applicable`, with explicit standards/profile, evidence, deviations,
  consumer proof, rollback, and next issue.
- Add deterministic validation and mutation coverage and publish a human
  report grouped by evidence state instead of a repository-wide percentage.
- Establish local-first capability trains as the default implementation unit:
  cohesive integration PRs should normally target 5,000-10,000 meaningful
  changed lines and include implementation, adapters, vectors, migration,
  documentation, and tests behind reviewable internal checkpoints.

## Capabilities

### Added capability

- `sdk-ts-parity-conformance`: define the cross-language parity relations,
  conformance evidence, milestone exit rules, and non-claims.

### Modified capabilities

- `identus-platform-core-migration`: make SDK-TS parity an independently
  steerable capability program rather than a late whole-SDK rewrite.
- `sdk-documentation-site`: distinguish completed engineering qualification
  from still-protected publication authority in the DID readiness surface.

## Non-goals

No SDK-TS source is changed. No mdoc/IDR-022 work, protocol implementation,
binding default switch, package publication, tag, stable API claim, `main`
promotion, or SDK/service deprecation enters this change. Public Rust and
TypeScript APIs are not required to have identical shapes.

## Delivery

Issue #594 owns this planning and reconciliation slice. Issue #502 remains the
language-adoption program and #492 remains the first reversible SDK-TS DID
canary. The resulting parity milestone can begin capability by capability once
that capability's own Rust, binding, and consumer prerequisites are satisfied;
it does not wait for every SDK-Rust feature to exist.

Implementation trains are prepared and qualified locally before a candidate
is pushed. The 5,000-10,000 changed-line range is a planning target, not a
merge gate: generated, vendored, lockfile, and evidence-artifact churn does not
count, unrelated work must not be bundled to hit the range, and a train is
split when it has independent rollback, security/release authority, ownership,
or reviewability. Targeted checks may run at signed internal checkpoints; the
complete local gate runs on the assembled candidate, and hosted CI remains the
required final confirmation rather than the primary discovery loop.

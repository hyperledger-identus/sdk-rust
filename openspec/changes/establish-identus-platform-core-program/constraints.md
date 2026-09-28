# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/415
Constraint blockers: none

## Existing entries affected

ADR 0097 through ADR 0101 and the support policy classify current bindings as
experimental. ADR 0110 defines reusable-module extraction criteria. ADR 0061
keeps upstream engines private behind Identus-owned facades. This program does
not activate support or supersede those decisions; it adds migration,
compatibility, and repository-disposition governance.

## Introduced or changed constraints

Reusable Identus semantics SHALL have one authoritative Rust owner. Language
repositories SHALL retain only idiomatic facade, packaging, platform adapter,
application orchestration, and temporary compatibility responsibilities that
are explicitly inventoried.

Every capability SHALL have a stable identifier, current and target owner,
disposition, specification basis, test-authority classification, compatibility
classification, binding target, deprecation phase, legacy-bug decision, and
evidence links before implementation migration begins.

Every consumer-visible change SHALL enter a machine-readable migration ledger
before merge. Breaking changes, deprecations, and compatibility modes require
an announced replacement, detection path, migration evidence, earliest removal
release, and rollback. Non-breaking additions and fixes are recorded so release
notes can be generated from the same evidence.

No legacy behavior becomes normative solely because a test asserts it. Bug
simulation requires a released-consumer dependency, bounded behavior, explicit
opt-in or version scope, tests, telemetry/detection where feasible, owner, and
sunset. Security or privacy defects fail closed and SHALL NOT be silently
preserved as compatibility behavior.

## Introduced or changed limitations

The initial inventories are architectural surveys, not line-by-line parity or
support claims. Source line and test counts do not prove quality. The program
does not guarantee that every historical capability will move to Rust; some
will be dropped, deprecated, replaced upstream, or remain platform-specific.

No date-based repository archival, flag-day rewrite, automatic semantic
translation across FFI, or universal binding technology is promised. Service
ports and React Native production support require separate evidence.

## Consumer and product impact

No runtime consumer changes in this bootstrap. It establishes the evidence and
release-note machinery used by later migrations. Consumers ultimately keep
idiomatic TypeScript, Swift, Kotlin, and React Native APIs while reusable
behavior converges on the Rust core. Compatibility shims remain temporary and
observable rather than becoming a second permanent core.

## Activation and rollback

Activation requires issue-bound OpenSpec planning, accepted ADRs, immutable
repository snapshots, capability/test/change-ledger templates, per-SDK
milestones, public discussions, issue decomposition, factory validation,
signed/DCO commits, and green protected CI. Rollback removes only the program
artifacts; no runtime, package, persisted data, or external consumer changes
are made by this slice.

## Evidence

Issue #415, the three pinned SDK revisions, repository manifests and tests,
existing binding ADRs, issue #223's React Native gate, the new migration
roadmap, and GitHub discussions provide the bootstrap evidence. Future slices
must attach capability-specific conformance, consumer, target, and release
receipts.

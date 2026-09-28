# Establish the Identus platform-core program

## Why

Identus currently implements overlapping cryptography, DID, credential,
messaging, storage, and edge-agent behavior in TypeScript, Swift, and Kotlin.
That duplication multiplies security review, conformance, bug fixes, and release
work. `sdk-rust` should become the authoritative reusable implementation while
language repositories retain the idiomatic APIs and platform integration that
belong in their ecosystems.

Repository retirement is not the immediate goal. The program must first
normalize capabilities, identify trusted evidence, close Rust and binding gaps,
and migrate one bounded capability at a time without surprising consumers.

## What changes

- Define `sdk-rust` as the authoritative owner of reusable Identus semantics.
- Inventory `sdk-ts`, `sdk-swift`, and `sdk-kmp` at immutable revisions and map
  their capabilities, tests, compatibility behavior, and platform concerns.
- Assign every inventoried capability a disposition: retain in a platform
  shell, move to Rust, replace with an upstream dependency, deprecate, drop, or
  defer pending evidence.
- Classify tests by authority so normative and cross-language vectors become
  shared conformance evidence rather than copying implementation accidents.
- Record every consumer-visible change in a machine-readable migration ledger,
  including non-breaking changes, breaks, deprecations, substitutions, and
  intentional legacy-bug compatibility.
- Define phased deprecation and retirement gates instead of deleting language
  repositories by date or code-volume target.
- Treat Swift/Kotlin native bindings, browser/Node TypeScript bindings, and
  React Native TurboModule bindings as related but distinct adapter programs.
- Keep `cloud-agent` and `mediator` as later service-decomposition programs;
  this change records discovery work but does not port them.

## Capability

### Added capability

- `identus-platform-core-migration`: govern capability normalization,
  compatibility evidence, phased migration, and repository disposition.

## Non-goals

This change does not migrate production behavior, publish a binding, promise
API parity, archive a repository, port `cloud-agent` or `mediator`, or expand a
generic crate with product-, chain-, transport-, storage-, or runtime-specific
policy. It does not force a UniFFI or React Native dependency upgrade.

## Delivery

Issue #415 owns this program bootstrap. A planning-only commit and factory
preflight precede ADRs, inventories, roadmap documents, issue decomposition,
and GitHub discussions. Later capability migrations require their own issues,
OpenSpec changes, compatibility evidence, and protected pull requests.

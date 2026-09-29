# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/417
Constraint blockers: none

## Existing entries affected

ADRs 0162 through 0164 authorize capability-level migration and test-authority
classification. ADR 0061 keeps third-party engines behind Identus-owned types.
ADR 0073 conditionally permits an isolated current AnonCreds v1 adapter, while
ADR 0076 rejects the assessed draft-07 SD-JWT engine. SDK-LIM-002 keeps FFI
unsupported and SDK-LIM-003 distinguishes portable compilation from runtime
support. This change refines discovery order and target naming without
activating a dependency, binding, support, or retirement claim.

## Introduced or changed constraints

SDK-TS SHALL be inventoried first and SHALL establish the initial normalized
capability catalog. SDK-Swift SHALL follow as a parity and deviation review.
SDK-KMP SHALL follow those two as compatibility evidence and SHALL NOT be the
default source of new SDK-Rust behavior.

Apollo, Castor, Pollux, Mercury, Pluto, and EdgeAgent SHALL NOT be introduced
as SDK-Rust crate names, Rust module names, target capability identifiers,
public types, or ownership boundaries. They MAY remain in immutable provenance,
historical reports, explicit legacy-import compatibility, and temporary
language-SDK migration facades.

A donor implementation or dependency SHALL NOT override an accepted standard,
current profile, official vector, accepted SDK profile, or maintained upstream
Rust option. SD-JWT and AnonCreds dispositions SHALL identify exact current
profiles and evaluate maintained Rust engines rather than reproduce SDK-TS,
SDK-Swift, or SDK-KMP dependency versions.

Every SDK-TS capability record SHALL link immutable source evidence and record
public, wire, persistence, runtime, test, dependency, consumer, security,
target-owner, disposition, compatibility, and rollback data. Incomplete
evidence forces `defer`; it does not authorize speculative implementation.

## Introduced or changed limitations

The inventory is a source and architecture report, not public API parity,
conformance, package support, SDK-TS deprecation, or repository retirement.
GitHub source search does not prove all consumers or version usage. Test counts
do not prove coverage or authority. The inventory may nominate fixtures and
libraries, but separate issues must prove provenance, semantics, dependency
cones, targets, security, and consumer migration before activation.

## Consumer and product impact

No runtime consumer changes. Existing SDK-TS 8.1.4 names and packages continue
unchanged. Future TypeScript migrations may retain compatibility aliases during
an announced support window, but SDK-Rust exposes only normalized
responsibility-based names.

## Activation and rollback

Activation requires the issue-linked planning commit and factory preflight,
ADR 0169, an immutable report and machine-readable inventory, strict factory
checks, a distinct review, signed/DCO commits, and green protected CI.
Rollback removes documentation and registry changes only. No donor, package,
wire, persistence, or runtime state changes in this slice.

## Evidence

Issue #417 and its sequencing/naming comments, SDK-TS release 8.1.4 at the
pinned revision, repository manifests/exports/tests/workflows, npm release
metadata, public consumer references, accepted SDK-Rust ADRs and constraints,
RFC 9901, and the current AnonCreds program supply the decision evidence.

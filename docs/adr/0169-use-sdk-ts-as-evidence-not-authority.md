# ADR 0169: use SDK-TS as discovery evidence, not normative authority

- **Status:** Accepted
- **Date:** 2026-09-29
- **Decision authority:** issue
  [#417](https://github.com/hyperledger-identus/sdk-rust/issues/417)
- **Related:** ADR 0061, ADR 0073, ADR 0076, ADR 0162, ADR 0163,
  ADR 0164
- **Constraint impact:** material; governs cross-SDK discovery, deviation
  resolution, target naming, and implementation activation

## Context

SDK-TS 8.1.4 is the newest released Identus language SDK and therefore the
best first source for discovering current product capabilities and consumer
expectations. It is not a specification. Its implementation contains draft-era
protocol choices, TypeScript-specific integration, embedded Rust forks, and
historical component names whose boundaries do not consistently match the
responsibilities required by a reusable Rust SDK.

Starting from an older SDK would miss current behavior. Copying the newest SDK
would make release recency an accidental source of truth. Either approach can
preserve obsolete cryptography, protocol drafts, defects, or platform coupling.

## Decision

### Discovery sequence

Use SDK-TS first to establish the capability inventory, SDK-Swift second to
identify parity and meaningful deviations, and SDK-KMP last to recover required
compatibility with the oldest supported implementation. The sequence orders
research; it does not rank implementations as normative.

### Deviation authority

Resolve each deviation independently using this order:

1. final standards, errata, security requirements, and official conformance
   suites or vectors;
2. an accepted Identus profile or architecture decision where the standard
   intentionally leaves policy open;
3. current maintained upstream Rust implementations that satisfy the SDK's
   facade, dependency, target, resource, and security constraints;
4. reviewed Identus contracts and observed requirements of named released
   consumers;
5. interoperable implementation evidence across SDK-TS and SDK-Swift, then
   SDK-KMP compatibility evidence; and
6. donor implementation precedent.

An earlier item can be narrowed only by a documented profile allowed by that
authority. A legacy behavior that contradicts security, privacy, or a final
standard is fixed or rejected; consumer compatibility is handled through a
bounded facade when safe, not by corrupting the generic core.

Every unresolved deviation remains `defer`. It receives a bounded research or
implementation issue recording the governing version/profile, alternatives,
test authority, known consumers, migration, and rollback. Discovery never
silently activates a dependency or feature.

### Target architecture and naming

Name SDK-Rust capabilities and owners by responsibility and protocol, such as
`crypto.hd-derivation`, `did.document`, `credentials.sd-jwt`, or
`messaging.didcomm-v2`. Do not create public crates, modules, capability IDs,
types, or ownership boundaries named Apollo, Castor, Pollux, Mercury, Pluto,
or Edge Agent.

Those labels may appear only as immutable provenance, compatibility evidence,
or `source_aliases` describing where donor behavior was found. Language SDKs
may retain the labels during a separately governed transition period.

### Format-specific normalization

- SD-JWT base behavior targets RFC 9901. SD-JWT VC behavior pins and assesses
  its current profile separately; an SDK-TS npm dependency is not the Rust
  engine decision.
- AnonCreds targets the current AnonCreds 1.0 specification and maintained
  implementation evidence. SDK-TS's embedded revision is compatibility input,
  not an automatic dependency.
- DIDComm pins the selected DIDComm version and protocol profiles separately
  from SDK-TS's embedded engine and agent workflow implementations.
- Existing SDK-Rust behavior that is newer or more bounded than SDK-TS is not
  downgraded to achieve superficial parity.

## Consequences

- SDK-TS provides a complete, current discovery baseline without becoming a
  shadow specification.
- Swift and KMP research can amend the inventory with compatibility evidence
  without reopening every responsibility boundary.
- Standards and maintained Rust libraries can replace stale donor choices,
  while real consumer dependencies remain visible and deliberately migrated.
- Historical names remain searchable for provenance but cannot shape the new
  public architecture.
- Each deviation costs explicit analysis, which is intentional: parity is a
  set of reviewed decisions rather than a bulk port.

## Alternatives rejected

- Treat the latest SDK as normative: release recency does not prove protocol,
  security, or architectural correctness.
- Use the oldest SDK as the compatibility baseline: misses current capabilities
  and preserves the largest legacy surface.
- Require identical behavior across all SDKs before Rust work: contradictions
  cannot be resolved by consensus among implementations.
- Recreate historical component boundaries in Rust: produces low-cohesion
  modules and carries transitional branding into the target architecture.
- Select dependencies during inventory: bypasses exact supply-chain, target,
  facade, and conformance review.

## Verification and rollback

The SDK-TS inventory is machine-validated for immutable source revision,
responsibility-based IDs, finite dispositions and authorities, explicit
remaining platform ownership, compatibility risk, and decision basis. Deferred
or upstream-replacement rows link a follow-up issue before implementation.

If later evidence invalidates a preliminary disposition, update the inventory
and this decision's successor before implementation. Already published
compatibility evidence is retained. A consumer migration remains reversible
until its capability-specific acceptance and deprecation gates pass.

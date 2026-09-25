# Exact-diff evidence and architecture review

Review status: completed
Review date: 2026-09-26
Base: `develop@cd4d4eceef92cf18bb5c991f2957f2fac261d53e`
Implementation head: `920a3f2894755cc77963139b0646aec290a89b79`
Specification commit: `ba02af8f86affc291438953be0d2b8ced85c79b5`
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-implementation diff, issue #391,
ADR 0156, the exact candidate fixture and lock, the completed capability
matrix, all eight clean-room vectors, measurement qualifications, target and
supply-chain evidence, and rollback boundaries.

## Findings

1. **Acceptance coverage — accepted.** Every seam and evidence request in
   issue #391 maps to the report, fixture, ADR, or an explicit limitation.
2. **Architecture — accepted.** The evidence supports retaining
   `identus-oid4vci`, using an Identus-owned future OID4VP shell, and considering
   exact `siros-dcql 0.3.0` only as a private engine behind strict owned types.
   No candidate type or dependency enters a production graph.
3. **Protocol evidence — accepted with stated scope.** Eight independent
   positive and negative vectors cover matching, holder binding, duplicate
   identifiers, empty queries, JSON value types, required alternatives,
   enumeration bounds, and known parser tolerance. They are normative
   behavior probes, not interoperability or full OID4VP conformance claims.
4. **Security and resources — accepted.** Input bytes and public diagnostics
   are bounded/redacted at the research adapter. The report explicitly records
   the additional structural, inventory, match, and combination bounds needed
   before production adoption. No unsafe instrumentation was introduced.
5. **Measurements — accepted as diagnostics.** Dependency names, source-line
   proxy, clean compile wall time, maximum RSS, target compilation, and
   allocation structure are reproducible and qualified. None is presented as
   guaranteed deletion, runtime performance, device support, or a CI SLO.
6. **Compatibility and rollback — accepted.** This change adds documentation
   and isolated fixture tests only. Removing the fixture/report has no API,
   wire, persisted-data, release, or downstream migration cost.
7. **Documentation hygiene — corrected.** Exact-diff checking found three
   trailing-space markers in the report header; they were removed before the
   reviewed delivery commit.

## Residual limitations

- There is no production OID4VP implementation or named consumer yet.
- Browser/device runtime, network interoperability, signing, encryption, and
  end-to-end presentation exchange were not measured.
- Candidate allocation counts were assessed structurally rather than through
  an unsafe global allocator.

## Review decision

No unresolved correctness, evidence, security, privacy, compatibility,
architecture, dependency, or delivery finding remains. ADR 0156 does not need
amendment; production adoption remains gated by a separate issue and design.

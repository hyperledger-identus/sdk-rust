# Pre-implementation semantic and security review

Review status: completed
Review date: 2026-09-29
Base: develop@a5a54d6fac55912cd79d1f6112311375c96ccac3
Unresolved blockers: none

## Scope reviewed

The review inspected issue #447, the complete current OID4VP typestate,
OpenID4VP 1.0 Final with current errata, ADRs 0157/0158/0165, the private DCQL
facade, payload scanner, limit/error contracts, dependency graph, and proposed
composed state.

## Findings

1. **State correctness — accepted.** A new composed owner is safer than adding
   routing fields to the query engine or asking callers to retain raw payload.
   It preserves signature lineage and makes routing plus DCQL a prerequisite
   for later response work without claiming verifier trust.
2. **Parse ownership — accepted.** JAR verification already scans all JSON
   under strict bounds and rejects duplicate names. One later semantic map can
   feed routing and DCQL; a second semantic parse or payload copy is unnecessary.
3. **Profile scope — accepted.** Exact `vp_token` plus HTTPS `direct_post` is a
   cohesive first route. Redirects, encrypted responses, SIOPv2, DC API, and
   HAIP require different policy or crypto inputs and remain successors.
4. **Resource/privacy posture — accepted.** New nonce and URI limits compose
   with existing payload/DCQL limits. All retained verifier values need
   zeroizing ownership or sensitive accessors and static diagnostics.
5. **Compatibility/dependencies — accepted.** The change is additive on an
   unpublished crate, preserves the DCQL-only transition, and adds no package
   or dependency edge.
6. **Network boundary — accepted.** HTTPS syntax is routing evidence only.
   DNS, address policy, TLS, redirects, execution, and SSRF remain explicitly
   caller-owned.

## Review decision

The planning contract is bounded, reversible, internally cohesive, and ready
for immutable preimplementation evidence. No unresolved correctness, protocol,
security, privacy, compatibility, dependency, or architecture blocker remains.

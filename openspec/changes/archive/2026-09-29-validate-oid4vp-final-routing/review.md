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

# Post-implementation exact-diff architecture and security review

Review status: completed
Review date: 2026-09-29
Develop base: `a5a54d6fac55912cd79d1f6112311375c96ccac3`
Reviewed implementation: `cca83767d0a7bc896a2d43679d5fed3f017afbea`
Unresolved blockers: none

## Scope reviewed

The review re-read the complete base-to-implementation diff, the public API,
all new static error contracts, routing/DCQL parse ownership, limits, clean-room
tests, architecture inventories, ADR 0168 and the OpenSpec contract. It also
inspected the exact-head code-health report and every focused, workspace,
factory, MSRV and portable-target result.

## Findings

1. **Protocol boundary — accepted.** The transition accepts only exact
   `vp_token` plus `direct_post`, validates required nonce and response
   destination in contractual order, and composes the already correlated JAR
   client evidence with one strict DCQL query. It does not claim complete
   client-prefix authorization or broader response-profile conformance.
2. **Parse and state ownership — accepted.** One bounded duplicate-safe payload
   is semantically deserialized once. The same owned request map feeds routing
   and the private DCQL constructor; raw JSON is destroyed with the consumed
   verified state and is not recoverable from the result.
3. **Least authority — accepted.** The result proves syntax and request
   coherence, not verifier trust or permission to contact the endpoint. Network
   execution, DNS/address policy, TLS, redirects, timeout/retry and SSRF defense
   remain caller-owned and are stated on the sensitive URI accessor.
4. **Resource and privacy posture — accepted.** New retained strings have
   independent positive byte ceilings and zeroizing storage. Safe enums and
   count/length evidence are public; controlled strings require explicitly
   sensitive accessors. Error, display and debug paths remain static/redacted.
5. **Compatibility and dependency boundary — accepted.** The API and error
   catalogue changes are additive on an unpublished crate. The query-only
   transition remains behavior-compatible. No manifest, lockfile, dependency,
   feature, unsafe/native, runtime, transport or downstream edge changes.
6. **Cohesion and decomposition — accepted.** Production behavior is isolated
   in a 244-line routing module with small private seams in existing owners.
   The exact-head code-health audit reports no new OID4VP function or module
   signal and no production module crosses the 1,000-line attention threshold.
7. **Successor ownership — accepted.** Issue #487 now owns bounded direct-post
   response construction, and IDR-024 points to that open successor. This slice
   does not pre-empt its presentation-result model or wire-encoding decision.

## Residual limitations

- HTTPS parsing is not endpoint authorization or SSRF safety and no network
  interoperability result is claimed.
- Client-prefix key authorization, verifier trust, freshness/replay, consent,
  credential verification and presentation generation remain caller-owned or
  later slices.
- Redirect modes, `direct_post.jwt`, JWE, SIOPv2, DC API, HAIP and broader
  format policy remain unsupported.
- Hosted Linux exact-head CI is still mandatory before merge.

## Final disposition

The implementation is cohesive, bounded, reversible and least-authority. No
unresolved correctness, protocol, security, privacy, compatibility,
architecture, dependency or delivery finding remains for hosted review.

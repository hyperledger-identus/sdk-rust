# Exact-diff protocol, architecture and security review

Review status: completed
Review date: 2026-10-28
Base: develop@d33321e1dd6ae47cad934d34e6ba87b366f82190
Specification commit: d201379d9300d6852620d69d7104fd31f80a0239
Implementation head: 7309ae46866c981c85443f3da738bfa67069e5d5
Unresolved blockers: none

## Scope reviewed

The review inspected the complete exact-base diff, issue #544 and parent #541,
ADR 0183, canonical and delta specifications, security-policy state ownership,
public API and error contracts, test/E2E/property/fuzz/benchmark evidence,
resource and quality registries, portable targets, and factory fixtures.

## Findings and decisions

1. **Protocol authority — accepted.** OpenID4VP 1.0 Final and RFC 7519 remain
   normative. The SDK does not inherit TypeScript DTO or error shapes.
2. **Security transition — accepted.** Typed routing is available only from
   consumed `RequestObjectSecurityEvidence`; raw verified state cannot route.
3. **Audience binding — corrected and accepted.** The Request Object must carry
   exactly one audience equal to the caller's discovery audience. Additional
   recipients fail closed and Request Object `iss` is not treated as authority.
4. **Temporal semantics — corrected and accepted.** Integral `iat` is required;
   `exp`, `nbf`, and `jti` are optional but bounded. Checked arithmetic prevents
   overflow, and a request is expired exactly at `exp + allowed_clock_skew`.
5. **Replay ownership — accepted.** Replay checking is an async atomic
   caller-owned port. Unavailable storage fails closed and an accept-once stateful
   double proves the required semantic without claiming a durable adapter.
6. **Resource and privacy boundary — accepted.** Existing bounded JWT parsing
   remains authoritative, new claims are bounded, and public errors and Debug
   surfaces do not expose claim values, JWT IDs, or payload bytes.
7. **Dependency decision — accepted.** The slice reuses the existing JOSE
   boundary and adds no JWT framework or runtime/network dependency.
8. **E2E and adversarial evidence — accepted.** Direct-post and encrypted
   direct-post flows cross the new transition; boundary generation, compile-fail
   routing, fuzz, benchmark, and differential fixtures cover the declared risks.
9. **Portability — accepted with bounded claim.** WASM, iOS, and Android compile
   evidence passed. Linking, packaging, runtime, and language-facade support are
   explicitly excluded.
10. **Rollback — corrected and accepted.** Rollback removes the security-gated
    successors together with the new evidence transition; it never restores the
    earlier unsafe direct-routing surface.
11. **Quality declaration — accepted.** Request-security obligations are
    satisfied by named evidence. The stale unrelated crypto fuzz evidence stays
    visible and required under #525.
12. **Delivery scope — accepted.** The change is SDK-only, chain-agnostic, and
    does not implement verifier trust, persistence, transport, UI, or downstream
    adoption.

## Independent review sequence

Claude CLI was quota-limited, so Codex CLI performed the required independent
review under the documented fallback rule. Three remediation passes closed all
medium findings and documentation contradictions. The final exact-head delta
review reported no blocker, high, or medium issue.

## Residual limitations

- A production replay store and clock adapter remain adopter responsibilities.
- Verifier-key trust and authorization remain separate work under issue #545.
- Performance values are local baselines and portable evidence is compile-only.
- Hosted exact-head CI, PR review, metrics publication, and guarded merge remain
  repository delivery evidence rather than local correctness evidence.

## Review decision

The implementation is bounded, fail-closed, target-neutral, documented, and
supported by proportionate E2E and adversarial evidence. No unresolved protocol,
correctness, architecture, security, privacy, dependency, or delivery finding
blocks the issue-linked pull request.

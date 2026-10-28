# Exact-diff architecture and security review

Review date: 2026-10-28
Issue: #545
Develop base: `f0edaf820b3bc12b45faae02b60c55db3d3040ff`
Planning contract: `d3a17eb3baf74652045f823dc988b20eeaf70711`
Preimplementation receipt: `9cfbcdfe860b2ee47ed2704e00d302ec6ce912c8`
Implementation under review: `0c194d4a365ae9dc8b51efe2f8abb40eb54ac6ba`
Reviewer: Codex CLI, used under the repository's Claude-or-Codex fallback rule

## Resolved findings

### R-1 — The trust port lacked the signed direct-post destination

Severity: P1 before correction

The first independent review found that attestation and X.509 policy adapters
received the verified key and client identifier but not the signed
`response_uri`. They therefore could not enforce the verifier-to-destination
binding required by those trust profiles without reparsing the payload outside
the protocol boundary.

Resolution: `VerifierTrustInput` now carries a borrowed, exact, redacted-by-
default view of the signed response URI. The clean-room E2E policy asserts the
expected destination, while the ADR, architecture note and OpenSpec contract
make destination binding a caller-owned trust decision. Commit:
`c7bbf3705ce644bfb24e5f1855cc6e2a90bb60e5`.

### R-2 — Two existing fuzz targets bypassed verifier authorization

Severity: P1 before correction

The remediation review found that the older direct-post fuzz binaries still
constructed routing state directly from `RequestObjectSecurityEvidence`. That
left two build targets inconsistent with the new consuming authorization
transition and prevented the complete fuzz package from compiling.

Resolution: both targets now use the shared accepting verifier-authorization
helper before typed routing. All three OID4VP fuzz binaries compile and pass a
4,096-run smoke, and the dedicated Request Object target passed a 300-second
ASan soak. Commit: `f8868f315dbbc8ef2dc6db419a4dd28c9559a211`.

### R-3 — Federation trust chains were read from the payload

Severity: P2 before correction

The final branch review found that `openid_federation` authorization read
`trust_chain` from the Request Object payload even though OpenID Federation
places that evidence in the protected JWS header. A conforming chain was
therefore omitted from the trust-port input, while a payload substitution could
be accepted.

Resolution: verifier authorization now borrows the already bounded chain from
the verified protected header, applies its own profile limits, rejects a chain
for every other client-id scheme, and rejects any payload `trust_chain` before
invoking the policy. The scheme E2E test now constructs protected-header
Federation evidence and includes a negative payload-substitution regression.
Commit: `736601b758a1bf24ce53ef090b863c5e21a66682`.

### R-4 — Verifier-info string limits crossed the DCQL boundary

Severity: P2 before correction

The exact-branch remediation review found two inconsistent explicit-limit
paths. String-valued `verifier_info.data` was governed only by the aggregate
metadata-byte ceiling, while every DCQL credential identifier was governed by
the verifier-info string ceiling even when no verifier-info entry referenced
it.

Resolution: string-valued verifier-info data and referenced credential IDs now
obey the verifier-info string ceiling. Unreferenced DCQL identifiers remain
owned by `DcqlLimits`, so a narrow verifier-info policy cannot reject an
otherwise valid query. Focused regressions cover both boundaries. Commit:
`0c194d4a365ae9dc8b51efe2f8abb40eb54ac6ba`.

## Architecture review

- The protocol crate owns scheme parsing, bounded verifier metadata and the
  consuming verifier-authorization state transition. Trust remains behind an
  async caller port; no network, certificate, federation or DID resolver is
  embedded in the protocol core.
- The exact internal dependency cone is `identus-core`, `identus-crypto`,
  `identus-did`, `identus-jose`, `identus-openid4vc` and
  `identus-presentations`. The conformance guard freezes the cone and reverse
  edges.
- No third-party JOSE or DID type crosses the public API. The exact verified
  SDK JWK is retained as evidence rather than re-derived by a trust adapter.
- Typed authorization routing consumes verifier authorization evidence; callers
  cannot reach the routing transition from signature-only evidence.
- The change is runtime-neutral and does not add HTTP, storage, chain, product,
  platform UI or durable trust-store behavior.

## Security review

- Final-defined signed-JAR schemes are explicit. `redirect_uri`, `origin` and
  unknown first-colon prefixes fail closed.
- Metadata, verifier information, trust-chain entries and collection sizes
  have independent positive limits. Duplicate, ambiguous and misplaced
  evidence is rejected before a trust callback.
- The trust callback receives the classified scheme, exact signed client
  identifier, verified public key, signed response destination and bounded
  profile evidence. An accepting callback is evidence of a caller decision,
  not an SDK trust claim.
- Debug, display and error surfaces expose only static or structural data; key,
  identifier, metadata and destination canaries remain absent.
- The implementation introduces no `unsafe`, ambient I/O, filesystem,
  environment, transport or panic-based validation path.

## Final review status

All four independent review findings above are resolved. A final Codex CLI
review of exact implementation head
`0c194d4a365ae9dc8b51efe2f8abb40eb54ac6ba` reported no actionable defects.
The review sandbox could not acquire Cargo's build lock, so independently run
focused, workspace, fuzz, benchmark and Nix evidence remains the executable
verification authority.

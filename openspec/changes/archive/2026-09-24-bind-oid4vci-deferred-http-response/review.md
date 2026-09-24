# Exact-diff architecture and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@b504d671142486490eb8a91ff320143cac5cee7a
Implementation head: 916fa58ac5774a9e14266dedfc18900d0bf165f3
Reviewed head: 916fa58ac5774a9e14266dedfc18900d0bf165f3
Specification commit: 7802f2bded31fa12438da994f2a98780b6825369
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #345, ADR 0138,
OpenID4VCI 1.0 Final section 9.2, the immutable preimplementation receipt,
request-owned transaction state, composite limits, the issued/pending outcome,
the error-catalogue suffix and all focused and workspace verification output.

## Findings

1. **Protocol behavior — accepted.** Exactly `200` selects the existing
   immediate response parser and exactly `202` selects the existing deferred
   response parser. Both require bounded `application/json`; unsupported status
   is rejected before any untrusted metadata or body is parsed.
2. **Correlation — accepted.** The originating opaque transaction identifier is
   retained under `Zeroizing<String>` and a pending response is returned only
   after exact equality with the request identifier. Issued responses make no
   unsupported proof-count or credential-cardinality claim.
3. **Security and privacy — accepted.** Public diagnostics remain static. The
   request, outcome, response and bridged-error Debug/Display surfaces do not
   disclose transaction identifiers, credentials or caller canaries. All
   caller-supplied fields and bodies remain explicitly bounded.
4. **Architecture — accepted.** The slice reuses existing body parsers and the
   shared Content-Type recognizer. It introduces no HTTP client, TLS, bearer
   token, timer, retry, persistence, trust, format, chain or product coupling.
5. **Compatibility — accepted.** The API addition is append-only, the immutable
   171-entry v1 error fixture remains unchanged, and the five live error entries
   follow ADR 0137's append-only suffix rule. No manifest, dependency, feature,
   lockfile, unsafe/native, wire or target contract changes.

## Residual limitations

- Error responses such as `invalid_transaction_id` and
  `credential_request_denied` are not handled by this success-response slice.
- Scheduling, retry, invalidation and terminal transaction policy remain with
  the caller until a separately specified lifecycle component exists.
- Encrypted request and response handling remains a later JOSE/JWE milestone.

## Review decision

The implementation is cohesive, chain-neutral and bounded to request-bound
success-response validation. No unresolved correctness, security, privacy,
compatibility, architecture, dependency or delivery finding remains.

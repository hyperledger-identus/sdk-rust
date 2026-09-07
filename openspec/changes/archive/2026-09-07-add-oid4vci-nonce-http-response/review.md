# Semantic contract review

## Scope reviewed

- Issue #137 and exact base
  `38c0fc4313ef735fe564a1cb5080af38f4d2bf14`.
- OpenID4VCI 1.0 Final section 7.2, RFC 9110 field/media grammar and RFC 9111
  Cache-Control/no-store semantics at their recorded immutable hashes.
- Existing request, response core, limits, static errors, dependency boundary,
  target policy and consumer isolation.

## Findings

1. **Resolved — the transport envelope is normative rather than policy.**
   Final requires 2xx, `application/json` and Cache-Control including
   `no-store`; checking those inputs does not select product trust or network
   behavior.
2. **Resolved — request binding does not overclaim provenance.** The method is
   available only on a validated request, but its contract states that the
   caller still proves actual execution, endpoint origin and network policy.
3. **Resolved — retry policy remains external.** Borrowing rather than consuming
   the request permits retry without claiming that replay or remediation is
   safe.
4. **Resolved — headers are bounded and structurally parsed.** Independent
   positive limits stop field amplification. Token, parameter, quoted-string
   and list parsing prevents substring, quoted-delimiter and injection
   confusion without adding an HTTP library.
5. **Resolved — JSON classification is interoperable but narrow.** Type and
   subtype comparison is case-insensitive as RFC 9110 requires. Valid
   parameters do not change `application/json` classification and are ignored;
   multiple or malformed values fail without sniffing.
6. **Resolved — uncacheability requires the exact directive.** Only a bare
   case-insensitive directive named `no-store` passes. Extension names,
   arguments and quoted occurrences cannot impersonate it.
7. **Resolved — validation order protects body diagnostics.** Limits, status,
   media and cache checks precede the existing body parser, so a transport
   failure neither interprets nor exposes a malicious body.
8. **Resolved — optional DPoP remains a separate capability.** Discarding or
   interpreting DPoP here would create proof/lifecycle policy; the adapter
   retains it until an issue-first DPoP contract exists.
9. **Resolved — compatibility and provenance stay bounded.** The API is
   additive and unpublished; no dependency, feature, manifest, target,
   consumer, donor source or fixture changes.

## Decision

The proposal, design, capability requirements, program replacement and task
map are semantically complete, objectively testable, reversible and within the
standing mandate. No correctness, security, privacy, compatibility,
provenance, target or product-scope blocker remains before implementation.

# Exact-diff implementation review

## Review boundary

- Base: `38c0fc4313ef735fe564a1cb5080af38f4d2bf14`
  (`origin/develop`).
- Reviewed implementation head:
  `3f9cca16173339f33c1ae5eca70d6c1ebdafcd34`.
- Reviewed every changed path and the complete `origin/develop...HEAD` diff,
  including the public API, validation order, private byte parser, error
  bridges, tests, ADR, blueprint, backlog replacement and OpenSpec artifacts.
- Confirmed `Cargo.toml`, `Cargo.lock`, and `crates/oid4vci/Cargo.toml` are
  byte-identical to the base.

## Findings

1. **Resolved — Cache-Control whitespace did not initially follow the exact
   directive grammar.** A first focused test exposed acceptance of whitespace
   before `=`. Commit `1aeac284f54b439f63bccfb47938e3963882fc43`
   contains the correction and a regression case; malformed field input now
   fails before body parsing.
2. **Resolved — empty media parameter elements are valid RFC 9110 grammar.**
   Exact normative review found that `parameters = *( OWS ";" OWS [ parameter ] )`
   permits empty elements. Commit
   `da0faa762167f95be846eb9b2ae0d9ce37dadbad` aligns the parser, contract and
   regression corpus without accepting malformed named parameters.
3. **Resolved — generated TOML needed canonical layout.** The first Nix run
   reached and passed the release, target and supply-chain work before Taplo
   rejected only `archive-intent.toml` formatting. Commit
   `3f9cca16173339f33c1ae5eca70d6c1ebdafcd34` applies the repository formatter;
   the complete rerun passed all 27 native checks.
4. **Verified — the parser consumes complete effective field values.** It
   rejects comma-combined Content-Type, malformed tokens, illegal OWS around
   parameter/directive `=`, unterminated or illegal quoted strings, control
   injection, qualified `no-store`, and substring or quoted impersonation.
5. **Verified — the API remains transport-neutral and least-authority.** It
   borrows the request, accepts only decoded effective values, retains none of
   them, performs no I/O, and delegates the body only after status and header
   validation.
6. **Verified — resource and diagnostic behavior is explicit.** Independent
   positive byte bounds precede syntax work; all new errors are fieldless,
   stable, redacted, and mapped to the OID4VCI capability.
7. **Verified — compatibility and scope are additive.** Runtime dependencies,
   features, manifests, lockfile, target policy, consumers, chain-specific
   repositories, publication and release state are unchanged.

## Decision

No unresolved correctness, security, privacy, compatibility, provenance,
target, test, documentation or product-scope finding remains. The exact diff
is locally approved for guarded archive and ready PR delivery to `develop`.

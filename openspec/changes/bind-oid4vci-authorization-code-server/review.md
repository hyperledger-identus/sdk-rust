# Exact-diff architecture and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@79ab576281bdb07071c9e96ed8c5d24fa2113d0b
Implementation head: e4ca6257f81c24bc5a0364b2d3fb5676e0e630fc
Reviewed head: e4ca6257f81c24bc5a0364b2d3fb5676e0e630fc
Specification commit: feaaa4ecf15d49361ff1f9ebee22c24928a6a9c5
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #350, ADR 0140,
OpenID4VCI 1.0 Final sections 4.1, 5.1 and 12.3, RFC 8414 section 2, the
immutable preimplementation receipt, semantic check precedence, error
catalogue decomposition, public compatibility and all focused/workspace
verification.

## Findings

1. **Protocol behavior — accepted.** The transition requires an offered
   Authorization Code grant, exact effective server advertisement, exact
   optional hint agreement, effective exact `authorization_code` support and
   a validated Authorization Endpoint in deterministic order. Omitted grant
   metadata uses the existing RFC 8414 default.
2. **Least authority — accepted.** The resulting owned state carries only the
   already-validated offer/issuer metadata and selected partial server
   metadata. A Token Endpoint is not required until code exchange; client,
   redirect, state, PKCE, PAR and transport inputs are absent.
3. **Reuse and cohesion — accepted.** The implementation composes existing
   grant, issuer-metadata and server-metadata types without reparsing JSON or
   copying remote values. Four new errors occupy a focused seventh private
   catalogue, preserving the 39-record maximum for every catalogue.
4. **Compatibility — accepted.** All public behavior is additive. Four
   fieldless variants and stable codes append after the immutable error
   prefix. No existing discriminant, code, message, dependency, feature,
   lockfile, unsafe/native, wire, target or stored-data contract changes.
5. **Security and privacy — accepted.** Optional `issuer_state` remains in its
   existing zeroizing owner. Debug and errors expose no issuer state, server
   identifier, endpoint, JSON or canary text. The API explicitly does not
   claim metadata provenance, trust, reachability or callback mix-up safety.

## Residual limitations

- Authorization Request inputs and serialization remain issue #352.
- PKCE/CSRF generation, PAR, browser/callback handling and Final section 12.3
  response binding remain caller-owned or later issue-scoped behavior.
- Discovery, HTTP/TLS, token exchange, trust, persistence and successful
  issuance are not provided.

## Review decision

The implementation is a cohesive additive state transition with deterministic
diagnostics, bounded ownership and no unjustified authority. No unresolved
correctness, security, privacy, compatibility, architecture, dependency or
delivery finding remains.

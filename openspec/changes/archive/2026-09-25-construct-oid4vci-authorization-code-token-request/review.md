# Exact-diff architecture, security and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@75b3a60bb58ed185e88109516767b7e51977e317
Implementation head: 62bca407cd54c63a8db473b89a778f53fb6987ca
Reviewed head: 62bca407cd54c63a8db473b89a778f53fb6987ca
Specification commit: 6b91c5be533f2a520f9ce074d62ef10578ff5ef8
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #358, ADR 0144,
OpenID4VCI Final section 6.1, RFC 6749 sections 3.2.1 and 4.1.3, RFC 7636
section 4.5, RFC 9700 section 2.1.1, the immutable preimplementation receipt,
value ownership/decomposition, validation precedence, exact form output, error
catalogue evolution, public compatibility and focused/workspace evidence.

## Findings

1. **Protocol behavior — accepted.** Only a correlated success can construct
   the request. The explicitly named unauthenticated public-client transition
   always emits exact `grant_type`, code, redirect URI, client ID and verifier
   in deterministic RFC-base-plus-PKCE order for the selected Token Endpoint.
2. **Least authority — accepted.** Construction consumes the code lineage,
   drops state, PKCE challenge, request URI and the complete Credential Offer,
   and retains only public issuer/server metadata, the selected configuration,
   exact issuer evidence and the zeroizing body required by the next boundary.
3. **Resource and failure behavior — accepted.** Positive endpoint/body limits
   are independent and inclusive. Missing endpoint, endpoint overflow and
   checked body overflow have deterministic precedence. The final review
   removed invariant `expect` calls from shipping code; construction and public
   access are panic-free under safe inputs.
4. **Security and privacy — accepted.** The body is zeroizing and available
   only through an explicitly sensitive accessor. Debug and all static errors
   omit endpoint, code, verifier, redirect, client, metadata and body values.
   `NotAdvertised` is preserved rather than upgraded into mix-up evidence.
5. **Reuse and cohesion — accepted.** The implementation reuses the private
   exact form codec and generic Token Request transport constants. Three new
   errors occupy a focused private catalogue. No dependency, feature, lockfile,
   unsafe/native code, I/O or transport authority is added.
6. **Compatibility — accepted.** The public API and three errors are additive
   and unpublished. Existing error order/prefix, parsers, wire forms, defaults,
   dependencies, targets and stored-data behavior remain exact.

## Decomposition note

The factory reports 29 changed paths and 1,157 text lines, exceeding both
review-guidance thresholds. The executable delta is one 162-line protocol
module, one focused six-test file, one 27-line private error catalogue and
small consuming helpers/exports. The balance is the mandatory planning
contract, ADR, inventories and archived capability deltas. Splitting those
artifacts would temporarily separate the public transition from its resource,
error, ownership and roadmap contract, so this remains one cohesive and
independently reversible slice.

## Residual limitations

- The selected Authorization Server's support for an unauthenticated client is
  caller-established; no `token_endpoint_auth_method` metadata is modeled.
- Confidential-client methods, Authorization Details narrowing, DPoP, HTTP,
  retry and response binding are absent.
- Endpoint reachability, server trust, authorization, token provenance/storage
  and successful issuance are not claimed.
- Request-bound Token Endpoint HTTP response handling remains issue #360.

## Review decision

The exact diff is a bounded additive public-client request transition with
deterministic output, least-authority state, static diagnostics and no
unresolved correctness, security, privacy, compatibility, architecture,
dependency or delivery finding.

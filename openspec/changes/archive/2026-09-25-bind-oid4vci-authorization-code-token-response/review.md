# Exact-diff architecture, security and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@e5cdbc3ccdac110acb3e36384045e288ce0d440b
Implementation head: fb944e7973d4328d1c9b78d934f2f4c5b44ef5d6
Reviewed head: fb944e7973d4328d1c9b78d934f2f4c5b44ef5d6
Specification commit: 41314c8b2227451cf8c951fdfcd025d6ae93f666
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #360, ADR 0145,
OpenID4VCI Final sections 6.2 and 6.3, RFC 6749 sections 5.1 and 5.2, RFC
9110 response/media semantics, RFC 9111 cache semantics, the immutable
preimplementation receipt, ownership and secret lifetime, validation
precedence, HTTP field reuse, response lineage, error-catalogue evolution,
public compatibility and focused/workspace/portable evidence.

## Findings

1. **Protocol behavior — accepted.** Exact status selects the parser before
   any body inspection: `200` is success, `400` is an OAuth error, and `401`
   is an OAuth error restricted to exact `invalid_client`. Every other status
   fails before untrusted headers or body are parsed.
2. **Ownership and lineage — accepted.** Binding consumes the request exactly
   once and drops its zeroizing form body before remote validation. Either
   outcome owns the exact retained Credential Issuer, Authorization Server,
   selected Credential Configuration and unchanged issuer-identification
   evidence; callers cannot substitute lineage at this boundary.
3. **Resource and failure behavior — accepted.** Positive independent limits
   cover each effective header value and the existing success/error body
   policies. Status, Content-Type, Cache-Control, Pragma and body failures have
   deterministic precedence. Exact and one-over header and both body limits
   are exercised.
4. **Security and privacy — accepted.** Both branches require strict bounded
   JSON media type plus bare `no-store` and `no-cache` directives. Debug and
   static diagnostics omit code verifier, token, endpoint, metadata, headers
   and remote body values. No response-provenance or cache-compliance claim is
   inferred from caller-supplied values.
5. **Reuse and cohesion — accepted.** The implementation reuses the existing
   bounded Token Response and Token Error Response cores and generalizes the
   private directive scanner while preserving its earlier `no-store` wrapper.
   Nine new errors occupy one focused private catalogue. No dependency,
   feature, lockfile, unsafe/native code, I/O or transport authority is added.
6. **Compatibility — accepted.** The public types, transition and errors are
   additive and unpublished. Existing parser behavior, error prefix/order,
   defaults, wire forms, target policy and stored-data behavior remain exact.

## Decomposition note

The factory reports 26 changed paths and 1,695 text lines, exceeding both
advisory thresholds. The executable delta is one 219-line protocol module,
one focused eight-test file, one 62-line private error catalogue and small
request, limit, parser and export additions. The balance is the mandatory
planning contract, receipt, ADR, architecture inventories and roadmap
evidence. Splitting these artifacts would temporarily detach the public
transition from its status, resource, error, ownership and successor contract,
so this remains one cohesive and independently reversible slice.

## Residual limitations

- Callers still own response origin, TLS, redirects, decompression, header
  combination, timeouts, cancellation and retry policy.
- Client authentication, DPoP, token verification, authorization, storage and
  issuance completion are absent.
- A syntactically valid cache directive is evidence in the supplied response;
  it does not prove that an HTTP client or intermediary honored it.
- Token Authorization Details correlation with the selected Credential
  Configuration remains focused successor issue #362.

## Review decision

The exact diff is a bounded additive request/response transition with
deterministic status and field precedence, early request-secret erasure,
preserved lineage, static diagnostics and no unresolved correctness, security,
privacy, compatibility, architecture, dependency or delivery finding.

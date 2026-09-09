# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-09
Base: develop@a8fdedc8b39aa93df807a88c7dc4e861cf5367df
Implementation head: 29a9c47
Specification commit: 4206f90
Implementation commit: 29a9c47
Unresolved blockers: none

## Scope reviewed

The review inspected the complete `develop@a8fdedc...29a9c47` diff, issue
#241, OpenID4VCI 1.0 Final sections 9 and 12.2.4, ADR 0107, public metadata
surface, strict scanner, limits/errors, sixteen metadata tests, ledgers and
local verification.

## Findings

1. **Normative shape — accepted.** The optional member is retained only when it
   is exactly one non-empty JSON string. Omission remains successful and does
   not infer a fallback endpoint.
2. **Endpoint policy — accepted.** The established validator requires an
   absolute HTTPS URL with a host, rejects userinfo/fragments and admits the
   Final-permitted port, path and query components.
3. **Resource behavior — accepted.** The complete metadata byte/depth/node
   budgets and decoded duplicate-name checks remain active. The shared endpoint
   byte ceiling applies independently to the new retained value.
4. **Type boundary — accepted.** A distinct `DeferredCredentialEndpoint`
   prevents accidental substitution for Credential or Nonce Endpoint state and
   exposes only an exact borrowed string.
5. **Privacy and diagnostics — accepted.** Endpoint and metadata Debug expose
   presence/shape only. New oversize and unsafe errors are fieldless, stable,
   capability-attributed and contain no caller data.
6. **Compatibility — accepted.** Existing metadata inputs, accessors, exact JSON
   retention and the ten-argument limits constructor are unchanged. The API is
   additive and unpublished.
7. **Architecture and supply chain — accepted.** The change reuses the current
   scanner and validator and adds no dependency, feature, unsafe, runtime,
   transport, storage, FFI or target authority.
8. **Roadmap evidence — accepted.** The stale machine-readable IDR-023 pointer
   is corrected from #149 to the active #241 child while status remains
   `in_progress`; no completion claim is made.

## Residual limitations

- Syntax does not prove retrieval provenance, issuer control, reachability or
  deployment network safety.
- Deferred request construction, token/transaction validity, interval
  scheduling, correlation, invalidation, encryption and retry remain absent.
- Target compilation is not runtime/device/product support evidence.

## Review decision

The slice is cohesive, additive, bounded, redaction-safe and independently
reversible. No unresolved correctness, security, privacy, compatibility,
architecture, dependency or delivery finding remains before hosted review.

# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-09
Base: develop@52221cc0e9d2f2cef7c6c1a6eef8e3eddfc98810
Implementation head: a0da98d
Specification commit: 16ade1a
Implementation commits: 755cda9 and a0da98d
Unresolved blockers: none

## Scope reviewed

The review inspected the complete `develop@52221cc...a0da98d` diff, issue
#239, OpenID4VCI Final requirements, ADR 0106, both public constructors, shared
request builder, stable errors, eleven focused tests, ledgers and local gates.

## Findings

1. **Typed authority — accepted.** The additive constructor accepts only
   `TokenResponseWithAuthorizationDetails`; no raw caller-provided dataset
   identifier can cross this API boundary.
2. **Selection and binding — accepted.** Both source-order indices are checked.
   The selected detail's configuration must exactly match one configuration in
   the already matched offer before any request is constructed.
3. **Wire exclusivity — accepted.** The authorized route emits exactly
   `credential_identifier`; the compatible route continues to emit exactly
   `credential_configuration_id`. Both share deterministic JSON-string
   escaping and stable proof ordering.
4. **Resource behavior — accepted.** The existing positive proof-count,
   per-proof, complete JSON-body and complete Authorization limits apply to both
   selectors through one private builder. Checked arithmetic is retained.
5. **Token and proof behavior — accepted.** Both routes enforce the same
   case-insensitive Bearer type, RFC 6750 token grammar and non-empty bounded
   holder-produced proof collection.
6. **Privacy and diagnostics — accepted.** Access tokens, proof bytes,
   endpoints, configuration IDs and dataset identifiers remain absent from
   Debug and all fieldless stable errors. Owned authorization and body bytes
   remain zeroizing.
7. **Compatibility and architecture — accepted.** The existing constructor and
   request type are unchanged. The capability adds no dependency, feature,
   unsafe, runtime, transport, storage or product-policy authority.
8. **Delivery scope — accepted.** No donor/downstream mutation, publication,
   trust assertion or release claim occurs.

## Residual limitations

- The SDK does not choose which recognized detail or dataset a product should
  request.
- Local offer correlation does not prove issuer, token, metadata, dataset,
  proof or credential trust.
- HTTP execution, authorization-code flow, request/response correlation,
  persistence, retry and recovery remain later issue-first layers.
- Target compilation is not runtime/device/product support evidence.

## Review decision

The slice is cohesive, additive, bounded, redaction-safe and independently
reversible. No unresolved correctness, security, privacy, compatibility,
architecture, dependency or delivery finding remains before hosted review.

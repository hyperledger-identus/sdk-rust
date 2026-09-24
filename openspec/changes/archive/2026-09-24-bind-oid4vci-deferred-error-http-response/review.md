# Exact-diff architecture and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@b077af2ba011cdcce9f9b4f62da17e873396c9c6
Implementation head: b1f6c73cc71309cf7d6902ef15459a28a3bba6e3
Reviewed head: b1f6c73cc71309cf7d6902ef15459a28a3bba6e3
Specification commit: 2f1ecc98f3c11c800fc33a59a10fbdeaef8802c5
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issue #348, ADR 0139,
OpenID4VCI 1.0 Final sections 8.3.1 and 9.3, the immutable preimplementation
receipt, parser composition, endpoint-specific classification, lifecycle
guidance, public API compatibility and all focused/workspace verification.

## Findings

1. **Protocol behavior — accepted.** Exact `invalid_transaction_id` and
   `credential_request_denied` receive deferred-specific classifications.
   Every other accepted known or extension code retains the existing generic
   classification and exact bounded core.
2. **Reuse and cohesion — accepted.** Status/media/body precedence, resource
   bounds, JSON grammar, descriptions, extensions and generic
   `invalid_request` rejection remain owned by the existing Credential Error
   HTTP parser. No parallel parser, limit or error taxonomy was introduced.
3. **Compatibility — accepted.** A new wrapper and new enum avoid widening the
   existing public closed `CredentialEndpointErrorKind`, so exhaustive generic
   matches are unaffected. No manifest, dependency, feature, lockfile,
   unsafe/native, wire, target or stored-data contract changes.
4. **Security and privacy — accepted.** The wrapper owns only the existing
   bounded zeroizing core. Debug exposes classifications and the core's
   redacted shape; exact codes, descriptions, request transaction IDs and
   canaries do not enter diagnostics.
5. **Lifecycle authority — accepted.** `should_stop_polling` is a pure query
   true only for the Final's explicit `credential_request_denied` guidance.
   It performs no retry, timer, cancellation, invalidation or persistence and
   makes no false origin/correlation claim.

## Residual limitations

- RFC 6750 authorization challenge parsing remains a separate wire/security
  boundary.
- The request association does not prove server origin or transaction
  correlation because Final error bodies carry no transaction identifier.
- Retry, remediation and transaction mutation remain caller policy.

## Review decision

The implementation is a cohesive additive composition of existing bounded
primitives. No unresolved correctness, security, privacy, compatibility,
architecture, dependency or delivery finding remains.

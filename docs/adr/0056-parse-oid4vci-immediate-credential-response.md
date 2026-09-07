# ADR 0056: parse the Final immediate Credential Response as opaque values

- **Status:** Accepted for issue #141
- **Date:** 2026-09-07
- **Decision authority:** standing component authority under ADR 0004
- **Related work:** issues #7, #20, #139, #141; OpenID4VCI 1.0 Final section
  8.3

## Context

The SDK can construct the supported unencrypted Final Credential Request, but a
headless consumer still has to parse the successful response before it can hand
credential values to format-specific verification. Final defines mutually
exclusive immediate and deferred success branches. The immediate branch has a
non-empty array whose credential values can be strings or objects and whose
meaning belongs to the selected credential format.

A generic parser must preserve those opaque values without claiming format or
trust semantics. It must also tolerate extensions without leaving duplicate
member, depth, node, array or retained-allocation work unbounded.

## Decision

1. Add an explicitly named `ImmediateCredentialResponseCore` for the
   unencrypted JSON body. Reject `transaction_id` with a dedicated unsupported
   deferred error and reject `interval` on the immediate branch.
2. Require a non-empty bounded `credentials` array whose ordered object entries
   each contain a string or object `credential` value.
3. Retain every exact credential JSON slice in zeroizing ownership. Expose its
   representation kind and, for string values, a separately decoded sensitive
   string accessor. Do not interpret a format or encoding.
4. Accept an optional non-empty opaque `notification_id` in zeroizing storage.
   Validate and discard unique extensions while rejecting duplicate names.
5. Bound complete response bytes, depth, nodes, top-level members, credential
   count, entry members, individual and aggregate retained credential bytes,
   and the notification identifier independently. Keep errors fieldless and
   diagnostics redacted.

## Consequences

- Headless adapters can receive the interoperable Final immediate envelope and
  route its opaque values to separately governed format verifiers without
  reimplementing JSON shape and resource validation.
- The type proves body syntax only. HTTP status/media/provenance, request
  correlation, error/deferred/encrypted responses, format decoding,
  cryptographic verification, issuer/schema/status trust, notification
  execution, storage and product policy remain separate layers.
- The additive unpublished API changes no dependency, feature, target,
  consumer, release or `main` state.

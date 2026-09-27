# Why

IDR-024 is queued after the completed OID4VCI milestone and build-versus-adopt
decision. Oxid already consumes an `openid4vp` by-reference request shape, but
its parser, localhost policy, application orchestration, and Midnight profile
belong downstream. The SDK needs one chain-neutral bounded ingress before it
can retrieve or validate an OID4VP Request Object.

# What changes

- Activate an unpublished `identus-oid4vp` crate for one transport-only slice.
- Parse bounded `openid4vp://authorize` Request Object references with owned,
  redacted client and URI values plus explicit GET/default or POST intent.
- Add strict form, duplicate, URI, resource, diagnostic, and portability
  evidence without fetching or interpreting referenced content.
- Assign IDR-024 to issue #394 and record the first B10 delivery in the
  blueprint and inventory.

# Capabilities

## New capabilities

- `oid4vp-authorization-request-invocation`: a bounded, transport-neutral
  OpenID4VP 1.0 Final by-reference invocation contract.

# Non-goals

- Inline Authorization Request parameters, by-value Request Objects, JAR/JWS
  verification, client identifier prefix interpretation, metadata, DCQL,
  transaction data, scope, nonce/replay, response modes, transport execution,
  trust, consent, credential selection, presentation creation, or responses.
- No SIROS or full-framework production dependency and no consumer mutation.
- No publication, release, certification, runtime-target, or compatibility
  commitment beyond source-level experimental behavior.

# Delivery

Issue #394 and ADR 0157 control this additive pre-release slice. Planning and
readiness precede code; reviewed signed/DCO delivery targets `develop`.

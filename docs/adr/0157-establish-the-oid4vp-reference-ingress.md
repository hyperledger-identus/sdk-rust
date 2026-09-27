# ADR 0157: establish the OID4VP reference ingress

- **Status:** Accepted under the IDR-024 roadmap mandate
- **Date:** 2026-09-28
- **Issue:** [#394](https://github.com/hyperledger-identus/sdk-rust/issues/394)
- **Decision authority:** IDR-024 in the accepted SDK blueprint/backlog and
  issue #394 under ADR 0004 standing authority
- **Supersedes:** no prior OID4VP implementation ADR; applies ADR 0156

## Context

The SDK has bounded generic presentation semantics and complete OID4VCI wallet
core behavior but no OID4VP wire owner. OpenID4VP 1.0 Final accepts encoded
parameters, Request Objects by value, and Request Objects by reference. Oxid
demonstrates a by-reference consumer shape, but its localhost, Midnight,
session, proof, consent and application policy cannot move into a generic SDK.

ADR 0156 rejected full-framework adoption, retained Identus ownership of the
OID4VP public shell and left SIROS DCQL production reuse conditional. The first
slice should therefore establish transport authority without prematurely
choosing the DCQL engine or trusting a referenced Request Object.

## Decision

1. Create unpublished `identus-oid4vp` as the generic OID4VP wire/state owner.
2. Its first public contract accepts only the Final Request Object by-reference
   invocation at the static `openid4vp:` authorization endpoint, serialized as
   `openid4vp:?…` when query parameters are present.
3. Retain only bounded decoded `client_id`, syntactically safe HTTPS
   `request_uri`, and GET/default or explicit POST retrieval intent.
4. Enforce independent byte/pair/name/value limits, strict decoding, decoded
   duplicate rejection, safe URI shape, zeroizing ownership and static errors.
5. Ignore bounded unknown outer parameters as Final requires, but fail closed
   for unsupported `transaction_data` and other transport alternatives.
6. Do not fetch or validate Request Objects and do not interpret prefixes,
   metadata, DCQL, nonce, response modes, trust, consent or presentations.
7. Do not adopt SIROS or another OID4VC framework in this slice. Later DCQL
   reuse requires the separate ADR 0156 production gate.

## Consequences

Consumers gain a reusable least-authority ingress and later slices receive a
clear transition point for retrieval and JAR validation. The narrow crate
duplicates a small amount of private form-decoding structure rather than
coupling protocol-specific error taxonomies. It remains experimental and does
not make a runtime, release, certification or downstream compatibility claim.

Oxid's loopback HTTP demonstration remains downstream-only. A production
consumer can adopt the SDK boundary after its request delivery is HTTPS or a
separate explicitly bounded local-development adapter owns the exception.
Oxid's `openid4vp://authorize` invocation route is also downstream product
syntax and is not represented as the Final static endpoint.

## Alternatives rejected

- **Port the Oxid adapter:** imports Midnight and product/session policy.
- **Put OID4VP in `identus-presentations`:** collapses generic semantics into
  one protocol's wire and transport rules.
- **Activate `identus-openid4vc`:** preserves a quarantined catch-all rather
  than the roadmap's cohesive protocol crate.
- **Adopt SIROS now:** DCQL is not involved in reference classification and its
  production facade remains undecided.
- **Implement all three transports now:** expands parser and trust surface
  before a first consumer-shaped transition is proven.

## Verification and rollback

Focused Final examples, negative/resource/redaction tests, stable/MSRV and
portable compiles, dependency guards, workspace/factory gates and exact-diff
review are required. Rollback deletes the unpublished crate and additive
records; no registry, data, consumer or migration action is needed.

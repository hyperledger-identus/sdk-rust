# Construct bounded OID4VP encrypted direct-post responses

## Why

`identus-oid4vp` validates a signed Final Authorization Request and constructs
bounded plain `direct_post` success and error responses. High-assurance profiles
need `direct_post.jwt`, but the Final specification defines that response as an
unsigned encrypted JWT, not the signed JARM shape used by older drafts. The SDK
must add the protected path without copying JWE cryptography, exposing a third
party JOSE API, or making a broad dependency cone part of every consumer.

## What changes

- Validate the Final encrypted-response metadata carried by a verified Request
  Object and retain one bounded public recipient key plus content algorithm.
- Add an SDK-owned, algorithm-closed JWE encryption request and injected
  encrypter port in `identus-jose`.
- Consume validated request/DCQL lineage into a bounded plaintext response,
  invoke the injected port, validate the returned compact-JWE envelope, and emit
  deterministic `response=<JWE>` form material.
- Keep plain protocol errors as the explicit Final fallback.
- Publish the dependency assessment, ADR, architecture diagrams, E2E contract,
  vectors, change ledger, and migration/rollback guidance.

## Capabilities

### Added capabilities

- `jose-jwe-encryption-port`: bounded SDK-owned request/result boundary for one
  ECDH-ES/A128GCM compact JWE operation.
- `oid4vp-encrypted-direct-post-response`: Final `direct_post.jwt` success
  construction from validated request lineage.

### Modified capabilities

- `oid4vp-direct-post-response`: share presentation/DCQL payload construction
  between plain and protected response modes.

## Non-goals

No built-in JWE backend, signing/JARM, HTTP, verifier trust, consent, proof
generation/verification, `dc_api.jwt`, HAIP policy, SIOPv2, persistence,
consumer mutation, publication, or certification claim.

## Delivery

Issue #535 owns the slice under IDR-024. A production adapter may be added only
after a separately reviewed engine passes the dependency, target, safety, and
public-recipient-key gates recorded here.

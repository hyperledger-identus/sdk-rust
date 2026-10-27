# ADR 0181: use an SDK-owned JWE port before adopting an engine

- **Status:** Accepted for issue #535 implementation
- **Date:** 2026-10-27
- **Decision authority:** issue #535 under IDR-024 and ADR 0004
- **Related:** ADR 0159; ADRs 0179 and 0180; OpenID4VP 1.0 Final sections 8.3 and 8.3.1

## Context

OpenID4VP Final `direct_post.jwt` requires an unsigned encrypted JWT. The SDK
does not implement JWE. Repeating ECDH, Concat KDF, ephemeral-key, protected
header and AEAD logic locally would create unnecessary security-critical code,
but adopting a broad or immature library would weaken the SDK dependency and
portable-target boundaries.

Exact local qualification found:

- `jose-rs 0.10.0` is stable, portable, public-JWK capable and forbids unsafe,
  but adds about 80 packages absent from the workspace, including unrelated
  algorithm families and prerelease cryptography through `kryptering`;
- modular `no-way-jose 0.1.0-rc.2` adds about 35 packages and compiles on all
  targets, but is prerelease, contains unsafe JSON conversions, and its ECDH
  encryption API requires the recipient private key instead of accepting only
  the verifier public key;
- `jwt-simple 0.15.0` lacks direct `ECDH-ES`; and
- `josekit 0.10.3` introduces native OpenSSL and does not meet WASM policy.

## Decision

1. Keep `identus-jose` as the protocol-neutral public facade.
2. Add a narrow SDK-owned JWE encryption port supporting only P-256
   `ECDH-ES`, `A128GCM`, one public recipient, compact serialization and bounded
   plaintext/output.
3. Keep verifier metadata, Final response payloads and form semantics in
   `identus-oid4vp`; no OpenID type enters `identus-jose`.
4. Adopt no production JWE dependency in this change. Do not implement JWE
   cryptographic composition locally.
5. Permit test/oracle qualification outside the workspace graph. It is
   evidence, not a shipping backend or support claim.
6. Add a production adapter only through a new issue/ADR after an exact engine
   passes stable-version, dependency, public-key, unsafe, advisory, redaction,
   resource, MSRV and native/portable runtime gates.
7. Keep engine DTOs and errors private so a later adapter replacement does not
   change `identus-jose` or `identus-oid4vp` APIs.

## Consequences

Protocol implementation and E2E state composition can progress now, while
cryptographic implementation remains reusable and replaceable. Consumers must
provide a conforming encrypter adapter until the SDK publishes one. This is an
explicit limitation, not an implicit claim of complete out-of-box encrypted
response support.

The narrow facade adds some maintained validation code, but no cryptographic
primitive or broad dependency cone. A real adapter can later reuse a mature
crate without reshaping the protocol layer.

## Reconsideration and rollback

Reconsider on stable `no-way-jose` public-recipient support, a narrow-feature
`jose-rs`, or another qualified stable engine. Rollback removes the additive
port and protected response transition before publication; existing JWS and
plain OID4VP behavior remain unchanged.


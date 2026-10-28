# ADR 0192: require all gates before adopting a JWE engine

- **Status:** Accepted for issue #538 qualification
- **Date:** 2026-10-28
- **Decision authority:** issue #538 under ADRs 0004 and 0181
- **Related:** ADR 0159; ADR 0181; OpenID4VP 1.0 Final sections 8.3 and 8.3.1

## Context

The bounded SDK-owned JWE port needs a production implementation and independent
decryptability evidence. Current Rust crates force a choice between a broad
advisory-bearing dependency graph, prerelease or private-key-only APIs, a higher
MSRV, native backends, missing direct ECDH-ES, or incomplete implementations.

`jose-rs 0.10.0` is the closest fit: public-recipient encryption, independent
round-trip and host/WASM/iOS/Android compilation pass. With default features
disabled it still directly requires `rsa 0.10.0-rc.19` and enables the same RSA
through `kryptering/rustcrypto`. The authoritative audit rejects that exact
graph under RUSTSEC-2023-0071. No feature selection removes the edge.

## Decision

1. Retain the SDK-owned engine-neutral port from ADR 0181.
2. Require exact stable version, public-recipient API, MSRV, license,
   unsafe/native-code, dependency-cone, RustSec, redaction, resource and
   host/WASM/iOS/Android gates before proposing a production adapter.
3. Require an independent public-encrypt/private-decrypt round-trip and compare
   algorithms, `kid`, protected-header shape and plaintext rather than random
   ciphertext bytes.
4. Keep rejected-engine oracles isolated from the workspace graph and lockfile.
5. Defer current production adoption. Do not suppress RUSTSEC-2023-0071, patch a
   broad facade locally, or reimplement JWE cryptographic composition.
6. Keep the production-adapter quality debt open until a supported repository
   adapter executes the independent oracle.

## Consequences

The SDK does not yet provide out-of-box encrypted-response cryptography, but its
public contract remains reusable and replacement-friendly. Exact external
evidence proves feasibility without making a rejected dependency part of the
product. Consumers continue to inject a conforming adapter.

## Reconsideration

Requalify when `jose-rs` narrows its algorithm graph and passes audit, stable
`no-way-jose` accepts public recipients without the reviewed unsafe issue,
`reallyme-jose` meets the SDK MSRV, or another maintained stable engine passes
every gate. Repeat the complete qualification for the exact proposed release.

## Rollback

This decision is behavior-neutral. It can be superseded by a later adoption ADR
without migrating public DTOs, wire formats or persisted data.

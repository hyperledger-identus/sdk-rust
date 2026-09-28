# ADR 0159: narrow JOSE and move OID4VCI proof policy

- **Status:** Accepted
- **Date:** 2026-09-28
- **Issue:** [#398](https://github.com/hyperledger-identus/sdk-rust/issues/398)
- **Decision authority:** issue #398 under ADR 0004 standing authority
- **Supersedes:** ADR 0036, ADR 0037, and ADR 0040 only for crate ownership and
  dependency placement; their proof behavior and trust separation remain
  accepted
- **Applies:** ADR 0034, ADR 0035, ADR 0061, and ADR 0110

## Context

`identus-jose` began as the bounded reusable JWS layer, then accumulated the
OID4VCI holder proof, issuer verifier, DID dereferencing, X.509/federation and
attestation ports, clock, freshness, replay policy, and profile errors. The
result is a primitive crate with a protocol-specific surface and a dependency
on `identus-did`. This weakens reuse and reverses the intended direction from
protocols toward primitives.

The Rust JOSE ecosystem was reassessed before retaining local code. Current
evaluated releases do not jointly support the RFC 9864 `Ed25519` identifier,
the SDK's strict canonical and duplicate-rejecting bounded parser, static
redacted errors, exact algorithm/key binding, portable target policy, and
Identus-owned security states. Broad adoption would introduce more coupling
than maintained-risk reduction.

## Decision

1. Keep `identus-jose` as the public Identus-owned facade for bounded JWS
   Compact/header mechanics and signature capabilities.
2. Move all OID4VCI proof claims, construction, issuer parsing, provider ports,
   DID resolution, trust, time, replay, staged profile states, tests, and static
   profile errors to `identus-oid4vci`.
3. Remove `identus-jose -> identus-did`; make the protocol crate own its direct
   DID dependency.
4. Permit the generic protected header to retain bounded untrusted extension
   carriers, but assign them no protocol or trust meaning in JOSE.
5. Retain existing RustCrypto-backed operations privately. Treat assessed JOSE
   crates as conformance or operation-level engine candidates, not a public
   replacement.
6. Require a new issue/ADR and exact facade-parity evidence before adding any
   production JOSE dependency.

## Consequences

The primitive dependency graph becomes reusable and cohesive. OID4VCI gains
the dependencies and errors that belong to its policy. The import path for
unpublished `Oid4vci*` Rust types changes from `identus_jose` to
`identus_oid4vci`; no compatibility re-export preserves the wrong layer.

The SDK continues to maintain a deliberately small amount of strict compact
and header code. That maintenance is justified only while available engines
cannot replace it without semantic or dependency regressions. The research
ledger provides concrete reconsideration triggers.

## Alternatives rejected

- **Keep the current placement:** preserves source imports but makes every
  generic JOSE consumer transitively identity-protocol aware.
- **Create a third proof-profile crate now:** adds a package boundary without a
  second protocol consumer; OID4VCI is the cohesive current owner.
- **Adopt a complete JOSE/JWT library:** no candidate meets the exact facade,
  algorithm, strictness, portability, and coupling requirements.
- **Duplicate compact/signature mechanics in OID4VCI:** increases security and
  maintenance risk and violates dependency direction.

## Verification and rollback

Moved tests must prove unchanged wire behavior, resource limits, errors,
provider call counts, and staged claims. The manifest/metadata graph must prove
JOSE has no DID/protocol dependency. Workspace/factory, Clippy, docs, MSRV,
primary, and portable-target evidence must pass. Rollback reverts the cohesive
move; there is no stored-data, registry, or network migration.

# ADR 0130: use opaque validated JOSE retained values

- **Status:** Accepted for implementation
- **Date:** 2026-09-17
- **Decision authority:** issue #299 and the
  `harden-jose-retained-inputs` OpenSpec change
- **Related:** ADR 0125; `SDK-SEC-003`; `SDK-LIM-007`

## Context

The JOSE codec and OID4VCI proof builders validate key identifiers, X.509
certificate chains, and identified-client strings before encoding. Their public
enums nevertheless accept raw owned payloads, allowing arbitrarily large
standalone values to be constructed and retained before a later builder rejects
them. This is the remaining JOSE compatibility exception in the repository
input-resource audit.

The crates are unpublished at version `0.0.0`. Searches of the current Oxid,
Lace ID Portal, Midnight Identity, and NeoPRISM worktrees found no direct
sdk-rust use of the affected variants, so the source migration can be completed
without a coordinated downstream edit.

## Decision

1. Keep the existing enum alternatives but replace raw retained payloads with
   public opaque validated types: `JwsKeyId`, `JwsX5c`, and
   `Oid4vciProofJwtClientId`.
2. Expose named fallible constructors and borrowed accessors. Do not expose an
   unchecked constructor, mutable payload, raw-field escape hatch, or
   compatibility alias.
3. Validate key references with `JwsLimits` and identified clients with
   `Oid4vciProofJwtLimits` before returning a retained typed value. Revalidate
   when the value enters a builder with tighter limits.
4. Route bounded wire parsing through the same value invariants while retaining
   parser-first allocation ceilings and existing static errors.
5. Preserve accepted JSON/compact wire bytes, enum alternatives, high-level
   builder/verifier signatures, algorithm behavior, and dependency cone.
6. Narrow only the direct JOSE enum clause of `SDK-LIM-007`. Allocation before
   SDK entry and every other named exception remain disclosed.

## Consumer migration

Replace raw variants with explicit fallible construction:

```rust
let kid = JwsKeyReference::key_id(value, limits)?;
let chain = JwsKeyReference::x5c(certificates, limits)?;
let client = Oid4vciProofJwtClient::identified(client_id, proof_limits)?;
```

Borrowed inspection uses the enum or value accessors rather than raw-field
ownership. High-level `ProtectedHeader::new` remains unchanged.

## Consequences

- Every successfully constructed retained JOSE key-ID, certificate-chain, or
  identified-client value is bounded at the type boundary.
- Existing accepted wire values and provider inputs remain unchanged.
- Direct raw variant construction is an intentional source break before
  publication.
- The typed guarantee does not retroactively bound allocation by callers,
  transports, generic deserializers, FFI bridges, or JavaScript engines.

## Rejected alternatives

- Later builder validation leaves the audited standalone-retention bypass.
- Private whole enums remove useful reusable caller selection and cause broader
  churn.
- Public structs with private internal enums close construction but discard the
  existing enum model without additional safety benefit.
- A new bounded-value dependency expands the cone for three small
  profile-specific wrappers and does not improve interoperability.

## Rollback

Restore raw enum payloads and workspace callers, then restore the JOSE exception
in the input-boundary inventory and `SDK-LIM-007` atomically.

# Harden JOSE retained-input construction

## Why

Issue #299 owns the remaining JOSE compatibility exception identified by the
repository input-resource audit. Public enum variants currently accept raw
`String` or `Vec<String>` payloads, so standalone `JwsKeyReference::KeyId`,
`JwsKeyReference::X5c`, and `Oid4vciProofJwtClient::Identified` values can retain
arbitrary input before a protected-header or proof builder validates them.

## What changes

- Replace raw retained enum payloads with public opaque validated value types.
- Keep the enum shapes and provide named constructors and borrowed accessors so
  callers do not need to depend on representation details.
- Apply `JwsLimits` to key identifiers and X.509 chains, and
  `Oid4vciProofJwtLimits` to identified-client values, at construction.
- Route protected-header parsing and OID4VCI proof parsing through the same
  validated constructors.
- Migrate workspace callers and remove only the direct-JOSE-enum clause from
  `SDK-LIM-007` after API, resource-bound, target, and review evidence passes.

## Impact

- Affected specs: `jws-compact`, `oid4vci-proof-jwt`, and
  `sdk-input-resource-governance`.
- Affected crate: `identus-jose`; workspace test callers in `identus-oid4vci`
  migrate to named constructors.
- Public source compatibility: intentional pre-release migration for direct
  raw enum construction; the enum alternatives, accepted values, wire JSON,
  serialized compact tokens, and builder/verifier behavior remain unchanged.
- Dependencies, features, lockfile, MSRV, and dependency direction: unchanged.
- Consumers: no direct sdk-rust constructor usage was found in the inspected
  Oxid, Lace ID Portal, Midnight Identity, or NeoPRISM worktrees.

## Non-goals

- No new JOSE algorithms, certificate validation, trust policy, or OpenID
  profile behavior.
- No change to the existing size limits or error taxonomy.
- No downstream repository mutation.
- No claim that caller, transport, Serde, FFI, or JavaScript allocation before
  typed validation is bounded.

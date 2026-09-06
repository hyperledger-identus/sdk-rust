# Review: bounded OID4VCI Credential Offer semantics

## Pre-implementation semantic review — 2026-09-06

### Scope and architecture

- **Pass:** the owner remains the chain-neutral, unpublished
  `identus-oid4vci` protocol crate; no consumer or quarantined umbrella crate is
  activated.
- **Pass:** consuming an `EmbeddedCredentialOffer` makes transport and semantic
  validation states explicit without adding network or trust authority.
- **Pass:** grant internals, metadata matching, state machines, crypto, storage,
  chain behavior, FFI, adoption, publication, and release are excluded.

### Standards and compatibility

- **Pass:** the contract matches OpenID4VCI 1.0 Final sections 4.1.1 and 12.2.1:
  required HTTPS issuer; non-empty unique string array; optional object grants;
  ignored top-level extensions.
- **Pass:** an empty configuration ID remains accepted because the Final text
  constrains the array, uniqueness, and element type but does not state that an
  individual string is non-empty.
- **Pass:** grants absence/empty-object behavior is preserved without claiming
  authorization flow support.
- **Pass:** the API is additive and unreleased; exact JSON preservation enables
  later grant/extension interpretation without a lossy migration.

### Security and privacy

- **Pass:** section 13.5's untrusted-offer rule is explicit; issuer validation
  proves syntax only and never trust, origin, metadata agreement, or safety.
- **Pass:** raw offer, issuer, configuration, grant, extension, and potential
  Pre-Authorized Code values are zeroized and excluded from diagnostics.
- **Pass:** field/type confusion and resource exhaustion have explicit failure
  cases; the semantic visitor remains inside prior byte/depth/node ceilings.

### Resource and portability review

- **Pass:** independent positive semantic ceilings preserve the existing
  transport constructor/API and bound decoded field allocation.
- **Pass:** selective deserialization with `IgnoredAny` avoids a generic value
  tree and fixed-width conversion of ignored numeric extensions.
- **Pass:** `serde` is workspace-owned and compatible with the existing target
  matrix; no async, HTTP, crypto, runtime, platform, or chain dependency enters.

### Provenance and isolation

- **Pass:** no donor code or fixture is copied. Official and independently
  reconstructed values are sufficient for this bounded semantics slice.
- **Pass:** Oxid and Lace states/digests are recorded and remain read-only.

## Decision

The contract is semantically ready for implementation. No blocking finding or
protected decision remains. Structural factory validation is separate evidence.

## Post-implementation review

Pending implementation and verification.

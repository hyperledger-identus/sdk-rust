# Review: OID4VCI Credential Issuer Metadata core

## Pre-implementation semantic review — 2026-09-06

### Scope and architecture

- **Pass:** the owner remains the chain-neutral, unpublished
  `identus-oid4vci` crate; no consumer or quarantined umbrella crate is
  activated.
- **Pass:** the new consuming transition can only follow validated embedded
  offer grants and validated unsigned issuer metadata.
- **Pass:** discovery, HTTP, trust, authorization flow selection, format
  profiles, crypto, storage, FFI, consumer adoption, publication, and release
  remain outside this slice.

### Standards and compatibility

- **Pass:** the contract is pinned to OpenID4VCI 1.0 Final sections 4.1.1,
  12.2.1, and 12.2.4: exact issuer matching, required credential endpoint and
  configurations, default issuer Authorization Server, and constrained hints.
- **Pass:** metadata and nested configuration extensions remain exact and
  uninterpreted, preserving forward compatibility without claiming support.
- **Pass:** the API is additive and unreleased; preceding transport and grant
  validation states remain unchanged.

### Security and privacy

- **Pass:** syntactic metadata agreement never represents trust, endpoint
  safety, discovery authenticity, or authorization.
- **Pass:** raw JSON and content-bearing strings use zeroizing ownership and
  are excluded from errors and `Debug` output.
- **Pass:** duplicate keys, type confusion, unsafe endpoints, hint ambiguity,
  and resource exhaustion have explicit fail-closed cases.

### Resource and portability review

- **Pass:** independent positive ceilings cover bytes, nodes, depth, string
  lengths, Authorization Servers, and configuration entries.
- **Pass:** selective scanning avoids a generic value tree and preserves
  arbitrary-magnitude unknown numbers lexically.
- **Pass:** the normal dependency cone remains `identus-core`, `serde_json`,
  `uriparse`, and `zeroize`; no runtime, HTTP, crypto, or chain dependency
  enters.

### Provenance and isolation

- **Pass:** the normative Final document is hash-pinned; Oxid and Lace shapes
  are behavior-only evidence with exact revisions and path digests.
- **Pass:** no donor code or fixture is copied and all consumer checkouts remain
  read-only.

## Decision

The contract was semantically ready for implementation. No protected decision
or unresolved blocker remained. Structural factory validation was recorded
separately.

## Post-implementation review

### Exact reviewed delta

- Base: `78860abcffee5f2a10a5377ba3bce17bc67c164b`.
- Specification: `084da90681b0c97e3e3c8d2e1831eca0cae1e170`.
- Candidate implementation: `fb3f9294b012fd8b1f1e2186d0e3d3af5c90710b`.
- Method: fresh requirement-to-code review, complete base diff inspection,
  dependency-tree inspection, error/redaction audit, boundary audit, consumer
  receipt recheck, and independent focused plus repository gates.

### Resolved findings

- **Diagnostics:** generic scanner failures initially referred only to an
  embedded Credential Offer. The messages now name an OID4VCI JSON object so
  metadata failures remain accurate and still redact input.
- **Boundary evidence:** the initial exact-bound test did not independently pin
  every new metadata ceiling. Exact-limit acceptance and one-less rejection
  now cover JSON bytes, depth, nodes, issuer, endpoint, Authorization Server
  length/count, configuration ID/format length, and configuration count.

### Final findings

- **Architecture/API — pass:** staged ownership prevents metadata matching
  before transport, offer semantics, and grant validation. The result carries
  only agreed metadata and adds no I/O or authority.
- **Standards — pass:** exact issuer/configuration/hint matching, omitted
  Authorization Server defaulting, endpoint validation, required configuration
  formats, and extension retention follow the pinned Final profile.
- **Security/privacy — pass:** every content-bearing error is static, `Debug` is
  redacted, bounded parsing is fail-closed, and no untrusted metadata is
  promoted to trust or endpoint selection.
- **Resource behavior — pass:** allocations and collection growth are bounded;
  duplicate keys are rejected at every parsed object level.
- **Portability/dependencies — pass:** Rust 1.85, browser-WASM, Android ARM64,
  iOS ARM64, Darwin Nix, strict lint/docs, plain Cargo, and tests pass with an
  unchanged normal dependency cone.
- **Provenance/isolation — pass:** no donor material was copied. Consumer
  revisions, status, and path digests match preflight.

No blocking or advisory implementation finding remains. Hosted CI and an
exact-head hosted review remain mandatory before merge.

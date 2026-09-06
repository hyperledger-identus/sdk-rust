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
- **Pass:** selective lexical scanning avoids a generic value tree and
  fixed-width conversion of ignored numeric extensions.
- **Pass:** the dependency cone remains unchanged; no async, HTTP, crypto,
  runtime, platform, or chain dependency enters.

### Provenance and isolation

- **Pass:** no donor code or fixture is copied. Official and independently
  reconstructed values are sufficient for this bounded semantics slice.
- **Pass:** Oxid and Lace states/digests are recorded and remain read-only.

## Decision

The contract is semantically ready for implementation. No blocking finding or
protected decision remains. Structural factory validation is separate evidence.

## Post-implementation review

### Exact reviewed delta

- Base: `ff227f68927d0958230600c4f0d448d585c08ad0`.
- Candidate implementation: `b62bd5bb21f7ea8de21a778ed09a52f26f80944d`.
- Review method: fresh requirement-to-code pass, complete base diff inspection,
  dependency-tree inspection, error/redaction audit, standards edge-case audit,
  and independent rerun of focused plus repository gates.

### Findings

- **Architecture/API — pass:** the state transition consumes only validated
  embedded transport; references cannot be misrepresented as fetched offers.
  The new API remains additive, unpublished, non-serializing, and explicit.
- **Standards — pass:** issuer URL components, required field cardinality,
  decoded-ID uniqueness/order, optional object grants, unknown top-level
  members, and empty-ID handling match the pinned Final text.
- **Security/privacy — pass:** content-bearing values use zeroizing ownership;
  custom `Debug` exposes only safe type/count/presence state; every error and
  core bridge is static. No network, trust, flow-selection, or replay claim is
  introduced.
- **Resource behavior — pass:** field bytes and count are checked before public
  semantic construction. The second scan reuses the exact transport
  depth/node limits and lexically skips unknown numeric magnitude without a
  generic JSON value tree.
- **Portability/dependencies — pass:** the normal dependency tree is unchanged;
  Rust 1.85, browser-WASM, Android ARM64, iOS ARM64, Darwin Nix, plain Cargo,
  docs, lint, and tests pass.
- **Provenance/isolation — pass:** no donor material was copied. Consumer
  revisions, status, and path digests exactly match preflight.

No blocking or advisory implementation finding remains.

## Hosted exact-head review follow-up

- **Finding resolved:** hosted review of `8cf1fe9e3310910ff56a37f7621e54f82ddcadd8`
  identified that `uriparse` 0.6.4 rejects RFC 3986 IPvFuture hosts despite
  the semantic contract accepting syntactically valid HTTPS issuer hosts.
- **Resolution:** the issuer validator now recognizes the IPvFuture host
  production directly and substitutes only that literal with a known IPv6
  host before asking `uriparse` to validate every remaining URI component.
  The temporary normalized URI uses zeroizing storage.
- **Regression evidence:** a valid IPvFuture issuer with port and path passes;
  malformed version, empty address, invalid character, suffix, and port forms
  fail closed. A fresh exact-head hosted review remains required.

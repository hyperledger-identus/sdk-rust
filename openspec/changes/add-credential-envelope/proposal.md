## Why

`identus-credentials` is still a quarantined marker, while every planned
credential format and wallet flow needs the same small boundary for preserving
an encoded credential, naming its format, carrying a detached proof, and
holding format-private material. Oxid has a useful holder-side precedent, but
its closed Midnight-only format enum and product record model cannot become the
generic SDK contract.

## What Changes

- Replace the marker with a real, format-neutral credential-envelope API.
- Add an open, bounded `CredentialFormat` identifier rather than a closed enum.
- Add distinct bounded types for credential payload, detached proof, and
  format-private material.
- Zeroize owned private material while redacting every artifact from safe
  formatting and stable errors.
- Reclassify the crate as an implemented experimental credential-semantics
  package and add consumer-shaped tests for multiple unrelated formats.

## Capabilities

### Added Capabilities

- `credential-core`: preserve bounded format-owned credential artifacts behind
  a small, chain-neutral envelope and redaction-safe error boundary.

### Modified Capabilities

- `crate-ring-layout`: replace the `identus-credentials` placeholder contract
  with an implemented credential-semantics member without changing its layer.
- `sdk-governance-evidence`: reclassify `identus-credentials` in the bootstrap
  inventory from placeholder to implemented experimental code.

## Impact

- **Public API:** the unreleased marker crate gains its first real API. Crate
  naming remains experimental and is not a publication commitment.
- **Dependencies:** `identus-credentials` adds only the existing workspace
  `zeroize` dependency and retains `identus-core`.
- **Compatibility:** no wire format, codec, persistence model, DID type,
  verification policy, format algorithm, or consumer repository changes.
- **Source:** adapts the artifact-boundary ideas from
  `MediaNoxLabs/oxid@bfe3b481568dc738f0732c2b27548fab8721fd95`
  under Apache-2.0; no donor file is copied wholesale.
- **Scope:** issue #71, child of #6 / `IDR-007` and #20.

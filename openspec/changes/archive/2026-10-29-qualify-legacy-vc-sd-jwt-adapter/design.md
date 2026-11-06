# Design

## Context

The canonical profile correctly rejects `vc+sd-jwt`. The named SDK-TS
consumer nevertheless has a released draft-03 population. The identifier is
inside the signed JWS header, so migration is a validation and reissuance
problem rather than an aliasing problem.

## Goals / Non-Goals

**Goals:**

- accept only the source-bound draft-03 common subset through an unmistakable
  opt-in API;
- reuse RFC 9901 verification/disclosure mechanics and canonical profile
  validation rather than duplicate crypto or parsing;
- keep legacy and canonical states impossible to confuse in Rust's type
  system;
- preserve finite work, static diagnostics, target portability, and a
  measurable removal path;
- provide cross-language and offline E2E evidence suitable for a later thin
  SDK-TS adoption facade.

**Non-Goals:**

- full draft-03 implementation, legacy issuance, header rewriting, automatic
  fallback, canonical conversion without reissuance, or SDK-TS API emulation;
- millisecond dates, legacy status list retrieval/acceptance, Type Metadata,
  trust, HTTP, persistence, telemetry transport, FFI, publication,
  certification, or downstream edits.

## Decisions

### Keep the adapter in the profile crate

The adapter is one transitional ingress of `identus-sd-jwt-vc`, not a new
format engine or crate. It uses only private helpers and existing dependencies.
The module may be removed later without changing the RFC 9901 engine or
canonical profile implementation.

### Require an explicit versioned adapter

The public zero-sized adapter names draft-03 and exposes a stable capability
identifier. Its methods parse an exact `vc+sd-jwt` envelope and preserve that
provenance through typed unverified, issuer-verified, and disclosure-verified
legacy states. A generic compact parse cannot be passed to validation. No
canonical parser dispatches to the adapter, and no generic `From` conversion
exists between legacy and canonical states.

```text
CredentialEnvelope(format = vc+sd-jwt)
  --LegacyVcSdJwtDraft03Adapter::parse_envelope-->
UnverifiedSdJwt
  --existing RFC 9901 issuer verification + disclosure processing-->
DisclosureVerifiedSdJwt
  --LegacyVcSdJwtDraft03Adapter::validate-->
LegacyVcSdJwtDraft03
  + LegacyMigrationReceipt(canonical_reissuance_required = true)
```

The final value retains the verified base for caller-owned selection and key
binding but cannot become `Draft22SdJwtVc`. A consumer that needs canonical
wire identity must obtain a fresh issuer signature through the existing
`issue_draft22` path.

### Validate a closed common subset

The adapter requires exact `typ=vc+sd-jwt`, `iss`, and `vct`. It applies the
same bounded claim shape, collision-resistant type, numeric-date,
`nbf <= exp`, confirmation-method, retained-output, and non-selective common
claim rules as the canonical profile. `sub` and `iat` may remain selectively
disclosable.

It rejects `status`, `aka_vcts`, and `vct#integrity` before validation because
their specification lineage differs or they do not exist in draft-03. Its
input structure has no Type Metadata or status evidence fields. Numeric dates
remain finite seconds. For `nbf`, `exp`, and `iat`, an absolute numeric value
at least `1_000_000_000_000` is rejected as an ambiguous millisecond-style
representation; the adapter never divides or normalizes it. Canonical
draft-22 NumericDate handling is unchanged.

### Preserve distinct evidence and errors

`LegacyVcSdJwtDraft03` and `LegacyMigrationReceipt` are distinct public types.
The receipt reports fixed profile identity, common-subset validation,
issuer-evidence class when supplied, and the constant reissuance requirement.
It contains no source data. Static error categories distinguish wrong legacy
identity from unsupported draft semantics without disclosing the rejected
field or value.

Optional issuer evidence reuses the existing structural correlation contract.
It does not make the issuer trusted. Draft-22 Type Metadata and status
evidence cannot be passed to this adapter.

### Reuse existing limits

No new configurable budget is needed. Envelope/compact parsing uses
`SdJwtLimits`; common claim and retained-output validation uses
`SdJwtVcLimits`. Legacy-only work consists of a fixed set of root-member and
protected-type checks. The adapter cannot fetch, recurse beyond existing JSON
claims, allocate an independent graph, or retain extra credential material.

### Source-bound vectors and differential evidence

Clean-room vectors identify exact draft-03 and draft-22 hashes, SDK-TS
revision, npm version, package revision/integrity, selectors, transformation,
and expected semantic result. Normal tests execute only checked-in data and
Rust public APIs; Node, npm, network, or the consumer checkout are not runtime
dependencies.

The differential packet records where SDK-TS/package 0.7.2 accepts behavior
the adapter rejects, including millisecond dates and legacy status behavior.
Those are intentional safer incompatibilities, not agreement failures.

## Risks / Trade-offs

- **Partial migration support:** some legacy credentials remain rejected. This
  is preferable to silent draft confusion; each extension needs independent
  evidence.
- **Temporary public API:** the adapter is deliberately public so downstream
  consumers can migrate, but its removal is a future breaking change. The
  capability ID, transitional docs, and explicit removal gate make that debt
  visible.
- **Shared private helpers:** reuse avoids divergence but could accidentally
  broaden legacy acceptance. Focused tests assert the closed unsupported set
  and canonical/legacy state separation.
- **No in-place canonical conversion:** reissuance is operational work for
  consumers. It is cryptographically honest because the protected type cannot
  be changed under the original signature.

## Decomposition note

The implementation should remain below ten changed production/evidence files
where practical: one adapter module, narrow helper visibility changes, public
exports/errors, focused conformance/properties/E2E tests, vector/quality/
ledger records, and documentation. Existing fuzz and benchmark harnesses are
extended rather than duplicated. If the evidence packet naturally exceeds
the file-count attention threshold, the PR explains why the profile, vectors,
and executable evidence must land atomically.

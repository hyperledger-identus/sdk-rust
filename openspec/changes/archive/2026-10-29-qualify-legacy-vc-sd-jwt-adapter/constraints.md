# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/572
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 govern product neutrality, inward
dependencies, and reusable-module boundaries. SDK-SEC-001 through SDK-SEC-003
govern unsafe code, private material, and bounded inputs. SDK-COMPAT-001
through SDK-COMPAT-005, SDK-DELIVERY-001, SDK-DEP-001, SDK-LIM-002,
SDK-LIM-003, and SDK-LIM-005 through SDK-LIM-007 remain effective. ADR 0193
and issue #572 authorize only an explicit input-only compatibility adapter for
the named released consumer.

## Introduced or changed constraints

- Canonical protected type and envelope/media identity remain `dc+sd-jwt` and
  `application/dc+sd-jwt`; no SDK API emits `vc+sd-jwt`.
- Legacy input is accepted only through the versioned draft-03 adapter and
  exact `vc+sd-jwt` envelope/protected identity. Canonical parsers and profile
  validation retain fail-closed behavior.
- A signed legacy credential is never relabeled or converted into
  `Draft22SdJwtVc`; canonical migration requires issuer reissuance.
- The adapter accepts only semantics proven common to drafts 03 and 22 over
  RFC 9901. Draft-specific status, additional types, integrity metadata, Type
  Metadata, and millisecond-date heuristics fail closed.
- Existing byte/count/depth/work/output ceilings and static redaction rules
  apply. No credential, identifier, claim, date, URI, key, disclosure, or
  comparison value enters diagnostics, receipts, metrics, or logs.
- The production dependency cone remains unchanged and excludes SDK-TS, npm
  packages, donor types, networking, runtime, storage, trust, and product
  policy.
- The adapter has no issuance or automatic-fallback surface and records an
  objective removal gate.

## Introduced or changed limitations

- Only the verified draft-03 common subset is accepted. This is not general
  draft-03 support or byte-for-byte compatibility with every credential that
  SDK-TS 8.1.4 may accept.
- Legacy `status`, `aka_vcts`, `vct#integrity`, Type Metadata, millisecond
  dates, and draft-era status-list behavior are unsupported.
- A successful adapter result proves issuer signature, disclosures, and common
  structural semantics only. It does not establish canonical draft-22
  identity, issuer trust, status, time acceptance, schema, storage safety, or
  product acceptance.
- Portable compilation remains compile-only target evidence and creates no
  binding, runtime, package, certification, publication, or downstream
  adoption claim.

## Consumer and product impact

Rust adopters receive a narrow migration path for stored/released SDK-TS
`vc+sd-jwt` credentials. SDK-TS DTOs, errors, DID coupling, task runtime, and
time heuristics remain outside Rust. Existing products keep their current
engine until a separate downstream adoption issue; no consumer repository is
changed here.

## Activation and rollback

Activation requires the exact issue #572 preflight, immutable source and
fixture provenance, adapter/API/resource/error review, the declared quality
evidence, offline E2E, independent Claude or Codex review, green protected CI,
and an issue-linked PR. Rollback removes the additive unpublished adapter,
legacy vectors, and proposed ledger record while preserving canonical
draft-22 and RFC 9901 behavior.

Remove the adapter after `@hyperledger/identus-sdk` and every supported stored
credential population have a measured migration to canonical `dc+sd-jwt` and
no supported deployment needs legacy input for the agreed compatibility
window. Removal requires a separate consumer-visible issue and migration
receipt; elapsed time alone is insufficient.

## Evidence

Evidence authorities are draft-03, draft-22, RFC 9901, issue #572, ADRs 0193
and 0194, the immutable SDK-TS/package receipts, canonical requirements,
`credentials.sd-jwt-vc.legacy-draft03-adapter.quality.v1`, clean-room vectors,
target/dependency/advisory receipts, independent review, and protected CI.

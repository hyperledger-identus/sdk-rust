# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/569
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 govern inward dependencies, focused module
ownership and effect isolation. SDK-SEC-001 through SDK-SEC-003 govern secret
handling, finite input/work and unsafe code. SDK-COMPAT-001 through
SDK-COMPAT-005, SDK-DEP-001, SDK-DELIVERY-001, SDK-LIM-002, SDK-LIM-003,
SDK-LIM-005 through SDK-LIM-007 and SDK-LIM-010 remain effective. ADR 0193,
IDR-021 and issue #569 authorize the exact draft-22 profile.

## Introduced or changed constraints

- `identus-sd-jwt-vc` depends inward on the profile-neutral RFC 9901 engine and
  generic credential/presentation seams; those crates cannot depend on it.
- The supported profile is exactly `draft-ietf-oauth-sd-jwt-vc-22` with text
  SHA-256 `da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124`.
- Canonical protected type is `dc+sd-jwt`; canonical media type is
  `application/dc+sd-jwt`. The profile never emits or automatically accepts
  `vc+sd-jwt`.
- RFC 9901 issuer verification, disclosure processing and key binding remain
  distinct from profile validation, issuer authorization, metadata trust and
  status acceptance.
- Profile claims and evidence inputs use public byte/count/depth/work/output
  limits checked at the narrowest safe boundary.
- Public errors, Debug/Display, receipts and metrics exclude credentials,
  claims, salts, digests, identifiers, URLs, metadata content, key material,
  status handles, audience, nonce and comparison values.
- Production dependencies exclude assessed candidate engines and donor DTO,
  error, runtime, HTTP, DID, trust, storage, custody and rendering types.
- No unsafe code, native dependency, panic on hostile input, network, file
  system, executor, global mutable state, private-key ownership, or product
  policy enters the profile crate.

## Introduced or changed limitations

- Only draft-22 over RFC 9901 compact serialization and SHA-256 is supported;
  later drafts/RFCs and JWS JSON Serialization require separate capabilities.
- Type Metadata retrieval and display/rendering are not performed. Validation
  is limited to caller-supplied bounded metadata and credential-relevant rules.
- Issuer key discovery, X.509/JWK-set validation and status retrieval are
  represented by caller-supplied evidence; the SDK does not choose trust
  anchors, authorized issuers, status policy, cache policy, or network routes.
- `vc+sd-jwt` compatibility is not activated here. Issue #572 owns a possible
  opt-in input-only adapter for SDK-TS 8.1.4.
- Portable compilation is not browser/mobile runtime, packaging, FFI, device,
  certification or deployed interoperability evidence.

## Consumer and product impact

Rust consumers gain an additive unreleased canonical SD-JWT VC profile and
offline integration evidence. Existing products keep their current engines
until separately adopted. SDK-TS DTOs/errors remain adapter concerns and no
downstream repository changes in this slice.

## Activation and rollback

Activation requires the exact issue #569 preflight, immutable source and
fixture provenance, API/resource/error review, property/fuzz/benchmark/
differential/target evidence, offline protocol E2E, independent Claude or
Codex review, green protected CI and an issue-linked PR. Before publication,
rollback removes the additive profile crate and active ledger entry while
preserving the base engine, vectors and decision history.

## Evidence

Evidence authorities are draft-22, RFC 9901, issue #569, ADR 0193, canonical
requirements, `credentials.sd-jwt-vc.draft22.quality.v1`, immutable vector
metadata, target/dependency/advisory receipts, independent review and
protected CI.


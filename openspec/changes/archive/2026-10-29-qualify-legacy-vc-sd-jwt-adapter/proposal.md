# Qualify the legacy `vc+sd-jwt` input adapter

## Why

`@hyperledger/identus-sdk` 8.1.4 is a released named consumer that stores,
routes, issues, and verifies SD-JWT VC values as `vc+sd-jwt`. Its immutable
dependency is `@sd-jwt/sd-jwt-vc` 0.7.2, an implementation of
`draft-ietf-oauth-sd-jwt-vc-03`. SDK-Rust deliberately implements exact
draft-22 and rejects that legacy protected type.

Migration cannot be implemented as a string alias: the protected type is part
of the signed JWS, and draft-03 differs materially from draft-22 in status,
type metadata, additional type, and integrity semantics. A bounded explicit
input adapter is needed so callers can recognize and verify the proven common
subset without weakening canonical `dc+sd-jwt` issuance or representing a
legacy signed credential as draft-22.

## What Changes

- Add an explicit, input-only draft-03 `vc+sd-jwt` adapter inside
  `identus-sd-jwt-vc`; it is never an automatic fallback and has no issuance
  or serialization path.
- Require exact legacy envelope and protected-type identity, then reuse the
  RFC 9901 verification/disclosure states and the draft-22 validator only for
  semantics proven common to drafts 03 and 22.
- Accept the bounded common claim subset (`iss`, `vct`, optional `nbf`, `exp`,
  `cnf`, `sub`, and `iat`) while rejecting draft-specific ambiguity including
  `status`, `aka_vcts`, `vct#integrity`, type metadata, and millisecond date
  heuristics.
- Return a distinct legacy migration state and redacted receipt that always
  requires canonical reissuance. It cannot be converted into
  `Draft22SdJwtVc` without a new issuer signature.
- Add immutable SDK-TS, npm package, draft-03, and draft-22 provenance;
  cross-language compatibility vectors; property/fuzz/benchmark/differential
  evidence; and an offline E2E proving explicit acceptance, canonical
  rejection, and unchanged canonical output.
- Document telemetry and removal rules without mutating SDK-TS or promising a
  downstream release.

## Capabilities

### New Capabilities

- None. The adapter is a transitional input surface of the existing
  `sd-jwt-vc-draft22` capability, not another canonical credential profile.

### Modified Capabilities

- `sd-jwt-vc-draft22`: add an explicit source-bound draft-03 migration adapter
  that preserves exact draft-22 output and type identity.

## Impact

The change is additive and unreleased. It adds no production dependency and
does not change canonical issuance, parsing, envelope routing, profile
validation, errors, or protocol capabilities unless a caller deliberately
selects the legacy adapter. SDK-TS remains read-only and retains its existing
engine until a separate downstream adoption issue. Removal is required after
the named consumer and supported stored credentials migrate to canonical
`dc+sd-jwt`.

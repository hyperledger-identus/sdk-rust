# ADR 0105: validate OID4VCI Token Response authorization details explicitly

- **Status:** Accepted for implementation
- **Date:** 2026-09-09
- **Decision authority:** standing product mandate and IDR-023 issue #237
- **Related work:** issues #7, #20, #127, #139, and #237
- **Normative baseline:** OpenID4VCI 1.0 Final sections 3.3.4 and 6.2;
  RFC 9396 sections 2 and 7

## Context

The bounded successful Token Response core records only that
`authorization_details` exists. This is intentionally compatible with generic
OAuth responses but cannot safely supply the Credential Dataset identifier
required by the Final Credential Request branch. Validating during core parse
would make the partial state stricter and conflate OAuth syntax with an
OpenID4VCI-specific transition.

## Decision

1. Preserve `TokenResponseCore::parse` and its presence-only behavior.
2. Add a consuming, explicit validation transition that reparses the retained
   zeroizing response under its original byte/depth/node policy plus independent
   positive Authorization Details limits.
3. Require a non-empty RFC 9396 array and at least one recognized
   `openid_credential` entry. Each recognized entry requires a bounded
   `credential_configuration_id` and non-empty bounded unique
   `credential_identifiers`.
4. Reject dataset-identifier reuse within or across recognized entries so a
   later selector cannot observe an ambiguous identifier.
5. Traverse and discard bounded unknown fields and unknown authorization-detail
   types. Do not treat ignored extension data as typed authority.
6. Retain recognized identifiers in zeroizing storage and expose them only by
   explicit borrowed accessors. Formatting and errors report no caller values.
7. Keep request construction, metadata correlation, token/issuer trust and all
   product policy outside this state.

## Consequences

- A headless wallet gains the exact typed input required by a later
  Credential Request-by-identifier transition.
- Existing core callers and accepted wire inputs remain compatible.
- Validation performs a second bounded linear scan; this is an intentional
  cost for a separate proof state and does not add a dependency.
- Unknown authorization mechanisms remain interoperable but confer no SDK
  capability.
- Authorization Request/Code flow, credential-identifier selection, HTTP,
  trust, storage, downstream adoption, publication and release remain
  issue-first follow-up work.

## Rejected alternatives

- `serde_json::Value` would erase duplicate-member evidence and allocate before
  field limits are applied.
- A general OAuth/RAR dependency would widen the runtime cone without supplying
  Identus bounds, redaction or state semantics.
- Immediate Credential Request construction would combine independently
  reviewable parsing, selection and correlation decisions.

## Rollback

Remove the additive limits, state, parser transition, errors, tests and
canonical capability. The unchanged partial Token Response core requires no
consumer or data migration.

# Construct JWT Credential Requests from authorized dataset identifiers

## Why

Issue #239 advances IDR-023 after #237 added a bounded typed Token Response
Authorization Details state. OpenID4VCI 1.0 Final requires a wallet to send one
returned `credential_identifier` in the Credential Request and forbids the
configuration-ID selector in that branch. The existing constructor supports
only the mutually exclusive configuration-ID branch.

## What changes

- Add a checked constructor that selects one recognized authorization detail
  and one Credential Dataset identifier.
- Require the selected detail's configuration ID to match the offer/metadata
  state exactly.
- Emit the Final `credential_identifier` body with existing JWT proof and
  Bearer transport semantics.
- Share validation/encoding machinery with the existing configuration-ID
  constructor while preserving its API and fail-closed behavior.
- Add static selection/mismatch errors, tests, ADR 0106 and roadmap evidence.

## What does not change

No Authorization Request/Code flow, ranking, access-token or issuer trust,
proof generation, HTTP, response correlation, storage, downstream mutation,
publication, release or chain-specific behavior is added.

## Capabilities

### Modified capabilities

- `oid4vci-jwt-credential-request`: add the Final authorized dataset-selector
  request branch alongside the existing configuration selector.
- `ssi-upstream-program`: advance IDR-023 from #237 to #239 while retaining
  `in_progress` status.

## Authority

Issue #239, OpenID4VCI 1.0 Final sections 3.3.4 and 8.2, ADR 0105, and the
standing SDK delivery mandate.

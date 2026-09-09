# Authorized JWT Credential Request research

Research class: routine
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

`CredentialOfferWithMetadata::try_create_jwt_credential_request` currently
selects one offered configuration and emits `credential_configuration_id`. It
intentionally rejects any Token Response that contains Authorization Details.
Issue #237 now provides a typed state whose recognized entries own bounded,
unique Credential Dataset identifiers, but no request transition consumes it.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 3.3.4 and 8.2.
- [RFC 6750](https://www.rfc-editor.org/rfc/rfc6750), section 2.1, for the
  unchanged Bearer Authorization field.

The Final specification requires `credential_identifier` when credential
Authorization Details were returned and forbids `credential_configuration_id`
in that request. Proofs remain optional in the abstract protocol but this
existing JWT-proof constructor deliberately requires one or more JWT proofs.

## Compatibility and dependency evidence

The change is additive. The current configuration-ID method signature, output
and rejection of presence-only Authorization Details remain unchanged. The new
method accepts only the stronger #237 typed state. It reuses the current
`identus-core` plus `identus-jose` cone and adds no dependency or feature.

Oxid and Lace ID Portal remain read-only evidence sources at the revisions
recorded by #237; neither supplies a reusable Rust authorized-request model.
Final examples and negative cases are independently reconstructed.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Add a sibling constructor over the typed token state | `adopt` | Keeps the mutually exclusive Final selectors explicit and preserves compatibility. |
| Add optional identifier to the existing method | `not-adopt` | Creates invalid selector combinations and weakens the type-state boundary. |
| Accept a raw identifier string | `not-adopt` | Loses proof that it came from a validated bounded Token Response entry. |
| General OAuth/RAR crate | `not-adopt` | Does not own this SDK's offer correlation, proof type or bounded transport state. |
| Consume/invalidate the access token | `defer` | Replay and one-shot token policy belong to a later protocol state machine, not request encoding. |

## Security, privacy and maintenance evidence

The request carries a Bearer token, proofs and a correlating dataset identifier.
Existing zeroizing Authorization/body storage and redacted Debug remain. The
selected identifier is copied only into the zeroizing bounded body. Checked
indices and exact configuration agreement prevent out-of-range and cross-offer
selection. Existing proof count/size, Authorization bytes and complete body
bytes bound all new work and allocation.

The private refactor removes duplicated construction logic rather than adding
a second encoder. No unsafe, network, time, entropy, native or ambient authority
is introduced. Maintenance and target posture remain unchanged.

## Rejected or deferred candidates

Authorization Request/Code/PKCE execution, metadata discovery, access-token
validation, product identifier ranking/choice, proof creation, HTTP, response
correlation changes, storage, downstream adoption, publication and release are
excluded. Token replay/invalidation remains deferred until a protocol engine
owns the lifecycle.

## Open questions and blockers

None. Configuration agreement is exact and case-sensitive because the values
are opaque metadata keys. This constructor proves deterministic request
construction, not that an Authorization Server legitimately issued the token
or identifier.

## Evidence commands

```text
scripts/factory research-ready add-authorized-jwt-credential-request
scripts/factory constraints-ready add-authorized-jwt-credential-request
cargo test -p identus-oid4vci
nix flake check
```

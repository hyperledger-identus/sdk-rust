# Authorization Code Token Authorization Details correlation research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

`RequestBoundAuthorizationCodeTokenResponse` owns the exact #360 lineage and a
bounded `TokenResponseCore`. The request lineage proves that the SDK emitted
one Authorization Details entry for one selected Credential Configuration.
The core reports only whether response Authorization Details exist.

`TokenResponseCore::try_validate_authorization_details` already reparses the
retained zeroizing JSON under its original JSON budgets plus explicit positive
Authorization Details limits. It requires at least one recognized
`openid_credential` entry, bounded configuration and dataset identifiers,
global dataset-identifier uniqueness, and ignores bounded unknown types and
fields. `TokenResponseWithAuthorizationDetails` deliberately has no request
lineage. The older Credential Request helper accepts that detached state plus
an independently supplied matched offer and merely checks that a configuration
was offered, which is weaker than exact pre-authorization selection.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 3.3.4, 5.1.1 and 6.2, retrieved 2026-09-25.
- [RFC 9396](https://www.rfc-editor.org/rfc/rfc9396.html), sections 6 and 7,
  retrieved 2026-09-25.

OpenID4VCI requires a Token Response `authorization_details` value when the
Authorization Request used Authorization Details. Each recognized response
entry carries the `credential_configuration_id` and a required non-empty array
of unique Credential Dataset identifiers authorized for that configuration;
the wallet later uses one of those identifiers with the access token. Unknown
Token Response members and unknown fields of a recognized authorization detail
are ignored. RFC 9396 permits Authorization Server enrichment and omission of
values and permits unknown authorization-detail types to coexist, but the
definition of the concrete detail type controls its semantics.

## Correlation and ambiguity decision

The #354 Authorization Request contains exactly one `openid_credential` entry
for the exact selected Credential Configuration. Therefore #362 applies this
closed authority policy after response-local parsing:

1. Any recognized entry whose `credential_configuration_id` differs from the
   exact selected configuration fails with a configuration-mismatch error.
2. After all recognized entries match, more than one recognized entry fails
   with a distinct ambiguity error. Multiple authorized datasets belong in the
   one entry's `credential_identifiers` array.
3. Exactly one matching recognized entry advances. Its source-ordered unique
   identifiers become the only exposed dataset authority.
4. Unknown authorization-detail types remain bounded, counted and ignored.
   They neither fail correlation nor confer credential authority.

Mismatch precedes duplicate-entry ambiguity independently of source order.
This rejects response-side authority expansion and prevents downstream index
selection across duplicate configuration objects without forbidding RFC 9396
extensions unrelated to `openid_credential`.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Existing `TokenResponseCore::try_validate_authorization_details` | `adopt` | It already owns strict bounded parsing, identifier uniqueness and zeroizing token/JSON state. |
| Existing `TokenResponseWithAuthorizationDetails` | `adopt-and-privately-decompose` | Preserve its public response-local API while moving its validated pieces into the request-bound result without reparsing or cloning secrets. |
| #360 request lineage | `adopt` | It is the only exact selected-configuration authority and prevents metadata or configuration substitution. |
| Existing `try_create_authorized_jwt_credential_request` | `reference-only` | Its serializer and limits are reusable later, but its detached matched-offer input is too weak for this correlation slice. |
| `oauth2`, `openidconnect`, or generic RAR crates | `not-adopt` | They do not model OID4VCI credential identifiers, this exact typed lineage or the existing resource/error contract. |

No dependency, feature, manifest, lockfile, unsafe/native or network capability
is needed.

## Security, privacy and maintenance evidence

Only the success branch type exposes the consuming correlation method; an
OAuth error cannot enter it. The method consumes both the request-bound state
and its `TokenResponseCore`. On missing, malformed, mismatched or ambiguous
details, token and retained JSON owners drop and zeroize rather than returning
a reusable typed success.

The correlated result owns the exact #360 public lineage, the existing
secret-bearing Token Response core, the one matched entry's zeroizing dataset
identifiers and the bounded unknown-type count. It has no Clone, Serde or
Display. Debug exposes counts and issuer-identification evidence only. New
errors are fieldless and static.

## Compatibility and dependency evidence

The API and errors are additive and unpublished. Existing response-local
parsing and public accessors remain unchanged. The new transition reuses
`TokenAuthorizationDetailsLimits`; it adds no second limit vocabulary or
allocation beyond moving the already bounded parsed identifiers. Existing
JSON byte/depth/node limits and positive per-detail limits remain authoritative.

WASM, Android ARM64 and iOS ARM64 remain compile-only evidence. No serialized
shape, dependency, feature, target or stored-data contract changes.

## Rejected or deferred candidates

Accepting extra recognized configurations is rejected because the exact
Authorization Request selected one configuration. Silently choosing the first
matching entry is rejected because repeated matching objects are ambiguous.
Rejecting bounded unknown types is rejected because RFC 9396 permits mixed
authorization requirements and they confer no typed credential authority.

Dataset selection, proof generation, Credential Request construction, HTTP,
token validation/storage, retry/recovery policy and downstream adoption remain
independently owned. Request-bound Credential Request construction continues
in issue #364.

## Open questions and blockers

None for the bounded correlation slice.

## Evidence commands

```text
scripts/factory research-ready correlate-oid4vci-token-authorization-details
scripts/factory constraints-ready correlate-oid4vci-token-authorization-details
cargo test -p identus-oid4vci --test authorization_code_token_authorization_details
cargo test -p identus-oid4vci
scripts/factory check
nix flake check
```

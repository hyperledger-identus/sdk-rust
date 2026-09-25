# Request-bound authorized-dataset Credential Request research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

`CorrelatedAuthorizationCodeTokenResponse` owns exact request lineage, the
secret-bearing Token Response core and one recognized detail's bounded unique
dataset identifiers. `CredentialOfferWithMetadata::try_create_authorized_jwt_credential_request`
already implements the Final wire shape and limits, but accepts independently
supplied metadata plus response-local state and therefore cannot prove exact
Authorization Code request lineage.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 6.2 and 8.2, retrieved 2026-09-25.
- [RFC 6750](https://www.rfc-editor.org/rfc/rfc6750.html), section 2.1,
  retrieved 2026-09-25.

The wallet uses one `credential_identifier` returned in Token Response
Authorization Details and sends proof(s) plus the access token to the
Credential Endpoint. The selector is mutually exclusive with
`credential_configuration_id`. Existing SDK code already implements the
required bounded JWT proof array, Bearer grammar and deterministic JSON shape.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Existing bounded serializer | `adopt-and-extract-private-helper` | It already owns exact proof, Bearer, JSON and allocation behavior; one private helper can serve all routes. |
| #362 correlated state | `adopt-and-consume` | It is the only state that proves exact selected-configuration lineage and authorized datasets. |
| Existing detached authorized constructor | `retain-for-unpublished-compatibility` | It remains useful to current callers but is weaker and is not the new preferred authority path. |
| Raw identifier or replacement metadata input | `reject` | Either would reopen substitution and confused-deputy risk. |
| New crate dependency | `not-adopt` | The work is a local ownership/serialization transition already covered by existing types. |

## Compatibility and dependency evidence

The API is additive and unpublished. Existing constructors, output bytes,
errors, limits and public accessors remain exact. A private serializer helper
is generalized without adding a dependency, feature, manifest, lockfile,
unsafe/native, network or serialized-data change. Existing WASM, iOS ARM64 and
Android ARM64 compile-only evidence remains applicable.

## Security, privacy and maintenance evidence

The new method consumes the correlated state. Checked index selection occurs
before token/proof output construction. Endpoint, access token and dataset
identifier are borrowed only from that state, then the result independently
owns the duplicated endpoint and zeroizing Authorization/body values. Failure
drops the token and dataset owners. Existing static errors and redacted Debug
surfaces are reused; no remote value enters an error.

## Rejected or deferred candidates

Configuration selection, dataset selection policy and proof generation remain
caller-owned. Consuming the proof values is unnecessary because they are
caller-produced independent artifacts rather than authority carried by the
token response. HTTP execution, response binding, token trust/storage and
issuance lifecycle remain separate slices.

## Open questions and blockers

None for the bounded constructor.

## Evidence commands

```text
scripts/factory research-ready construct-request-bound-oid4vci-credential-request
scripts/factory constraints-ready construct-request-bound-oid4vci-credential-request
cargo test -p identus-oid4vci --test authorization_code_credential_request
cargo test -p identus-oid4vci
scripts/factory check
```

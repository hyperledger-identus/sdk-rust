# Authorization Code server-binding research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

`CredentialOfferWithMetadata` already proves exact Credential Issuer and
Credential Configuration agreement and validates grant-level Authorization
Server hints against issuer metadata. `AuthorizationServerMetadataCore`
already proves a bounded HTTPS issuer, optional HTTPS Authorization Endpoint,
optional Token Endpoint, and explicit or RFC 8414-defaulted grant types.
`CredentialOfferWithPreAuthorizedServer` demonstrates the intended owned
state-transition pattern for the other grant.

The missing behavior is the Authorization Code counterpart. Without it, every
consumer must independently decide whether its selected server was advertised,
matches the offer hint, supports `authorization_code`, and exposes the
endpoint needed to start the flow.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 4.1, 5.1 and 12.3, retrieved 2026-09-25.
- [RFC 8414](https://www.rfc-editor.org/rfc/rfc8414.html), section 2,
  retrieved 2026-09-25.

Final section 4.1 defines the optional `authorization_code` grant. Its opaque
`issuer_state`, when present and when the flow is chosen, must be included in
the later Authorization Request. Its optional `authorization_server` selects
one entry only when issuer metadata contains multiple Authorization Servers
and must exactly match an advertised entry. Section 5.1 places the eventual
request at the selected Authorization Endpoint. Section 12.3 requires the
wallet to prevent Authorization Server mix-up by using a distinct redirect
URI per issuer or by validating the returned issuer identifier; that later
callback boundary is not implied by server selection.

RFC 8414 section 2 requires `authorization_endpoint` unless no supported grant
uses it. An omitted `grant_types_supported` means exactly
`authorization_code` and `implicit`; therefore omission is sufficient for this
transition, while an explicit list must contain exact `authorization_code`.

## Compatibility and dependency evidence

All required inputs and bounds already exist. The transition can compose
existing typed states and string comparisons without reparsing JSON, cloning
opaque state, changing defaults, or adding a dependency, feature, lockfile,
target, unsafe/native surface or network capability.

The result should mirror the pre-authorized state structurally while remaining
a separate type: its required endpoint and later security inputs differ. The
existing Authorization Code grant continues to own optional `issuer_state`,
so the transition adds no secret copy and preserves existing redaction.

The issuer/Authorization Server metadata error catalogue already contains the
standing maximum of 39 records. The four new transition diagnostics therefore
belong in a small authorization-code server-binding catalogue. This preserves
the review ceiling and keeps a new semantic boundary out of a full parser and
metadata catalogue.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Existing matched offer and bounded server metadata | `adopt` | They already own all validated inputs and limits. |
| Owned Authorization Code server state | `adopt` | Gives later builders a least-authority typed predecessor. |
| RFC 8414 effective grant default | `adopt` | Omission means `authorization_code` plus `implicit`. |
| Exact hint and advertised-server comparisons | `adopt` | Matches Final semantics and avoids normalization ambiguity. |
| Require Authorization Endpoint | `adopt` | Authorization Code uses the authorization endpoint. |
| Dedicated four-record error catalogue | `adopt` | Preserves the 39-record review ceiling and semantic cohesion. |
| Reuse the pre-authorized result type | `not-adopt` | It proves the wrong grant and Token Endpoint capability. |
| Build the Authorization Request in this slice | `defer` | Client, redirect, state, PKCE and credential-selection inputs need a separate contract. |
| Discovery or endpoint reachability checks | `defer` | Network and trust remain caller-owned boundaries. |
| Require Token Endpoint now | `not-adopt` | It is not needed to begin authorization and would exceed least authority. |

## Security, privacy and maintenance evidence

The state owns only existing bounded objects. Debug exposes neither remote
identifiers, endpoints nor `issuer_state`. Selection proves structural
agreement only; it does not prove metadata provenance, endpoint control,
server trust, reachability, authorization response issuer binding, client
eligibility or successful issuance.

No entropy, clock, network, redirect, browser, storage or ambient authority is
introduced. Future Authorization Request and callback slices must independently
address PKCE, CSRF state and section 12.3 mix-up defenses.

## Rejected or deferred candidates

Authorization URL serialization, PAR, PKCE, redirect/callback processing,
authorization details, scope selection, OAuth errors, token exchange,
credential formats, consumer adoption, publication and release remain out of
scope.

## Open questions and blockers

None for the bounded server-selection transition.

## Evidence commands

```text
scripts/factory research-ready bind-oid4vci-authorization-code-server
scripts/factory constraints-ready bind-oid4vci-authorization-code-server
cargo test -p identus-oid4vci
nix flake check
```

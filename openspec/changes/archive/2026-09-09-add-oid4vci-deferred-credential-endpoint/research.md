# Deferred Credential Endpoint metadata research

Research class: routine
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

`CredentialIssuerMetadata` retains a mandatory `CredentialEndpoint` and an
optional `NonceEndpoint`. The strict scanner currently traverses but discards
`deferred_credential_endpoint`, so downstream code cannot distinguish omission
from an advertised Final endpoint. A deferred response body core exists, but
request construction correctly remains blocked on typed metadata.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 9 and 12.2.4; retrieved document SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.

Section 12.2.4 defines the member as optional, HTTPS-only and able to contain a
port, path and query. Omission means the issuer does not support the Deferred
Credential Endpoint. Section 9 separately defines later request, response,
token, TLS, encryption and transaction lifecycle behavior.

## Compatibility and dependency evidence

The public change is additive. Existing metadata inputs, required fields,
constructor arity, retained JSON and `NonceEndpoint` behavior remain unchanged.
The current scanner, zeroizing string ownership, shared endpoint byte budget
and dependency-free HTTPS validator already implement the needed mechanics.
No crate, feature, lockfile or target change is justified.

Oxid and Lace ID Portal remain read-only behavior evidence. No donor type,
fixture or implementation is copied; the Final specification is authoritative.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Reuse the current metadata scanner and endpoint validator | `adopt` | Preserves duplicate detection, complete resource bounds, exact text, URL policy and no new dependency. |
| Reuse the existing endpoint byte budget independently | `adopt` | Avoids a breaking limits-constructor expansion while applying the same URL ceiling. |
| Add the `url` crate | `not-adopt` | The established validator already proves this deliberately narrow syntax; a new parser cone adds no required capability. |
| General OAuth/OpenID client crate | `not-adopt` | Does not supply this Final VC issuer metadata type or SDK-specific bounds/redaction. |
| Parse through `serde_json::Value` | `not-adopt` | Would erase duplicate-member evidence and allocate ahead of field limits. |
| Construct the deferred request in the same issue | `defer` | Endpoint discovery and sensitive transaction/token request state are independently reviewable boundaries. |

## Security, privacy and maintenance evidence

The endpoint is untrusted remote metadata and can be correlating. It remains in
zeroizing ownership, is available only through an explicit borrow and never
enters Debug, Display or errors. Empty and mistyped members fail as metadata;
oversize and unsafe values use fieldless stable errors. Aggregate JSON byte,
depth and node limits plus the independent endpoint string limit bound all
work. The established validator rejects non-HTTPS, missing hosts, userinfo and
fragments while permitting Final port/path/query components.

No unsafe, network, DNS, redirect, time, entropy, native, storage or ambient
authority is introduced. Maintenance and portable target posture are unchanged.

## Rejected or deferred candidates

Deferred request construction, interval scheduling, transaction reuse and
invalidation, response correlation, access-token validity, application-layer
encryption, HTTP execution, retry/recovery, downstream adoption, publication
and release are excluded.

## Open questions and blockers

None. The shared limit keeps its existing accessor name for source
compatibility; documentation will state that it applies independently to each
Credential, Nonce or Deferred Credential Endpoint.

## Evidence commands

```text
scripts/factory research-ready add-oid4vci-deferred-credential-endpoint
scripts/factory constraints-ready add-oid4vci-deferred-credential-endpoint
cargo test -p identus-oid4vci
nix flake check
```

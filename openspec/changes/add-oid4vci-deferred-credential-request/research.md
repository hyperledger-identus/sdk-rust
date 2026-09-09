# Deferred Credential Request research

Research class: routine
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

`DeferredCredentialResponseCore` owns a bounded, zeroizing
`DeferredTransactionId`, and `CredentialIssuerMetadata` optionally owns a
validated HTTPS `DeferredCredentialEndpoint`. These are the complete typed
inputs for the unencrypted Final request, but the SDK does not yet bind them or
emit the required JSON body. Consumers would otherwise hand-roll escaping,
media type and endpoint-presence behavior.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  section 9.1; retrieved document SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.

Section 9.1 requires POST, a required string `transaction_id`, and
`application/json` for an unencrypted request. It separately requires a valid
access token and TLS at execution time, permits request and response encryption,
and allows extensions. Those execution, encryption and extension concerns are
not necessary to construct this narrow core request.

## Compatibility and dependency evidence

The public change is additive. It reuses the current endpoint and transaction
types, `zeroize`, and existing `serde_json` dependency. No crate, feature,
lockfile or target change is justified. The request-body limit is independent
because callers can configure response transaction limits larger than the
default; construction must remain bounded even for such valid existing state.

Oxid and Lace ID Portal remain possible read-only conformance evidence. No
donor code, fixture or public type is needed: the Final document and existing
SDK types are sufficient.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Existing endpoint and transaction types | `adopt` | They already encode the exact provenance-free syntax and resource boundaries required as inputs. |
| Existing `serde_json` serializer into a bounded writer | `adopt` | Produces standards-correct escaping without an unbounded intermediate allocation or new dependency. |
| Fixed/default-only transaction body ceiling | `not-adopt` | Existing response limits are caller-configurable; a separate positive request-body policy is honest and composable. |
| Hand-written JSON escaping | `not-adopt` | Duplicates a well-known serializer and creates avoidable correctness risk. |
| General OAuth/OpenID HTTP client | `not-adopt` | Adds runtime/coupling while not owning the SDK's typed Final request boundary. |
| Generic `Serialize` on secret-bearing state | `not-adopt` | Makes accidental logging/persistence easier and obscures the explicit exposure boundary. |
| Include bearer access token in the value | `defer` | Token ownership, headers, DPoP and transport lifecycle are separate security boundaries. |
| Include request/response encryption | `defer` | Requires separately researched JWE, metadata and key-agreement behavior. |

## Security, privacy and maintenance evidence

The transaction handle is bearer-adjacent and correlating. The constructed
body remains zeroizing, is available only through an explicitly sensitive byte
accessor, and never enters Debug, Display, errors or generic Serde. A bounded
writer refuses output above the caller policy before retaining those bytes.
Endpoint and response values remain non-Clone public types.

The request advertises that an access token is required but stores none. It
performs no networking, redirect, DNS, TLS, time, entropy, storage, native or
ambient operation. Maintenance and portable target posture are unchanged.

## Rejected or deferred candidates

HTTP execution, Authorization header construction, access-token validity,
interval scheduling, transaction invalidation/persistence, automatic retry,
encrypted bodies, response encryption parameters, extensions, error response
parsing, response correlation, credential verification/storage, downstream
adoption, publication and release remain excluded.

## Open questions and blockers

None. Repeated construction is allowed because a deferred transaction can be
polled more than once until issuance completes; replay, timing and terminal
invalidation remain explicit caller responsibilities.

## Evidence commands

```text
scripts/factory research-ready add-oid4vci-deferred-credential-request
scripts/factory constraints-ready add-oid4vci-deferred-credential-request
cargo test -p identus-oid4vci
nix flake check
```

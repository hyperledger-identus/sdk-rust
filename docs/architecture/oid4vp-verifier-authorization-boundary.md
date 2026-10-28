# OID4VP verifier authorization boundary

`identus-oid4vp` separates cryptographic validity from verifier identity and
product trust. A Request Object reaches authorization routing only through this
state lineage:

```text
compact JAR
  │ bounded parse + caller-selected verification key
  ▼
VerifiedRequestObject
  │ exact client_id / wallet_nonce correlation
  ▼
RequestObjectSecurityEvidence
  │ exact audience + time + atomic replay acceptance
  ▼
VerifierAuthorizationEvidence
  │ Final prefix + exact verified key + bounded metadata
  │ caller-owned VerifierTrustPolicy accepts the binding
  ▼
ValidatedAuthorizationRequest
  │ DCQL and direct_post / direct_post.jwt construction
  ▼
inert response material
```

## Scheme contract

| Client identifier | Core behavior | Adapter obligation |
| --- | --- | --- |
| no prefix | pre-registered scheme; inline metadata is ignored | look up registration, metadata and authorized key |
| `openid_federation` | bounded entity id and optional `trust_chain`; inline metadata is ignored | validate Federation chain/policies and authorize exact key |
| `decentralized_identifier` | validate DID, require a `kid` DID URL for that exact DID | resolve/dereference the verification method and authorize its relationship/key |
| `verifier_attestation` | require bounded protected `jwt` evidence | validate nested JWT, issuer trust, `sub`, `cnf`, redirects and exact outer key |
| `x509_san_dns` | validate DNS-name syntax and require bounded `x5c` | validate chain, SAN, redirect rules and exact leaf key |
| `x509_hash` | validate canonical 32-byte SHA-256 base64url and require bounded `x5c` | validate chain, leaf hash and exact leaf key |
| `redirect_uri` | reject in the signed-JAR profile | use a separately specified unsigned flow |
| `origin` | reject outside DC API | use the future DC API capability |
| unknown prefix | reject by default | propose an extension with its own ADR and vectors |

The full prefixed client identifier is never replaced by its inner identifier.
It remains the exact request identity and audience-facing value. The port also
receives the exact textual signed `response_uri`, when present, so attestation
and X.509 policies can enforce their verifier-to-destination redirect rules.

## Adapter skeleton

Implement `VerifierTrustPolicy` in the runtime or product adapter. Inspect
`VerifierTrustInput::scheme()`, then fully validate the selected scheme using
the exact `verification_key()`, protected header, full/original client IDs,
bounded metadata, verifier-info and trust-chain evidence. Return:

- `Ok(())` only after the exact key is authorized for the exact full client;
- `Rejected` for invalid or untrusted evidence; or
- `Unavailable` when a required resolver or trust source cannot decide.

The port is deliberately asynchronous but executor-neutral. A DID adapter can
compose `identus_did::DidResolver`/`DidUrlDereferencer`; PKI, Federation and
attestation adapters can use independently qualified crates without expanding
the protocol core dependency cone. Never accept merely because the outer JWS
signature is valid—the port is the authorization step.

## Metadata and encryption

`client_metadata` and `verifier_info` are bounded, verifier-controlled evidence,
not display-safe content. `verifier_info.credential_ids` must refer to a DCQL
credential query before the trust adapter runs. Federation and pre-registration
ignore inline client metadata because their trusted metadata comes from their
external scheme.

The existing `direct_post.jwt` parser remains a separate confidentiality
boundary. It accepts exactly one public P-256 `ECDH-ES` recipient and A128GCM.
That key selection does not authenticate the verifier. Inline encryption
metadata is not used for Federation or pre-registration in this bounded profile;
an external trusted-metadata successor requires a separately specified API.

## What the evidence does not prove

`VerifierAuthorizationEvidence` records one caller-policy acceptance. It does
not expose a trust score or policy rationale and does not prove consent,
credential validity, response-URI network safety, HTTP execution, durable
replay storage, certification, or product authorization. Errors and `Debug`
surfaces reveal only static categories and bounded counts.

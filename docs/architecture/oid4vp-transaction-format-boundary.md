# OID4VP transaction and credential-format boundary

Issue [#547](https://github.com/hyperledger-identus/sdk-rust/issues/547)
closes the protocol-core gap without turning `identus-oid4vp` into a
credential proof engine.

```text
signed + verifier-authorized request
                │
                ├── DCQL format + bounded meta ──► CredentialFormatCapability
                │                                      │ static decision
                ├── exact transaction_data ──────► TransactionDataCapability
                │                                      │ static decision
                ▼                                      ▼
      validated DCQL/query linkage + exact encoded transaction
                │
                ▼
       caller-selected query ID + credential format
                │
                ▼
          TransactionBinding ─────────────► owning SD-JWT/mdoc/VC crate
                                               creates/verifies proof
```

## Ownership

The OID4VP crate owns Final envelope syntax, exact base64url representation,
common `type` and `credential_ids` fields, resource limits, DCQL references,
format identity, selection linkage, consumed request lineage, and redacted
errors. It refuses to construct response material until a request containing
transaction data has passed the selected-input binding transition.

Credential-format crates own metadata meaning, transaction-type compatibility,
hashing or embedding rules, proof creation and verification, keys, and native
credential values. Adapters own transport, storage, consent, trust, and product
policy. There is no ambient registry and no SDK-TS/Swift/KMP DTO or error shape
in the Rust contract.

## Supported identifiers and extension rule

The SDK has typed discriminants for `dc+sd-jwt`, `mso_mdoc`, `jwt_vc_json`, and
`ldp_vc`. Other bounded identifiers are retained as redacted extensions. Empty
DCQL metadata remains a baseline path; non-empty metadata and every transaction
type require exactly one injected capability. Unknown or duplicate mandatory
semantics fail closed.

## Assurance boundary

The clean-room E2E signs a request, validates audience/replay and verifier key,
validates non-empty SD-JWT-shaped metadata and a payment-shaped transaction,
links it to a selected DCQL input, and exposes the exact encoded value only to
the proof-engine boundary. This proves protocol lineage and adapter feasibility,
not SD-JWT VC, mdoc, W3C VC, HAIP, certification, live interoperability,
credential authenticity, or user consent.

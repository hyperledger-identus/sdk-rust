# ADR 0184: Bind OID4VP verifier identity through policy ports

- Status: Accepted
- Date: 2026-10-28
- Issue: [#545](https://github.com/hyperledger-identus/sdk-rust/issues/545)

## Context

OID4VP Final client identifier prefixes choose how a Wallet obtains and trusts
the verifier key and metadata. The SDK already verifies request signatures and
security claims, but those facts do not authorize the key for the client. A
bundled DID/PKI/Federation trust engine would impose deployment policy, expand
the dependency cone, and couple the portable core to runtimes and root stores.

The `redirect_uri` prefix cannot sign requests, while this SDK path requires a
signed JAR. `origin` belongs only to the Digital Credentials API. Final also
requires the full prefixed client identifier to be used throughout the flow.

## Decision

Add a protocol-local typed classifier and consuming verifier-authorization
transition. The transition supplies the exact key that verified the JWS, exact
full/original client identifier, signed response destination, protected header,
and bounded scheme metadata to one async caller-owned trust port. The SDK alone
constructs the successor.

Support pre-registered, `openid_federation`, `decentralized_identifier`,
`verifier_attestation`, `x509_san_dns`, and `x509_hash` through that port.
Reject `redirect_uri`, `origin`, and unknown prefixes in this profile. Reuse
generic DID and JOSE types; add no trust-engine dependency.

Parse payload `client_metadata` and `verifier_info` plus protected-header
`trust_chain` under explicit limits and Final source rules. Reject payload
substitution of Federation trust-chain evidence. Keep `direct_post.jwt`
recipient selection a separate closed encryption function, not evidence of
verifier identity.

## Consequences

Adopters must implement the selected scheme completely and fail closed. They
may choose DID resolvers, PKI/root policy, Federation, attestation issuers, and
pre-registration stores appropriate to their product without changing the SDK
contract. Typed routing can no longer bypass verifier authorization.

The SDK maintains bounded classification and metadata code plus a small port
surface. It does not produce a trust score, own roots/network/storage, accept
unsigned redirect-URI requests, or cover DC API origins. Those require separate
specification and evidence.

## Rejected alternatives

- Treat every colon value as a URI or pre-registered fallback: ambiguous and
  unsafe; Final requires prefix-specific processing.
- Bundle concrete X.509/Federation/DID clients: couples core to deployment and
  runtime policy.
- Let adapters mint trusted states: loses SDK state-lineage guarantees.
- Treat encryption metadata as verifier trust: confuses confidentiality with
  authentication.
- Weaken signed-JAR requirements for `redirect_uri`: contradicts ADR 0183 and
  needs a separately designed unsigned flow.

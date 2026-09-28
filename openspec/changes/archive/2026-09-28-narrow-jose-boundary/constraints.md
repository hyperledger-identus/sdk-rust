# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/398
Constraint blockers: none

## Existing entries affected

ADR 0036, ADR 0037, and ADR 0040 place the OID4VCI proof profile inside
`identus-jose`; ADR 0034 and ADR 0035 define the lower generic JOSE layer. ADR
0061 requires a narrow private engine behind Identus-owned types. The new ADR
supersedes only the profile-ownership/dependency placement, not the accepted
proof behavior or trust separation.

## Introduced or changed constraints

`identus-jose` may depend inward on `identus-core` and `identus-crypto`, but not
on DID, OIDC/OID4VCI, transport, storage, wallet, product, chain, clock, replay,
or trust-policy crates. It may retain bounded untrusted JOSE header evidence
carriers, but it cannot interpret their protocol meaning.

OID4VCI proof construction, profile parsing, key-reference resolution,
attestation/federation provider ports, policy, clock, freshness, replay, and
their errors belong to `identus-oid4vci`. Third-party JOSE engines remain
private and operation-scoped; no dependency type or error becomes public.

## Introduced or changed limitations

The SDK continues to implement only its selected compact JWS algorithms and
profiles. It does not become a general JWT/JWE library. Moving ownership is an
unpublished API break for the two source crates and does not guarantee source
compatibility for unversioned consumers.

## Consumer and product impact

Generic JOSE consumers stop inheriting DID/OID4VCI policy. OID4VCI consumers
import the same proof states from `identus-oid4vci` rather than
`identus-jose`. Wire behavior and product/chain responsibilities are unchanged;
no downstream repository is mutated in this slice.

## Activation and rollback

Activation requires issue-bound planning, ADR and OpenSpec evidence, exact
profile test movement, error-contract tests, absence of `identus-did` from the
JOSE manifest/graph, dependency guards, full workspace tests, Clippy/docs,
factory checks, signed/DCO commits, and green protected CI. Rollback reverts
the cohesive extraction; no storage, registry, wire, or external migration is
involved.

## Evidence

Issue #398 records the directed decision. Discussion #399 records the exact
codebase audit at revision `61b8650452812900203658f3072df11b0b2a378c`.
ADR 0159, the library assessment, dependency metadata, moved conformance tests,
and final factory/CI receipts provide the implementation evidence.

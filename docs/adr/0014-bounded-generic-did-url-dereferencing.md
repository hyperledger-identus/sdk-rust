# ADR 0014: isolate bounded generic DID URL dereferencing

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** issue #46, child of #5 / IDR-006
- **Related work:** #43, #44, #45, #10 and #50

## Context

The SDK can represent DID URLs/documents and resolve DIDs, but consumers still
need the portable part of W3C DID URL dereferencing. Putting that logic in every
chain method would duplicate parsing and security rules. Putting retrieval in
the generic layer would instead bind a reusable SSI domain crate to transports,
SSRF policy and product authorization. The relevant W3C algorithm is also
explicitly at risk in the current editor's draft.

## Decision

1. Add an opt-in `GenericDidUrlDereferencer` over an injected
   `Arc<dyn DidResolver>`; preserve all existing ports and registry behavior.
2. Parse parameters once without form semantics, reject malformed/duplicate
   names, and project resolution inputs through bounded typed options.
3. Resolve once, then return bare documents or exact verification-method and
   service resources using the resolved document DID as identity base.
4. Enforce requested DID Core verification relationships and retain current CID
   error identifiers for invalid methods/relationships.
5. Select services conjunctively by exact identifier/type. Return filtered DID
   documents or URI lists containing string endpoints only.
6. Resolve `relativeRef` with RFC 3986 plus a conservative same-authority,
   base-directory scope; reject direct/nested encoded traversal and backslashes.
7. Never retrieve returned endpoints. Custom method/extension resources,
   transports, redirects, recursion, cycles and SSRF policy stay outer.
8. Keep the adapter runtime/chain/cache/clock/global-state free, bounded,
   redaction-safe and independently removable.

## Consequences

- NeoPRISM and Midnight resolvers can share exact resource and service behavior.
- Lace and Oxid can consume typed document resources or endpoint URIs without
  inheriting a transport implementation.
- Method-specific paths and endpoint maps remain explicit extension points.
- The stricter relative-reference policy may reject references accepted by a
  browser; products can supply another dereferencer where broader navigation is
  knowingly authorized.
- A standards change can remove the adapter without changing DID resolution.

## Provenance

W3C DID Resolution `71a5005`, W3C CID `8d2398e`, NeoPRISM `d6ad1ec`,
midnight-identity `427f857`, Lace ID Portal `804de0a`, Oxid `bfe3b48` and
legacy Apollo `ccee22b` were inspected read-only. No donor source or fixture is
copied and no downstream tree is modified.

## Rollback

Revert issue #46's pull request. The feature is opt-in, unpublished, stateless
and performs no external retrieval.

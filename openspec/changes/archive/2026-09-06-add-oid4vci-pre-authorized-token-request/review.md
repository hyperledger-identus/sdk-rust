# Semantic contract review

## Scope reviewed

- Issue #125 and exact base `f8941b16c1474aca64a18c7b538286a570323410`.
- OpenID4VCI 1.0 Final section 6.1 and RFC 6749 Token Endpoint/form rules at
  the immutable hashes recorded in the design.
- Existing grant, metadata, server-binding, input, error, limit, and redaction
  contracts.
- Read-only Oxid and Lace evidence listed in issue #125.

## Findings

1. **Resolved — make body exposure deliberate.** Headless transports require
   exact bytes, so the body is public only through an explicitly sensitive
   accessor and has no Clone, Display, Debug-content, or Serde contract.
2. **Resolved — avoid a hidden allocation denial-of-service.** Exact encoded
   length is checked before allocation under a positive independent limit.
3. **Resolved — erase both source and wire material.** Construction consumes
   the predecessor, and the resulting endpoint/body allocations zeroize on
   drop.
4. **Resolved — keep request claims narrow.** The state contains only mandatory
   parameters and static transport guidance; execution, client concerns,
   optional selectors, responses, trust, and replay remain explicit non-goals.
5. **Resolved — specify interoperability bytes.** Fixed order and uppercase
   UTF-8 form escaping are testable SDK contracts, while order is not claimed
   as a server requirement.

## Decision

The proposal, design, capability requirements, program replacement, and task
map are semantically complete, objectively testable, reversible, and within
the standing mandate. No unresolved blocker remains before implementation.

# Post-implementation exact-diff review

- **Date:** 2026-09-07
- **Reviewed production head:** `c8a92b5c91a54ffa2b08bfc87bdc0f05e06973a8`
- **Exact diff:** `develop@f8941b16...c8a92b5c`
- **Method:** fresh architecture/API/standards/security/resource review after
  focused, workspace, and full Nix gates
- **Result:** no unresolved finding

## Exact-diff findings

1. The only construction path consumes the prepared input state. It obtains
   its endpoint and both request secrets exclusively through the validated
   predecessor chain, preserving offer, metadata, grant, server, and input
   invariants.
2. The body emits fixed names once in fixed order. `tx_code` presence comes
   from the private prepared input rather than a caller-controlled flag.
3. The encoder operates on UTF-8 octets with the RFC 6749 Appendix B literal,
   space, and uppercase-percent rules. Tests pin both mandatory request shapes
   and the RFC example's mixed ASCII/non-ASCII vector.
4. Exact encoded size uses checked arithmetic before `String::with_capacity`.
   The positive 16,384-byte default covers the predecessor defaults' worst
   expansion, and exact/one-less tests cover the limit edge.
5. The predecessor is not retained. Its original zeroizing secret allocations
   drop after construction, while the independently owned typed endpoint and
   encoded body zeroize with the result.
6. Raw body access is explicit, documented as sensitive, and necessary for a
   headless transport. The request has no Clone, Display, or Serde surface;
   Debug contains only byte count and Transaction Code presence.
7. Two fieldless errors map to stable static `oid4vci.*` codes and messages.
   Canary tests cover request Debug plus direct and bridged error Debug/Display.
8. The endpoint remains a validated `TokenEndpoint`; POST and media type are
   static guidance. No execution, client identity/authentication, optional
   selectors, response, trust, or replay claim entered the API.
9. Six focused tests cover the public/wire/security contract in both feature
   modes. No dependency, manifest, lockfile, feature, parser, existing limit,
   unsafe, consumer, chain, or product code changed.
10. Focused/workspace Cargo and all 27 compatible Nix checks pass, including
    Rust 1.85, WASM, Android, iOS, strict lints/docs, supply-chain policy, and
    the 493-test principal suite. Consumer receipts match preflight exactly.

Verdict: READY for specification synchronization and pull-request review.

## Archive-stage correction

The guarded archive appended extra blank lines at the two canonical specs' ends
of file. They were removed before the archive commit; this was a
text-hygiene-only correction with no requirement or implementation change.

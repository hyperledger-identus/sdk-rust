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

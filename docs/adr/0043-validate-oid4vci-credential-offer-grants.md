# ADR 0043: validate known OID4VCI Credential Offer grants

- **Status:** Accepted
- **Date:** 2026-09-06
- **Decision authority:** standing autonomous agent authority under ADR 0004
- **Related work:** #7, #20, #111, #113, #115; `IDR-023`

## Context

The #111 transport and #113 core-semantic boundaries retain complete bounded
Credential Offer JSON, but intentionally expose only whether `grants` exists.
OpenID4VCI 1.0 Final defines Authorization Code and Pre-Authorized Code grant
objects, optional Authorization Server hints, and Transaction Code input
requirements. Consumers need those common wire rules before later metadata
matching or authorization state can choose or execute a flow.

Pre-Authorized Codes are bearer-adjacent, short-lived, single-use values. An
offer and its issuer state, endpoint hints, Transaction Code guidance, and
extensions are untrusted input. Parsing their shape must not imply freshness,
trust, replay resistance, metadata agreement, or authorization.

## Decision

1. Add `CredentialOfferWithGrants` as a distinct state obtained only by
   consuming a validated `CredentialOffer` under positive grant limits.
2. Expose optional typed Authorization Code and Pre-Authorized Code alternatives
   without selecting between them; accept either, both, or neither.
3. Require every grant value to be an object. Preserve unknown grant names and
   members in exact retained JSON while selectively validating the two Final
   known grant shapes.
4. Model a present `tx_code` object, including `{}`, as requiring a Transaction
   Code. Preserve optional mode/length/description and expose the Final
   effective default mode of `numeric`.
5. Accept only `numeric`/`text`, positive bounded integer lengths, and
   descriptions bounded by both configurable bytes and the Final 300-character
   ceiling. Keep unknown Transaction Code members opaque.
6. Validate grant Authorization Server hints as RFC 8414 HTTPS issuer
   identifiers. Defer metadata membership, multi-server applicability,
   discovery, and trust to a later state that possesses issuer metadata.
7. Bound issuer state, Pre-Authorized Code, Authorization Server, description,
   and advertised length independently. Reuse the existing input byte,
   depth/node, duplicate-name, and lexical-number bounds.
8. Zeroize all owned content strings, provide only explicit value accessors,
   use redacted custom diagnostics, and extend the fieldless static error/code
   taxonomy. Keep the normal dependency cone unchanged.

## Consequences

- Wallet adapters can inspect a standards-pinned offer grant without taking on
  network, crypto, product, chain, storage, or UI authority.
- The third selective scan adds bounded CPU work but preserves explicit
  validation states and arbitrary-magnitude unknown extension numbers.
- Lace ID Portal's current `tx_code: null` remains legacy-negative evidence;
  Final requires an object when Transaction Code requirements are present.
- Syntax acceptance does not make a Pre-Authorized Code safe to log or use and
  does not authorize a flow. Replay/freshness, Transaction Code collection,
  metadata matching, grant selection, and token exchange remain later slices.
- The API is additive, experimental, unpublished, and creates no release or
  downstream compatibility promise.

## Alternatives considered

- **Deserialize the complete offer to a generic JSON tree.** Rejected because
  it would duplicate bounded validation states and lose the existing lexical
  handling of arbitrary-magnitude unknown numbers.
- **Choose one grant in the SDK.** Rejected because Final leaves the choice to
  the wallet and selection requires metadata, product capability, and policy.
- **Accept `tx_code: null` for current consumer compatibility.** Rejected from
  the Final profile; consumers may translate legacy input under their own
  compatibility policy before entering this state.
- **Validate Authorization Server metadata agreement immediately.** Rejected
  because this slice owns only offer data and performs no retrieval.

## Migration and rollback

No consumer depends on the unpublished API. A focused revert removes the grant
module, limits/errors/tests/specification, and restores opaque-only handling in
the prior `CredentialOffer` state while preserving #111 and #113.

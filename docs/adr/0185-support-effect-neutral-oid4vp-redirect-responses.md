# ADR 0185: support effect-neutral OID4VP redirect responses

- **Status:** Accepted and implemented by issue #546
- **Date:** 2026-10-28
- **Decision authority:** issue #546 under parent #541 and the standing SDK mandate
- **Related:** ADRs 0168, 0179, 0180, 0181, 0183, and 0184

## Context

OpenID4VP Final permits `vp_token` with OAuth response modes, illustrates
`fragment` for same-device delivery, and defines an optional redirect returned
after direct-post processing. The SDK already owns bounded DCQL/state response
semantics but only emits direct-post material. Implementing redirect encoding
independently in each adapter would duplicate protocol invariants and weaken
the verified request lineage.

## Decision

1. Support `fragment`, `query`, and `form_post` as inert SDK response material;
   retain `direct_post` and `direct_post.jwt` unchanged.
2. Accept one exact signed `redirect_uri` only for redirect modes and one exact
   signed `response_uri` only for direct-post modes. Mixed destinations fail.
3. Require verifier authorization of the exact destination before routing.
4. Reuse one bounded DCQL/state parameter core while keeping mode-specific
   public types explicit and effect-neutral.
5. Consume sent direct-post material before validating its status-200 JSON
   acknowledgement and optional absolute redirect URI.
6. Treat a unique `response_code` as opaque verifier output. Do not infer
   entropy, redeem it, or authorize navigation.
7. Keep HTTP, browser/deep-link dispatch, HTML rendering, privacy, consent,
   session storage, and verifier product policy outside the crate.
8. Reuse the strict allocation-bounded HTTP media-type and cache-directive
   parser in `identus-openid4vc` across OID4VP and OID4VCI. Protocol crates
   retain their own status, limits, and response semantics.

## Consequences

The enum and routing constructors expand before publication, so exhaustive
downstream matches require migration. Consumers gain one Rust-native bounded
protocol core across native, WASM, and mobile targets without importing a
browser or HTTP runtime. Syntax and accepted verifier policy are not a claim
that navigation is safe or that the verifier processed a presentation.
The shared parser adds one deliberate inward dependency to OID4VCI while
removing duplicated security-sensitive field parsing.

## Alternatives rejected

- **Leave all redirect modes to adapters:** duplicates DCQL, state, escaping,
  and bounds while losing request-state evidence.
- **Support only fragment:** contradicts the generic SDK goal and leaves the
  standardized OAuth modes fragmented across adopters.
- **Execute navigation or HTTP:** imports runtime and product policy into the
  protocol crate.
- **Validate response-code randomness locally:** impossible from one opaque
  sample and would create false security evidence.

## Rollback

Before publication, remove the additive modes and acknowledgement successor,
restore the existing routing requirement, and retain all issue/research
evidence. Existing direct-post security and verifier authorization must not be
weakened.

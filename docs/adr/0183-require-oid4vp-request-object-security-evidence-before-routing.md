# ADR 0183: Require OID4VP Request Object security evidence before routing

- Status: Accepted
- Date: 2026-10-28
- Issue: [#544](https://github.com/hyperledger-identus/sdk-rust/issues/544)

## Context

ADR 0158 deliberately stopped at bounded signature and claim-correlation
evidence. OpenID4VP Final additionally requires discovery-bound audience
validation, while RFC 9101/7519 supply temporal and JWT-ID semantics. Allowing
typed routing from signature evidence makes these checks optional for adopters.

General JWT crates duplicate the established JOSE and bounded scanner layers
and do not own atomic caller replay semantics. `identus-jose` must remain a
protocol-neutral primitive. The SDK also prohibits ambient clocks and product
persistence in protocol cores.

## Decision

Add a protocol-local consuming transition in `identus-oid4vp`. It accepts an
explicit discovery audience, finite age/skew policy, `identus_core::WallClock`,
and atomic replay guard. It captures bounded `aud`, integral `iat`, optional
`exp`/`nbf`, and optional `jti` from the verified payload. Typed DCQL and
authorization routing move to the returned security-evidence state.

Static discovery uses `https://self-issued.me/v2`; dynamic discovery uses the
exact issuer obtained by the caller. The Request Object `iss` claim is ignored
as required by Final and is never used to bootstrap expected audience. Replay
storage, durability, key authorization, and trust remain outside the crate.

No new JWT dependency is adopted. The existing bounded scanner, JOSE facade,
clock port, redaction model, and portable target contract are retained.

## Consequences

Adopters must perform one explicit security transition before typed routing.
This is a declared pre-release source break that eliminates a security-policy
bypass. Callers keep freedom to choose in-memory, database, or distributed
atomic replay implementations and to source discovery evidence appropriately.

The crate maintains a small amount of protocol-specific claim parsing and time
arithmetic. Fractional NumericDate values are rejected; this is reconsidered if
interoperability evidence requires them. Security evidence still does not imply
verifier trust, consent, credential validity, or transport authorization.

## Rejected alternatives

- Adding policy to `identus-jose`: rejected as protocol coupling.
- Adopting a general JWT framework: rejected for duplicate parsing/crypto,
  dependency-cone growth, ambient-time defaults, and absent atomic replay.
- Trusting Request Object `iss` as audience: rejected by OpenID4VP Final.
- Exposing check-then-store replay hooks: rejected as race-prone.
- Keeping typed routing on `VerifiedRequestObject`: rejected because it leaves
  the required security transition optional.

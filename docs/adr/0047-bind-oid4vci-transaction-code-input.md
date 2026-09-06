# ADR 0047: bind OID4VCI Transaction Code input without claiming validity

- **Status:** Accepted for issue #123
- **Date:** 2026-09-07
- **Decision authority:** standing autonomous component authority in ADR 0004
- **Related work:** issues #7, #20, #121, and #123

## Context

OID4VCI 1.0 Final requires a Pre-Authorized Code Token Request to contain a
`tx_code` string when the Credential Offer contained a `tx_code` object,
including an empty object. Absence of that object means the parameter is not
expected. The existing SDK validates the offer-side requirements and binds the
offer to an eligible Authorization Server, but it has no state for accepting
the caller's sensitive input.

The offer may advertise input mode, length, and description. The Final text
frames these values as input-screen guidance and leaves actual Transaction Code
validation and wrong-code errors to the Authorization Server. Treating those
hints as a local authentication verdict would overstate what the Wallet can
prove.

## Decision

1. Add a consuming transition from
   `CredentialOfferWithPreAuthorizedServer`, optional caller-owned `String`,
   and positive `TransactionCodeInputLimits` to
   `CredentialOfferWithPreAuthorizedTokenInput`.
2. Require input exactly when the offer's Pre-Authorized Code grant contains a
   `tx_code` object. Missing and unexpected input fail distinctly.
3. Reject empty input and input above an independent 256-byte default limit.
   Count exact decoded UTF-8 bytes without trimming or normalization.
4. Wrap the moved string in `Zeroizing<String>` before validation so both
   success and failure paths erase the owned allocation.
5. Keep the raw code private to the crate. Public state exposes only input
   presence and the predecessor state; Debug and every error remain data-free.
6. Preserve advertised input mode, length, and description as UI guidance
   through the predecessor. Do not claim the input is correct or enforce those
   hints as Authorization Server policy.
7. Defer form encoding, Token Request construction and execution, client
   identity/authentication, response handling, replay/retry policy, trust, and
   downstream adoption.

## Consequences

- A later request-builder slice can consume one state with exact input-presence
  agreement and a bounded secret lifecycle.
- A Wallet can render and validate its UI using issuer hints without the SDK
  misrepresenting that local validation as server acceptance.
- Empty values fail early even though no local code correctness is claimed.
- The public API is additive and unpublished; no wire, Serde, dependency,
  feature, manifest, lockfile, parser, or target contract changes.

## Rejected alternatives

- **Enforce advertised mode and length as protocol validity.** Rejected because
  the Final specification presents them as UI aids and assigns actual code
  validation to the Authorization Server.
- **Accept borrowed `&str`.** Rejected because the prepared state must own the
  value and erase its allocation without copying caller-controlled content.
- **Expose `as_str` publicly now.** Rejected because no public consumer needs a
  raw secret-reading surface before the in-crate request builder exists.
- **Combine input, form encoding, HTTP, and response handling.** Rejected as a
  larger security and compatibility boundary than one reversible slice.

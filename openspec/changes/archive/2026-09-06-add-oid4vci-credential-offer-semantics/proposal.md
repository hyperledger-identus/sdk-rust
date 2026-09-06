# Add bounded OID4VCI Credential Offer semantics

## Why

Issue #111 established a safe, exact Credential Offer transport but deliberately
left the decoded JSON uninterpreted. Oxid and Lace ID Portal still need one
chain-neutral implementation of the Final required-member rules before either
consumer can begin metadata discovery or choose an authorization flow.

## What changes

- Add a distinct validated `CredentialOffer` state created only by consuming an
  `EmbeddedCredentialOffer`.
- Validate the Final `credential_issuer` URL and the non-empty, duplicate-free
  `credential_configuration_ids` array under explicit semantic limits.
- Require `grants`, when present, to be an object while leaving its members
  opaque for the next protocol slice.
- Preserve the exact transport JSON and all unknown top-level members.
- Keep bearer-adjacent content out of diagnostics and preserve the crate's
  existing no-network, no-trust, no-crypto dependency boundary.

## Non-goals

This change does not interpret authorization-code, Pre-Authorized Code, or
transaction-code grant members; retrieve or compare issuer metadata; perform
HTTP, DNS, redirects, authorization, token, nonce, credential, or deferred
operations; choose trust or consent policy; add format/chain extensions; edit
consumers; publish; release; or promote to `main`.

## Impact

- **Issue:** #113, child of #7 and #20 / `IDR-023`.
- **Owner:** existing unpublished `identus-oid4vci` protocol-semantics crate.
- **Compatibility:** additive experimental API; no wire output and no released
  compatibility promise.
- **Dependencies:** retain `identus-core`, `serde_json`, `uriparse`, and
  `zeroize`; no runtime,
  transport, crypto, chain, storage, or product dependency.
- **Rollback:** revert the additive semantic types and records while retaining
  the #111 transport boundary.

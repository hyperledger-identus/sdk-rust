# Design

## Owned semantic transition

Add `CredentialOfferWithAuthorizationRequestInput`, privately owning one
`CredentialOfferWithAuthorizationCodeServer`, an index into its immutable
offered Credential Configuration list, and validated client, redirect, state,
verifier and derived challenge values.

Add
`CredentialOfferWithAuthorizationCodeServer::try_with_authorization_request_input`.
It consumes the server-bound predecessor and accepts a configuration index,
client identifier, redirect URI, CSRF state, verifier and positive input
limits. Validation order is deterministic:

1. Require the selected configuration index to exist.
2. Bound, then validate the non-empty client identifier against RFC 6749
   `VSCHAR` grammar.
3. Bound, then validate an absolute redirect URI with no fragment or userinfo.
   HTTPS, loopback HTTP and private-use custom schemes remain representable;
   redirect policy belongs to the application/client registration.
4. Bound, then validate non-empty RFC 6749 `VSCHAR` state.
5. Validate the RFC 7636 verifier length of 43 through 128 ASCII characters
   and its unreserved-character grammar.
6. Compute SHA-256 over the verifier ASCII bytes and canonical unpadded
   base64url encode the 32-byte digest.

The configuration is represented by an index rather than copying a
caller-provided identifier. This makes selection O(1), cannot retain an
unbounded duplicate, and exposes the existing validated identifier by shared
reference.

## Credential request mechanism

This slice preserves configuration intent but deliberately chooses neither
`authorization_details` nor scope. Final section 5.1 permits those mechanisms,
and the current bounded issuer-metadata core does not retain the optional
configuration scope. A later serializer issue must decide the mechanism with
the needed format/profile and server capability evidence rather than silently
emit both or invent a scope.

## Dependency and secret boundary

ADR 0102 keeps `oauth2 5.0.0` oracle-only. `identus-oid4vci` instead adds a
direct, default-disabled internal dependency on `identus-crypto` with only
`hash` and `base64` features. No new external package or lockfile entry is
introduced; the primitives already exist in the release graph and are
WASM-safe.

Client identifier and redirect URI are not OAuth secrets, but every retained
input uses a redacted Debug surface. CSRF state and verifier are held in
`Zeroizing<String>`. The public challenge is stored under an OID4VCI-owned
type; no crypto or third-party type crosses the public API.

## Error and compatibility surface

Append static errors for invalid limits, absent configuration index,
invalid/oversized client identifier, invalid/oversized redirect URI,
invalid/oversized state, and invalid/oversized verifier. Extend the focused
authorization-code catalogue while it remains below the standing 39-record
ceiling. Existing ordered errors, wire behavior and public types are unchanged.

## Risks and mitigations

- PKCE pair drift: derive S256 internally; never accept a challenge argument.
- Entropy overclaim: the caller generates state and verifier; the SDK proves
  grammar and relationship only.
- Redirect overreach: validate URI structure and forbidden fragment/userinfo,
  but do not choose platform registration policy.
- Credential ambiguity: retain one exact offered configuration by index and
  defer wire-mechanism choice.
- Callback overclaim: document that this predecessor does not validate the
  authorization response, issuer, state return, redirect or code.

## Rollback

Remove the additive module, direct feature-minimal internal dependency,
exports, errors, tests, ADR and capability. Server binding and every prior
OID4VCI state remain intact.

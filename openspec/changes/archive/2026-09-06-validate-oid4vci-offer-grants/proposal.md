# Validate OID4VCI Credential Offer grant shapes

## Why

Issues #111 and #113 established bounded transport and core Credential Offer
semantics while deliberately keeping grant members opaque. Oxid and Lace ID
Portal need one chain-neutral interpretation of the two grant objects defined
by OpenID4VCI 1.0 Final before later metadata and authorization-state slices
can safely select or execute a flow.

## What changes

- Add a distinct grant-validated offer state produced by consuming a validated
  `CredentialOffer` under explicit grant limits.
- Parse the Final `authorization_code` and Pre-Authorized Code grant objects,
  their optional Authorization Server identifiers, and Transaction Code
  requirements without selecting a grant.
- Preserve absent/empty grants, concurrent known grants, opaque unknown grant
  extensions, and the exact embedded JSON.
- Reject known-member type confusion and invalid or excessive secret/state,
  identifier, description, input-mode, and length values.
- Keep all bearer-adjacent content zeroized and out of diagnostics while
  retaining the crate's no-network, no-trust, no-crypto dependency boundary.

## Non-goals

This change does not select a grant; retrieve or match issuer/Authorization
Server metadata; enforce when an Authorization Server hint may be used; build
authorization or token requests; implement PKCE, PAR, replay, freshness,
single-use, Transaction Code collection, or protocol state; perform network or
trust operations; add chain extensions; edit consumers; publish; release; or
promote to `main`.

## Impact

- **Issue:** #115, child of #7 and #20 / `IDR-023`.
- **Owner:** existing unpublished `identus-oid4vci` protocol-semantics crate.
- **Compatibility:** additive experimental API; no wire output and no released
  compatibility promise.
- **Dependencies:** unchanged normal dependency cone; no async, HTTP, crypto,
  DID, storage, chain, platform, or product dependency.
- **Rollback:** remove the additive grant state/types and restore opaque grant
  handling while retaining transport and core offer semantics.

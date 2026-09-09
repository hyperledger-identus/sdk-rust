# Validate OID4VCI Token Response authorization details

## Why

Issue #237 advances IDR-023 after the successful Token Response core began
recording only whether `authorization_details` is present. OpenID4VCI 1.0 Final
requires a wallet to use returned credential dataset identifiers in later
Credential Requests. The SDK needs a typed, bounded validation state before
that request transition can be implemented safely.

## What changes

- Add independent positive limits for Authorization Details count,
  credential-identifier count, and decoded string sizes.
- Add an explicit consuming transition from `TokenResponseCore` to a typed
  validated state.
- Validate the non-empty RFC 9396 array, extract `openid_credential` entries,
  and require a configuration identifier plus non-empty unique credential
  identifiers on each such entry.
- Ignore bounded fields unknown to an `openid_credential` entry and bounded
  authorization-detail types unknown to this SDK capability.
- Keep all caller-controlled identifiers out of `Debug`, `Display`, and stable
  error messages.

## What does not change

The existing partial parser remains compatible and presence-only. This change
does not build Authorization or Credential Requests, validate access tokens,
match Issuer Metadata, perform HTTP, add a dependency, mutate a downstream
repository, publish, or release.

## Capabilities

### New capabilities

- `oid4vci-token-authorization-details`: bounded typed validation of credential
  Authorization Details returned in a successful Token Response.

### Modified capabilities

- `ssi-upstream-program`: advance IDR-023's active bounded child from #149 to
  #237 while retaining `in_progress` status.

## Authority

- Issue #237, child of #7 and program #20.
- OpenID4VCI 1.0 Final section 6.2 and RFC 9396 section 7.

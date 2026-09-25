# Construct request-bound OID4VCI Credential Requests

## Why

Issue #362 proves that one Token Response authorizes source-ordered Credential
Dataset identifiers for the exact configuration selected before authorization.
The existing request constructor still accepts detached metadata and detached
response-local authorization state, allowing callers to recombine otherwise
valid objects. Issue #364 needs the first one-shot continuation from correlated
authority to the existing bounded JWT-proof wire request.

## What changes

- Consume only `CorrelatedAuthorizationCodeTokenResponse`.
- Select one exact authorized dataset by checked source-order index.
- Reuse the existing Bearer/proof/body/authorization validation and serializer.
- Derive the Credential Endpoint, token and dataset identifier only from the
  consumed correlated state; accept no replacement metadata or selector text.
- Preserve existing unpublished constructors while adding the stronger path.
- Add tests, ADR 0147, architecture evidence and focused successor #366.

## What does not change

No proof generation, HTTP, response handling, token validation/storage, DPoP,
client authentication, retry/recovery policy, product selection, trust,
consumer, chain, release or publication behavior is added.

## Capabilities

### Modified capabilities

- `oid4vci-jwt-credential-request`: add consuming request-bound dataset request
  construction through the existing bounded serializer.
- `oid4vci-authorization-code-token-correlation`: make Credential Request
  construction the one-shot continuation.
- `ssi-upstream-program`: keep IDR-023 in progress under focused successor #366.

## Authority

Issue #364, OpenID4VCI 1.0 Final sections 6.2 and 8.2, ADR 0106, ADR 0146,
and the standing SDK delivery mandate.

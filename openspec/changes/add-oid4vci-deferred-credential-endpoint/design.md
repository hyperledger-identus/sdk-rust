# Design

## Public metadata surface

Add `DeferredCredentialEndpoint` beside `CredentialEndpoint` and
`NonceEndpoint`. It owns an exact validated string in `Zeroizing<String>`,
exposes only `as_str`, and has a content-free Debug implementation.
`CredentialIssuerMetadata::deferred_credential_endpoint` returns an optional
borrow. Omission is successful and no fallback is inferred.

## Parser and validation

The strict scanner recognizes the decoded member name once and parses a
non-empty string with `max_credential_endpoint_bytes`. The semantic constructor
then applies `is_valid_https_endpoint`, mapping oversize and unsafe cases to
new field-specific errors. Complete JSON byte/depth/node and duplicate-name
checks remain inherited.

The shared limit remains one field/accessor to preserve the existing public
ten-argument constructor. Documentation expands its meaning to Credential,
Nonce and Deferred Credential Endpoint values, each independently.

## Errors and privacy

Add fieldless `DeferredCredentialEndpointTooLarge` and
`UnsafeDeferredCredentialEndpoint` variants with stable `oid4vci.*` codes.
No endpoint, JSON, parser offset or cause enters direct or bridged diagnostics.

## Risks and mitigations

- Endpoint confusion: a distinct type prevents use as a Credential or Nonce
  Endpoint without an explicit later transition.
- SSRF inference: syntax validation grants no network authority; execution and
  deployment egress policy remain out of scope.
- API breakage: the limits constructor and every existing method stay intact.
- Resource drift: the existing strict scanner and independent shared byte
  budget cover the new retained value.

## Rollback

Remove the additive field, type, accessor, errors, tests, ADR and canonical
capability. Existing metadata parsing and callers remain compatible.

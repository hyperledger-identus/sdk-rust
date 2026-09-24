# Design

## Request-bound outcome surface

Add `DeferredCredentialHttpResponseLimits`, composed from one positive maximum
Content-Type byte count plus existing `ImmediateCredentialResponseLimits` and
`DeferredCredentialResponseLimits`. Add a non-Clone
`DeferredCredentialOutcome` with `Issued(ImmediateCredentialResponseCore)` and
`Pending(DeferredCredentialResponseCore)` branches and redacted Debug inherited
from those cores.

`DeferredCredentialRequest::validate_response` borrows the request and accepts
the numeric status, optional Content-Type, body bytes and composite limits. It
returns exactly one typed successful outcome without performing HTTP.

## Private correlation state

`DeferredCredentialRequest` retains the decoded originating transaction
identifier in private zeroizing storage in addition to the already-zeroizing
serialized body. Construction copies only the already bounded decoded value.
No public accessor, Clone, generic Serde, Display or diagnostic content is
added.

## Validation order

1. Reject statuses other than `200` and `202` with one static error.
2. Reject an absent, oversized or non-JSON Content-Type with static errors.
3. For `200`, parse the body through `ImmediateCredentialResponseCore` and
   return `Issued`.
4. For `202`, parse through `DeferredCredentialResponseCore`, compare its
   transaction identifier exactly with the retained request identifier, reject
   mismatch with a static error, then return `Pending`.

The body parsers retain their independent limits and error taxonomy. The
transition does not reinterpret a protocol error response as success.

## Errors and privacy

Add fieldless invalid-limits, unsupported-status, Content-Type-too-large,
invalid-Content-Type and transaction-mismatch variants with stable
`oid4vci.*` codes and static messages. No error carries status, media type,
body or transaction content.

## Risks and mitigations

- Transaction substitution: compare the parsed pending identifier to the exact
  request-owned identifier before returning the value.
- Resource amplification: bound Content-Type first and delegate body bounds to
  the existing parsers.
- Secret leakage: retain only zeroizing private state and keep errors and Debug
  content-free.
- Unsound request binding: do not claim immediate proof-count correlation
  because this request does not retain the originating credential request.
- Lifecycle coupling: validation remains repeatable and does not schedule,
  retry or invalidate transactions.

## Rollback

Remove the additive limits, outcome, validator, errors, tests, ADR and private
retained field. Existing request serialization and standalone response parsing
remain source and wire compatible.

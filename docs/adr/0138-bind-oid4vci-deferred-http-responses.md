# ADR 0138: bind OID4VCI deferred HTTP responses to their request

- **Status:** Accepted for implementation
- **Date:** 2026-09-25
- **Decision authority:** issue
  [#345](https://github.com/hyperledger-identus/sdk-rust/issues/345)
- **Related:** ADR 0057, ADR 0109, ADR 0137, and component issue
  [#7](https://github.com/hyperledger-identus/sdk-rust/issues/7)
- **Applies to:** `identus-oid4vci` only

## Context

The SDK constructs a bounded unencrypted Final Deferred Credential Request and
separately parses bounded immediate and deferred Credential Response bodies.
The request discards its decoded transaction identifier after serialization,
so a headless wallet must reimplement HTTP status/media-type branching and the
Final requirement that a pending response return the same transaction ID.

The existing immediate request binding cannot be reused directly: a deferred
request has two successful outcomes and does not retain the originating proof
count. Pretending it does would create an unsound credential-count claim.

## Decision

`DeferredCredentialRequest` privately retains a zeroizing copy of its decoded
transaction identifier. `validate_response` accepts caller-supplied status,
effective Content-Type, body and `DeferredCredentialHttpResponseLimits`.
Statuses other than 200/202 fail before remote fields. Content-Type is bounded
and must satisfy the existing `application/json` rules.

Status 200 delegates to the bounded immediate parser and returns
`DeferredCredentialOutcome::Issued`. Status 202 delegates to the bounded
deferred parser, requires exact equality with the request-owned transaction ID
and returns `Pending`. The operation remains repeatable and owns no HTTP,
token, TLS, timer, retry or transaction invalidation behavior.

Five fieldless errors cover invalid composite limits, status, Content-Type
size, Content-Type syntax and transaction mismatch. Under ADR 0137 their enum
variants and router records append after the immutable v1 prefix; exact feature
tests own their codes, kinds, messages, conversions and redaction.

## Consequences

- Consumers receive one exhaustive typed issued/pending transition and can
  remove duplicated successful-response glue.
- A substituted pending transaction fails before state is returned.
- Body and header work remain independently bounded through existing parsers.
- The request retains one additional bounded zeroizing identifier copy.
- Immediate outcomes deliberately make no proof-count correlation claim.
- Error responses, encryption and transaction lifecycle remain later slices.

## Alternatives rejected

Reparsing the serialized request body duplicates sensitive parsing and couples
correlation to wire encoding. Returning an uncorrelated deferred core violates
the Final rule. Reusing semantically inaccurate existing errors obscures the
failure contract. A transport client would add runtime and policy coupling.
Constant-time equality is unnecessary for this local correlation check and
would not create authentication.

## Verification and rollback

Verification requires exact 200/202 examples, status/media/body limits,
Content-Type grammar, equal/mismatched transaction IDs, repeated validation,
diagnostic redaction, append-only error compatibility, full crate tests,
strict Clippy/docs/format, factory/OpenSpec, portable Nix evidence and hosted
CI. Rollback removes the additive types, validator, private retained identifier
and five suffix errors while preserving existing request construction and body
parsers.

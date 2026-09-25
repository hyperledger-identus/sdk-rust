# ADR 0139: compose Deferred Credential payload errors from the generic core

- **Status:** Accepted for implementation
- **Date:** 2026-09-25
- **Decision authority:** issue
  [#348](https://github.com/hyperledger-identus/sdk-rust/issues/348)
- **Related:** ADR 0057, ADR 0109, ADR 0138, and component issue
  [#7](https://github.com/hyperledger-identus/sdk-rust/issues/7)
- **Applies to:** `identus-oid4vci` only

## Context

OpenID4VCI 1.0 Final section 9.3 defines Deferred Credential errors by reusing
the section 8.3.1 Credential Error Response, adding
`invalid_transaction_id`, and directing a wallet to stop polling when exact
`credential_request_denied` is returned. The SDK already has a strict bounded
Credential Error body and HTTP parser, but it treats the new endpoint-specific
code as an extension and deliberately assigns no lifecycle policy.

Adding the code to the existing public closed `CredentialEndpointErrorKind`
would mix generic and deferred concerns and could break exhaustive downstream
matches. Duplicating the parser or its limits would create two owners for the
same inherited wire contract.

## Decision

`DeferredCredentialRequest::validate_error_response` delegates status,
Content-Type, body, limits, JSON shape, code grammar, description handling and
static diagnostics to `CredentialErrorResponseCore::parse_http_response`.
It returns a `DeferredCredentialErrorResponse` that owns the bounded core.

`DeferredCredentialErrorKind` classifies exact `invalid_transaction_id`, exact
`credential_request_denied`, or wraps the existing generic classification for
every other accepted code. `should_stop_polling` returns true only for exact
`credential_request_denied`, reflecting the standard's explicit guidance as
pure data without performing a lifecycle action.

The request-owned transaction identifier is not compared because the error
body carries no identifier. Calling through the request binds the structural
transition in the API; it does not prove response origin or correlation.

## Consequences

- Generic parsing, resource bounds and diagnostics remain single-sourced.
- Deferred-specific semantics are additive and orthogonal to the existing
  closed generic classification enum.
- Callers can read the exact bounded code and deliberately untrusted
  description through the retained core.
- Debug exposes only classifications and the core's existing redacted shape.
- Repeated validation is side-effect free; scheduling, retry, cancellation,
  invalidation, persistence and RFC 6750 challenges remain outside the SDK
  primitive.
- No error, limit, dependency, feature, lockfile, wire or target contract is
  added or changed.

## Alternatives rejected

Widening `CredentialEndpointErrorKind` couples endpoint-specific semantics and
risks exhaustive-match breakage. Reimplementing the body/HTTP parser creates
drift. Treating every error as terminal invents policy the Final does not
state. Requiring Cache-Control turns a non-normative example into a false wire
requirement. A transport or polling engine would add runtime and product
authority.

## Verification and rollback

Verification requires exact invalid-transaction and denied examples,
inherited known/extension codes, status/media/body precedence, forbidden
generic `invalid_request`, repeatability, stop-guidance purity, diagnostic
redaction, full crate tests, strict Clippy/docs/format, factory/OpenSpec,
portable Nix evidence and hosted CI. Rollback removes the additive wrapper,
enum, request method, tests and capability without changing generic error or
successful-response behavior.

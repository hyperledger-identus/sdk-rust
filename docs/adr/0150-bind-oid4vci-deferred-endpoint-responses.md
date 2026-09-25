# ADR 0150: bind OID4VCI deferred endpoint responses

- **Status:** Accepted
- **Date:** 2026-09-25
- **Issue:** [#370](https://github.com/hyperledger-identus/sdk-rust/issues/370)
- **Decision authority:** ADR 0109, ADR 0148, ADR 0149 and issue #370

## Context

The authority-preserving Deferred Credential Request carries the exact
issuer-advertised endpoint, bearer Authorization, transaction and originating
proof count. Its existing success and payload-error validators are borrowed and
independent: they cannot enforce one-shot bearer lifetime, and the HTTP 200
path deliberately has no proof-count evidence.

OpenID4VCI 1.0 Final assigns issued credentials to HTTP 200, a still-pending
exactly correlated transaction to HTTP 202, and inherited Credential Error
Responses to HTTP 400. Each proof key binds at most one issued credential.

## Decision

1. Add one consuming, status-first Deferred Credential Endpoint classifier with
   closed issued, pending and deferred-error outcomes for exact 200, 202 and
   400 statuses.
2. Compose only the existing successful Deferred Credential HTTP limits and
   Credential Error HTTP limits. Reuse the existing bounded wire parsers,
   static errors and redacted response states.
3. On HTTP 200, copy only the proof count, erase the complete request before
   remote parsing, and reject credential cardinality above that count. Return
   the existing `RequestBoundImmediateCredentialResponse`.
4. On HTTP 202, erase the serialized request body before parsing. Retain the
   exact issuer, endpoint, zeroizing Authorization and proof count only when
   the parsed transaction equals the request transaction. Return the existing
   `RequestBoundDeferredCredentialResponse` so only it can construct the next
   authorized request.
5. On HTTP 400, erase the complete request before parsing and retain only the
   deferred payload-error response plus originating proof count in a new
   redacted terminal wrapper.
6. On unsupported status, media/body error or transaction mismatch, return the
   existing static error and retain no request authority.
7. Keep all borrowed validators behavior-compatible, repeatable and available
   as weaker structural APIs. Add no error, dependency, feature, wire shape,
   unsafe/native code, HTTP, timing, retry, polling or product effect.
8. Hand M4 capability/fixture reconciliation to focused issue
   [#372](https://github.com/hyperledger-identus/sdk-rust/issues/372).

## Consequences

The strongest deferred path is now one-shot end to end: terminal and invalid
branches cannot keep the bearer capability, while an exact pending response can
continue without detached replacement inputs. Issued credentials receive the
same proof-count guard as immediate Credential Endpoint issuance. Existing
structural callers do not break.

The type transition proves bounded syntax and owned lineage, not HTTP origin,
issuer control, token validity, endpoint reachability, TLS, transaction
freshness, retry safety, credential validity, storage or trust.

## Reconsideration and rollback

Reconsider the outcome when encrypted responses, access-token rotation or a
separately evidenced transport executor changes authority ownership. Rollback
removes the additive classifier, composite limits and terminal error wrapper;
the borrowed validators remain unchanged.

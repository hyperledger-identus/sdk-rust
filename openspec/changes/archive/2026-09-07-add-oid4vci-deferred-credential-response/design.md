# Design: bounded OID4VCI deferred Credential Response core

## Context

OpenID4VCI 1.0 Final section 8.3 defines a deferred Credential Response with
mutually exclusive `transaction_id` and `credentials` branches. The deferred
branch requires a `transaction_id`, a positive JSON-number `interval`, and
HTTP status 202. The current immediate body parser rejects the branch but does
not expose a reusable deferred representation.

This slice owns body syntax only. A later request-bound HTTP capability will
validate status/media and correlation. Read-only Oxid at
`fe6db7b87efdaf8d72b42808974b06ce8260ce1c` rejects deferred responses, while
Lace ID Portal at `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`
does not supply a normative Rust implementation. No source or fixture is
copied or transformed.

## Goals / Non-Goals

**Goals:**

- parse one complete duplicate-safe deferred response object under explicit
  byte, depth, node, member, identifier, and interval limits;
- retain the decoded non-empty transaction identifier and exact positive JSON
  number lexeme without floating-point conversion;
- reject immediate-only fields and keep extensions bounded but authority-free;
- keep direct, Debug, and bridged diagnostics free of remote values;
- preserve Rust 1.85, browser-WASM, Android ARM64, and iOS ARM64 portability.

**Non-Goals:**

- HTTP 202, Content-Type, Cache-Control, transport, endpoint provenance, or
  request correlation;
- conversion to a timer duration, wait/backoff/retry policy, wall-clock or
  monotonic scheduling, transaction freshness or invalidation;
- Deferred Credential Request construction or polling lifecycle;
- encrypted responses, Authorization Errors, credential verification/storage,
  notifications, FFI, downstream adoption, publication, release, or `main`.

## Decisions

### Preserve a positive JSON number instead of converting it

`DeferredCredentialInterval` owns the exact valid JSON-number lexeme. A value
is mathematically positive when it has no leading minus and its mantissa
contains at least one non-zero digit. This accepts integer, fractional, and
exponent syntax exactly, rejects positive and negative zero, avoids overflow or
rounding, and assigns no scheduling semantics.

The exact lexeme has its own positive byte bound and enters zeroizing ownership
only after that bound succeeds. If later object validation fails, normal drop
semantics erase the retained value before the error is returned.

### Treat the transaction identifier as capability-bearing material

`DeferredTransactionId` stores the decoded non-empty value in zeroizing memory
and exposes it only through an explicitly sensitive accessor. Neither it nor
the interval is printed by Debug or any error. The type does not prove that the
Issuer minted the identifier, that it is fresh, or that using it is authorized.

### Keep branches unambiguous

The parser requires both deferred members. `credentials` and `notification_id`
fail as branch violations even if their values would otherwise parse. Unknown
unique members are traversed under the same aggregate limits and discarded, as
required for extension compatibility.

This parser does not replace the immediate parser and does not weaken its
explicit deferred rejection.

## Risks / Trade-offs

- **Consumers need a numeric delay** -> retain the exact standards value and
  leave rounding, caps, clocks, and scheduling to a later policy-bearing type.
- **Tiny positive exponents can be impractical** -> syntax remains distinct
  from policy; the exact lexeme is bounded and no arithmetic occurs here.
- **A transaction identifier can authorize later retrieval** -> zeroize it,
  redact diagnostics, and name its accessor sensitive.
- **Extensions may define future behavior** -> traverse and discard them; this
  core assigns no meaning or retention authority.

## Migration Plan

Land as an additive unpublished API. Existing immediate response parsing and
errors remain unchanged. Rollback is one focused revert before publication or
adoption. HTTP binding, Deferred Credential Request construction, polling, and
consumer adoption remain separate issues.

## Open Questions

None. The Final explicitly uses JSON `number`, not `integer`; exact lexical
retention is the narrow reversible interpretation.

## Provenance

- OpenID4VCI 1.0 Final section 8.3, retrieved 2026-09-07,
  `https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html`,
  HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- Oxid read-only evidence:
  `MediaNoxLabs/oxid@fe6db7b87efdaf8d72b42808974b06ce8260ce1c`,
  Apache-2.0; no source or fixture copied.
- Lace ID Portal read-only evidence:
  `input-output-hk/lace-id-portal@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`;
  no source or fixture copied and no license grant relied upon.

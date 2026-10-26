# ADR 0175: bound generic DID values for PRISM long-form state

- **Status:** Proposed for implementation
- **Date:** 2026-10-26
- **Decision authority:** issue
  [#524](https://github.com/hyperledger-identus/sdk-rust/issues/524)
- **Related:** ADRs 0093, 0125, 0169, 0170, and 0174; issues #493, #521,
  #523, and #524
- **Constraint impact:** material; changes public lexical ceilings and cache
  bypass behavior while preserving unrelated ingress and codec budgets

## Context

The generic DID parser currently accepts 2,048 bytes. The pinned PRISM method
specification embeds a protobuf create operation in long-form DIDs and permits
50 verification methods plus 50 services. An exact isolated model at the
pinned schema produced identifiers from 37,686 to 111,003 bytes depending on
key representation, Unicode endpoint bytes, and finite contexts. Existing
repository fixtures below 700 bytes do not cover that profile.

Unlimited parsing is not acceptable. Nor should a method-specific need widen
the crypto text codec, generic JSON ingress, cache retention, method-name, or
HTTP transport policies. PRISM projection also repeats the identifier about
198 times at maximum key/service cardinality, so a 128,075-byte identifier can
contribute roughly 25.4 MB to native output before other document content.

## Decision

### Use finite literal generic lexical ceilings

`MAX_DID_BYTES` becomes the literal public value 131,072.
`MAX_DID_URL_BYTES` and `MAX_URI_BYTES` become 139,264. A DID URL independently
caps the path/query/fragment suffix at 8,192 bytes, so the latter envelope is
a 128 KiB DID plus an 8 KiB relative/query/fragment allowance. DID document
verification and service IDs use `Uri`, and the existing `From<DidUrl>`
conversion is infallible; matching ceilings prevent it from constructing a
URI that `Uri::parse` would reject. Literal definitions are required by the
architecture mapping checker. Parsing stays bounded and checks length before
success allocation.

These values admit the future `identus-prism-did` profile proposed by #521:
96,000 raw create-operation bytes, at most 128,000 unpadded base64url
characters, and a maximum full identifier of 128,075 bytes. That profile is
not implemented or promised by this ADR; #521 owns its exact context, field,
decode, work, and semantic bounds.

### Decouple method names from enclosing identifiers

`MAX_DID_METHOD_BYTES` remains 2,042. ADR 0093's derivation from
`MAX_DID_BYTES` is superseded, but its owned validated-newtype decision remains
in force. Embedded state expands the method-specific identifier, not the
method name. Generic `Did` and `DidUrl` parsing enforces the same independent
method-component ceiling as `DidMethod`.

### Treat cache capacity as independent retention policy

The normalized cache key remains capped at 65,536 bytes and capacity remains
at most 4,096 entries. A valid request that cannot fit the key budget resolves
upstream without consulting the cache or clock and reports
`KeyBudgetBypass`. This applies to both cache failure modes because capacity is
known policy, not an infrastructure failure. Actual clock/backend/entry
failures retain their configured bypass or fail-closed behavior.

### Keep method output and transport budgets separate

Generic hostile JSON ingress remains 256 KiB for documents and 512 KiB for
resolution results. Issue #521 must derive a separate native PRISM projection
and serialization budget and stable adapter outcome. Any high-budget re-ingress
must be an explicit parser rather than a global default increase.

`MAX_CRYPTO_TEXT_BYTES` and HTTP resolver behavior remain unchanged. PRISM owns
a bounded private base64url codec. Long-form initial-state
resolution is local; deployment infrastructure continues to own HTTP
request-target limits. WASM and UniFFI delegate lexical validation but their
host runtimes own preallocation before Rust entry.

## Consequences

- Generic values can carry the measured finite PRISM profile without learning
  PRISM semantics.
- Generic DID documents can represent long-form-prefixed verification and
  service IDs without violating the DID URL-to-URI conversion invariant.
- Exported limit values and cross-language adapter contracts deliberately
  change; smaller accepted values remain compatible.
- Large valid identifiers may not be cached and may not fit arbitrary external
  transports. Both limitations are explicit rather than accidental failures.
- Exact-boundary fixtures, fuzz probes, WASM/UniFFI canaries, architecture
  inventories, and generated adapter mappings must change atomically.
- Maximum-profile native projection remains gated on #521 rather than silently
  weakening generic untrusted JSON parsing.

## Alternatives rejected

- **Keep 2 KiB:** prevents specification-relevant long-form PRISM values from
  reaching the method crate.
- **Remove limits:** makes resource work attacker-controlled.
- **Use the crypto base64url value:** couples protocol state to an unrelated
  4 KiB key/text policy.
- **Raise cache keys above maximum DID plus options:** multiplies large
  identities across thousands of retained entries without product evidence.
- **Raise generic JSON ingress to projection worst case:** weakens every DID
  consumer to accommodate a method-owned output shape.
- **Require HTTP transport for long-form values:** confuses local initial-state
  resolution with network resolution and depends on external request limits.

## Verification and rollback

Verification includes exact/one-over native and cross-language vectors,
redacted errors, cache bypass under both modes, fuzz boundary probes, WASM and
UniFFI canaries, target checks, a directional parser benchmark, the resource
inventory, generated adapter mappings, factory checks, independent review, and
protected CI.

Before publication, one focused revert restores the prior policy. After
publication, reducing accepted lexical limits or changing cache outcomes needs
a superseding ADR and compatibility ledger. Method-specific output and
transport decisions remain independently reversible.

# Context

The adapter has one fixed route and owns all HTTP behavior. The OpenAPI surface
must remain synchronized without infecting chain-neutral DID types with a
documentation framework.

# Goals

- Return a typed, deterministic OpenAPI 3.1 document for implemented behavior.
- Keep the dependency and public API opt-in and local to the outer adapter.
- Describe exact representations, common options, bounds and status mapping.
- Preserve router and wire behavior byte-for-byte.

# Decisions

## Construct the document with Utoipa's model only

Use exact `utoipa 5.5.0` with default features disabled. Build `OpenApi`, path,
operation, parameters, responses, content and small component schemas through
the public model builders. Do not use `utoipa-gen` macros or add derives to DID
Core. One public feature-gated function returns the Utoipa document so an
already framework-coupled host can merge it directly.

## Describe the fixed mount-relative route

The document contains only `/{did}`. The runtime router keeps the same path and
consumers keep selecting an external prefix by nesting. The SDK does not accept
or rewrite a caller string because that would create a second untrusted path
grammar and allow the document to diverge from runtime routing.

## Keep schemas truthful and extensible

Define component schemas for a DID document and resolution result at the
top-level contract needed by this adapter. Preserve extension capability using
open object schemas rather than claiming every method-specific member. The GET
operation documents the required DID path, four common optional query fields,
string method extensions in prose, and the existing byte/member limits.

The success response offers all three implemented media types. Standard error
statuses offer only `application/did-resolution`. Every response documents the
`Vary: Accept` header. No POST, dereferencing, server or authorization surface
appears.

## Test serialized structure, not formatting

Serialize to `serde_json::Value` and assert operation count/method, parameters,
media types, response statuses, component references and absence of unsupported
paths/methods. Constructing twice must yield identical values. Tests do not pin
pretty-print whitespace or Utoipa's internal field order.

# Compatibility matrix

| Build/request | Result |
| --- | --- |
| Default or no-default feature build | Existing graph/API/runtime behavior |
| `openapi` feature build | Additive document function and Utoipa model |
| Router invocation | Byte-for-byte unchanged |
| Host nesting | Runtime and document remain mount-relative at `/{did}` |
| Unknown method query option | Runtime accepts bounded string; document describes extension support without enumerating names |

# Rollback

Revert the focused PR. The optional dependency, feature and document function
disappear; no runtime, data, wire or default-consumer migration is required.

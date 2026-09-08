# ADR 0091: decode bounded DID Resolution GET options

- **Status:** Accepted for implementation
- **Date:** 2026-09-08
- **Issue:** [#203](https://github.com/hyperledger-identus/sdk-rust/issues/203)
- **Parent:** [#10](https://github.com/hyperledger-identus/sdk-rust/issues/10)
- **Decision authority:** W3C DID Resolution GET binding and SDK boundary rules

## Context

ADR 0090 introduced a bounded Axum DID Resolution route but deliberately
rejects every non-empty query. The W3C GET binding carries all resolution
options except `accept` as URL query parameters. DID Core already owns the
typed, extensible `ResolutionOptions`; the missing component is a strict outer
transport decoder.

Common URL/form crates do not provide the desired semantics. In particular,
ADR 0083 established that `form_urlencoded 1.2.2` converts plus to space,
preserves malformed percent escapes and decodes invalid UTF-8 lossily. Full
`url` and serde form stacks add broader coupling without fixing that mismatch.

## Decision

Implement one private decoder in `identus-did-resolver-http`, modeled on the
existing generic DID URL decoder but independently bounded for HTTP query
options. Do not add a production dependency or expose a generic codec API.

Limit raw input to 8 KiB, parameter count to 32, decoded names to 256 bytes and
decoded values to 4 KiB. Split raw `&`/first `=` before a single strict `%HH`
decode; preserve literal `+`; reject invalid UTF-8, controls, empty names,
missing `=`, duplicate decoded names and every exceeded ceiling.

Map exact `expandRelativeUrls`/`noCache` booleans and typed
`versionId`/`versionTime`; reject both version selectors together. Preserve
unknown names as string-valued extensions. Reject query `accept`, because the
binding maps it exclusively from the HTTP header. Negotiated document media
sets option `accept`; full-result media does not. Collapse every decoder or
builder failure to the static W3C `invalidOptions`/400 response before resolver
invocation.

## Consequences

- Hosts gain the W3C GET option surface without recreating a parser.
- DID Core and the dependency graph remain unchanged.
- Negative behavior is explicit, bounded and redaction-safe.
- Extension values are strings; GET does not invent JSON scalar coercion.
- Similar private parsing remains in DID URL dereferencing. Sharing is deferred
  until exact contract identity and a second consumer justify an abstraction.
- The W3C draft, host-only, unpublished and downstream limitations remain.

## Alternatives rejected

### Adopt `form_urlencoded` or `serde_urlencoded`

Their browser-form plus and permissive/lossy decoding semantics conflict with
the URI-query and fail-closed contract. Prevalidation would retain the risky
scan while adding parsing and allocation.

### Adopt `url`

The adapter already receives the raw query. Full URL/IDNA support adds a broad
cone and still exposes form-style query-pair behavior.

### Extract a public shared query codec

Resolution options and DID URL parameters currently differ in ownership,
limits and error mapping. A public abstraction would freeze semantics before a
named second exact consumer exists.

### Infer JSON types for extension values

Method options define their own scalar semantics. Treating text such as
`true`, `0` or `null` as JSON primitives would be an undocumented transport
transformation and make exact round-tripping surprising.

## Verification and rollback

In-memory tests cover common and extension projection, representation merge,
literal plus, encoded delimiters, malformed encodings/UTF-8/controls,
duplicates, invalid types, version conflict, every exact limit, no-call and
redaction. Full repository/factory/dependency/Nix/hosted gates remain required.
Revert the focused PR to restore blanket query rejection; no migration exists.

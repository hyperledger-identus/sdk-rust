# Design: bounded Token Error Response core

## Context

OID4VCI 1.0 Final section 6.3 delegates Token Error Responses to OAuth 2.0
section 5.2 and clarifies how `invalid_request`, `invalid_grant`, and
`invalid_client` apply to the Pre-Authorized Code flow. RFC 6749 requires an
`error` code, permits `error_description` and `error_uri`, defines six codes,
allows protocol extensions to register more codes, and leaves response sizes
undefined.

Lace ID Portal emits `invalid_grant` plus an optional description. Oxid
currently collapses token endpoint failures into product-level rejection.
These independent Apache-2.0 behaviors are evidence only; the standards govern
the SDK contract.

## Goals and non-goals

Goals:

- parse one bounded top-level JSON object with duplicate rejection;
- preserve the exact open-registry error code and classify standard codes;
- validate RFC character and URI-reference rules for known fields;
- bound and zeroize every retained remote string;
- keep diagnostics and Debug free of caller-controlled response content;
- ignore structurally valid bounded extensions semantically.

Non-goals:

- HTTP status, media type, cache headers, authentication challenge, endpoint
  execution, or correlation with a request;
- retry, replay, remediation, localization, UI safety, navigation, SSRF policy,
  issuer trust, or client blame;
- success/error response union, Authorization Details, nonce, Credential
  Request/Response, storage, FFI, downstream adoption, publication, or release.

## Decisions

### Keep the code registry open and classification closed

`TokenEndpointErrorCode` owns the exact RFC `NQSCHAR` string. Its `kind`
method returns `TokenEndpointErrorKind`, a closed Copy enum covering the six
RFC 6749 codes plus `Extension`. This preserves interoperability with future
registered or deployment-specific codes without allowing arbitrary strings to
be mistaken for a known semantic class.

The core does not decide whether a code is truthful, retryable, actionable, or
appropriate for the request. OID4VCI's Transaction Code clarifications remain
documentation semantics rather than new wire values.

### Treat optional metadata as untrusted remote content

`error_description` follows `1*NQSCHAR`, is retained in zeroizing storage, and
is available only through `expose_untrusted_description`. It is developer
information, not localized or safe end-user copy.

`error_uri` must be non-empty, use RFC's restricted ASCII character set, and
parse as a URI-reference. `TokenErrorUri` exposes the exact validated string
but explicitly provides no dereference operation. Relative references remain
valid because RFC 6749 does not require an absolute or HTTPS URI; resolving or
following them belongs to a later transport/policy layer.

### Bound all work and discard extensions after validation

Defaults are 32,768 total JSON bytes, depth 16, 512 nodes, 256 decoded error
bytes, 4,096 description bytes, and 2,048 URI bytes. Every maximum is positive
and depth cannot exceed the repository maximum. Aggregate size fails before
copying; known strings receive independent decoded bounds.

The strict scanner validates complete JSON, all duplicate-decoded member
names, depth, and node count. Unknown members are then discarded. Unlike the
successful response, no future semantic transition needs the raw error body,
so the core stores only response length and retained known strings. The caller
remains responsible for erasing its input allocation.

### Keep the public state and diagnostics narrow

`TokenErrorResponseCore` exposes response length, exact code, classification,
optional-field presence, the explicitly untrusted description, and validated
URI wrapper. It has no Clone, Display, Serde, raw JSON, network, or FFI API.
Debug reports only response byte count, known classification, and optional
presence. Static fieldless construction errors bridge to stable `oid4vci.*`
codes without parser causes or remote values.

## Verification

- Positive tests cover the Final minimal example, all six RFC classes, an
  extension code, both optional fields, and ignored nested extensions.
- Negative tests cover missing/duplicate/wrong-type fields, all string
  grammars, URI syntax, every independent bound, total bytes, depth, nodes,
  malformed/trailing JSON, and empty values.
- Canary tests cover response/code/URI Debug plus direct and bridged errors.
- Focused tests run in both feature modes with strict Clippy/docs; workspace,
  factory, Rust 1.85, WASM/mobile, supply-chain, and full Nix gates remain
  mandatory.
- Consumer HEAD/status and referenced-file hashes must match preflight.

## Provenance

Normative sources are OpenID4VCI 1.0 Final section 6.3, immutable HTML SHA-256
`f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`,
and RFC 6749 section 5.2 plus Appendix A.7-A.9, RFC Editor HTML SHA-256
`535362fa3b4ca668d4734c244c6ed8c811f2ccd7ea1612a4201e5d8922a74b4e`.

Read-only Apache-2.0 behavior evidence is pinned in issue #129 at Oxid
`5ba38b9b` and Lace ID Portal `804de0a9`; no production source is copied.

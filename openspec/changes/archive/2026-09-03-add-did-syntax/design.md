## Context

W3C DID Core 1.0 defines a DID as `did:` followed by a lowercase method name
and a method-specific identifier, and defines a DID URL by composing that DID
with RFC 3986 path, query and fragment grammar. This is a lexical boundary:
being syntactically valid does not make a DID method registered, resolvable or
authorized.

The donor audit found useful consumer shapes but no implementation suitable
for direct extraction. NeoPRISM at
`8becb225132efb1d9302b2c5f6ed4d87b84e8685` wraps IOTA Identity's parser;
adopting it would make the SDK core depend on a much larger DID stack.
midnight-identity at `427f8571950c42967a18726cbcbefecc19ef8d79`
performs only prefix and split checks. Apollo at
`ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` supplies cross-language
conformance context, not DID syntax. Oxid at
`685f9670af4846d52697a4cfeb94779758ae1075` supplies Midnight consumer
shapes. Lace ID Portal at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` remains evidence-only because
repository-level license evidence is unresolved. No donor source is copied.

## Goals / Non-Goals

**Goals:**

- Represent only syntactically valid absolute DIDs and DID URLs.
- Keep parsing dependency-free, linear, bounded and portable to wasm/mobile.
- Make repeated component reads zero-allocation.
- Apply identical validation to native and serde construction.
- Expose stable redacted public errors and useful non-sensitive local reasons.

**Non-Goals:**

- DID documents, verification relationships, services, resolution,
  dereferencing, relative DID URLs, method registration or method-specific
  identifier semantics.
- PRISM, Midnight, `did:key`, `did:web` or other method implementations.
- URI normalization, equivalence, percent-decoding or mutation builders.
- Package publication/rename, downstream repository edits or full fuzzing.

## Decisions

### Decision 1: define semantic owned values, not string aliases

`Did` and `DidUrl` own one immutable string and cache byte offsets discovered
during validation. `parse(&str)` performs one allocation after validation;
`TryFrom<String>` reuses the caller's allocation. Accessors return slices for
the method, method-specific identifier, embedded DID, path, query and
fragment. Query and fragment accessors exclude their delimiters; path includes
its leading slash. Presence is represented separately so empty `?` and `#`
components remain distinguishable from absence.

`DidUrl` does not embed a separately allocated `Did`. `as_did_str()` borrows
the validated DID prefix, while `to_did()` explicitly allocates a standalone
value. `From<Did> for DidUrl` moves the original allocation.

### Decision 2: implement the normative ASCII grammar directly

One byte-oriented state machine validates:

- lowercase `did:` and a non-empty `[a-z0-9]+` method;
- a non-empty method-specific identifier made from ASCII alphanumerics,
  `.`, `-`, `_`, `:`, and complete `%HH` escapes, never ending in `:`;
- `path-abempty`, so every path begins with `/` and each path byte is an RFC
  3986 `pchar` or complete escape;
- query and fragment bytes from RFC 3986 `pchar`, `/`, `?`, and complete
  escapes, with at most one fragment delimiter.

Raw spaces, controls, non-ASCII bytes, malformed escapes and delimiter misuse
are rejected. The parser validates but does not percent-decode or normalize,
preserving the caller's standards-valid representation exactly. A regex, URL
crate or IOTA Identity dependency is rejected because each expands dependency
and behavior surface without improving this closed grammar.

### Decision 3: bound untrusted input before parsing

The SDK sets `MAX_DID_BYTES` to 2,048 and `MAX_DID_URL_BYTES` to 4,096. DID
Core does not impose these limits; they are an SDK denial-of-service policy
and are public constants so protocol adapters can reject oversized inputs
consistently. Length is measured in bytes and rejected before scanning or
allocation.

### Decision 4: separate lexical validity from method policy

The generic parser accepts every method name and method-specific identifier
allowed by DID Core, including methods unknown to this SDK. Method crates may
layer stricter semantics through conversion from `Did`. This prevents the
generic crate from becoming a mutable method registry and lets PRISM and
Midnight evolve independently.

### Decision 5: retain the current package name until publication governance

The architectural capability is `did-core`, but the code lands in today's
unpublished `identus-did` package. Renaming or publishing it as
`identus-did-core` is governed by issue #3 and must consider registry
availability, migration and the complete workspace inventory. This lexical
slice creates no needless rename churn.

## Threat Contract

**Assets:** identifier integrity, parser availability, privacy of rejected
input and a stable boundary for method/document code.

**Threats addressed:** acceptance of truncated percent escapes, delimiter
confusion, bare-DID/DID-URL confusion, method-case drift, raw whitespace and
control injection, unbounded parser work, repeated component allocation and
error reflection of attacker-controlled identifiers.

**Residual boundaries:** lexical validity does not prove method registration,
resolution success, key authorization, document authenticity, URI
equivalence or safe display in another context. Protocol and method layers
must apply those policies.

## Test and Verification Strategy

- Pin the DID Core grammar examples and RFC 3986 component edge cases.
- Cover PRISM, Midnight, web and key-shaped identifiers without asserting
  method-specific semantics.
- Exhaust delimiter, empty-component, ASCII, percent-escape and maximum-length
  boundaries for both native and serde construction.
- Prove component slices, exact round trips, `TryFrom<String>` allocation
  reuse and the `Did`/`DidUrl` conversions.
- Run a release-mode deterministic throughput diagnostic over representative
  valid and invalid inputs; record observations without a flaky CI threshold.
- Run focused, workspace, wasm/mobile, docs, lint, formatting, OpenSpec,
  supply-chain and Nix gates, then review the exact PR head.
- Create a separately scoped fuzzing follow-up linked to parent #5.

## Migration Plan

1. Land this issue-linked OpenSpec contract and ADR as a signed commit.
2. Implement the parser, semantic types and deterministic conformance suite.
3. Measure parser throughput, create the fuzzing follow-up and complete exact
   local verification/review evidence.
4. Sync the canonical `did-core` spec, archive the change and merge only after
   exact-head CI is green. Method ports and downstream adoption remain
   independently issue-driven.

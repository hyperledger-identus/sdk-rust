## Context

There are three different URI concerns in the workspace:

1. `identus-did::Uri` is a bounded, exact-preserving absolute RFC 3986 value
   backed by roughly 180 lines of local grammar and IP-literal validation.
2. OID4VCI validates absolute HTTPS identifiers/endpoints and URI references
   with runtime `uriparse 0.6.4`, plus local protocol and security policy.
3. `identus-core::Url` is an inherited, narrower hierarchical value used only
   by its own tests; it has no intrinsic byte limit and accepts more component
   content than a full RFC parser checks.

The first two are current consumers of a strict RFC grammar engine. The third
would require a separate acceptance-boundary and foundation-dependency
decision without current payoff.

## Goals

- Replace meaningful generic URI grammar code and the runtime `uriparse` seam.
- Preserve every public SDK type, exact input representation, resource limit,
  stable error variant and protocol-specific policy decision.
- Use borrowed candidate views only; never retain or re-export upstream types.
- Make accepted and rejected behavior explicit and regression-tested.
- Improve IPvFuture handling by parsing it directly rather than mutating a
  secret-bearing temporary URI for a second parser pass.

## Decisions

### Use the candidate only as a private grammar engine

The workspace pins `fluent-uri = "=0.4.1"` with default features disabled.
The implementation uses `fluent_uri::Uri::parse(&str)` for absolute URIs and
`fluent_uri::UriRef::parse(&str)` for URI references. No allocation,
normalization, building, resolution, IRI, serde, networking or error-trait
feature is needed for borrowed validation.

SDK wrappers retain the original string or zeroizing allocation. Candidate
errors and component types do not enter public signatures, diagnostics or
serialization.

### Keep policy outside grammar

For OID4VCI HTTPS values, successful grammar parsing is necessary but not
sufficient. The existing boundary continues to require an ASCII-case-
insensitive `https` scheme, a present authority with non-empty host, no
userinfo, no fragment and the call-site-specific query rule. Existing byte
ceilings run before parsing.

For Token Error `error_uri` and URI-shaped token types, URI-reference parsing
remains composed with the existing visible-ASCII/non-empty field rules. The
grammar crate does not decide OAuth or logging policy.

### Preserve DID URI behavior and evidence

`identus-did::Uri` performs its 4,096-byte bound before candidate parsing and
retains exact caller spelling. Its existing `UriSyntaxError` categories remain
stable; detailed candidate offsets/kinds are not exposed. DID and DID URL
parsers remain local because their method-specific grammar and cached parts are
not replaced by a generic URI parser.

The existing deterministic `uriparse 0.6.4` comparison remains a development
oracle. Tests add direct candidate regressions for IPvFuture, malformed percent
encoding, authority, relative references and ASCII URI versus IRI separation.

### Correct artifact provenance

The assessed crates.io archive checksum is
`bc74ac4d8359ae70623506d512209619e5cf8f347124910440dbc221714b328e`.
Its `.cargo_vcs_info.json` points to lightweight tag/release commit
`d9a6a20614f34b00476837eb8904fb01ca3e54df`; that commit is unsigned. Packaged
`src/lib.rs` SHA-256
`91463a1f23b0d1a7a1d56e6d643c4a6e7b7194c6b8244ea500ec48d054675cad`
matches the same file at that commit. The later `76e10ae` revision is not
release provenance and must be removed from living adoption claims.

## Compatibility matrix

| Boundary | Grammar type | SDK policy retained | Intentional behavior |
| --- | --- | --- | --- |
| `identus-did::Uri` | absolute URI | 4,096-byte ceiling, ASCII URI profile, exact spelling, stable errors | Same valid/invalid boundary; IPvFuture remains accepted. |
| Credential Issuer Identifier | absolute URI | HTTPS, non-empty host, no userinfo/query/fragment, existing byte bound | Same protocol boundary; direct IPvFuture acceptance. |
| OID4VCI endpoints/references | absolute URI | HTTPS, non-empty host, no userinfo/fragment, call-site query rule, byte bounds | Same protocol boundary. |
| Token Error `error_uri` | URI reference | non-empty bounded visible ASCII, static error | Same protocol boundary. |
| URI-shaped token type | URI reference | non-empty bounded token field and existing type-name branch | Same protocol boundary. |

Any unexplained difference is a blocker. A standards-correct difference may be
accepted only when this specification and regression corpus state it
explicitly before merge.

## Dependency and security shape

The direct dependency is one MIT crate declaring Rust 1.68 and
`#![forbid(unsafe_code)]`. Its resolved normal cone adds five package names to
this workspace: `fluent-uri 0.4.1`, MIT-0 `borrow-or-share 0.2.4`, MIT OR
Apache-2.0 `ref-cast 1.0.27` and `ref-cast-impl 1.0.27`, and `syn 3.0.5`;
existing `proc-macro2`, `quote` and `unicode-ident` packages are reused.
`ref-cast` and its derive implementation use unsafe reference casts to create
transparent immutable component views over validated input. The derives check
the transparent representation and generated type relationship; SDK code
receives only borrowed parser views and adds no unsafe block. No native code,
FFI, network, clock or runtime enters the graph. MIT-0 is OSI-approved and is
added explicitly to the repository license allowlist.

## Rollback

Revert the focused PR. Restore the local DID URI validator, OID4VCI
`uriparse` calls and the IPvFuture workaround, then remove `fluent-uri` from
the workspace dependency and lockfile. Public data and wire migration are not
required because owned representations never change.

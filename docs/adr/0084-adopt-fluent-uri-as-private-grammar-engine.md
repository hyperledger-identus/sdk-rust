# ADR 0084: adopt fluent-uri as a private grammar engine

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#157](https://github.com/hyperledger-identus/sdk-rust/issues/157)
- **Refines:** ADR 0061's `fluent-uri` disposition
- **Decision authority:** RFC 3986, DID Core URI requirements and current
  OID4VCI URI/URI-reference policy

## Context

The SDK maintained roughly 180 lines of generic RFC 3986 parsing in
`identus-did::Uri` while OID4VCI used `uriparse 0.6.4` at four runtime seams.
`uriparse` rejects RFC IPvFuture hosts, which forced OID4VCI to substitute an
IPv6 host into a zeroizing temporary before validating the rest of a URI.
Maintaining two production grammar engines and a workaround increases drift
without providing product-specific value.

`fluent-uri 0.4.1` provides strict borrowed RFC 3986 URI and URI-reference
parsers, accepts IPvFuture, preserves caller spelling and declares Rust 1.68.
Its public types do not need to cross an SDK boundary. The exact crates.io
artifact is MIT and records release commit
`d9a6a20614f34b00476837eb8904fb01ca3e54df`; its checksum is
`bc74ac4d8359ae70623506d512209619e5cf8f347124910440dbc221714b328e`.

The integrated cone adds five package names: `fluent-uri 0.4.1`,
`borrow-or-share 0.2.4`, `ref-cast 1.0.27`, `ref-cast-impl 1.0.27` and
`syn 3.0.5`. Existing `proc-macro2`, `quote` and `unicode-ident` are reused.
`borrow-or-share` uses the OSI-approved MIT-0 license, which was not previously
listed in `deny.toml`. `ref-cast` contains unsafe traits and casts generated for
transparent reference wrappers. `fluent-uri` and `borrow-or-share` themselves
forbid unsafe code and the cone contains no native code or FFI.

## Decision

Pin `fluent-uri = 0.4.1` with default features disabled and use it privately:

1. Replace only the generic grammar mechanics inside `identus-did::Uri`.
   Retain the 4,096-byte precheck, exact owned string, scheme/error facade,
   serialization and every public signature.
2. Replace the four OID4VCI `uriparse` runtime seams. Retain the current HTTPS,
   authority, non-empty host, no-userinfo, query/fragment, visible-ASCII,
   resource-limit and redaction policies in SDK code.
3. Retain exact `uriparse 0.6.4` only as the DID development oracle. It is not
   a normal dependency of DID or OID4VCI.
4. Do not enable IRI, normalization, resolution, serde or networking behavior.
   Do not migrate `identus-core::Url`, `Did` or `DidUrl` in this decision.
5. Add MIT-0 to the repository license allowlist. This permits the specific
   permissive license across the workspace; it is not a crate-name bypass.
6. Accept the transitive `ref-cast` unsafe boundary because its macros verify
   transparent representations, its use is constrained to immutable borrowed
   component views, and no dependency type or unsafe operation enters SDK
   source or public API. Any future mutable reachability requires a new review.

Parser equivalence includes rejected input. IPvFuture, malformed percent
encoding, invalid authority/userinfo, query/fragment policy, non-ASCII input,
resource ceilings and exact spelling are deterministic acceptance gates.

## Consequences

- DID and OID4VCI share one focused RFC grammar engine while keeping cohesive
  policy and ownership in their respective crates.
- The OID4VCI IPvFuture substitution workaround and production `uriparse`
  edge are removed.
- The workspace gains five locked packages and a second `syn` major version.
  The duplicate is accepted because it is build-time-only and forced by the
  exact candidate cone; it remains visible as a `cargo deny` warning.
- MIT-0 becomes an explicitly permitted OSI license.
- Public/wire behavior and downstream APIs do not change.

## Alternatives rejected

### Keep both existing implementations

This avoids a dependency but preserves duplicated standards mechanics and the
IPvFuture workaround.

### Use WHATWG `url`

WHATWG parsing and normalization do not match an exact-preserving generic RFC
3986 URI value.

### Move every URL-like value at once

`identus-core::Url` has no consumer outside its tests and a distinct,
under-specified hierarchical contract. DID and DID URL grammar is
method-specific. Migrating either would broaden risk without current payoff.

## Verification and rollback

Acceptance requires focused and full workspace tests, the factory validation,
Rust 1.98 host/no-default-feature and supported target checks, documentation,
license/advisory gates, and a distinct exact-diff review. Revert the focused PR
to restore the local DID parser, OID4VCI `uriparse` calls and IPvFuture
workaround. No data migration is needed because stored strings and public
types are unchanged.

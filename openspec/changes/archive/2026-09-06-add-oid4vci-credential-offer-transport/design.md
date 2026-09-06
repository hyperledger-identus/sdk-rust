# Design: bounded OID4VCI Credential Offer transport

## Context

Issue #111 begins B09 / `IDR-023` from
`develop@ddf64219ada97f9cb2287cfcb6da808d98f8b1a9`. OpenID4VCI 1.0 Final
sections 4.1.2 and 4.1.3 define a Wallet invocation carrying exactly one query
parameter: an embedded JSON Credential Offer under `credential_offer`, or an
HTTPS reference under `credential_offer_uri`. Full offer semantics and
reference retrieval are intentionally later slices.

The final specification was published 2025-09-16 and retrieved 2026-09-06 from
<https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html>.
RFC 3986 supplies URI scheme and percent-encoding rules. The final text is
pinned; errata adoption requires explicit compatibility review.

## Provenance and isolation

No donor code or fixture is copied. Official examples and independently
reconstructed consumer-shaped values form the test evidence.

| Repository | Revision | Evidence | SHA-256 | License/classification |
| --- | --- | --- | --- | --- |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | `crates/adapters/identity-ingress/src/lib.rs` | `e2a3c96e10a623e2798f3b6f3e7728c97a2dd393d38f71124d98637f4e6be810` | Apache-2.0; conformance-only |
| Oxid | same | `crates/adapters/openid4vci/src/lib.rs` | `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af` | Apache-2.0; conformance-only |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | `crates/issuer-services/src/credential_offer.rs` | `35629c3462af6d83210f046c9adabda6c00d75cc247df111f17387d8e62f5b60` | no repository license evidence; behavior observation only |

Oxid remains at the stated revision with pre-existing untracked `.claude/`
and `.pi/taskflows/`. Lace ID Portal remains at the stated revision with
pre-existing `.pi-subagents/`, `.pi/`, and `tmp/`. Neither tree is edited.

## Decisions

### D1 — activate a focused protocol crate

The new package is named `identus-oid4vci` and belongs to protocol semantics.
It is experimental and `publish = false`. The inherited
`identus-openid4vc` marker remains quarantined because an umbrella protocol
facade is neither needed nor accepted. An additive package is reversible and
does not decide crates.io ownership under #3.

The package depends only on `identus-core` for shared component/error
vocabulary and on serialization/URI/zeroization utilities. It has no DID,
JOSE, crypto, HTTP, async, storage, chain, or product dependency.

### D2 — transport and semantic offer parsing are separate states

`CredentialOfferRequest::parse` returns either
`CredentialOfferRequest::Embedded(EmbeddedCredentialOffer)` or
`CredentialOfferRequest::Referenced(CredentialOfferReference)`. Embedded state
proves only that decoded bytes are a bounded, duplicate-free complete JSON
object. It does not claim that required offer members or grants are valid.

The embedded type exposes the retained JSON only through an explicit borrowed
accessor. The reference type does the same for a validated HTTPS URI. Neither
implements `Display`, `Serialize`, or content-bearing `Debug`.
This lets a later semantic/fetching layer consume the exact accepted value
without conflating transport validity with protocol validity.

### D3 — the invocation grammar is deliberately closed

The registered scheme is matched ASCII-case-insensitively under RFC 3986, but
the accepted form is otherwise exactly
`openid-credential-offer://?<name>=<value>`. There is no authority, path,
fragment, empty query, or extra query component. The raw parameter name must
be exactly `credential_offer` or `credential_offer_uri`; percent-encoded name
aliases are rejected.

Exactly one raw query pair is accepted. An ampersand therefore always denotes
an additional parameter and is rejected; ampersands inside a value must be
percent encoded. This rejects duplicate parameters, both transports, unknown
extensions, and Lace's legacy `issuer_origin` parameter. Extension points are
preserved inside the embedded JSON object as required by section 4.1.1.

### D4 — decoding is strict and bounded

The complete invocation byte length is checked before scanning. Query values
use application/x-www-form-urlencoded decoding: `+` becomes space and `%HH`
decodes one byte. Raw non-ASCII bytes, malformed escapes, NUL, and decoded
invalid UTF-8 fail closed. The decoder checks the output ceiling before each
push and holds intermediate bytes in `Zeroizing<Vec<u8>>`.

Default limits are 32,768 invocation bytes, 16,384 decoded embedded bytes,
2,048 decoded reference bytes, JSON depth 16, and 128 aggregate JSON nodes.
All limits must be positive and are observable. Callers may choose smaller or
larger positive values within their deployment policy.

### D5 — embedded JSON rejects ambiguous/resource-hostile forms

A streaming Serde visitor validates the full input before a public value is
created. It counts every scalar/container node, checks container depth, and
uses an object-local bounded list of zeroizing decoded member names so literal
and escaped duplicate names are equivalent. Allocated decoded string values
are also zeroized. The same scan classifies the root, so trailing JSON and a
non-object root are rejected without building a second JSON tree. The raw
object is retained; it is not normalized or reserialized.

The node ceiling bounds aggregate array/object expansion. The byte ceiling
bounds member-name retention and scalar storage. Semantic field limits and
extension interpretation belong to the later Credential Offer object slice.

### D6 — referenced offers prove syntax, not network safety

`uriparse` validates the decoded reference as an absolute URI. The scheme must
be HTTPS, the host must be non-empty, and user information plus fragments are
rejected. Ports, paths, and queries remain allowed by the final specification.

No HTTP request, redirect, DNS resolution, IP classification, caching, media
type check, or issuer trust decision occurs. Those are outer adapter and
protocol-state responsibilities with their own SSRF and privacy contract.

### D7 — sensitive values never enter diagnostics

An embedded offer may contain a Pre-Authorized Code, and a reference URI may
itself be a bearer capability. Both are stored in `Zeroizing<String>`, erased
on drop, and omitted from custom `Debug`. Errors are fieldless, use static
messages/codes, and bridge into `identus-core` under the `oid4vci` capability
without echoing any URI, JSON, query name, or value.

### D8 — evidence remains layered

Unit/integration tests cover official by-value/by-reference forms, an
Oxid-shaped final offer, the known Lace extra-query incompatibility, every
error class, exact bounds, percent/UTF-8 failures, duplicate decoded names,
depth/nodes, and canary redaction. A deterministic ignored release diagnostic
measures parse throughput without a machine-specific threshold.

The repository inventory, crate-ring rulebook, portable target lists,
blueprint, and `IDR-023` row are updated. Full Nix checks remain authoritative
for MSRV, Linux/macOS, WASM/mobile, lint, tests, docs, and supply chain.

## Risks and trade-offs

- Exact `://?` shape rejects alternate URI spellings that a general URL
  parser might normalize. This matches the registered OpenID invocation and
  avoids authority/path ambiguity; a later compatibility slice can add another
  spelling only with interoperable evidence.
- Opaque embedded JSON postpones required-member validation. The type name and
  docs state this explicitly, while still delivering a reusable safe ingress
  boundary.
- `uriparse` validates syntax but not destination trust. Returning a typed
  reference rather than fetching it preserves least authority.
- Linear duplicate checks trade a bounded amount of CPU for collision-free
  comparison and zeroizing owned names; the aggregate node ceiling caps that
  work while keeping the public representation exact.

## Migration and rollback

This is a new unpublished package. No current crate or downstream depends on
it. A focused revert removes its workspace, rulebook, inventory, support,
backlog, ADR, and spec entries. Full Credential Offer semantics, network
adapters, consumer adoption, namespace reservation, and publication remain
separate work.

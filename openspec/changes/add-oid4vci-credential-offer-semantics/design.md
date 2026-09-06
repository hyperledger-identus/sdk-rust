# Design: bounded OID4VCI Credential Offer semantics

## Context

Issue #113 continues B09 / `IDR-023` from
`develop@ff227f68927d0958230600c4f0d448d585c08ad0`. OpenID4VCI 1.0 Final
section 4.1.1 defines the three Credential Offer members; section 12.2.1
defines the Credential Issuer Identifier; section 13.5 requires wallets to
treat all offer values as untrusted.

The Final HTML was published 2025-09-16, retrieved 2026-09-06, and hashed as
`f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
Errata and later revisions require a separate compatibility review.

## Provenance and isolation

No donor source or fixture is copied. Official examples and independently
reconstructed consumer-shaped inputs are test evidence.

| Repository | Revision | Evidence | SHA-256 | Classification |
| --- | --- | --- | --- | --- |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | `crates/adapters/identity-ingress/src/lib.rs` | `e2a3c96e10a623e2798f3b6f3e7728c97a2dd393d38f71124d98637f4e6be810` | Apache-2.0; behavior only |
| Oxid | same | `crates/adapters/openid4vci/src/lib.rs` | `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af` | Apache-2.0; behavior only |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | `crates/issuer-services/src/credential_offer.rs` | `35629c3462af6d83210f046c9adabda6c00d75cc247df111f17387d8e62f5b60` | no repository license evidence; behavior observation only |

Oxid has pre-existing untracked `.claude/` and `.pi/taskflows/`. Lace has
pre-existing untracked `.pi-subagents/`, `.pi/`, and `tmp/`. Neither checkout
may be changed by this slice.

## Decisions

### D1 — model transport and semantics as different states

`CredentialOffer::try_from_embedded` consumes the validated transport. It
retains that exact JSON internally while exposing separately decoded,
validated issuer/configuration values. A semantic value cannot be constructed
from unchecked JSON, and a failed transition drops the bearer-adjacent input.

The API does not implement `Serialize` or `Display`; custom `Debug` shows only
type shape and safe counts/booleans. Explicit accessors are required to inspect
the issuer, IDs, or retained JSON.

### D2 — use a selective Serde visitor without materializing unknown values

A custom map visitor decodes only the required strings/array and checks that
`grants` has map shape. `IgnoredAny` consumes unknown top-level values and
opaque grant content. This avoids constructing an unbounded generic JSON tree
or converting arbitrary-magnitude numeric extensions, while the preceding
transport scanner remains authoritative for full syntax, depth, node, and
duplicate-name bounds.

The direct `serde` dependency is already workspace-owned and contains no
runtime authority. `serde_json` remains the concrete syntax adapter.

### D3 — keep semantic limits separate

`CredentialOfferSemanticLimits` adds positive maxima for issuer bytes, each
configuration ID's decoded bytes, and ID count. Defaults are 2,048, 256, and
32. Keeping these limits separate avoids breaking the existing five-argument
transport limit constructor and lets callers tune wire and semantic resource
policies independently.

The Final standard requires a non-empty array of unique strings but does not
forbid an empty string as a metadata key. The SDK therefore accepts an empty
ID while enforcing count, uniqueness, and byte ceilings rather than adding
product policy.

### D4 — validate only issuer identifier syntax

The identifier uses the existing URI parser. HTTPS comparison is
ASCII-case-insensitive; the complete retained identifier remains exact and
case-sensitive. A host is required, userinfo/query/fragment are forbidden, and
port/path are accepted. No normalization, DNS, fetch, metadata construction,
origin authentication, or trust decision occurs.

### D5 — defer grant internals without silently accepting type confusion

The Final standard permits absent or empty grants and requires a grants object
when present. This slice records only presence and validates map shape. It does
not expose grant members as usable protocol instructions. Exact JSON retention
allows a later child issue to add authorization-code and Pre-Authorized Code
types without losing extensions or secrets.

### D6 — extend the existing static error taxonomy

New fieldless variants distinguish invalid semantic limits, required fields,
issuer identifiers, configuration IDs, and grants shape. Their stable core
codes and public messages contain no input. All owned decoded strings use
`Zeroizing`, as does the retained transport JSON.

## Risks and trade-offs

- Re-parsing validated JSON costs CPU, but the existing byte/node/depth ceilings
  make work deterministic and selective parsing avoids a full value tree.
- Accepting an empty configuration ID is unusual but standards-compatible;
  metadata matching can later determine whether that key actually exists.
- Merely reporting grant presence is intentionally weak. It prevents the core
  semantic type from implying flow support before grant-specific validation.
- Issuer syntax does not establish trust. Documentation and types preserve this
  distinction because the Final security section requires independent checks.

## Migration and rollback

The API is additive in an unpublished crate and has no downstream dependency.
A focused revert removes the semantic module, errors, limits, tests, ADR, and
spec additions while preserving the transport delivered by #111.

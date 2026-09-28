# Design

## Module map

`limits.rs` becomes a private facade with module declarations and explicit
re-exports. Its children have these responsibilities:

| Module | Responsibility |
| --- | --- |
| `limits/policy.rs` | Sole numeric default definitions and `MAX_CONFIGURABLE_JSON_DEPTH` |
| `limits/offer.rs` | Credential Offer transport/semantic/grant policy and Transaction Code input |
| `limits/token.rs` | Pre-authorized and authorization-code token request/HTTP policy, Token Response, Authorization Details, and Token Error |
| `limits/credential.rs` | Credential Error/Nonce, JWT request, immediate/deferred request/response/HTTP, and endpoint composition |
| `limits/metadata.rs` | Credential Issuer and Authorization Server metadata |

The lifecycle modules remain private. `limits.rs` re-exports their public
types, and the existing `lib.rs` list re-exports those names at the crate root.

## Central policy invariant

Every numeric literal used by an existing `Default` implementation moves
unchanged to one named `pub(super)` constant in `policy.rs`. The public JSON
depth ceiling is defined there and re-exported. Constructors keep their exact
zero checks, cross-field checks, maximum-depth comparison, and error variants.
Accessors and composite defaults remain with their owning public type.

Constants are deliberately not deduplicated merely because two lifecycle
fields currently share a number: independent policy roles keep independent
names so a future change cannot couple unrelated limits accidentally.
Composite defaults continue to call the existing nested types' `Default`
implementations rather than duplicating nested policy.

## Compatibility boundary

The public crate-root names, visibility, derives, method signatures, constness,
field privacy, and trait implementations remain byte-for-byte equivalent at
the Rust API level. No protocol consumer imports a child module. Error
selection, validation ordering, effective defaults, and JSON-depth behavior
must match base exactly.

## Characterization and ratchet

The complete OID4VCI suite is the pre-move characterization corpus. Review
also records every public type, method signature, `Default` value, validation
predicate, and error variant before and after the move. Code-health evidence
must remove the aggregate `limits.rs` hotspot without creating an over-threshold
descendant, public forwarding API, macro-hidden implementation, duplicated
policy literal, or new mixed responsibility.

## Verification

Focused tests and strict Clippy precede workspace/factory/Nix gates. The
immutable OID4VCI error hash and manifests must remain unchanged. A distinct
exact-diff architecture/security review checks central policy ownership,
lifecycle cohesion, public API equivalence, and absence of accidental numeric
or validation changes before protected CI and merge.

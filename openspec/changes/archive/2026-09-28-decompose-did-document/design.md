# Design

## Module map

`document.rs` remains a private facade with private children:

| Module | Responsibility |
| --- | --- |
| `document/cardinality.rs` | `OneOrMany<T>` representation, serde cardinality, and non-empty construction |
| `document/extension.rs` | `ContextObject`, `ContextEntry`, extension budgets, bounded JSON validation, and context cleanup |
| `document/verification.rs` | `VerificationMethod`, relationships, public JWK/multibase validation, and verification cleanup projection |
| `document/service.rs` | Service endpoint representations, `Service`, type/endpoint/extension validation, and service cleanup projection |
| `document/model.rs` | `DidDocument`, builder, raw parsing, aggregate validation, serde construction, relationship checks, and whole-document cleanup |

The children remain private. The facade explicitly re-exports the existing
public and crate-private vocabulary, retains resource constants and reserved
names, and owns only shared error projection or collection validation that
would otherwise introduce a false dependency direction.

## Ownership and dependency direction

`cardinality` is dependency-light. `extension` owns untrusted recursive JSON
budgets and iterative context cleanup. `verification` and `service` depend on
cardinality/extension primitives but not on the aggregate document. `model`
composes all roles and owns cross-resource uniqueness, relationship binding,
wire preflight, and whole-document rejection cleanup. Narrow `pub(super)`
validation and cleanup collectors replace field access; concrete fields remain
private to their semantic owner.

## Compatibility boundary

The crate-root export list, public names and paths, fields, derives, signatures,
constness, serde names/cardinality, raw wire preflight, resource constants,
accepted/rejected shapes, validation precedence, error variants, duplicate
rules, public-only key rules, debug redaction, iterative cleanup, and semantic
round trips remain equivalent. No other DID module or manifest changes.

## Characterization and ratchet

The all-feature DID suite is the pre-move corpus, especially
`did_document.rs` and `did_document_hardening.rs`: native/serde equivalence,
duplicate-name scanning, exact budgets, hostile-depth cleanup, multibase/JWK
rules, context and service maps, relationship collisions, round trips, and
redacted errors. Code-health removes only the DID document hotspot and may not
create an over-threshold descendant or broaden child-module visibility.

## Verification

Focused DID tests and strict Clippy precede workspace/factory/Nix and portable
target gates. A distinct exact-diff review checks cohesion, minimal visibility,
public/wire/error stability, validation order, JSON budgets, non-recursive
cleanup, suite/method neutrality, and absence of resolution/policy drift.

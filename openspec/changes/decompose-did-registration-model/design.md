# Design

## Module map

`registration.rs` remains a private facade with private children:

| Module | Responsibility |
| --- | --- |
| `registration/public_data.rs` | Bounded public JSON object, reserved/private-member policy, hostile-value validation, and iterative cleanup |
| `registration/request.rs` | Opaque identifiers, secret policy, document mutations, action/job models, and create/update/deactivate/continue/cancel requests |
| `registration/result.rs` | Failure classification, lifecycle state, result construction, and request/result consistency |
| `registration/port.rs` | Runtime-neutral boxed future and object-safe registrar port |

The facade explicitly re-exports the existing public items. Children remain
private and share only narrow crate-private validators.

## Compatibility boundary

The crate-root export list, public names and paths, fields, derives, method
signatures, constness, identifier limits, JSON limits, validation predicates
and ordering, error variants, debug redaction, serialization, iterative cleanup,
and future/trait semantics remain equivalent. There is no adapter dispatch or
runtime change.

## Dependency direction

`public_data` owns recursive JSON policy. `request` may use public data and
document validation. `result` may use request identities plus public data and
document metadata. `port` depends only on the public request/result vocabulary.
No child depends on the facade and no generic helper framework is introduced.

## Characterization and ratchet

The all-feature DID suite is the pre-move corpus, including identifier bounds,
hostile JSON, redaction, request accessors, lifecycle matrices, metadata,
registry dispatch, and future cancellation behavior. Code-health removes only
the registration hotspot and may not create a new over-threshold descendant or
broaden visibility.

## Verification

Focused DID tests and strict Clippy precede workspace/factory/Nix and portable
target gates. A distinct exact-diff review checks module cohesion, visibility,
validation and cleanup preservation, API/source equivalence, and absence of
adapter/runtime drift.

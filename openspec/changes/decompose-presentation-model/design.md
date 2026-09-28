# Design

## Module map

`model.rs` remains a private facade with private children:

| Module | Responsibility |
| --- | --- |
| `model/value.rs` | Bounded query IDs, purpose, replay challenge, credential handles, and claim intent |
| `model/request.rs` | Claim requests, descriptor filters, credential queries, and semantic presentation requests |
| `model/selection.rs` | Credential candidates, candidate-set validation, selected claims, credential selections, and disclosure plans |
| `model/artifact.rs` | Artifact bindings/payloads, generated-presentation coverage and aggregate budgets, and value-free receipt projection |

The children remain private. The facade explicitly re-exports all existing
public items, retains the public resource constants, and owns the single
generic duplicate detector shared by request, selection, and artifact
validation.

## Ownership and dependency direction

`request` depends on validated values and credential metadata. `selection`
depends on request lookup and validated value roles. `artifact` depends on the
validated request/disclosure plan and owns receipt projection because receipts
are derived solely from generated artifacts. Cross-sibling access uses narrow
`pub(super)` lookup/validation methods; concrete fields remain private to their
owner where possible.

## Compatibility boundary

The crate-root export list, public names and paths, fields, derives, signatures,
constness, resource constants, accepted/rejected values, validation order,
error variants, debug redaction, value-free intent, request/candidate/selection
binding, format orthogonality, artifact coverage, byte budgets, and receipt
projection remain equivalent. No lifecycle or protocol module changes.

## Characterization and ratchet

The all-feature presentation suite is the pre-move corpus, including scalar
boundaries, deterministic hostile inputs, DCQL/Midnight/unrelated-format
orthogonality, candidate and disclosure negative matrices, artifact coverage,
aggregate budgets, debug redaction, receipt projection, and lifecycle tests.
Code-health removes only the presentation hotspot and may not create a new
over-threshold descendant or broaden visibility.

## Verification

Focused presentation tests and strict Clippy precede workspace/factory/Nix and
portable target gates. A distinct exact-diff review checks cohesion, minimal
visibility, public/error/source stability, resource limits, value retention,
format neutrality, and absence of lifecycle/protocol drift.

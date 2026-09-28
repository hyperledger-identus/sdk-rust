# Design

## Module map

`resolution.rs` becomes a private facade with explicit private modules and
re-exports. Its children have these responsibilities:

| Module | Responsibility |
| --- | --- |
| `resolution/value.rs` | Bounded `MediaType`, `DidResolutionDateTime`, and `VersionId` values plus their exact scalar grammars |
| `resolution/operation.rs` | Standard error kinds/problem object and shared resolution/dereferencing operation metadata |
| `resolution/document_metadata.rs` | DID document metadata, builder, method-consistency checks, extension validation, and hostile-value cleanup |
| `resolution/result.rs` | DID resolution success/failure/deactivated state machine and standard crate-owned failure helper |
| `resolution/dereferencing.rs` | Bounded dereferenced JSON/content metadata and dereferencing success/failure state machine |
| `resolution/wire.rs` | DID-specific raw JSON limits, duplicate-name preflight, and scanner-error projection |

The children remain private. The facade re-exports exactly the existing public
items to the unchanged `lib.rs` list and re-exports the crate-private standard
failure helper only for current orchestration consumers.

## Ownership and visibility

Public fields remain private. Cross-sibling semantic composition uses narrow
`pub(super)` methods: operation metadata budget validation, document metadata
budget/method validation, dereferencing content/metadata budget validation, and
wire preflight. Internal concrete storage types remain private to their owner.
The facade contains no validation, policy, serialization, or allocation logic.

Common JSON validation is not abstracted into a new framework. Each owner keeps
its reserved-member policy and cleanup function adjacent to its data. The
existing crate-level `document::JsonBudget`, `wire_json` scanner, and
`json_cleanup` primitives remain shared dependencies rather than being copied.

## Compatibility boundary

The crate-root export list, names, visibility, derives, method signatures,
constness, trait implementations, JSON names, nullable handling, validation
predicates and order, exact limits, error variants, and cleanup behavior remain
equivalent. Raw result parsing remains size check, wire preflight, then serde
and semantic validation. No resolver, cache, registry, or HTTP module changes.

No incidental cleanup is included. Exact-head inspection confirmed the current
base already contains single documentation, serde, and validation operations;
the implementation is an ownership-only move.

## Characterization and ratchet

The complete `identus-did` all-feature suite is the pre-move characterization
corpus, including generated resolution/dereferencing state matrices, hostile
JSON cleanup, duplicate-member/wire-limit cases, error stability, cache
behavior, and the parse-throughput diagnostic. A mechanical inventory compares
crate-root exports and public declarations before and after the move.

Code-health evidence removes only `did-resolution-model-and-wire`; it must not
add an over-threshold descendant, forwarding-only public API, duplicate policy,
or broader visibility. Other hotspots and issue #50 retain their owners.

## Verification

Focused DID tests and strict Clippy precede workspace/factory/Nix gates.
Immutable DID error evidence, manifests, dependencies, public exports, and
source-distribution metadata must remain unchanged. A distinct exact-diff
architecture/security review checks module cohesion, minimal visibility,
validation and cleanup preservation, absence of cache/runtime drift, and the
code-health ratchet before protected CI and merge.

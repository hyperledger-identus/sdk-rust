## Context

The archived `add-nix-tooling` change left the workspace with a single placeholder crate `identus-ssi` (an empty `src/lib.rs`, `version = "0.0.0"`, `resolver = "3"`, `warnings = "deny"`, `clippy::all = "deny"`) and no shared error model. The Identus seed branch (`sdk-rust-seed`) defines a mature `identus-core` with a redaction-safe error surface, stable typed codes, error families, capability attribution, and a two-surface bridging convention that lets domain crates keep idiomatic `FromStr` errors while exposing stable codes for bindings and conformance fixtures. This change ports the *domain-facing* portion of that model into the foundation crate.

## Goals / Non-Goals

**Goals:**
- Establish `identus-core` as the zero-workspace-dependency foundation crate.
- Provide a redaction-safe `IdentusError` whose `Display` renders only a stable code and a public message, never secret or internal context.
- Provide stable typed `ErrorCode`s and `ErrorKind` families as compatibility contracts for conformance fixtures and future bindings.
- Provide `CapabilityId` attribution so errors name their owning capability.
- Document the two-surface bridging convention so later domain crates adopt it uniformly.

**Non-Goals:**
- Binding-facing DTOs (`ResultEnvelope`, `ErrorEnvelope`) and `RedactionPolicy`: deferred to the bindings change, which is the first change with a binding consumer. Redaction safety is structural (`Display` renders only `&'static str` fields), so it does not depend on the deferred `RedactionPolicy` enum.
- Any domain adopter (`identus-did`, `identus-messaging`): they land with their own crates in later changes.
- Modifying workspace lints, resolver, or version: those are out of scope for an error-conventions change.

## Decisions

### Decision 1: Adopt the seed's error model, domain-facing only (Option 2)

Ship `IdentusError`, `ErrorCode`, `ErrorKind` (all 11 families), `CapabilityId`, `IdentusResult`, and `Component`. Defer `ResultEnvelope`, `ErrorEnvelope`, `RedactionPolicy`, and `to_error_envelope()` to the bindings change.

**Rationale**: the binding-facing envelope has no consumer until a binding exists; shipping it now is speculative. The redaction guarantee is structural — `Display` renders only `code` (a `&'static str`) and `public_message` (a `&'static str`) — so it holds without the `RedactionPolicy` enum, which exists to make *policy decisions* about diagnostic exposure, a concern that only matters at a binding boundary.

**Alternatives considered**:
- *Verbatim (full surface including envelopes)*: rejected as speculative; no binding consumer exists, and bundling the binding surface recreates the over-broad scope this change was narrowed to avoid.
- *Slimmest (drop `CapabilityId`, bridging, `Component`)*: rejected; `CapabilityId` and the bridging convention are the actual value, and `Component` is free.
- *Bundle first adopter (`identus-did`)*: rejected; bundles a domain capability into the error-contract change, breaking single-concern coherence.

### Decision 2: Redaction safety is structural, not policy-driven

`IdentusError` carries only `&'static str` fields and a `Display` impl that renders `"{code}: {public_message}"`. Secret or internal context never enters the value; it stays in adapter-local logs.

**Rationale**: this is the seed's core invariant. Keeping it structural (not gated by a runtime `RedactionPolicy` check) means the guarantee cannot be bypassed by a future constructor without changing the type itself.

### Decision 3: Two-surface bridging documented, not demonstrated

The convention: each domain crate keeps its idiomatic error type (for `FromStr`) and adds `to_identus_error()` + `parse_with_core_error()`. No adopter ships in this change; `identus-core`'s own tests construct errors to prove the contract.

**Rationale**: a convention documented with zero adopters is a rule on paper, but `identus-core`'s self-tests exercise the type end-to-end (construct, display, attribute), which is sufficient evidence for the foundation. The first real adopter (`identus-did`) validates the bridging pattern when it lands.

**Alternatives considered**: bundle `identus-did` to demonstrate the convention — rejected per Decision 1's scope discipline.

### Decision 4: 11 `ErrorKind` families from day one

`ErrorKind` starts with all 11 seed families: `InvalidInput`, `Unsupported`, `NotFound`, `Conflict`, `PolicyViolation`, `VerificationFailed`, `Transport`, `Storage`, `Crypto`, `Trust`, `Internal`. New families require acceptance criteria.

**Rationale**: these are stable contracts (conformance fixtures and bindings match on them). Adding them lazily would churn the enum as each domain crate lands; adding the planned set once is cheaper and matches the seed's stated "needed by planned contracts" basis.

### Decision 5: OpenSpec-native artifact mapping; no `docs/architecture/` file

The convention's requirements live in `specs/core-error-conventions/spec.md` (with a `## Purpose` carrying the enduring invariants), which persists after archival as the permanent spec baseline. The per-change decisions live in this `design.md` (which becomes archived history). No separate `docs/architecture/core-error-conventions.md`.

**Rationale**: OpenSpec `spec.md` is already a durable home; duplicating into `docs/architecture/` creates dual maintenance. The `docs/architecture/` tree is not introduced by this change.

**Note on the `## Purpose` section**: the synced main spec (`openspec/specs/<capability>/spec.md`) carries a `## Purpose` (as the archived `nix-tooling` spec demonstrates). This change authors that Purpose content in the delta spec's `## Purpose` so the enduring invariants land verbatim in the living spec at sync time, rather than relying on a brief agent-synthesized statement.

## Risks / Trade-offs

- **[Risk] convention with no live adopter** → Mitigation: `identus-core`'s self-tests exercise the type; the first domain crate (`identus-did`) will validate the bridging pattern. If the pattern proves awkward when first adopted, a follow-up change amends the convention (cheap, since no adopters exist yet).
- **[Risk] `ErrorKind` ships families with no current user** → accepted; they are stable forward contracts, and adding them later is more disruptive.
- **[Trade-off] binding surface deferred** → the bindings change will need to add `ErrorEnvelope`/`ResultEnvelope`/`RedactionPolicy` and extend the bridging convention with `to_error_envelope()`. Accepted; that is the change with the consumer.
- **[Risk] `## Purpose` in a delta spec is an extension of the canonical `## ADDED Requirements`-only format** → Mitigation: validated with `openspec validate --strict`; if the tooling rejects it, the Purpose content falls back to this `design.md` and is lifted into the living spec at sync time.

## Migration Plan

1. Land rename + filled `identus-core` + tests in one PR, on a branch off `main` after `add-nix-tooling` (already archived/landed).
2. Verify: `cargo build`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all pass; `nix flake check` stays green (rename is glob-transparent); `[workspace.metadata.crane] name` updated.
3. Merge to `main`.

**Rollback**: revert the PR. No other crate depends on `identus-core` yet.

## Open Questions

- None blocking. Directory name (`crates/core`), version (`0.0.0`), and lint policy (unchanged) were resolved during design and are reflected in the tasks.
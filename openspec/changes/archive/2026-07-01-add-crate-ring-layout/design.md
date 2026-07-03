## Context

`add-core-error-conventions` (in flight) establishes `identus-core` as the foundation crate. The Identus seed branch (`sdk-rust-seed`) defines a 13-crate hexagonal ring, a dependency-graph fixture, and a conformance guard. The seed's guard is a Rust `#[test]` that `include_str!`s the `.json` fixture and shells out to `cargo metadata --no-deps` to drift-check the fixture's snapshot against the real graph; a companion Node tool (`tools/generate-workspace-dependency-graph.mjs`) generates the fixture. Neither pattern survives this workspace's toolchain: the `cargo metadata` subprocess is forbidden in the crane build sandbox under `nix flake check`, and crane's `filterCargoSources` keeps only `*.rs`, `*.toml`, `Cargo.lock`, and `.cargo/config` — so the `.json` fixture, the Node generator, and the seed's many `include_str!`'d docs are all stripped from the sandboxed source. This change ports the ring's *rulebook contract* (the 7 layers, crate membership, and `allowed_target_layers_by_source_layer` policy) as an in-source Rust `const`, and ports the guard as a pure-Rust manifest parser (no subprocess, no `.json`, no `serde_json`).

## Goals / Non-Goals

**Goals:**
- Establish the full 13-crate ring (12 stubs plus `identus-core`) as named boundaries.
- Declare the full intended inward dependency edges in each stub's manifest so the guard has a real ring to enforce.
- Add an executable Rust guard that enforces the layer rules (encoded as an in-source `LAYER_RULES` const) against the workspace manifests, running through the existing crane `rust-test` nix check.
- Keep the guard Node-free and subprocess-free.

**Non-Goals:**
- Any domain behavior in the stubs (types, ports, protocols). Each stub is a doc-comment + `COMPONENT`; real content lands with the change that implements that domain.
- Cross-crate conformance drift tests. The domain crates are stubs with no public contracts to drift against; those tests land with the changes that fill each crate.
- A human architecture-overview document (`components.md` in the seed). The architecture lives in the manifests, the guard, the stub descriptions, and this spec.
- JSON Schema validation of a `.json` fixture. There is no `.json` fixture in this change (the rulebook is an in-source Rust `const`); a schema policy is deferred to when there are enough heterogeneous JSON fixtures to justify it.

## Decisions

### Decision 1: All 12 stubs at minimal code depth (Q8=A)

Each stub's `src/lib.rs` is a module doc-comment + `pub const COMPONENT: identus_core::Component`. No early typed surfaces, no capability ids, no port types.

**Rationale**: porting the seed's early typed surfaces (`TrustPolicyDecision`, etc.) would quietly turn "ring layout" into "ring layout + a chunk of the trust/credential domain model." Minimal stubs give the guard its full 13-crate graph and give change 3's catalog real crates to reference, while leaving each domain's types to the change that implements that domain.

**Alternatives considered**:
- *Seed-early-typed stubs*: rejected as scope creep into domain modeling.
- *Phased subset (only `did`, `crypto`, `trust`)*: rejected; a 3-crate graph isn't worth guarding, and change 3's catalog would reference crates that don't exist yet.

### Decision 2: Rust `#[test]` guard with an in-source rulebook `const` (Q9=B)

The guard is a `#[test]` in `identus-conformance` that reads `crates/*/Cargo.toml` and the root `Cargo.toml` (via the `toml` crate), identifies workspace-internal dependencies by membership in `[workspace.dependencies]`, and asserts every workspace-internal `[dependencies]` edge obeys the layer rules. The layer rules — the 7 layers, their crate membership, and `allowed_target_layers_by_source_layer` (ported from the seed's fixture) — live as an in-source `pub(crate) const LAYER_RULES` in `identus-conformance`, not in a `.json` file. The guard reads no `.json` and invokes no subprocess. It runs through the existing crane `rust-test` nix check with no Node, no `serde_json`, and no nix config change.

**Rationale**: the workspace is Rust + Nix with no Node (`add-nix-tooling` deliberately kept Node out), and the only CI test runner is crane's `cargoNextest` over `cleanCargoSource`. Two seed patterns are therefore unavailable: the `cargo metadata` subprocess (forbidden in the nix build sandbox) and `include_str!`/`serde_json` reads of the `.json` fixture (crane's `filterCargoSources` keeps only `*.rs`, `*.toml`, `Cargo.lock`, and `.cargo/config`, so the `.json` fixture is stripped from the sandboxed source and would fail the guard under `nix flake check`). Reading manifests directly via `toml` (`.toml` survives the filter) and encoding the rulebook as a Rust `const` (no file read at all) makes the guard crane-safe with zero nix config. The seed's *rulebook contract* is borrowed verbatim; only the medium (`.json` → `const`) and the runner (subprocess → pure Rust) change. The fixture in this change is static policy, not a generated snapshot, so inlining it costs no generator-drift guarantee.

**Alternatives considered**:
- *Port the seed's `include_str!` + `cargo metadata` guard verbatim*: rejected; the subprocess is forbidden in the crane sandbox and the `.json` fixture is stripped by `cleanCargoSource`, so it would not run under `nix flake check`.
- *Keep the `.json` fixture and widen `cleanCargoSource` to preserve `fixtures/**/*.json`*: rejected; it contradicts "nix-tooling: no change," audits one more CI surface (and `buildDepsOnly` if `include_str!` is used), and re-opens the door to the seed's other crane-incompatible patterns under the guise of "the filter handles it."
- *Hand-rolled Nix check*: rejected; reinvents toml parsing and adds a CI surface to maintain.
- *Defer the executable guard to change 3*: rejected; it creates an unguarded window between change 2 and change 3, defeating the guard's reason to exist.

### Decision 3: Full intended inward edges + 13-entry workspace.dependencies (Q10=A)

Each stub's `Cargo.toml` declares its full intended inward deps (e.g. `identus-trust` → core+crypto+did), porting the seed's manifest deps. The root `Cargo.toml` adds all 13 `[workspace.dependencies]` entries.

**Rationale**: the guard (Decision 2) checks Cargo dependency edges against the layer rules. With only minimal deps (just `identus-core`, all the stubs use), the guard could only prove "everything depends on core," not ring direction. The real intended edges must be declared for the guard to verify the ring is *correct as specified*. Declaring unused deps is harmless: `unused_crate_dependencies` is allow-by-default, so it stays silent under `warnings = "deny"` (the seed relies on exactly this). This makes the manifests the source of truth for the architecture; filling a crate later is `lib.rs` growth, not manifest churn.

**Alternatives considered**:
- *Minimal deps (only `identus-core`)*: rejected; defeats the guard.

### Decision 4: OpenSpec-native, no `docs/architecture/` files (Q11=A)

The ring rules live in this spec's `## Purpose` (syncs to the living `openspec/specs/crate-ring-layout/spec.md`) and its requirements; per-change decisions in `design.md` (archived). No `adr-crate-layout.md`, no `current-workspace.md`, no `components.md`.

**Rationale**: consistent with `add-core-error-conventions` (Q4=A + the archive-Purpose refinement). The ring rules are enduring architecture principles — the category routed to the spec's `## Purpose`. The graph snapshot (`current-workspace.md`) is redundant with the manifests + guard; the component overview (`components.md`) is forward-looking and its per-crate responsibilities already live in each stub's module doc-comment + `COMPONENT.summary`. OpenSpec `spec.md` is the durable home; a parallel `docs/architecture/` doc is dual maintenance.

**Alternatives considered**:
- *OpenSpec + trimmed `docs/architecture/` (ADR + snapshot)*: rejected; reintroduces dual maintenance rejected in change 1.
- *Full `docs/architecture/` port*: rejected; most over-broad.

### Decision 5: Guard lives in `identus-conformance` as change 2's content

The guard is layout enforcement (change 2's concern) but lives in `identus-conformance` (its natural home). Change 2 adds this one `#[test]` to `identus-conformance`; change 3 adds the spec catalog. The guard is the conformance crate's first real content; the catalog is its second.

**Rationale**: the guard enforces the ring, which is change 2's subject; placing it in `identus-conformance` (the verification crate) keeps the enforcement co-located with conformance's mandate without making change 2 own conformance's *catalog* (change 3's job).

## Risks / Trade-offs

- **[Risk] stubs declare deps they don't use** → accepted; `unused_crate_dependencies` is allow-by-default, so no warning under `warnings = "deny"`. The seed relies on the same. When a crate is filled, the deps become used.
- **[Risk] the guard is the only enforcement; a bad edit between checks could land** → Mitigation: the guard runs in `nix flake check` and (via the existing `nix-checks.yml` CI) on every PR/push to `main`.
- **[Trade-off] no human architecture-overview doc** → accepted; the architecture is distributed across manifests + guard + stub descriptions + spec. A future `add-architecture-docs` change can synthesize an overview if it earns its keep.
- **[Risk] `## Purpose` in the delta spec is an extension of the canonical format** → Mitigation: validated with `openspec validate --strict` (same approach as change 1, which passed).

## Migration Plan

1. Land the 12 stubs + `[workspace.dependencies]` block + guard test (with the `LAYER_RULES` const) in one PR, on a branch off `main` after `add-core-error-conventions` merges.
2. Verify: `cargo build --workspace`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --workspace` pass; `nix flake check` green (guard runs under `rust-test`); the guard catches a deliberately-inserted outward dependency (negative test).
3. Merge to `main`.

**Rollback**: revert the PR. No domain crate has behavior depending on the ring yet.

## Open Questions

- None blocking. Stub depth (minimal), guard runner (Rust), manifest depth (full edges), and docs (none) were resolved during design.
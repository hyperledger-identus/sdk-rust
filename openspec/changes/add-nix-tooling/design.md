## Context

`sdk-rust` is a greenfield git submodule within the `identus-workspace`. It currently contains only documentation scaffolding (README, AGENTS.md, `.github/workflows` for DCO/CodeQL/file-hygiene, backlog config, openspec skills). There is no `flake.nix` and no `Cargo.toml`.

The workspace root flake (`identus-workspace/flake.nix`) is the established style reference: `flake-parts` + `devshell` (numtide) + `rust-overlay` (oxalica) + `nixpkgs` unstable, with a modular `nix/{apps,devshells,checks}` layout imported into `mkFlake`. The workspace `AGENTS.md` already declares `sdk-rust` as a submodule that owns its own flake (`cd sdk-rust && nix develop -c <command>`), but that flake has not yet been created.

The SDK's purpose is to implement the Identus SSI capability in Rust (DID core, crypto, resolvers, credentials, DIDComm). That work is future; this change establishes the reproducible environment and the check/CI guardrails that all subsequent feature proposals will rely on.

## Goals / Non-Goals

**Goals:**

- Provide a reproducible Nix devshell for `sdk-rust` contributors, self-contained in the submodule (no dependency on the workspace root flake at runtime).
- Provide an automated checks module covering both Nix hygiene (deadnix/statix/nixfmt) and Rust quality gates (fmt, clippy, test, deny, audit), such that `nix flake check` is fully green on day one.
- Establish a stub Cargo workspace so the checks have a real cargo project to exercise and so future proposals have a landing zone with a fixed crate/workspace name.
- Provide formatting apps (`nix run .#format`, `nix run .#format-nix`).
- Enforce the checks in CI via a `nix-checks.yml` workflow.
- Support both `x86_64-linux` and `aarch64-darwin` systems.

**Non-Goals:**

- Implementing any Identus SSI functionality (DID, crypto, credentials, DIDComm). The stub crate is an empty placeholder.
- WASM artifact production tooling (`wasm-pack`, `wasm-bindgen-cli`). The `wasm32-unknown-unknown` target is included in the toolchain, but the packaging tooling is deferred to the proposal that first needs browser interop.
- Providing a nightly Rust toolchain. The default (and only) devshell and all checks run on stable; nightly is deferred to the proposal that first proves a nightly need.
- Modifying the workspace root flake or any other submodule.
- Mobile/FFI cross-compilation targets (aarch64-linux/darwin as *build targets* for sdk-kmp/sdk-swift interop). Darwin is supported only as a *dev/CI system*, not as a cross-compile target.

## Decisions

### Decision 1: Flake + stub cargo workspace (not tooling-only)

The flake ships alongside a minimal Cargo workspace (`Cargo.toml` with `[workspace]` and one member `crates/identus-ssi/` containing a placeholder `src/lib.rs`).

**Rationale**: The rust checks (`cargo fmt --check`, `cargo clippy`, `cargo test`) are no-ops without a `Cargo.toml`. A tooling-only flake would defer the rust checks and ship a checks module that cannot validate itself end-to-end. The stub workspace fixes the crate name (`identus-ssi`) once, in this change, so later proposals don't bikeshed it. It also makes `nix flake check` green on day one.

**Alternatives considered**:
- *Tooling-only flake*: defer rust checks until first feature proposal. Rejected because it delivers only half the checks value and ships a flake that can't self-verify.
- *Larger cargo scaffold* (multiple crates, workspace-level shared deps): rejected as premature; the structure should emerge from feature proposals.

### Decision 2: crane for Rust checks (not hand-rolled derivations)

Rust checks are built with [crane](https://github.com/ipetkov/crane) via `craneLib = crane.mkLib pkgs`, using `cargoFmt`, `cargoClippy`, `cargoNextest`/`cargoTest`, `cargoDeny`, `cargoAudit`. The Nix hygiene check (`lint-nix`) remains hand-rolled, mirroring the workspace root's `nix/checks/lint-nix.nix`.

**Rationale**: crane builds the dependency derivation once and shares it across clippy/test/doc/build checks, giving incremental caching that hand-rolled `stdenv.mkDerivation` snippets cannot match. For an SSI SDK that will pull heavy crypto deps, avoiding per-check recompilation is the difference between a usable and an unusable `nix flake check`. crane is flake-parts native and gives `cargoDeny`/`cargoAudit` helpers that handle advisory DB fetches sanely in the Nix sandbox. It also future-proofs `nix build .` of the actual SDK artifact.

**Alternatives considered**:
- *Hand-rolled derivation checks* (match workspace `lint-nix.nix` idiom): rejected for Rust checks specifically because of the per-check recompilation cost. The hand-rolled idiom remains correct for the Nix-hygiene check, which is what the workspace uses it for.
- *naersk*: rejected; crane is more actively maintained and has a richer check-helper API.

### Decision 3: Stable toolchain only

The default devshell and all checks use `rust-bin.stable.latest.default` (with `rust-src`, `rust-analyzer` extensions and the `wasm32-unknown-unknown` target). No nightly toolchain is defined.

**Rationale**: A fresh library has no demonstrated nightly requirement. Stable is the right baseline for a consumable library (matches `rustup default` for the broadest audience, matches what consumer CI uses). Defining a nightly toolchain now would be speculative: adding `rust-bin.nightly.latest.default` back to `nix/rust-toolchain.nix` is a 2-line change bundled with the proposal that first proves a nightly need (likely WASM/browser interop), at which point the decision is evidence-based. Including the `wasm32-unknown-unknown` target in the stable toolchain signals intent toward browser interop without committing to nightly.

**Alternatives considered**:
- *Nightly everywhere*: rejected; this library has no demonstrated nightly requirement, and a consumable library should match what consumer CI uses.
- *Stable default plus an opt-in nightly devshell*: rejected; an unused nightly attribute is dead code that preserves the "we might need nightly later" intention in the code while removing the only thing that would exercise it. Better to add nightly machinery at the moment of need, as a deliberate, evidence-based choice.

### Decision 4: Defer wasm packaging tooling

The toolchain includes the `wasm32-unknown-unknown` target. `wasm-pack` and `wasm-bindgen-cli` are **not** included in any devshell.

**Rationale**: There is a distinction between "the toolchain can target wasm" (cheap, keep) and "we ship wasm artifacts" (a real workflow that only matters once a `cdylib` crate targets the browser). `wasm-bindgen-cli` is version-locked to the `wasm-bindgen` macro version used by crates; pulling it in speculatively imports that fragility early. Adding both packages is a 2-line change bundled with the proposal that first introduces a `wasm-bindgen` dependency, where versions pin together by design.

**Alternatives considered**:
- *Include full wasm tooling now*: rejected because the version-fragility of `wasm-bindgen-cli` makes "set it up once now" unlikely to stay set up; better to add at the moment of need.

### Decision 5: Bundle CI workflow

A `.github/workflows/nix-checks.yml` workflow ships in this change, installing Nix (DeterminateSystems installer) and running `nix flake check` on a matrix of `ubuntu-latest` and `macos-latest`, with magic-nix-cache for build caching.

**Rationale**: The checks module and the CI that enforces it are a single logical unit. Shipping one without the other leaves checks unenforced — a `nix/checks/` module that nothing runs is documentation, not a guardrail. The workflow is small (~30 lines) and adds negligible review burden. It closes the loop on a self-verifying tooling change and is consistent with the workspace's treatment of CI as part of repo setup.

**Alternatives considered**:
- *Split CI into a follow-up proposal*: rejected; reviewing the flake without seeing how CI consumes it is less coherent, and the workflow is too small to justify a separate review cycle.

### Decision 6: Support both `x86_64-linux` and `aarch64-darwin`

`systems = [ "x86_64-linux" "aarch64-darwin" ];` in the flake. CI is a two-runner matrix.

**Rationale**: Decided by the user over the linux-only recommendation. The SDK is intended for cross-platform use (mobile FFI via sdk-kmp/sdk-swift is on the horizon), so establishing darwin support as a precedent now is a deliberate choice. The stub crate has no platform-sensitive deps, so darwin builds clean trivially on day one.

**Tradeoff acknowledged**: This deviates from the workspace root's `x86_64-linux`-only matrix and creates a maintenance asymmetry (sdk-rust is the one submodule that also builds darwin). `flake.lock` will contain darwin paths, CI will run a slower/scarcer `macos-latest` runner, and crane/openssl/pkg-config darwin gotchas become a real constraint as deps are added. This is accepted.

**Alternatives considered**:
- *`x86_64-linux` only (match workspace)*: rejected by the user; darwin support is wanted now.
- *Add darwin later via a scoped proposal*: rejected for the same reason.

### Decision 7: Modular `nix/` layout mirrors the workspace root

```
nix/
├── rust-toolchain.nix      ← rust-overlay → stable toolchain attrset
├── devshells/default.nix
├── checks/{default,lint-nix,rust-fmt,rust-clippy,rust-test,rust-deny,rust-audit}.nix
└── apps/{default,format,format-nix}.nix
```

**Rationale**: Consistency with the workspace root flake's modular layout (`nix/{apps,devshells,checks}`) makes the submodule immediately familiar to anyone who has worked in the workspace. Each check is its own file for the same reason the workspace splits `lint-nix` into its own file: isolated, reviewable, individually runnable. The `rust-toolchain.nix` helper centralizes the rust-overlay invocation so both devshells and the checks module share one definition.

**Alternatives considered**:
- *Single `nix/modules.nix`*: rejected; loses the per-check isolation the workspace idiom provides.
- *Co-locate toolchain in each devshell*: rejected; duplicates the rust-overlay invocation and drifts.

### Decision 8: Edition 2024 with MSRV pinned to the edition floor

The stub crate and root workspace use `edition = "2024"` with `rust-version = "1.85.0"` (the edition 2024 floor). `resolver = "2"` is kept explicitly at the workspace level even though edition 2024 implies it, for clarity and for readers who scan the manifest without inferring from edition.

**Rationale**: Edition 2024 has been stable since Rust 1.85 (Feb 2025), roughly 16 months before this change. For a greenfield SDK bootstrapped in mid-2026, starting on edition 2024 avoids a forced edition-bump PR later — edition migrations are mechanical but disruptive once real code exists, and this SDK will eventually hold crypto and FFI code where the edition's stricter `unsafe` rules and RPIT lifetime-capture semantics matter. Encoding those semantics from day one means reviewers never relax into 2021-era patterns that must be relearned.

Note on the resolver half of the original question: the `[workspace] resolver` field only accepts `"1"` or `"2"`; there is no `resolver = "3"`. The plan was already at the ceiling (`"2"`), so "newer resolver" was a non-option. Edition 2024 auto-selects resolver 2, so the explicit `resolver = "2"` is redundant-but-harmless; kept for explicitness.

MSRV is pinned to the edition floor (`1.85.0`) rather than a more recent stable-N to maximize compatibility. The direct Rust-artifact consumers are the workspace itself: the FFI consumers (`sdk-kmp`, `sdk-swift`) use Kotlin/Swift toolchains, not Rust, and `sdk-ts` already uses a nightly Rust toolchain. The audience is unambiguously on modern Rust, so `1.85.0` (the lowest value edition 2024 permits) gives the broadest reach compatible with the chosen edition.

**Alternatives considered**:
- *Edition 2021 (status quo)*: maximizes MSRV reach (floor ~1.56) but buys nothing in this workspace's context — no consumer is on a Rust old enough to need it, and modern crypto crates drop old MSRVs fast. Defers a mechanical-but-annoying migration to the first feature proposal that wants 2024 ergonomics.
- *Edition 2024 with a higher MSRV (stable-N, e.g. 1.88.0)*: marginally tighter toolchain guarantees but reduces reach without a concrete need. Rejected; the edition floor is the right MSRV until a dependency or feature forces a bump, at which point it becomes an evidence-based change.

## Risks / Trade-offs

- **[Risk] darwin CI flakiness / `macos-latest` runner scarcity** → Mitigation: the stub crate has no platform-sensitive deps, so the darwin leg is trivially green now. As deps are added, watch for darwin-specific openssl/pkg-config failures and address them in the proposal that introduces the offending dep. If darwin CI becomes a persistent burden, a follow-up change can gate darwin behind `if system == "x86_64-linux"` without removing the devshell support.
- **[Risk] crane is a new input pattern in the workspace** → Mitigation: sdk-rust is an independent repo with its own flake; crane does not leak into the workspace root's closure or style. The workspace root's hand-rolled `lint-nix` idiom is preserved for Nix-hygiene checks (which is what it's good at).
- **[Risk] `cargo-audit` / `cargo-deny` fetch advisory DBs in the Nix sandbox** → Mitigation: crane's `cargoAudit` and `cargoDeny` helpers handle DB fetching sanely. Pin `advisory-db` via crane's `input` mechanism so the check is reproducible and not impure.
- **[Risk] crate name `identus-ssi` is locked in early** → Mitigation: it's a placeholder name aligned with the "SSI capability" framing; renaming a workspace member later is mechanical (`cargo` handles it). The cost of deciding now is lower than the cost of bikeshedding across proposals.
- **[Trade-off] `flake.lock` is larger** (darwin paths) and CI is slower (two runners) than the linux-only alternative. Accepted per Decision 6.

## Migration Plan

This is a greenfield bootstrap; there is no existing state to migrate.

1. Land the flake + `nix/` tree + stub cargo workspace + `deny.toml` + `nix-checks.yml` in one PR on the `nix-tooling` branch.
2. Verify locally: `nix flake check` passes on both linux and darwin (or at least linux; darwin verification may require a macOS host).
3. Verify CI: the `nix-checks.yml` workflow runs green on the PR for both matrix legs.
4. Merge to `main`.

**Rollback**: revert the PR. There are no consumers of the flake yet (no feature code depends on it), so rollback is trivial.

## Open Questions

- None blocking. The crate name `identus-ssi` is assumed from the "SSI capability" framing; can be renamed mechanically later if a different name is preferred.
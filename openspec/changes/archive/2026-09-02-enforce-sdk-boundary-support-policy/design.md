## Context

`develop@a3eed8717035ab7d18eadd78977bb4e60d20b61a` already has a seven-layer
workspace dependency guard, Linux/macOS Nix checks, a browser-WASM smoke for
the entropy adapter and distinct default/KMP compatibility checks. It does not
inspect prohibited external identities or sources, run the declared MSRV, or
publish which targets and feature sets those checks support.

NeoPRISM is the Rust/Nix etalon at
`8becb225132efb1d9302b2c5f6ed4d87b84e8685`. Its `nix/rustTools.nix` pins
nightly `2026-03-18`; `nix/checks/neoprism-checks.nix` exercises default,
all-feature and individual-feature surfaces. SDK-Rust retains the same pin and
feature-gate discipline while requiring a stable `1.85.0` consumer surface.
NeoPRISM is evidence, never a build or source dependency.

## Goals / Non-Goals

**Goals:**

- make the documented chain-neutral boundary executable for direct and
  resolved dependencies;
- publish one reviewable compatibility source of truth and detect drift from
  Cargo/Nix configuration;
- distinguish host-tested behavior from compile-only evidence and uncommitted
  future targets;
- make the exact MSRV, etalon toolchain and implemented feature surfaces fail
  independently;
- close `IDR-002` and `IDR-003` without implying that later SSI components or
  downstream migrations exist.

**Non-Goals:**

- porting crypto, DID, credential or protocol behavior;
- changing MSRV, the NeoPRISM pin, crate names or release policy;
- runtime certification on browsers, Android or iOS;
- exposing a stable FFI, measuring release artifacts, or setting premature
  size/build-time budgets;
- changing any donor/consumer repository or the reserved `main` branch.

## Decisions

### 1. Enforce identity, location and resolved closure

The conformance guard examines dependency aliases and explicit Cargo
`package` identities in normal, development, build and target-scoped tables,
plus root `[patch]` and `[replace]` source overrides. Names are lower-cased and
underscores normalize to hyphens before matching explicit chain/product
families. It also examines Git URLs, override source selectors and local paths.
Local paths must canonicalize inside the repository, and workspace dependency
paths must resolve beneath `crates/`.

The lockfile is checked separately so a prohibited package or donor source in
the resolved dependency closure fails even if its direct wrapper has a neutral
name. Exact deny families avoid broad substrings such as `compact` that could
reject unrelated general-purpose crates.

### 2. Keep the rulebook deterministic and offline

The policy is repository data, not a network lookup or donor build. Tests use
synthetic manifests/lockfiles for bypass cases and the real workspace for the
positive architecture assertion. A policy change is therefore a visible,
reviewed source change.

### 3. Use a machine contract plus explanatory policy

`docs/architecture/sdk-support-policy.toml` is the machine-readable source of
truth. A deterministic repository command validates its schema and compares
the declared MSRV, etalon toolchain, Nix host systems, eligible packages,
feature sets and gate names to repository configuration. The adjacent
Markdown policy explains what each evidence tier does and does not promise.
Repeated host, target or feature identities are invalid rather than
last-entry-wins policy data.

The policy tiers are:

- `host-tested`: formatting, linting, tests, docs and dependency gates execute
  on the listed host system;
- `compile-checked`: eligible crates compile for the target, without a runtime
  or platform-integration claim;
- `planned`: no compatibility promise or required gate exists yet;
- `not-supported`: the current repository intentionally exposes no supported
  surface, such as FFI during bootstrap.

### 4. Separate the stable floor from the etalon ceiling

The existing nightly `2026-03-18` remains the reproducible development and
primary CI toolchain. A second minimal Rust `1.85.0` Crane instance compiles
every machine-declared default, minimal and opt-in feature surface. These gates
cannot be replaced by a version comparison or by the nightly builds because
only compilation of each selection on the declared floor detects accidental
language/library or feature-specific drift. Structural validation proves that
`msrvCraneLib` wraps the pinned stable toolchain, so a trustworthy variable
name cannot conceal nightly execution.

### 5. Compile only implemented portable crates on cross targets

The browser-WASM, Android ARM64 and iOS ARM64 gates cover implemented generic
crates and the entropy adapter where its backend is applicable. Empty roadmap
placeholder crates and the conformance-only crate do not become supported APIs
merely because the workspace can parse them. Cross-target checks do not link,
run, certify platform storage or certify FFI.

### 6. Treat features as separate public build surfaces

The matrix covers workspace defaults, `identus-crypto` with no default
features and KMP compatibility, and the entropy adapter with its empty,
deterministic and system-random feature surfaces. Mutually compatible feature
combinations are checked explicitly instead of relying on Cargo unification
from `--all-features` alone. Structural validation compares each gate's exact
package selection, default-feature mode and activated feature set with the
machine contract. Compile-checked targets declare their package-qualified
backend features using the same selection contract. Gate discovery follows the
literal import graph from `flake.nix` through `nix/checks/default.nix`, so a
detached check tree or orphaned module cannot satisfy a required check.
Nix line and block comments are removed before graph and gate discovery.
Workspace-wide gates explicitly select and resolve to the complete workspace
package set and reject exclusions, so implicit defaults cannot conceal lost
coverage.

### 7. Defer budgets truthfully

Before a candidate release and stable artifact shape exist, binary size and
build time are `measurement-only`: the policy defines the measurement point
but no pass/fail compatibility budget. A later issue must add reproducible
baselines, thresholds and regression handling before either becomes a support
promise.

## Risks / Trade-offs

- Name/source rules cannot prove semantic neutrality of an innocently named
  third-party crate. Dependency review and the source matrix remain necessary.
- Mobile and WASM compilation can pass while runtime integration fails. The
  policy labels these targets compile-only and requires downstream runtime
  evidence before promotion.
- Adding three Rust target standard libraries increases Nix toolchain size and
  CI time. Separate checks make the cost and failure domain visible.
- An MSRV build may expose dependencies whose own MSRV drifted despite a
  compatible SDK manifest. The lockfile is part of the supported candidate and
  should fail rather than silently resolving a different graph.

## Migration Plan

Add the two specifications and issue #22 first, then implement policy data,
guards and Nix gates. Verify locally, sync the current capability specs,
archive the completed change, and deliver one signed PR to `develop`. Update
the canonical rows only to evidence produced in the same PR. Rollback is a
normal revert; there is no runtime or consumer data migration.

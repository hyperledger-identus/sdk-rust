## Context

`develop@a05cb05575c7d99fc025f5be450da32c7a11a5c1` has 23 Rust gates whose
operations and Cargo selections are duplicated between Nix expressions and
`docs/architecture/sdk-support-policy.toml`. `scripts/check-support-policy.py`
removes Nix comments, follows imports, extracts definitions and tokenizes
quoted Cargo arguments. The approach caught substantial drift in #23, but it
cannot model the Nix language and should not become permanent infrastructure.

The #23 validator baseline on local `aarch64-darwin`, measured with 20 samples
on 2026-09-04, is 6.163 ms warm p50 / 7.134 ms warm p95 and 48.572 ms
process-cold p50 / 50.183 ms process-cold p95. These are engineering
diagnostics, not a compatibility budget.

## Goals / Non-Goals

**Goals:**

- define every compatibility gate and Cargo selection once as typed data;
- make Nix produce the actual Crane checks from that data;
- keep validation deterministic, offline and fail-closed;
- retain exact package, workspace, exclusion, feature, target, toolchain and
  operation drift detection without a Nix/Cargo text parser;
- give agents one small, documented extension point for future gates;
- measure validator latency consistently on both supported host families.

**Non-Goals:**

- changing the supported toolchains, hosts, targets, packages or features;
- making timing a release or compatibility threshold;
- replacing Nix, Crane or the human-readable support policy;
- porting SSI components, changing consumer repositories or publishing crates;
- changing live GitHub settings or the reserved `main` branch.

## Decisions

### 1. Separate compatibility claims from executable gate definitions

`docs/architecture/sdk-support-policy.toml` remains the normative compatibility
claim: supported hosts, compile-only targets, feature surfaces and limitations.
`nix/checks/gates.toml` becomes the normative execution manifest. A policy
claim references a gate name; the validator proves the referenced manifest
entry has the exact effective selection required by that claim.

The policy no longer duplicates an operation map. Each gate entry declares its
operation directly alongside the rest of its execution data.

### 2. Represent Cargo meaning, not a shell-like argument string

Each gate records its name, Crane operation, toolchain class, source class,
artifact class, locked/workspace/package/exclusion modes, library/all-target
selectors, default/all/selected features, target and operation-specific trailing
arguments. Nix deterministically renders those fields into the correct Crane
extra-argument attribute.

The manifest rejects duplicate names, unknown keys/enums, contradictory modes,
unknown packages/features and invalid operation/toolchain/artifact
combinations. The validator compares sets only where Cargo semantics are
unordered and preserves ordered trailing arguments where operation behavior
depends on order.

### 3. Generate the full Rust check matrix in the reachable root module

`nix/checks/default.nix` reads `gates.toml` with `builtins.fromTOML`, maps each
entry to one Crane derivation and merges those results with the repository's
non-Rust factory and text-hygiene checks. The former hand-written Rust gate
modules are removed, eliminating a second execution representation and the
orphan-module class of drift.

Because actual check attribute names are generated from the manifest, a gate
name left in a Nix comment, multiline string or dead `_module.args` value
cannot satisfy policy validation. Dynamic/interpolated decoys are inert: they
are neither parsed nor used to construct a gate.

### 4. Keep one narrow Nix reachability invariant

The offline validator may retain a narrow assertion that `flake.nix` imports
the root check module and that the root module consumes `gates.toml` to publish
the generated checks. It SHALL NOT parse operation calls or Cargo arguments
from Nix. Full `nix flake check` remains the executable proof that every
generated derivation evaluates and builds.

### 5. Benchmark parsing separately from process startup

`scripts/benchmark-support-policy.py` runs at least 20 successful samples in
two modes: in-process warm validation and process-cold checker invocation. It
prints machine-readable JSON containing platform, revision, sample count,
p50/p95 and the #23 comparison baseline when applicable. Hosted Linux/macOS CI
runs the same command as diagnostic evidence; no timing alone fails a build
unless a generous safety ceiling detects pathological regression.

## Risks / Trade-offs

- Generic Nix generation is denser than small hand-written modules. Strict
  schema validation and operation-specific construction keep failures local.
- TOML cannot express arbitrary Nix. That is intentional: a gate needing a new
  execution capability must extend the reviewed schema instead of hiding
  semantics in an opaque string.
- Process timing varies across runners. Results are diagnostics with a broad
  pathological ceiling, not a stable performance promise.
- A coordinated malicious edit can alter both producer and validator. Normal
  signed review, exact-head CI and the human-readable policy remain independent
  controls; this change removes accidental drift, not repository compromise.

## Migration and Rollback

Add and validate this contract first. Then introduce the manifest and generic
Nix generator, migrate all current gates without changing their effective
commands, refactor the validator/tests, and record timing evidence. Run the
complete factory and Nix matrix, review the exact diff, sync the current specs,
archive the change and deliver one signed issue-linked PR to `develop`.
Rollback is a normal revert with no consumer or data migration.

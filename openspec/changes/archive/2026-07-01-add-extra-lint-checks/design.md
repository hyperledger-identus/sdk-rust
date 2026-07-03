## Context

`sdk-rust` enforces Rust and Nix quality locally via `nix flake check` (rust-fmt, rust-clippy, rust-test, rust-deny, rust-audit, lint-nix) and provides `nix run .#format` / `.#format-nix` apps for auto-fixing. However, the file-hygiene checks that CI actually runs — markdown-lint, editorconfig, yaml lint, shellcheck — come from the shared remote `hyperledger-identus/.github/.github/workflows/lint-files.yml` reusable workflow referenced by `.github/workflows/file-hygiene.yml`. Contributors cannot run or auto-fix those checks locally, so failures surface only after a push. TOML files (`Cargo.toml`, `deny.toml`) have no formatter or linter at all, locally or in CI.

The `neoprism` submodule in the same workspace already solves the TOML formatting case with `taplo` (config in `taplo.toml`) and provides `markdownlint-cli2`, `yamllint`, `editorconfig-checker`, and `shellcheck` in its devshell, driven by justfile recipes. This change ports that pattern into `sdk-rust`'s existing flake-check idiom so that `nix flake check` reproduces CI locally and `nix run .#format` can auto-fix what is fixable.

## Goals / Non-Goals

**Goals:**
- Make `nix flake check` reproduce the file-hygiene checks that CI enforces (markdown, yaml, editorconfig, shell), so contributors can self-verify before pushing.
- Add TOML formatting and linting to both the check surface and the format apps, using `taplo` with a `taplo.toml` config aligned with neoprism.
- Provide the new tools in the default devshell so contributors can run and fix checks locally.
- Keep the change purely additive to the existing Rust/Nix checks and to CI workflows.

**Non-Goals:**
- Replacing or removing the shared remote `lint-files.yml` CI workflow — it remains the CI source of truth; the local checks mirror it.
- Adding new GitHub Actions jobs; `.github/workflows/*` are unchanged.
- Linting generated/vendored content (Crate lockfiles, `target/`, `openspec/`, `.pi/`, `.claude/`).
- Introducing a justfile or pre-commit framework; the flake check/app idiom is sufficient and consistent with the existing module structure.
- Changing any Rust crate behavior or public API.

## Decisions

### Decision 1: Implement as flake checks, not justfile recipes

**Choice:** Add `lint-text` and `lint-toml` as `stdenv.mkDerivation` checks under `nix/checks/`, mirroring the existing `lint-nix.nix` pattern (`cleanSourceWith` filter + `checkPhase` + `touch $out`).

**Alternatives considered:**
- *neoprism-style justfile recipes* — rejected because `sdk-rust` has no justfile-driven lint surface and already has the flake-check idiom; introducing justfile would fragment the single `nix flake check` gate.
- *pre-commit hooks* — rejected as an extra framework with no local-Nix reproducibility benefit.

**Rationale:** Keeps `nix flake check` as the one command that mirrors CI, reuses the proven `lint-nix.nix` derivation shape, and integrates with `nix-checks.yml` with zero CI changes.

### Decision 2: Two separate checks — `lint-text` and `lint-toml`

**Choice:** Keep text hygiene (markdown/yaml/editorconfig/shell) and TOML hygiene (taplo) as independent check derivations.

**Rationale:** Independent failure reporting; TOML also has an auto-fix path (`taplo format`) that the `format` app will exercise, whereas text fixes use `markdownlint-cli2 --fix` run manually from the devshell. Separation matches the distinct tool families.

### Decision 3: `taplo.toml` config aligned with neoprism

**Choice:** Add a root `taplo.toml` identical in spirit to neoprism's:

```toml
[formatting]
align_entries       = true
column_width        = 100
allowed_blank_lines = 1
indent_string       = "  "
array_auto_collapse = false
array_auto_expand   = false
compact_arrays      = false
```

**Rationale:** Workspace consistency — the same TOML style across `sdk-rust` and `neoprism`. `align_entries` and a 100-column width keep manifests readable without semantic change (taplo only adjusts whitespace/alignment, never key order or values).

### Decision 4: Extend `format` app and add `format-toml` app

**Choice:** Update `nix/apps/format.nix` to run `taplo format` on `*.toml` after `cargo fmt` and `nixfmt`. Add a new `format-toml` app paralleling `format-nix` for TOML-only formatting.

**Rationale:** Symmetry with `format-nix`; the composite `format` app remains the one-command fix-everything entrypoint. `taplo format` writes in place, matching how `nixfmt` and `cargo fmt` behave in the existing app.

### Decision 5: Add text/TOML tools to the default devshell

**Choice:** Add `taplo`, `markdownlint-cli2`, `yamllint`, `editorconfig-checker`, and `shellcheck` to `nix/devshells/default.nix`.

**Rationale:** Contributors need the tools available to run checks ad-hoc and to apply fixes (e.g. `markdownlint-cli2 --fix`, `taplo format`). All packages are already in nixpkgs and used by neoprism.

### Decision 6: `.editorconfig-checker.json` exclude file

**Choice:** Add `.editorconfig-checker.json` excluding `.git`, `target`, and any generated/vendored paths, mirroring neoprism's exclude file.

**Rationale:** editorconfig-checker scans the whole tree and would flag build artifacts / vendored content; an exclude file is the tool's native mechanism and keeps the check passing on the existing tree.

### Decision 7: Mirror, do not fork, the lint configs

**Choice:** Reuse the existing repo-root `.markdownlint.yml`, `.markdownlint-cli2.yaml`, `.yamllint.yml`, and `.editorconfig` as the config the local checks read. Adjust them only where needed to make the local checks pass on the current tree and to align with neoprism where the repo is missing a rule (e.g. add `MD041`/`MD029`/`MD036`/`MD059`/`MD040`/`MD028`/`MD060` toggles that neoprism sets, if the current tree requires them).

**Rationale:** The shared remote CI workflow reads these same repo-root configs, so the local checks naturally match CI without duplicating policy.

## Risks / Trade-offs

- **[taplo reformats `Cargo.toml` / `deny.toml`]** → taplo only changes whitespace/alignment, never semantics or key order; the diff is cosmetic. Implementation will run `taplo format` once and commit the result so the tree is baseline-clean.
- **[Local check config drift from CI]** → The shared `lint-files.yml` is remote and not inspectable locally. If it carries extra rules beyond the repo-root configs, local `nix flake check` could pass while CI fails. Mitigation: the repo-root configs are the contract the shared job reads, so mirroring them is the best available fidelity; the `file-hygiene.yml` job is retained as the CI backstop.
- **[editorconfig-checker flags existing files]** → Mitigated by the `.editorconfig-checker.json` exclude and by fixing any genuine violations (e.g. trailing whitespace, missing final newline) during implementation.
- **[Larger nixpkgs closure / check evaluation time]** → New derivations add build time to `nix flake check`, but none compile code (all are script-driven); impact is small. Tools are already cached via neoprism's usage in the same workspace.
- **[markdownlint-cli2 ignore patterns must cover `openspec/`, `.pi/`, `.claude/`, `target/`]** → The existing `.markdownlint-cli2.yaml` already ignores these; implementation verifies and extends the ignore list if new generated paths appear.

## Migration Plan

1. Add `taplo.toml` and `.editorconfig-checker.json`.
2. Reconcile `.markdownlint.yml` / `.markdownlint-cli2.yaml` / `.yamllint.yml` / `.editorconfig` with the current tree.
3. Add the new devshell packages, check derivations, and apps to the flake.
4. Run `nix run .#format` (which now includes `taplo format`) to baseline-format all TOML files; commit cosmetic TOML changes.
5. Run `nix flake check` locally and fix any reported text/editorconfig/yaml/markdown violations.
6. Verify `nix flake check` is green on both `x86_64-linux` and `aarch64-darwin`.

**Rollback:** Revert the flake module additions and config files; the cosmetic TOML reformat commit can be reverted independently with no semantic impact.

## Open Questions

- Whether to also add `shellcheck` to `lint-text` now or defer. Decision: include it, since neoprism's text-lint set includes it and it matches the "match the lint on CI" intent. If the shared CI workflow does not run shellcheck, including it is a strict superset (still correct, just stricter).
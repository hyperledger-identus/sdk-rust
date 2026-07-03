## Context

The `nix-tooling` flake already runs `rust-fmt`, `rust-clippy`, `rust-test`, `rust-deny`, and `rust-audit` via crane, sharing a single `cargoArtifacts` dependency derivation (the "Crane dependency caching" requirement) and a `rustSrc` cleaned source. The spec even names `cargoDoc` as a future consumer of that shared derivation, but no check wires it up. The workspace sets `[workspace.lints.rust] warnings = "deny"` in `Cargo.toml`, so `cargo doc` already fails on rustdoc warnings (e.g. `rustdoc::private_intra_doc_links`) without any extra flags. A recent regression shipped three newtypes whose doc comments linked to private `validate_*` helpers; only a manual `cargo doc` surfaced it. There is no CI signal today for rustdoc regressions.

## Goals / Non-Goals

**Goals:**
- Make `nix flake check` build the workspace rustdoc and fail on any rustdoc warning, so doc-link hygiene is enforced continuously like fmt/clippy/test.
- Reuse the existing shared `cargoArtifacts` / `rustSrc` plumbing so no new dependency build is introduced.
- Document the check in the `nix-tooling` spec (new "Rust doc check" requirement; extend the stable-toolchain enumeration).

**Non-Goals:**
- Publishing/hosting rendered docs. This check only verifies the doc build is warning-free; it does not upload artifacts.
- Documenting dependency crates (`--no-deps` is used to keep the check fast and focused on workspace code).
- Changing any Rust crate behavior or any doc-comment prose. The earlier doc-link fixes are bug fixes already in the tree, not part of this change.
- Adding `--document-private-items`. The intent is to catch regressions in the *public* doc surface that contributors and downstream consumers see.

## Decisions

### Decision: Use `craneLib.cargoDoc` with `--no-deps`, sharing `cargoArtifacts`/`rustSrc`

**Rationale:** Mirrors the established pattern of the other crane-based checks (`rust-clippy`, `rust-test`) and inherits the shared dependency derivation, satisfying the "Crane dependency caching" requirement. `--no-deps` limits the build to workspace crates, keeping it fast and surfacing only our own doc regressions rather than noise from third-party crates.

**Alternatives considered:**
- A standalone `cargo doc` script app invoked manually: rejected — the value is continuous enforcement via `nix flake check`, not an on-demand tool.
- A `RUSTDOCFLAGS="-D warnings"` override: rejected as unnecessary — the workspace lint already denies warnings, so the check inherits that for free. Adding the flag would duplicate policy in two places (workspace lints and the check) and risk drift.

### Decision: Rely on the workspace `warnings = "deny"` lint rather than a per-check `deny-warnings` flag

**Rationale:** Centralizes lint policy in `Cargo.toml`'s `[workspace.lints.rust]`, the same source clippy and the build already honor. The check therefore needs no lint configuration of its own; any future rustdoc lint the workspace chooses to deny is automatically enforced by the doc check too.

**Alternatives considered:** Setting `RUSTDOCFLAGS = "-D warnings"` in the crane derivation. Rejected for the drift reason above; only worth revisiting if a future need arises to enforce doc lint policy *stronger* than the workspace default specifically for docs.

## Risks / Trade-offs

- **[Risk] Doc-build cost added to `nix flake check`** → `cargoDoc` reuses `cargoArtifacts`, so only the doc render of workspace crates is new work; `--no-deps` avoids rendering third-party docs. Mitigation already in place via the shared derivation; acceptable incremental time.
- **[Risk] A third-party dependency emits a rustdoc warning under `--no-deps`** → `--no-deps` excludes dependency crates from documentation, so their warnings do not affect this check. If a workspace crate's own docs trigger a warning from a re-exported private path, that is exactly the regression we want to catch.
- **[Risk] Policy drift between workspace lints and the check** → Mitigated by *not* re-declaring lint policy in the check; the single source of truth is `[workspace.lints.rust]` in `Cargo.toml`.
- **[Trade-off] `--no-deps` means dependency docs are not validated** → accepted; validating third-party docs is out of scope and would couple our CI to upstream crate doc hygiene.
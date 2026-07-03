## Why

The `nix-tooling` spec already anticipates `cargoDoc` as a future consumer of the shared crane dependency derivation, but never defines it as a check. Meanwhile `cargo doc` is not exercised by any flake check, so rustdoc regressions — such as broken intra-doc links to private items (`rustdoc::private_intra_doc_links`, promoted to an error by the workspace's `warnings = "deny"` lint) — go undetected until a contributor happens to run `cargo doc` by hand. A recent incident of exactly this kind (three newtypes shipped with doc comments linking to private `validate_*` functions) forced a manual `cargo doc` run to discover. Adding a `rust-doc` check closes the gap and makes `nix flake check` reproduce the full local doc-build that contributors rely on.

## What Changes

- Add a `rust-doc` flake check (via `craneLib.cargoDoc`, `--no-deps`) to `nix/checks/rust-doc.nix`, reusing the shared `cargoArtifacts` derivation and `rustSrc`, and wire it into `nix/checks/default.nix`'s imports. Because the workspace already sets `[workspace.lints.rust] warnings = "deny"`, rustdoc warnings (including `private_intra_doc_links`) are promoted to errors with no extra `RUSTDOCFLAGS`.
- Extend the `nix-tooling` spec with a new "Rust doc check" requirement (pass on warning-free docs; fail on any rustdoc warning such as a private intra-doc link) documenting the check.
- Extend the `nix-tooling` "Checks use the stable toolchain" requirement's enumeration to include `rust-doc` alongside the existing crane-based checks.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `nix-tooling`: Adds a new "Rust doc check" requirement defining the `rust-doc` flake check (runs `cargo doc --no-deps` via crane, fails on rustdoc warnings, shares the crane dependency derivation), and extends the "Checks use the stable toolchain" requirement to enumerate `rust-doc` among the crane-based checks built on the stable toolchain.

## Impact

- **Nix flake**: `nix/checks/` gains a new `rust-doc.nix` check module; `nix/checks/default.nix` imports it. The check consumes the existing shared `cargoArtifacts` and `rustSrc` introduced by the crane-caching requirement, so no new dependency derivation is built.
- **`nix flake check`**: gains one additional check (`rust-doc`) on both `x86_64-linux` and `aarch64-darwin`. CI (`.github/workflows/nix-checks.yml`) is unchanged — it already runs `nix flake check`, so the new check is enforced on pull requests and pushes to `main` automatically.
- **No Rust crate behavior changes**: tooling-only. The doc-comment fixes that surfaced the need (in `crates/core/src/url.rs`, `crates/did/src/method.rs`, `crates/did/src/version.rs`) are bug fixes already in the tree and are not part of this change's scope; this change only guarantees such regressions are caught going forward.
- **Dependencies**: none new — `craneLib.cargoDoc` is already provided by the `crane` flake input in use by the existing checks.
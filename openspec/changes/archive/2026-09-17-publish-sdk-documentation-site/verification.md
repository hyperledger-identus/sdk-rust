# Verification

- **Date:** 2026-09-17
- **Planning head:** `f3696cee372ce1a80b3bc425c0a606f6e4a3a19d`
- **Implementation head:** `12bfbccc55edfe13528217c08ac670102f12517a`
- **Environment:** aarch64-darwin with the repository-pinned Nix toolchain

## Passed locally

- `nix build --print-build-logs .#docs-site`: mdBook and all three Graphviz
  diagrams built; lychee inspected 486 generated links with zero errors.
- `nix build .#checks.aarch64-darwin.docs-site`: the same site derivation is a
  first-class flake check.
- `nix build .#checks.aarch64-darwin.factory-contract`: all factory mutation,
  contribution-policy, support-policy, OpenSpec, and structural checks passed.
- `nix build .#checks.aarch64-darwin.lint-nix`: deadnix, statix, and nixfmt
  passed after formatting the new module.
- `nix build .#checks.aarch64-darwin.lint-text`: Markdown, YAML, EditorConfig,
  and shell checks passed with zero Markdown findings.
- `git diff --check`: passed.
- The generated dependency, ownership, and evidence-flow SVGs were rendered to
  PNG and visually inspected for legibility, direction, clipping, and overlap.

## Resolved verification findings

- mdBook 0.5 requires the Font Awesome brand prefix for the GitHub icon.
- Nested chapter index files must be linked as `index.md`, not `README.md`, to
  match generated output paths.
- The generated 404 page intentionally points at the deployment mount path, so
  offline checking excludes only that generated page while checking every
  other HTML page against the artifact root.
- The support-policy mutation fixture now includes the additive root Nix
  module, preserving its fail-closed graph test.

## Hosted delivery evidence

The protected pull request must independently pass the Linux `fast` gate and
one exact-head review. After merge, the Pages deployment must identify the
protected `develop` revision, return the public handbook, and be linked from
issues #324 and #326. Those hosted receipts are delivery evidence and are not
represented as already complete by this local verification.

No crate, tag, registry object, support promise, or certification is created.

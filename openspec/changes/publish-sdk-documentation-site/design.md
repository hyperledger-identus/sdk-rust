# Design

## Site structure

Use mdBook as a static handbook, with a tracked `site/book.toml`, ordered
`SUMMARY.md`, focused Markdown chapters, and a small local CSS layer. Content
is grouped by orientation, three-crate reference, architecture, adoption,
release readiness, and governance. Absolute GitHub links point to durable
repository policy; internal navigation remains relative and link-checked.

## Diagrams

Store architecture sources under `site/diagrams/*.dot`. A single build script
copies the site into a private temporary directory, renders each DOT source to
its corresponding `src/diagrams/*.svg`, builds mdBook, and validates the final
tree offline. Generated SVGs are artifacts, not tracked source. This prevents
hand-edited diagram drift and browser-side executable dependencies.

## Reproducible build

A Nix module exposes `packages.docs-site` and aliases the same derivation as
`checks.docs-site`. It supplies mdBook, Graphviz, lychee, and shell utilities
from the locked Nixpkgs input. `nix build .#docs-site` produces only the static
site closure. The required fast workflow builds the check so documentation and
diagram breakage cannot merge unnoticed.

## Deployment

A dedicated workflow triggers on pushes to `develop` and manual dispatch. The
build job checks out the exact revision, installs Nix, builds the Nix package,
configures Pages, and uploads the dereferenced static directory. A separate
deploy job uses the `github-pages` environment and only Pages/OIDC write
permissions. Concurrency cancels superseded deployments but never changes the
artifact identity of a completed run.

## Factual contract

The site names `identus-derive`, `identus-core`, and `identus-crypto` as the
candidate train, not stable releases. It derives responsibilities from current
crate docs/manifests and release boundaries from ADR 0113/`RELEASING.md`. It
states the remaining namespace, compiler, downstream, slow-evidence, and human
approval gates and links their live issues.

## Tests

- Build from a clean Nix source closure.
- Fail when `SUMMARY.md` names a missing page.
- Render every DOT source and reject Graphviz failures.
- Run offline link validation over generated HTML/assets.
- Run actionlint, markdown, YAML, Nix formatting/lint, factory/OpenSpec checks,
  and exact hosted `fast` CI.
- After merge, verify the Pages run SHA and public content/status.

## Rollback

Disable the Pages publishing source, then revert the workflow, Nix module,
build script, site content, README link, ADR, and canonical capability. No
crate, tag, registry object, consumer, or persistent data requires migration.


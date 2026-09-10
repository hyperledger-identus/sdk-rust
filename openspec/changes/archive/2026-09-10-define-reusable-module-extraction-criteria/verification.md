# Verification evidence

**Date:** 2026-09-10

## Passed

- `scripts/factory doctor` before planning edits on
  `develop@78e0656f860c0e569df2edb25d087c42561c51a5`
- `scripts/factory validate define-reusable-module-extraction-criteria`
- `scripts/factory research-ready define-reusable-module-extraction-criteria`
- `scripts/factory constraints-ready define-reusable-module-extraction-criteria`
- planning-only commit `85bc3c247d904b775133030b87711882bf22afad`
- `scripts/factory preflight define-reusable-module-extraction-criteria
  --issue 253 --write`
- `scripts/factory preflight define-reusable-module-extraction-criteria
  --validate-receipt`
- `scripts/factory check`
- `scripts/check-constraints.py .`
- `scripts/check-support-policy.py .`
- `nix build .#checks.aarch64-darwin.lint-text --print-build-logs`
- `git diff --check`

## Semantic architecture review

A distinct exact-diff pass compared the OpenSpec requirements, ADR 0110,
`SDK-ARCH-003`, the SDK blueprint, ADR 0061, and the existing SSI upstream
program. It checked that every hard gate is objectively evidenced, source
disposition is separate from delivery action, and downstream deletion requires
a merged immutable SDK candidate plus separate NeoPRISM evidence.

One issue was found and resolved: the first draft treated every concrete effect
as downstream-only, which contradicted the SDK's reusable optional
`identus-did-resolver-http` adapter. The final rule keeps deterministic core
logic behind ports while permitting separately selectable effect adapters;
chain and product policy still remains downstream.

The review found no remaining blocker, no route for a numeric score to override
a hard gate, no consumer dependency introduced into sdk-rust, and no claim that
the existing NeoPRISM beta has passed CI or completed adoption.

## Not run and why

- Cargo build, test, Clippy, docs, coverage, target builds, dependency-cone
  comparison, advisory scan, and full `nix flake check`: this decision changes
  documentation, OpenSpec, and a machine-validated constraint entry only; it
  changes no Rust, Cargo, Nix, workflow, target, or generated source.
- NeoPRISM tests and hosted CI: the ADR phase is read only in that repository.
  Issue #321 owns refreshed downstream compatibility evidence after this
  decision reaches sdk-rust `develop`.
- sdk-rust publication, release, `main` promotion, and Apollo deprecation:
  protected or explicitly out of scope.

No unrun command is represented as passing.

## Hosted CI triage

The first hosted `fast` run at head
`4036a01b8f8e00f9227b1965b407b06970ef640f` exposed an isolated-fixture
failure: `SDK-ARCH-003` pointed directly to ADR 0110, while the factory fixture
copies the canonical SDK blueprint rather than every later ADR. The constraint
now names the blueprint section containing the effective rule; ADR 0110 remains
its decision authority and detailed rationale.

`nix build .#checks.aarch64-darwin.factory-contract --print-build-logs` then
passed locally against the corrected tree. The failed hosted run is retained as
evidence and is not represented as passing; the new exact head requires its own
green hosted checks before merge.

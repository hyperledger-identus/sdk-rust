# Verification

## Exact state

- Base: `e197810b7d94cfa2e9182b7413110c5edac944db`
- Planning head: `2b9a9178fb0572fc9ab42ec4b249e710e7a39103`
- Live API observation: `2026-09-14T08:44:44Z`

## Passed locally

- `python3 scripts/tests/bootstrap-inventory.py` — 20 tests.
- `./scripts/check-bootstrap-inventory.py .`
- `./scripts/factory check`
- `git diff --check`
- `cargo fmt --all -- --check`

## Not run

No Rust implementation, Cargo manifest, dependency, target or Nix input changed,
so workspace build/test/Clippy and full `nix flake check` are delegated to the
required hosted `fast` canary. The change does not claim those commands passed
locally.

## Live evidence

The settings receipt records the effective rules and security controls. GitHub
returned HTTP 422 when repository-level secret scanning was requested because
the enterprise policy owns it. No bypass was attempted.

PR #264 is the exact-head hosted protection canary. Its first policy event
correctly blocked a malformed issue field in the pull-request body before any
merge; the corrected body and signed follow-up head are re-evaluated normally.

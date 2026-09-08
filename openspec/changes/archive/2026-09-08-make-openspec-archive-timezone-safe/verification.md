# Verification receipt

## Scope

- Issue: https://github.com/hyperledger-identus/sdk-rust/issues/218
- Base: `origin/develop@ee7332d9949bec6e31b2d2de6e0d46e9ece0835e`
- Branch: `codex/timezone-safe-openspec-archive`
- Component: repository-owned AI Software Factory archive facade
- Dependencies and downstream consumers: unchanged

## Pre-implementation gates

- `./scripts/factory research-ready make-openspec-archive-timezone-safe` — passed.
- `./scripts/factory constraints-ready make-openspec-archive-timezone-safe` — passed.
- Strict OpenSpec validation reported 52 passing specs/changes and zero failures.

## Focused evidence

- `git diff --check` — passed.
- `bash -n scripts/factory scripts/tests/factory-contract.sh` — passed.
- Nix-devshell `shellcheck scripts/factory scripts/tests/factory-contract.sh` — passed.
- `./scripts/tests/factory-contract.sh` — passed, including 159 support-policy
  cases, all existing archive protections, new symlink and ambiguous-result
  failures, and a successful archive whose fixture date is fixed to
  `2000-01-04` rather than the host date.
- Nix-devshell `./scripts/factory check` — passed with 52 OpenSpec items.

## Complete local evidence

- `/nix/var/nix/profiles/default/bin/nix flake check --print-build-logs` —
  passed all 29 local `aarch64-darwin` checks, including factory contract,
  text/shell lint, TOML/Nix lint, Rust formatting, build, strict Clippy,
  documentation, dependency policy, WASM/Android/iOS compilation and nextest.
- Workspace nextest: 666 passed, 22 skipped.
- `identus-crypto` KMP feature nextest: 131 passed, 1 skipped.
- Entropy feature lanes: 3 deterministic, 1 getrandom and 4 all-feature tests
  passed.
- The audit derivation completed under the repository's offline/yank policy;
  its known offline index lookup diagnostics did not fail the check.
- Linux hosted `fast` CI is not run locally and remains the merge gate.

## Review

A fresh exact-diff pass checked Bash 3.2 compatibility, quoted path handling,
pre/post snapshot identity, UTC-only early collision optimization, date-rollover
behavior, zero/multiple/new-symlink failure modes, artifact validation order,
test cleanup and the OpenSpec intentional-replacement hash. The first real
self-archive revealed that Bash 3.2 treats a declared empty local array as
unset during `"${array[@]}"` expansion under `set -u`; OpenSpec had completed
correctly, so the valid archive was preserved and the membership loop was
guarded with the nounset-safe `${array[0]+set}` test. Focused and full factory
evidence was rerun after that correction. No unresolved correctness, security,
documentation or scope finding remains.

## Isolation and rollback

No donor or consumer repository was inspected or mutated for this
repository-local change. Rollback is a focused revert and requires no data or
consumer migration.

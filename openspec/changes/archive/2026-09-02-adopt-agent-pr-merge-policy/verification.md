# Verification

## Candidate

- Repository: `hyperledger-identus/sdk-rust`
- Worktree: `sdk-rust-worktrees/agent-pr-merge-policy`
- Branch: `codex/agent-pr-merge-policy`
- Implementation base: `origin/develop@6399b880d5304e8d139ebbcb43d744b827731311`
- Corresponding issue: #16
- Rust public API, wire format and crate dependency impact: none
- Consumer repositories changed: no
- `main` changed: no

The branch is rebased onto the current `origin/develop` immediately before
publication; the pull request records the final head and merge base.

## Commands passed

- `scripts/tests/pr-policy.sh`
- `scripts/tests/factory-contract.sh`
- `scripts/check-factory.sh .`
- `./scripts/factory validate adopt-agent-pr-merge-policy`
- `./scripts/factory check` — 12 OpenSpec items passed
- ShellCheck for the changed factory and PR-policy scripts
- Actionlint for all repository workflows
- Markdownlint for repository documentation
- Yamllint for the workflow and issue-form changes
- `git diff --check`
- `nix flake check --print-build-logs` on `aarch64-darwin`
  - factory contract and PR-policy fixtures passed hermetically;
  - formatting, clippy, documentation, WASM, dependency and text gates passed;
  - default and KMP-compatible Nextest matrices each passed 104/104 tests.

## GitHub issue-object probe

- Existing issue #16 resolved to the repository `/issues/16` URL.
- Pull request #15 resolved to `/pull/15` and was rejected by the exact issue
  URL comparison used in the workflow.

## Non-blocking baseline diagnostics

- The existing hermetic cargo-audit derivation reports that its offline index
  cannot answer yanked-package lookups, then returns success. This predates the
  policy change and remains separate hardening work already recorded by the
  NeoPRISM toolchain alignment.
- The nixpkgs macOS fixup hook emitted the known non-fatal
  `audit-tmpdir.sh` segmentation warning while the lint derivation completed.
- `nix flake check` evaluated the local `aarch64-darwin` checks and reported
  `x86_64-linux` as omitted for the incompatible local system; CI supplies the
  Linux gate.

## Local review

The distinct review pass is recorded in `review.md`. All findings were resolved
before readiness and no blocking finding remains.

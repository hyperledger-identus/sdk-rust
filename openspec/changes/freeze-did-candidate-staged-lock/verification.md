# Verification

Verification date: 2026-09-29
Develop base: `7c7c8e9a02bebd65d63c00631208283778f12fe4`
Specification commit: `c39b0c8e2c597094d7a97b07064f5bc4d849d627`
Preimplementation receipt: `7752dbd8f4cdacbf77b11130ae58a22142879993`
Implementation head: `6acbe141705eba95ade1b09e1c9870f4fef08402`
Environment: aarch64-darwin with repository-pinned Nix/Rust tools

## Frozen candidate evidence

- `nix run .#did-candidate`: passed from clean implementation commit
  `a0fc4e2ba69110cbdefcb62d75448c0ba5396cd8`. Both package passes were
  byte-identical. The receipt binds `docs/release/did-candidate.lock` at
  SHA-256 `1f1d4206e2ced5bd74675d654876684536dd8f82bf79cd6de4c2fa67db904447`
  and records Rust/Cargo 1.98.1 plus pinned public-API, semver, and SBOM tools.
- `nix run .#did-candidate-matrix-primary`: passed eight native/iOS rows from
  the same clean commit on Rust 1.98.1; its lock digest is the descriptor hash.
- `nix run .#did-candidate-matrix-msrv`: passed eight native/iOS rows from the
  same clean commit on Rust 1.89.0; its lock digest is the descriptor hash.
- Candidate and matrix receipts all record `sourceDirty: false`. Evidence is
  retained outside the checkout and is not committed.

## Focused and repository evidence

- `python3 scripts/check-release-candidates.py`: passed the closed crypto/DID
  release-candidate contract.
- `python3 scripts/tests/release-candidates.py`: passed missing, byte-drift,
  stale-version, Git-source, and lane-local-resolution mutation cases.
- `./scripts/factory check`: passed factory and strict OpenSpec readiness.
- `nix build .#checks.aarch64-darwin.lint-text --no-link`: passed generated
  lockfile policy, Markdown, YAML, EditorConfig, and ShellCheck validation.
- `nix flake check`: all 40 compatible local checks passed, including 911
  workspace tests, primary/MSRV builds, WASM/Android/iOS targets, Clippy,
  deny/audit, formatting, docs, source, and factory policy. Linux remains
  hosted evidence and is not claimed by this macOS run.
- Pinned `actionlint` reports only the documented ADR-0111 `cache-mode`
  compatibility diagnostic when invoked without its approved ignore. The full
  factory derivation applies that narrow ignore and passed all other workflow
  diagnostics.
- `git diff --check` passed. Every branch commit is signed and DCO-bearing.

## Deferred hosted evidence

Required exact-head PR CI supplies Linux repository integration. This change
does not dispatch or rerun the weekly slow workflow. Issue #388 still requires
a later natural or explicitly authorized exact-revision Linux/macOS run and
the independent approval defined by the M5 freeze procedure.

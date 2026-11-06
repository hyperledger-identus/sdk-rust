# Verification receipt

Verification date: 2026-10-29
Issue: #581
Develop base: `63a78cd085f51fc82dfb2e92ccb51d2fb0263bb9`
Planning commit: `271ec871dca1aafc7e97e4b9e715c9746739a9e9`
Implementation commit: `957966c5a1afdaaf72f6a5963378970a77653798`
Reviewed implementation: `9a5b9361046fc63f9c96736670d08fde8351d1de`

## Lock and candidate evidence

- Review-only refresh proposed exact lock bytes at SHA-256
  `f7535a6819c9b9e4474c95b427b54646d46ccac51703c680086ef75c790fba4f`.
- The closed drift is six registry package updates: `equivalent` 1.0.2 to
  1.0.3, `lazy_static` 1.5.0 to 1.5.1, `serde_json` 1.0.151 to 1.0.154,
  `syn` 3.0.6 to 3.0.9, `tokio` 1.53.1 to 1.54.1, and `unicode-ident`
  1.0.26 to 1.0.27.
- Clean review-mode qualification completed deterministic two-pass archives,
  patched closure, public-API extraction, SBOM generation, license/source
  policy, and bounded receipt generation.
- Archive SHA-256 values are
  `7736e445ffce327fae19a55c7b8b7adf38887f93f56aba4a9a804d18d765fb68`
  for `identus-did` and
  `90366988dd00a4730d398431531d0066eb109ff48a1fcc8f0aed4d9673678480`
  for `identus-did-resolver-http`.
- Primary Rust 1.98.1 and MSRV Rust 1.89.0 candidate matrix lanes passed.
- Normal qualification now passes lock installation and candidate assembly,
  then stops only at additive `identus-did` public-API baseline drift. Issue
  #582 owns that separate baseline decision; the resolver HTTP baseline is
  byte-identical.

## Repository and documentation evidence

- `python3 scripts/check-release-candidates.py .`: passed.
- `python3 scripts/tests/release-candidates.py`: mutation suite passed.
- `scripts/factory check`: passed.
- `nix build .#docs-site --no-link`: passed.
- `git diff --check`: passed.
- All local commits carry verified signatures and DCO sign-off.

## Full Nix evidence

`nix flake check --print-build-logs` passed the complete host flake. Evidence
included Rust 1.98.1 and MSRV 1.89.0 builds, workspace and feature-profile
clippy, rustdoc, formatting, dependency and RustSec policy, iOS/WASM portable
builds, text/TOML linting, the complete factory mutation suite, and 1,195
workspace nextest tests (1,195 passed, 23 declared skipped).

## Independent review and successor ownership

Codex CLI 0.163.0 with `gpt-6-sol` at high reasoning reviewed the exact diff
through `9a5b9361046fc63f9c96736670d08fde8351d1de` and reported no actionable
regression. Issue #582 owns the additive API-origin rebaseline. Issue #388
retains ownership of the final exact-merge-SHA slow evidence and independent
release approval; this lock refresh does not claim either outcome.

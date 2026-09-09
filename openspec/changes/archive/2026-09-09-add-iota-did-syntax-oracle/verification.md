# Verification receipt

Verification status: passed locally
Verification date: 2026-09-09
Base: `origin/develop@eec038eb7397fb1209a136af6e6f0aaed3f39382`

## Focused fixture evidence

- `./scripts/check-iota-did-syntax-oracle.sh`: 2/2 host tests and strict
  all-target Clippy passed on exact Rust 1.98.1.
- The 30-case corpus records 23 exact intersections/shared rejections and seven
  classified syntax, resource-ceiling or representation divergences.
- The root manifest and lock contain no IOTA Identity packages. The isolated
  all-target fixture resolves 188 packages; the host normal/build tree has 159
  distinct package/version pairs.
- iOS aarch64 and Android aarch64 compile-only checks passed. The WASM check
  failed as expected at `getrandom 0.2.17` without its `js` feature; no target
  support claim is activated.
- `cargo deny` and `cargo audit --deny warnings` rejected the fixture lock as
  decision evidence: four unmaintained dependencies and the `atty 0.2.14`
  unaligned-read unsoundness advisory are reachable.
- Candidate-owned unsafe was found in `identity_core 1.5.1` and
  `did_url_parser 0.3.0`; first-party fixture code forbids unsafe.

## Repository gates

- `./scripts/factory check`: 57 OpenSpec/spec items passed.
- `./scripts/factory research-ready add-iota-did-syntax-oracle`: passed.
- `./scripts/factory constraints-ready add-iota-did-syntax-oracle`: passed.
- `git diff --check`: passed.
- `nix flake check --print-build-logs`: all 33 compatible aarch64-darwin checks
  passed, including 676/676 default workspace tests and 131/131 KMP-compatible
  crypto tests. Hosted Linux remains the independent merge gate.

The existing nonfatal offline yanked-index diagnostics and macOS Nix fixup
scanner warnings were observed. Neither failed a derivation.

## Exclusions

The reference fixture is not wired into CI. DID documents, credentials, IOTA
method/network behavior, runtime target validation, production adoption,
publication and downstream changes remain excluded.

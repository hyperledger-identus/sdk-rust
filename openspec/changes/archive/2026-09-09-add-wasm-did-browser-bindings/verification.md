# Verification receipt

Verification status: passed locally and in hosted browser evidence
Verification date: 2026-09-09
Base: develop@1b926b881caf996322200f6df3b059e01e0bcdaf
Implementation head: 8d605e228cc41bdee55be720d530b03bf9ce6ab1

## Browser package and runtime receipt

- Rust 1.98.1, wasm-bindgen runtime/CLI 0.2.121 and wasm-pack 0.15.0
  came from the exact workspace and pinned Nix inputs.
- `cargo test -p identus-wasm-did` passed 2/2 host contract tests and strict
  package Clippy passed.
- `cargo build --locked -p identus-wasm-did --target
  wasm32-unknown-unknown` passed.
- The separately locked browser consumer fixture passed its own `cargo deny`
  and pinned-advisory `cargo audit` checks before packaging and execution.
- Local Chrome 152.0.7977.83 with ChromeDriver 152.0.7977.65 passed both
  consumer-shaped browser test families, including JavaScript-owned result,
  error, getter, class-name and explicit-free behavior.
- Hosted Chromium 152.0.7977.64 and Firefox 154.0.1 each passed both test
  families with ChromeDriver 152.0.7977.64 and GeckoDriver 0.37.1. The exact
  implementation-head receipt is slow workflow job
  [102385054845](https://github.com/hyperledger-identus/sdk-rust/actions/runs/34326555322/job/102385054845).

## Determinism and API receipt

Two independent `wasm-pack --target web --release` builds produced
byte-identical complete package trees and matched the committed TypeScript API
snapshot.

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `identus_wasm_did.js` | 14,896 | `ad91b42cb1d2f2d7d1fd396b7a6019c5cf4d55d2bc563dee77f88a4c1ebb9c13` |
| `identus_wasm_did.d.ts` | 4,360 | `6e2b997da95670828d196bcb4c6de5b67e95650043f39273e3b4fd17223e3ae0` |
| `identus_wasm_did_bg.wasm` | 33,833 | `100b1d176093b9bfbbe87d8c7b58e5b2c761394ff0e2bc0dba002aff8baf575e` |
| `identus_wasm_did_bg.wasm.d.ts` | 1,594 | `e87404b51a37f0ddfaa3cee7bc816126e8a4e82c2d2b69b861d3094463f5adc3` |
| `package.json` | 319 | `d1c441370dc33e7febe04290a4a4e06951dd5f58e7cff9dfb43c8b492240cca3` |

These uncompressed sizes are observations, not release budgets. The runtime
normal/build dependency tree contains 35 rendered lines; the separately locked
browser consumer fixture contains 58.

## Repository receipt

- `scripts/factory check` passed research, material-constraint, OpenSpec,
  repository-policy and traceability gates.
- `nix flake check --print-build-logs` passed all 29 compatible
  aarch64-darwin checks, including Rust 1.98.1 native, all-feature,
  minimal-feature, KMP-compatible, WASM, iOS and Android builds; strict Clippy;
  rustdoc; formatting; policy; dependency/license/advisory gates; and release
  Nextest profiles.
- Nix reported x86_64-linux as locally incompatible; hosted Linux PR CI remains
  the independent merge authority.
- Exact-diff whitespace checks passed. Commits are GPG-signed and include DCO
  trailers.

## Exclusions

Node, React Native, downstream React bundlers, npm publication, physical
devices, release signing, production browser-version matrices, storage,
network, secrets, workers and threads remain unrun and out of scope.

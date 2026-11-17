# Verification

- **Date:** 2026-11-18
- **Issue:** #599
- **Develop base:** `6c0fb15a1a91dbcb7e08c0b471d7c5c1aa0624dd`
- **Verified implementation head:** `d1ae72d78ed96e25a062f5755528307dd6cabc47`
- **Environment:** aarch64-darwin, repository-pinned Rust 1.99.1, Node
  24.19.0, wasm-pack 0.15.0, wasm-bindgen 0.2.121, and fuzz nightly
  2026-10-15

## Focused Rust and WASM gates

- `cargo test -p identus-wasm-did`: 13 passed.
- `cargo clippy -p identus-wasm-did --all-targets --all-features -- -D warnings`:
  passed.
- `cargo fmt --all -- --check`: passed.
- The checked-in TypeScript declaration matched generated API v3 output.

## TypeScript target and differential gates

`SDK_TS_DID_HEAD_SHA=5aff2744b7637471c7b9d8a86aa2498185a8306c
nix develop .#typescript --command
./scripts/check-sdk-ts-did-target.sh differential` passed:

- package/type/provider boundary: 30 tests passed;
- browser: 22 DID, 12 DID Document, and 9 DID Resolution vectors passed;
- Node ESM/CommonJS: 2 tests passed;
- React Node SSR and browser hydration: passed;
- React Native: explicit `inapplicable` receipt;
- deterministic assembly SHA-256:
  `5de315302dacc56208b4026ba788f08bdba22931354aa21e451a430e5233c96e`.

Every selected target receipt binds the exact source head and succeeded. The
package, browser, Node, and React artifact digests remained stable across the
remediation.

## Fuzz and resource evidence

`nix develop .#fuzz --command ./scripts/fuzz-did.sh smoke did_resolution`
completed 4,096 deterministic libFuzzer executions with seed `424242`,
`max_len=524288`, timeout 5 seconds, RSS limit 1,024 MiB, and address sanitizer.
It found no crash; peak observed RSS was 353 MiB. Checked-in seeds plus generated
exact/over-limit input covered success, deactivation, dereferencing, duplicate
names, and arbitrary mutated branches.

`nix develop .#typescript --command
./scripts/check-sdk-ts-did-target.sh measurements` completed the directional
Node measurement harness with 101 samples after 21 warm-ups per operation.
It recorded web artifact size 377,545 bytes, Node artifact size 370,084 bytes,
and the deterministic assembly digest above. The output remains
`target-not-supported` because no regression/support threshold has been
accepted.

## Factory and evidence gates

- `./scripts/factory validate add-sdk-ts-did-resolution`: passed.
- `./scripts/factory check`: 127 OpenSpec items passed, 0 failed.
- `python3 scripts/check-support-policy.py`: passed.
- `python3 scripts/check-quality-evidence-declarations.py .`: 23 declarations
  and 92 obligations passed.
- `python3 scripts/tests/quality-evidence-declarations.py`: 8 mutations passed.
- `git diff --check`: passed.
- All ten commits in `origin/develop..d1ae72d7` have valid GPG signatures and
  DCO trailers.

## Reproducible target closure

`nix flake check --print-build-logs` passed every compatible aarch64-darwin
check against the clean exact implementation head
`d1ae72d78ed96e25a062f5755528307dd6cabc47`. The closure used Cargo/Rust
1.99.1 and included MSRV/primary/etalon builds, feature variants, strict
clippy, formatting, documentation, dependency policy, platform closures,
text/TOML linting, and 1,206 passing workspace tests. Nix reported
`x86_64-linux` as an incompatible local system; the hosted fast lane remains
responsible for that platform.

## Repository and release boundary

No downstream repository was mutated. No package was published, no tag or
release was created, and no stable API or React Native support claim is made.

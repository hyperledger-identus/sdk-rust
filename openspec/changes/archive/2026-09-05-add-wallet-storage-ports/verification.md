# Verification receipt

## Scope

- Issue: #89
- Backlog: IDR-010a under IDR-010
- Base: `5141044384e51cc26a73a19adb11d0f18f70f74a`
- Branch: `codex/idr-010a-storage-ports`
- Owner: `identus-wallet`

## Preflight repository isolation

| Repository | Revision | Existing state preserved |
| --- | --- | --- |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | `?? .claude/`, `?? .pi/taskflows/`; integration upstream gone |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | dirty nested `third_party/midnight-did` |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | `?? .pi-subagents/`, `?? .pi/`, `?? tmp/` |

No consumer edit is authorized. Postflight must match these receipts exactly.

## Provenance

No code or fixture is copied. Issue #89 records the exact conceptual evidence
paths, SHA-256 digests and available Apache-2.0 roots. Lace is evidence-only
because repository-level licensing remains unresolved.

## Commands

Implementation, focused, workspace, factory, target, Nix, diagnostic and
review evidence will be appended after each result is observed. Unrun commands
remain unclaimed.

## Focused implementation evidence

- `cargo test -p identus-wallet --all-targets`: 9 passed, 1 ignored.
- `cargo clippy -p identus-wallet --all-targets -- -D warnings`: passed.
- `cargo test -p identus-wallet --release --test storage -- --ignored
  --nocapture`: 1,000,000 calls across all five dynamic ports in 69.971 ms,
  approximately 14,291,635 calls/s; diagnostic only, with no threshold.
- `scripts/check-bootstrap-inventory.py`: 14 packages passed.
- `scripts/factory check`: 23 OpenSpec/factory items passed.
- `cargo test -p identus-conformance naming_guard_passes -- --nocapture`:
  passed and discovered the explicit port declarations.
- `git diff --check`: passed.

## Postflight repository isolation

Postflight revisions, status entries and all recorded source SHA-256 digests
match the issue #89 preflight exactly. Oxid remains at `bfe3b481` with its two
pre-existing untracked paths; midnight-identity remains at `427f8571` with its
dirty nested `third_party/midnight-did`; Lace ID Portal remains at `804de0a9`
with its three pre-existing untracked paths. No consumer file was modified.

## Full reproducible matrix

- `cargo fmt --all -- --check`: passed.
- `cargo test -p identus-wallet --no-default-features`: passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc -p identus-wallet --no-deps`: passed.
- `cargo test --workspace --all-features`: passed.
- `cargo test --workspace --no-default-features`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: passed.
- `nix flake check --print-build-logs`: all 30 compatible aarch64-darwin
  checks passed, including Rust 1.85 MSRV, native, Android AArch64, iOS
  AArch64, WASM, feature, lint, documentation, factory, dependency, license,
  advisory and release Nextest lanes. The principal suite ran 356 tests: 356
  passed and 16 manual diagnostics were skipped.

Nix reported `x86_64-linux` as incompatible with the local system; hosted
Ubuntu CI supplies that independent gate. Existing nonfatal offline crates.io
yanked-lookups and macOS fixup-hook diagnostics did not fail a derivation.

## Compatibility and boundary evidence

- Specification commit:
  `cec2b2f3bd4d011a53ebfaf3d5c9ea3aad97768d`.
- Implementation commit:
  `24c383549c48357e09ad6b8dde4235096bba6245`.
- The wallet crate's direct dependency tree contains only `identus-core` and
  the `identus-derive` procedural macro. No Cargo feature or external package
  was added.
- IDR-010 remains `specified`, not delivered: production in-memory and
  encrypted consumer adapters have not yet supplied conformance receipts.
- The distinct exact-diff review is recorded in `review.md`; its two draft
  corrections were applied before the implementation commit and it has no
  unresolved finding.

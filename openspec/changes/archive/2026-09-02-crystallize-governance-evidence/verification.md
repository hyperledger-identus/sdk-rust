# Verification evidence

Verified on 2026-09-03 from `codex/idr-001-governance-evidence`, based on
`origin/develop@1d890fdc61b68ca6cd3c53dbfb39e3c03bb2ed1f`.

## Contract evidence

| Evidence | Result |
| --- | --- |
| GitHub coordination | Delivery issue #25, protected follow-up #26 and reconciled parent #4 existed before implementation |
| Workspace coverage | 14 packages: five implemented, one verification-only and eight quarantined placeholders |
| Publication | Every member resolves to Cargo `publish = false`; `identus-core` dry-run publication is rejected |
| Dependency reduction | 20 unused speculative internal edges removed from seven placeholder manifests |
| Local drift suite | 20 positive/adversarial inventory tests plus the hermetic factory contract |
| Program status | `IDR-001` remains `in_progress` pending #26; namespace ownership remains #3 |

## Passing gates

- `./scripts/check-bootstrap-inventory.py` — 14 packages passed.
- `./scripts/tests/bootstrap-inventory.py` — 20/20 tests passed.
- `scripts/tests/factory-contract.sh` — backlog, support, inventory, PR and
  factory fixtures passed.
- `./scripts/factory check` — all 16 current specs and the active change
  passed before archive.
- `cargo metadata --locked --no-deps --format-version 1` — all members resolve
  to an empty publish registry list.
- `cargo build --workspace --locked` and the prior offline lock regeneration
  build passed.
- `cargo test --workspace --all-features --locked` passed.
- `cargo test --workspace --no-default-features --locked` passed.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
  passed.
- `cargo fmt --all -- --check`, `cargo doc --workspace --no-deps --locked`,
  Python bytecode compilation and `git diff --check` passed.
- `nix flake check --print-build-logs` passed all 27 compatible
  `aarch64-darwin` checks after the Markdown correction. It exercised factory,
  text/TOML/Nix hygiene, Rust 1.85 MSRV and feature lanes, strict clippy, docs,
  dependency policy, all Nextest variants and Android, iOS and browser-WASM
  release builds.

## Expected negative evidence

`cargo publish -p identus-core --dry-run --locked --offline` exited 101 with
Cargo's `package.publish must be true` refusal. This is the required
publication-denial result, not a failed quality gate.

## Iteration effort

The wall clock starts at issue #25 creation (`2026-09-02T20:32:21Z`) and
includes specification, implementation, review and full local verification
wait. It is an operational throughput measure rather than person-hours.

| Milestone | Elapsed from issue | Relative effort |
| --- | ---: | --- |
| Reviewed specification commit `308dcad` | 4 min 18 sec | Low |
| Implementation commit `24065d8` | 18 min 9 sec | Medium |
| Corrected full Nix matrix green | about 26 min | Medium |
| Distinct review complete | about 27 min | Low |

## Environment diagnostics

- Nix evaluated the local `aarch64-darwin` checks and omitted incompatible
  `x86_64-linux`; hosted CI supplies the Linux confirmation.
- The existing dependency-policy derivation emitted non-blocking unmatched
  license-allowance warnings.
- The nixpkgs macOS fixup hook emitted the known non-fatal
  `audit-tmpdir.sh` segmentation warning while affected derivations completed.
- The pre-existing hermetic advisory lane cannot fully answer online/yanked
  registry state; this change does not represent it as a fresh online audit.

## Repository boundary

Apollo, NeoPRISM, midnight-identity, Lace ID Portal and Oxid were not edited,
switched, staged, copied from or built. The SDK `main` branch, live GitHub
settings, publishing authority and release state were not changed.

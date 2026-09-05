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

# Make RustSec audit evidence authoritative

## Why

The repository's `rust-audit` Nix check currently inherits Crane's default
`--ignore yanked` argument. `cargo-audit` interprets that as an advisory ID,
not as a switch that disables yank lookup, so a hermetic build emits one
crates.io-index error per locked package while still succeeding. Separately,
older locally installed `cargo-audit` releases cannot parse current CVSS 4.0
records such as `RUSTSEC-2026-0073`. A green derivation therefore does not
currently make the advisory and yank evidence boundaries obvious.

## What changes

- Pin the Nix audit contract to `cargo-audit 0.22.2`, the reviewed release that
  supplies the official `--no-yanked` option and parses the pinned CVSS 4.0
  advisory database.
- Override Crane's default with the exact `--no-yanked` argument so the
  hermetic advisory gate performs no registry-index lookup.
- Add a deterministic Nix compatibility probe containing a valid CVSS 4.0
  advisory. An incompatible audit parser fails before its result can be
  treated as dependency evidence.
- Emit explicit, separate advisory and yank status evidence. The hermetic lane
  reports yank evidence as unavailable rather than passed.
- Add mutation-oriented tests for success, a known vulnerability, an
  incompatible parser, and unavailable yank evidence.

## Capability

### Modified capability

- `nix-tooling`: refine the RustSec audit contract and its evidence states.

## Non-goals

This change does not fetch a mutable crates.io index in Nix, waive any
advisory, add an audit suppression, replace `cargo-audit`, change the SDK
dependency graph, or claim that yank status is authoritative. A future online
or independently pinned registry-index gate requires its own issue and threat
contract.

## Delivery

Issue #401 owns the change. Planning and ADR evidence are committed before the
implementation preflight. The focused implementation then updates the Nix
gate, deterministic fixtures, tests, documentation, and Discussion #399.

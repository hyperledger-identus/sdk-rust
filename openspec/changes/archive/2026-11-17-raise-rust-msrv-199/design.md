## Context

The repository currently maintains a lower Rust 1.89 MSRV and Rust 1.98.1
primary/etalon providers. The sponsor selected Rust 1.99 as the new floor.
The same slow run that exposed the need for maintenance also found an unrelated
but blocking advisory in the isolated UniFFI generator lock.

## Decisions

### Use exact Rust 1.99.1 for every ordinary compiler class

`toolchain`, `etalonToolchain`, and `msrvToolchain` select exact stable Rust
1.99.1 from one upgraded stable-overlay pin. Gate identities remain distinct
so release receipts retain their semantic roles, but they no longer pretend to
prove two compiler versions. The sanitizer shell continues to use its existing
nightly.

### Move the candidate line instead of rewriting 0.1 history

Current machine policy moves to `0.2.0-rc.1` / `0.2.x`. Existing 0.1 release
notes, ADRs, archived receipts, and immutable candidate evidence remain
unchanged historical records. Current documentation gains an explicit migration
note rather than rewriting what older evidence proved.

### Update only canonical current statements and executable expectations

The implementation updates active Cargo/Nix policy, current support and
constraint documentation, candidate descriptors/checkers, and mutation tests.
Accepted ADRs and archived evidence are immutable; ADR 0200 supersedes their
compiler decisions prospectively.

### Patch the isolated tool lock without an advisory exception

Run Cargo's locked dependency update for `tools/uniffi-bindgen` so `rustix`
resolves to at least 1.1.6. Keep UniFFI 0.32.0 and its public generator contract
unchanged. The supply-chain checker remains fail-closed.

## Risks and mitigations

- **Consumer exclusion:** the new 0.2 line and migration note make the break
  explicit; NeoPRISM compiler evidence covers the named Rust consumer.
- **Cross-wired Nix providers:** current structural checks and mutations keep
  all three ordinary providers bound to the declared version.
- **Target artifact gap:** the complete local/hosted matrix must pass; no target
  is removed to make the upgrade green.
- **Historical evidence drift:** accepted ADRs, archives, and 0.1 receipts are
  not rewritten.
- **Advisory masking:** no ignore is added; the exact isolated lock must resolve
  a RustSec-patched `rustix`.

## Rollout

Commit this planning contract and ADR, bind the preimplementation receipt,
update policy/toolchain/lock/current docs/tests atomically, run focused and
complete local gates, complete independent review, archive the change, and
publish the base PR before the DID Document child PR.

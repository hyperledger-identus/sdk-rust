## Why

`identus-crypto` conditionally depends on `ed25519-bip32 0.4.3` for the exact
Cardano/IOG Ed25519-BIP32 V2 behavior required by Apollo parity. The dependency
is private behind SDK-owned key types, but its own `XPrv` formatting reveals all
96 private bytes, its drop path uses manual unsafe zeroing, and its manifest
activates every default `cryptoxide` algorithm.

Issue #179 defines an upstream-first hardening path so the SDK can reduce this
residual risk without reimplementing cryptographic mechanics or maintaining a
premature fork.

## What Changes

- Contribute a focused upstream patch that redacts both `Debug` and `Display`
  for `XPrv` while preserving the trait implementations for compatibility.
- Replace the manual unsafe wipe with the maintained, `no_std`-compatible
  `zeroize` primitive.
- Disable `cryptoxide` defaults and enable only the features used by this crate.
- Prove upstream vectors, `no_std`, Rust 1.81 and the supported host/portable
  compile matrix on the proposed patch.
- Record the upstream issue or pull request and defer the SDK pin update until
  a reviewed upstream release is available.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `dependency-research-readiness`: require an upstream-first remediation path
  for a conditionally adopted dependency before creating an SDK-maintained fork.

## Impact

- **Issue:** #179, follow-up to ADR 0078 and crypto parity epic #9.
- **SDK API/runtime:** unchanged; this iteration contributes upstream and
  records evidence only.
- **Upstream:** additive dependency hardening with a deliberate formatting
  behavior change from secret hex to a stable redaction marker.
- **Dependency cone:** adds `zeroize 1.8.2` to the upstream crate while removing
  unrelated compiled `cryptoxide` algorithms.
- **Rollback:** close the upstream contribution without merge and retain the
  existing SDK facade; a pinned fork remains a separately gated fallback.

# Proposal: close Apollo portable-target evidence

## Why

The Apollo parity ledger lists WASM, iOS and Android compile checks, but two
gate names have drifted from the support policy and the three receipts point to
an older revision. M2 needs one exact, reproducible closing receipt rather than
an inference from host-only CI.

## What changes

- Record the Rust 1.98.1 toolchain, portable package/feature surface, exact SDK
  revision and green manual slow-run receipt used to close M2 target evidence.
- Bind the three portable target rows to their exact support-policy gates,
  support tier, packages, features, limitations and common closing revision.
- Make the validator reject stale revisions, host-only substitutions, non-run
  links and target-policy drift.
- Render the target revision and immutable evidence link into the Discussion
  #178 report.

## Non-goals

No runtime, linking, packaging, device, browser, FFI, certification or release
support is added. Fast CI, supported targets, public APIs and consumer
repositories remain unchanged.

## Issue

Implements <https://github.com/hyperledger-identus/sdk-rust/issues/213> under
Apollo parity milestone issue #9.

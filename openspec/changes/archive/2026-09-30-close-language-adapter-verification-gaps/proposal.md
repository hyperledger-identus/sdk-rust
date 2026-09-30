# Close language-adapter verification gaps

## Why

Draft PR #509 establishes the first canonical Rust-to-language mapping
contract, but its post-remediation verification found merge-blocking gaps in
repository containment, canonical identity rendering, direction consistency,
and loss accounting. It also found that fixtures and version claims are too
narrowly implemented for the additive registry the contract promises.

Issue #510 isolates those findings after #509 exhausted its one automatic
remediation round. The correction remains a specification and repository
evidence slice; no binding or consumer behavior is implemented.

## What changes

- make every canonical evidence path repository-contained and free of symlink
  components before reading a Rust source constant;
- represent Cargo package and canonical Rust API path separately, while public
  error codes remain error identities rather than synthesized Rust symbols;
- enforce an explicit mapping/field direction compatibility matrix;
- require structured loss declarations for every lossy value or error mapping
  while retaining unsupported-value behavior as a separate concept;
- bind the single pinned SDK-TS revision to an exact-patch compatibility
  interval rather than claiming unevidenced releases;
- materialize every registry-declared canonical source in mutation fixtures;
- resolve every vector reference exactly once against the landed catalog and
  verify capability and target compatibility; and
- correct deterministic human rendering and mutation evidence accordingly.

## Capabilities

### Modified capabilities

- `language-adapter-mappings`: strengthens the unmerged v1 mapping contract
  and validator evidence before it becomes an accepted canonical capability.

## Impact

The change touches only the #509 registry, generated documentation, Python
validator/tests, and its OpenSpec evidence. It adds no dependency, Rust public
API, wire format, crate, target promise, generated binding, or consumer edit.
PR #509 remains draft and unmerged. The completed #510 branch will open one
replacement PR to protected `develop`; after that candidate passes, #509 is
closed as superseded rather than receiving another remediation push.

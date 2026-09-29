# Review evidence

## Scope reviewed

Reviewed the exact issue #470 source history from protected
`develop@3b5c91d384756fd0ef1ee36ef3e66f051f5950ba` through synchronized head
`39ee586cf11af5480a8c81e5ad6ba0181e405d9e`, including planning, receipt,
pre-change characterization, production code, protected-baseline
synchronization, public API inventories, code-health output, and local gates.

## Architecture and cohesion

- `RawProtectedHeaderVisitor` remains the Serde adapter and now owns only map
  iteration plus delegation to one closed private member vocabulary and one
  private partial-state owner.
- `ProtectedHeaderMember` is the single seven-name vocabulary.
  `RawProtectedHeaderFields` is the single owner of duplicate tracking, typed
  value collection, required-algorithm validation, exclusive key-reference
  validation, and final raw-header construction.
- `read_unique_value` consolidates the one genuine repeated typed-value
  invariant for JWK, X.509, and trust-chain members. String, bounded sequence,
  JWK, final validated-header, and error projection owners remain unchanged.
- No public parser framework, protocol coupling, generic header model,
  helper-per-field chain, or metric-only source split was introduced.

## Behavior and security

- Each name is decoded and classified before its value; unknown members remain
  rejected without value consumption. Known duplicates remain rejected before
  their repeated value is decoded.
- Typed value failures retain the same static invalid-value marker. Map
  collection completes before missing-algorithm and key-reference ambiguity
  finalization, with missing algorithm still first.
- The final `kid` / `jwk` / `x5c` priority, evidence values, and validated
  conversion are byte-for-byte unchanged.
- No caller-controlled value enters diagnostics, and no private key material,
  trust decision, algorithm allowlist, or verification behavior was added.

## Rust implementation

- The enum and collector are private, closed, stack-owned types. The generic
  helper is monomorphized and the conversion closures are non-capturing.
- The map remains a single source-order pass and owns the same `Option` values.
  No additional heap collection, clone, copied secret, unsafe, dynamic
  dispatch, synchronization, callback, lifetime widening, or I/O was added.
- Workspace tests, strict Clippy, rustdoc, source distribution, factory, public
  API, code-health, portable-target, MSRV, and canonical toolchain gates pass.

## Code-health and residual decisions

- The 67 / 13 / 31 `visit_map` signal disappears with no replacement
  classifier, collector, helper, or module signal and without threshold,
  exclusion, or generated-code changes.
- The existing `serialize` signal is deliberately outside this slice: one
  closed wire projection remains cohesive, small, and independently visible.
- All unrelated hotspot dispositions and the completed module-size program
  remain unchanged.

## Findings

No correctness, security, API, architecture, allocation, performance,
portability, or Rust-quality finding remains. Canonical baseline rebinding and
OpenSpec archive remain intentionally deferred to the protected-head closeout
PR after implementation merge.

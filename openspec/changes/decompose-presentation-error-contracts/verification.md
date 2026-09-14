# Verification receipt

## Identity and provenance

- Umbrella issue: `#271`; delivery issue: `#279`.
- Exact develop base: `105308771dbebceb473b99d9eb82b0fa0178ab09`.
- Planning-contract head: `f078fabe041b1fd6a3da000f37da96b37e5ffab9`.
- Reviewed implementation head:
  `2918cbcf3d66b531c60d34f81b90cac15aacb7aa`.
- Compiler: repository-pinned Rust `1.98.1`.
- Planning/stable golden: 48 rows, 15,892 bytes, SHA-256
  `3941cbdb1b3eedb26243b5caf1b8a11c4646c3789a3cab1415834c48d3a8ba49`.
- All commits through the reviewed head have valid local GPG signatures and
  DCO sign-offs.

## Compatibility and architecture evidence

- `cargo-public-api` compared `identus-presentations` at the exact base and
  reviewed head, omitting blanket and auto-trait implementations. It reported
  no removed, changed, or added public items. `RUSTC_BOOTSTRAP=1` was used only
  because rustdoc JSON remains an unstable compiler interface.
- `Cargo.toml`, `crates/presentations/Cargo.toml`, and `Cargo.lock` have an
  empty base/head diff. The crate still depends only on `identus-core` and
  `identus-credentials` and still declares no features.
- The public enum, constants, derives, non-exhaustive marker, root re-export,
  `From`, `Display`, `Error`, and `pub const fn to_identus_error` are unchanged.
  No public type, serialization/wire, protocol, FFI, binding, unsafe/native,
  retryability, issue #7, or issue #168 behavior changed.
- One wildcard-free private macro list routes all 48 variants and generates a
  unit-test inventory. The mappings are split into five crate-owned catalogues:
  request/query (13), candidate matching (9), disclosure selection (13),
  artifact assembly (9), and lifecycle (4). The private two-field record keeps
  the uniform `InvalidInput` kind and `presentation` capability centralized.

## Behavioral and immutable-golden evidence

- Default/`--all-features` and `--no-default-features` presentation runs each
  passed 35 tests, with two existing manual performance diagnostics ignored.
  The crate has no feature declarations, so these runs cover its effective
  default, minimal, and all-feature configurations.
- The new contract suite independently enumerates all 48 variants and public
  constants. It pins exact code, kind, capability, local display, safe public
  message, full public display, const conversion, redaction, and source-free
  behavior against the immutable exact-base fixture.
- `scripts/check-error-golden.py` validates both credentials and presentations
  with binding-specific hashes, provenance, and the planning blob read from the
  receipt's immutable `contractHeadSha`. Its mutation suite passed 47
  cases, including coordinated and single-copy drift, schema/provenance drift,
  missing/incomplete/ambiguous active and archive states, root confinement,
  and leaf/intermediate/root symlink rejection.
- The Rust tests embed only the stable crate fixture. Nix retains exactly the
  two stable error CSV suffixes and proves both planning copies are excluded.
  The synthetic factory fixture now exercises both bindings. Clean Nix source
  snapshots have no `.git` directory and therefore enforce the fixed hash,
  byte identity, and archive-wide exclusion; Git-backed local and hosted runs
  additionally enforce the receipt commit binding.

## Quality, target, and workspace evidence

- Strict package Clippy, warning-denied rustdoc, workspace formatting, Python
  compilation, `git diff --check`, OpenSpec validation, the factory structural
  check, and the full synthetic factory contract passed.
- Direct pinned-environment package checks passed for
  `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios`. These are compile checks, not runtime/device or binding
  support claims.
- The aarch64-darwin Nix workspace gate passed 711/711 tests with 22 skipped
  diagnostics. Strict all-target/all-feature Clippy, rustdoc, factory, and
  source-contract derivations also passed. Hosted Linux CI remains the
  independent Linux gate before merge.

## Maintainability measurements

Physical and nonblank source measures exclude test-only blocks from the
production total. They are review diagnostics, not a line-count budget.

| Measure | Base | Reviewed head | Result |
| --- | ---: | ---: | --- |
| Behavioral mapping decisions | 48 | 48 | unchanged; no false deduplication claim |
| Central `error.rs` physical lines | 417 | 298 | 28.5% smaller |
| Router review unit | 190 | 50 | 73.7% smaller |
| Largest domain catalogue | n/a | 55 | below the 60-line target |
| Relevant production physical lines | 417 | 538 | +121 lines |
| Relevant production nonblank lines | 406 | 513 | +107 lines |

The total increase is intentional: explicit domain ownership and named static
contracts improve review locality without hiding or duplicating the 48 actual
behavior decisions. This second crate still does not justify a shared runtime
error framework.

## Review and residual boundaries

Independent exact-head architecture/API/security review is clear. It verified
all 48 mappings, private ownership, unchanged manifests and public API, exact
redaction/source behavior, target evidence, and the disclosed line tradeoff.
Adversarial review then found a coordinated golden re-authorization path,
order-insensitive test comparison, missing `From` bridge coverage, and a
vacuous post-archive Nix assertion. Hardening binds the oracle to the receipt's
Git blob, rejects the coordinated mutation, checks enum order and both public
bridges, and excludes matching planning goldens from all active/archive paths.

This slice does not add a presentation-exchange feature, format, protocol,
error, dependency, wire schema, structured metadata, localization, binding,
consumer adoption, runtime/device support, or rule for data-bearing errors.
JOSE and OID4VCI remain separate issue-linked decisions under #280 and #277.

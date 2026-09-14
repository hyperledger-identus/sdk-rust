# Verification receipt

## Identity and provenance

- Umbrella issue: `#271`; delivery issue: `#277`.
- Exact develop base: `6217384f85ff72003a7b94482bf7879f4587be02`.
- Planning-contract head: `e0cd898dc3bf541d8af93155e6d0b2a5f85e864f`.
- Durable preimplementation-receipt commit:
  `3ef0bb76b4cdb4753280ecf121de410ff6198922`.
- Independently reviewed implementation head:
  `2c159721ae423129a04fcee85f5d126babffed90`.
- Compiler: repository-pinned Rust `1.98.1`.
- Planning/stable golden: 171 rows, 59,380 bytes, 175 LF lines,
  SHA-256
  `2c9e03381744b11902eb8b8bbc08934374781fad99d6561573cfcd62490bcd39`.
- Every slice commit through the reviewed implementation head has a valid GPG
  signature and DCO sign-off.

## Compatibility and architecture evidence

- `cargo-public-api 0.52.0` generated simplified `identus-oid4vci`
  inventories at the exact base and implementation head with
  `nightly-2026-09-02`; both had 1,656 lines and the comparison was empty.
  Nightly was used only for unstable rustdoc JSON. Product builds remain on
  pinned stable Rust 1.98.1.
- `Cargo.toml`, `crates/oid4vci/Cargo.toml`, `Cargo.lock`, features,
  dependencies, workflows, and canonical `oid4vci-*` specifications have an
  empty base/head diff.
- The public enum/order/discriminants, constants/paths, derives,
  `#[non_exhaustive]`, root re-exports, `CAPABILITY`, `From`, `Display`,
  `Error`, and `pub const fn to_identus_error` remain exact. Typed wire error
  models and Serde behavior are unchanged.
- One wildcard-free private macro list routes all 171 variants and produces
  only a test inventory. Six crate-owned catalogues contain exactly
  12/21/39/34/36/29 records. The private record contains only code, kind, and
  one static message; capability remains centralized.
- The result changes no protocol parsing, limits, transport, metadata, token,
  credential, nonce, deferred/immediate issuance, dependency, feature,
  consumer, #7, or #168 behavior.

## Behavioral and immutable-golden evidence

- All-feature and no-default-feature OID4VCI runs each passed the 191 existing
  tests plus one new unit and four new contract tests; the one existing
  release-mode throughput diagnostic remained ignored.
- The new suite independently enumerates all 171 variant/constant pairs and
  pins enum order, exact code, 167 `InvalidInput` plus four `Unsupported`
  kinds, capability, local/public/full display, `From`, const conversion,
  `Debug`, redaction, and both source-free results.
- All 21 previously unnamed issuer/authorization-server metadata variants now
  have explicit characterization.
- Stable and planning fixtures are byte-identical. The checker validates four
  exact hashes and reads the planning blob from the receipt's immutable
  `contractHeadSha` after validating schema, identity, timestamp/gates,
  base-to-contract and contract-to-head ancestry, and the planning-only diff
  boundary.
- The 101-case mutation suite covers all four bindings, strict integer receipt
  identity, missing/incomplete/ambiguous active/archive state, hash/provenance
  and schema drift, path escape, symlinked components, single-copy drift, and
  coordinated both-copy drift with receipt retargeting.
- Nix retains exactly four stable fixture suffixes and excludes every planning
  golden name across active and archived paths. Clean-source derivations
  enforce the stable inclusion and planning exclusion.

## Maintainability evidence

- Behavioral decisions remain 171 to 171, mapping sites remain one to one,
  and wildcard defaults remain zero to zero. This is cohesion and review
  locality work, not decision deduplication.
- The largest ownership catalogue contains 39 records. The former 860-line
  public bridge is now a small delegator; the explicit variant-to-record
  router remains centralized and exhaustive.
- `error.rs` moved from 1,381/1,371 to 721/707 physical/nonblank lines.
- The complete error-contract production surface moved from 1,381/1,371 to
  1,845/1,642 physical/nonblank lines: +464/+271.
- Whole-crate production moved from 8,526/7,852 to 8,991/8,124
  physical/nonblank lines: +465/+272. The growth is the disclosed cost of
  independently owned catalogues and compile-time evidence; no compression
  claim is made.

## Quality, target, and workspace evidence

- Strict package Clippy in all-feature and no-default modes, warning-denied
  rustdoc, workspace formatting, shell syntax, `git diff --check`, OpenSpec
  validation, factory structure, the exact golden checker, and the synthetic
  factory contract passed.
- Direct package checks passed for `wasm32-unknown-unknown`,
  `aarch64-linux-android`, and `aarch64-apple-ios`.
- The authoritative Nix WASM, Android AArch64, and iOS AArch64 release-build
  derivations passed. These results are compilation evidence only, not
  runtime, device, packaging, FFI, or binding support claims.
- The aarch64-darwin Nix workspace gate passed 721/721 tests with 22 skipped
  diagnostics. Workspace all-target/all-feature strict Clippy, rustdoc,
  factory-contract, and rust-source-contract derivations passed.
- Independent architecture/API/security review reported no actionable
  findings. Hosted Linux CI remains required before merge.

## Rollback

A focused revert restores the explicit match and removes only private
catalogues plus their test/factory evidence. There is no persisted data, wire,
release, consumer, or migration action in either direction.

## 1. Planning and readiness

- [x] 1.1 Inspect issue #270, exact source, ADR 0110, factory contracts, Nix
  tool availability, and named hotspots.
- [x] 1.2 Define population semantics, finite dispositions, touched-scope
  ratchet, anti-gaming rules, compatibility boundaries, and rollback.
- [x] 1.3 Pass research and constraint readiness, commit this planning-only
  contract, and persist issue #270 preflight before implementation.

## 2. Audit contract and architecture

- [x] 2.1 Add ADR 0115 and effective `SDK-ARCH-004` without an arbitrary hard
  metric threshold.
- [x] 2.2 Pin the analysis engine in the default Nix shell and implement the
  deterministic syntax-aware audit/report command with focused tests.
- [x] 2.3 Check in an exact-head production/test baseline and every named
  hotspot disposition with owner and evidence.

## 3. Focused production refactor

- [x] 3.1 Add characterization evidence for cache/registry standard resolution
  failures.
- [x] 3.2 Centralize that construction behind a crate-private helper in
  `resolution.rs`; make no other production refactor.

## 4. Verification and integration

- [x] 4.1 Run focused analyzer tests, deterministic report verification, DID
  characterization tests, formatting, strict all-target/all-feature Clippy,
  workspace tests/builds, factory/OpenSpec checks, and relevant Nix gates.
- [x] 4.2 Perform and record a distinct architecture/security review against
  the exact diff with zero unresolved blockers.
- [x] 4.3 Complete verification artifacts, pass factory ready/receipt/archive,
  push a signed+DCO branch, and open a ready issue-linked PR to `develop`.

## 5. Independent review remediation

- [x] 5.1 Bind the closed report schema and cheap populations to the
  policy-pinned Git tree; add exact generated exclusions and mutation tests.
- [x] 5.2 Handle contiguous outer attributes, comma-terminated fields/variants,
  and inherited out-of-line test module trees with focused fixtures.
- [x] 5.3 Add pinned full-report regeneration to weekly CI and rerun all
  affected factory, analyzer, DID, Cargo and Nix gates before pushing.
- [x] 5.4 Remove filename-based `src/tests.rs` trust, preserve nested inline
  module context during resolution, and cover same-named source collisions.
- [x] 5.5 Exclude macro token trees from module reachability, reject custom
  `#[path]` module loading, include outer doc attributes, and close primitive
  report/config schema types with focused negative fixtures.

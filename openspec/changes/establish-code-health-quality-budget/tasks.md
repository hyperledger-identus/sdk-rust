## 1. Planning and readiness

- [x] 1.1 Inspect issue #270, exact source, ADR 0110, factory contracts, Nix
  tool availability, and named hotspots.
- [x] 1.2 Define population semantics, finite dispositions, touched-scope
  ratchet, anti-gaming rules, compatibility boundaries, and rollback.
- [ ] 1.3 Pass research and constraint readiness, commit this planning-only
  contract, and persist issue #270 preflight before implementation.

## 2. Audit contract and architecture

- [ ] 2.1 Add ADR 0115 and effective `SDK-ARCH-004` without an arbitrary hard
  metric threshold.
- [ ] 2.2 Pin the analysis engine in the default Nix shell and implement the
  deterministic syntax-aware audit/report command with focused tests.
- [ ] 2.3 Check in an exact-head production/test baseline and every named
  hotspot disposition with owner and evidence.

## 3. Focused production refactor

- [ ] 3.1 Add characterization evidence for cache/registry standard resolution
  failures.
- [ ] 3.2 Centralize that construction behind a crate-private helper in
  `resolution.rs`; make no other production refactor.

## 4. Verification and integration

- [ ] 4.1 Run focused analyzer tests, deterministic report verification, DID
  characterization tests, formatting, strict all-target/all-feature Clippy,
  workspace tests/builds, factory/OpenSpec checks, and relevant Nix gates.
- [ ] 4.2 Perform and record a distinct architecture/security review against
  the exact diff with zero unresolved blockers.
- [ ] 4.3 Complete verification artifacts, pass factory ready/receipt/archive,
  push a signed+DCO branch, and open a ready issue-linked PR to `develop`.

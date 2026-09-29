# Verification evidence

## Identity

- Issue: #438
- Exact develop base: `b454c0ec0e72e64c3208c37d43fa404465518610`
- Planning commit: `831a50ca9dfd374a3ff2799cfde60fd8ec69c4b1`
- Preimplementation receipt commit: `d1e422d`
- Reviewed implementation head so far: `8cf91531677926b31962c79c4166d4817be05d10`
- Implementation PR: #439, exact head
  `0d82014718a8604b6812c698ead323198a4f85ef`
- Protected squash commit: `e4afece895eb3814b5d9b8c945d6ba457c6ad1c1`

## Behavioral compatibility

- The pre-change focused suite passed 5 tests with one ignored diagnostic.
- A test-only closed operation enum now binds the exact 16-call successful
  transcript before production movement.
- The post-change focused suite passes 7 tests with one ignored diagnostic
  across all five public storage ports.
- Revision-only corruption proves `read-after-insert` retains
  `RevisionMismatch`, while the three preservation steps retain their existing
  `ValueMismatch` projection.
- The sorted static step-name inventory in the original crate root and the new
  exact/list owners is identical.

## Public and dependency compatibility

- `cargo-public-api 0.52.0` with the repository compiler and scoped
  `RUSTC_BOOTSTRAP=1` reports no removed, changed, or added simplified public
  item between the exact base and implementation head.
- `Cargo.toml`, the crate manifest, and `Cargo.lock` are unchanged. The crate
  still has `identus-wallet` as its only runtime dependency and no feature.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Base | Implementation head |
| --- | ---: | ---: |
| Exact scenario function SLOC | 250 | no exact function above 100 |
| Exact scenario cognitive complexity | 18 | no exact function above 15 |
| Exact scenario cyclomatic complexity | 45 | no exact function above 15 |
| Wallet-conformance over-threshold modules | 0 | 0 |
| List traversal signal | 85 / 11 / 17 | 82 / 11 / 17 |

The remaining list signal is pre-existing and truthfully retained. Exact
improvement comes from five named lifecycle phases plus private evidence, not a
forwarding wrapper, moved test, generated implementation, or waiver.

The canonical baseline cannot safely pin a feature-branch commit because
protected delivery uses squash merge. The implementation PR therefore carries
this exact-head comparison. A second issue-linked closeout PR will generate and
pin the canonical report from the protected squash commit, remove the completed
hotspot disposition, archive the OpenSpec change, and close #438.

## Protected implementation evidence

- PR #439 passed DCO, pull-request policy, every file-hygiene check, and the
  exact-head fast gate in 7m49s before guarded merge.
- The guarded squash merge produced protected
  `develop@e4afece895eb3814b5d9b8c945d6ba457c6ad1c1`.
- A canonical v2 report generated from that exact protected revision has source
  fingerprint
  `abf66d32446c22bf387df1301a3a35c3d71e405e3c762d1d2998bdd6de7175c1`,
  population projection
  `c424bf04a8116abec41c15acbdf1c611bbd5319ac447fd333306cb2bcda724e5`,
  and report digest
  `d383b78ef4161fc9a0f69156e8c6561390083ec417d407be71fce31f86e77a92`.
- The refreshed report contains no production module above 1,000 authored
  nonblank lines and no exact-scenario function signal. It truthfully retains
  the list traversal at 82 SLOC / cognitive 11 / cyclomatic 17.
- Only the completed `wallet-conformance-run-exact` disposition is removed;
  all unrelated hotspot decisions remain byte-for-byte unchanged.

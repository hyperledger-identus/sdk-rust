# Verification

## Release-history correction

The initial local hypothesis was to regenerate the rc.1 baseline. Repository
evidence rejected that approach before implementation: the crypto train is
`published` and signed tag `v0.1.0-rc.1` exists. The checked-in baseline remains
byte-identical with SHA-256
`3e2c09eddba2de063436c76c8c4901b4cf5a28e0e70c659c12958a561da98909`.

## Exact compatibility evidence

The final ordinary `nix run .#crypto-candidate -- --allow-dirty --output ...`
passed in 67.470 seconds. It emitted:

- baseline SHA-256
  `3e2c09eddba2de063436c76c8c4901b4cf5a28e0e70c659c12958a561da98909`;
- current SHA-256
  `27ab65e38c024f42a64d252fb781a29a647ebf1a93576c7c7d7f00a68653abdc`;
- classification `additive-compatible`;
- zero removed lines;
- twelve added lines: each of the four reviewed Ed25519/X25519 validation
  methods appears in three cargo-public-api re-export views.

The same complete run assembled the three archives twice, compared their
bytes, inspected packaged fixture membership, extracted the closure, built all
consumer feature profiles, ran the packaged all-feature crypto tests, generated
SBOMs, and ran SemVer/source policy. The receipt records twelve additions and
zero removals.

## Focused regression evidence

- `python3 scripts/check-crypto-candidate.py`
- `python3 scripts/tests/crypto-candidate.py`
- `python3 scripts/check-release-candidates.py`
- `python3 scripts/tests/release-candidates.py`
- `scripts/factory validate reconcile-crypto-candidate-api-origin --strict`
- published-origin initialization denial
- line-multiset addition, removal, duplicate-count, and equality cases
- baseline digest/status mutations

The first Codex CLI review found one medium issue: an incompatible API failed
before its bounded diff could survive the atomic staging directory. The repair
renders and bounds evidence before classification, publishes only the diff on
rejection, and keeps packages, publication workspace, and success receipt
absent. Unit and mutation coverage exercise the bounded encoder and the
diagnostics-only branch.

The second Codex CLI review found that rejection was correct but its retained
report could still label a mixed add/remove delta `additive-compatible`. The
classifier now gives removals precedence and records `incompatible`; the
duplicate-occurrence regression asserts that classification explicitly.

The exact archived-head Codex CLI review then found that the slow workflow
skipped artifact upload after qualification failure. The candidate artifact
upload now runs under `always()`, preserving the diagnostics-only diff for
triage, and the release-candidate mutation suite rejects removal of that
failure-path upload condition.

A disposable exact-head worktree then added one synthetic published-origin
line and updated only its local descriptor digest. Complete candidate
preparation failed as required, retained exactly
`identus-crypto.public-api-diff.json`, classified the result `incompatible`,
named the one removed line, and exposed no packages, publication workspace, or
success receipt.

`nix flake check --print-build-logs` then passed all 35 compatible
`aarch64-darwin` checks. Evidence included the 1,195-test workspace run, MSRV,
etalon, WASM, Android, iOS, documentation site and link validation, dependency
and source policy, release/factory mutation gates, and formatting/linting.

The final Codex CLI review of `origin/develop...77372b0d` reported no actionable
defects. OpenSpec archive, protected CI, guarded merge, metrics, and the fresh
exact-SHA slow run remain pending.

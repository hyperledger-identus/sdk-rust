# Verification

## Exact-head local evidence

- `scripts/factory check`: passed before archive.
- `scripts/tests/factory-contract.sh`: passed.
- support-policy mutation suite: 180 tests passed.
- weekly slow live-audit suite: eleven tests passed, including manual-rerun
  rejection, disabled-workflow history lookup and scheduled head-branch
  binding.
- `actionlint .github/workflows/nix-checks.yml`: passed.
- `git diff --check`: passed.
- Nix `aarch64-darwin` checks: `factory-contract`, `lint-nix`, `lint-text`,
  `lint-toml` and `rust-fmt` passed.
- Network-explicit pre-activation audit: failed closed because GitHub still
  reported default `main`, as required before the post-merge settings change.

## Hosted evidence

PR #288 exact-head DCO, file hygiene, pull-request policy and `fast` passed at
`4d4ff3dea2bf42e570d2746af04bb35c62632360`. The automated review's retention
finding is resolved by the follow-up commit; hosted gates must rerun at that
new exact head before merge.

## Deliberately outstanding

The manual canary and first natural weekly run cannot exist before the workflow
merges and protected `develop` becomes default. Issue #276 owns both receipts
and remains open.

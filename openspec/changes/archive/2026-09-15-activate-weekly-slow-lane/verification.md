# Verification

## Exact-head local evidence

- `scripts/factory check`: passed before archive.
- `scripts/tests/factory-contract.sh`: passed.
- support-policy mutation suite: 182 tests passed.
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

PR #288 DCO, file hygiene, pull-request policy and `fast` passed at executable
and workflow head `7cf1dc907a01a84e0def592dbccf87d9dd5082df`. All review findings through that
head are resolved. GitHub's required checks and review state remain the
authoritative merge gate for any later documentation-only reconciliation head.

## Deliberately outstanding

The manual canary and first natural weekly run cannot exist before the workflow
merges and protected `develop` becomes default. Issue #276 owns both receipts
and remains open.

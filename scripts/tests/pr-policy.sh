#!/usr/bin/env bash

set -euo pipefail

if repository_root=$(git rev-parse --show-toplevel 2>/dev/null); then
  :
else
  repository_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
fi
checker="$repository_root/scripts/check-pr-policy.sh"
valid_body=$'- Issue: #16\n- Local review: passed by a fresh review pass\n- Constraint impact: routine\n- Limitations: none\n- Compatibility disposition: behavior-neutral\n- Change-ledger IDs: none\n- OpenSpec `compatibility-impact.toml`: present'
output_file=$(mktemp)
private_tmp=$(mktemp -d)
trap 'rm -f "$output_file"; rmdir "$private_tmp"' EXIT

assert_rejected() {
  local description=$1
  shift

  if "$@" >/dev/null 2>&1; then
    printf 'pr-policy test: accepted %s\n' "$description" >&2
    exit 1
  fi
}

GITHUB_OUTPUT="$output_file" PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY="$valid_body" "$checker" >/dev/null
grep -qx 'issue-number=16' "$output_file"

printf -v long_filler '%*s' 60000 ''
long_beginning_body="$valid_body"$'\n'"$long_filler"
long_end_body="$long_filler"$'\n'"$valid_body"

for long_body in "$long_beginning_body" "$long_end_body"; do
  : >"$output_file"
  TMPDIR="$private_tmp" GITHUB_OUTPUT="$output_file" \
    PR_BASE_REF=develop PR_DRAFT=false \
    PR_BODY="$long_body" "$checker" >/dev/null
  grep -qx 'issue-number=16' "$output_file"
done
if [[ -n $(find "$private_tmp" -mindepth 1 -maxdepth 1 -print -quit) ]]; then
  printf 'pr-policy test: checker retained private body material\n' >&2
  exit 1
fi

assert_rejected "a pull request targeting main" \
  env PR_BASE_REF=main PR_DRAFT=false PR_BODY="$valid_body" "$checker"
assert_rejected "a draft pull request" \
  env PR_BASE_REF=develop PR_DRAFT=true PR_BODY="$valid_body" "$checker"
assert_rejected "a pull request without an issue" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY='- Local review: passed locally' "$checker"
assert_rejected "a pull request without local review" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY=$'- Issue: #16\n- Constraint impact: routine\n- Limitations: none' "$checker"
assert_rejected "a pull request without constraint impact" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY=$'- Issue: #16\n- Local review: passed\n- Limitations: none' "$checker"
assert_rejected "a pull request without limitations" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY=$'- Issue: #16\n- Local review: passed\n- Constraint impact: routine' "$checker"
assert_rejected "an unchanged pull request template" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY=$'- Issue: <!-- Required: #123 -->\n- Local review: <!-- Required: passed -->\n- Constraint impact: <!-- Required: none/routine/material -->\n- Limitations: <!-- Required -->' \
  "$checker"

PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY=$'Closes #16\n- Local review: completed by maintainer\n- Constraint impact: none\n- Limitations: none\n- Compatibility disposition: behavior-neutral\n- Change-ledger IDs: none\n- OpenSpec compatibility-impact.toml: not applicable' \
  "$checker" >/dev/null

consumer_body=$'- Issue: #16\n- Local review: completed by maintainer\n- Constraint impact: material\n- Limitations: migration required\n- Compatibility disposition: consumer-visible\n- Change-ledger IDs: did.syntax.change.v1\n- OpenSpec `compatibility-impact.toml`: present'
PR_BASE_REF=develop PR_DRAFT=false PR_BODY="$consumer_body" "$checker" >/dev/null
windows_consumer_body=${consumer_body//$'\n'/$'\r\n'}
PR_BASE_REF=develop PR_DRAFT=false PR_BODY="$windows_consumer_body" "$checker" >/dev/null

assert_rejected "behavior-neutral work with ledger IDs" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY="${valid_body/Change-ledger IDs: none/Change-ledger IDs: did.syntax.v1}" "$checker"
assert_rejected "consumer-visible work without ledger IDs" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY="${valid_body/Compatibility disposition: behavior-neutral/Compatibility disposition: consumer-visible}" "$checker"
placeholder_body=${valid_body/Compatibility disposition: behavior-neutral/Compatibility disposition: consumer-visible}
placeholder_body=${placeholder_body/Change-ledger IDs: none/Change-ledger IDs: TBD}
assert_rejected "consumer-visible work with placeholder ledger IDs" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY="$placeholder_body" "$checker"
missing_impact_body=${valid_body/Compatibility disposition: behavior-neutral/Compatibility disposition: consumer-visible}
missing_impact_body=${missing_impact_body/Change-ledger IDs: none/Change-ledger IDs: did.syntax.change.v1}
missing_impact_body=${missing_impact_body/OpenSpec \`compatibility-impact.toml\`: present/OpenSpec compatibility-impact.toml: not applicable}
assert_rejected "consumer-visible work without an impact declaration" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY="$missing_impact_body" "$checker"

printf 'pr-policy tests: passed\n'
